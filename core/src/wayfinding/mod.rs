//! Module "Wegfindung & Passagierführung" (ADR-039): the flow fields in use
//! and their recomputation in the background, with their part of the save
//! game, the checks of a loaded state and their contribution to the state
//! hash.
//!
//! The module reads the module "Welt & Bau" (grid and targets are the input
//! of the flow fields). It reacts to [`InternalEvent::GridChanged`] with a
//! recomputation and adopts finished results in its stage of the step.

pub mod flowfield;
pub mod gauge;

use std::sync::Arc;
use std::time::Instant;

use rayon::ThreadPool;
use serde::{Deserialize, Serialize};

use crate::adoption::{Adoption, AdoptionMode};
use crate::build::{BlockPool, Blocks, Build, SavedTarget, restore_targets, saved_targets};
use crate::geometry::MapSize;
use crate::hash::StateHasher;
use crate::internal::{InternalEvent, InternalEvents, Stage};
use crate::savegame::invalid;
use crate::text::Reason;
use crate::time::Tick;
use flowfield::{FieldInput, FieldSet};
use gauge::{GaugeReading, Gauges, Token};

/// Duration of the last adopted recomputation of the flow fields, from the
/// request to the adoption at the start of a tick (measurement tool of step
/// 2.7). Wall-clock time; it never affects the simulated state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(rename_all = "camelCase")]
pub struct RecomputeStats {
    /// Revision of the adopted flow fields.
    pub revision: u32,
    /// Milliseconds from the request to the adoption.
    pub millis: u32,
}

/// State of the module: the flow fields in use and the open recomputation.
pub struct Wayfinding {
    fields: FieldSet,
    adoption: Adoption<FieldSet>,
    /// Input of the open request, kept for save games (S-5), counted as a
    /// snapshot copy while it lives.
    pending_input: Option<(FieldInput, Token)>,
    /// Revision and time of the newest request.
    recompute_started: Option<(u32, Instant)>,
    last_recompute: Option<RecomputeStats>,
    /// Counts of the background work, for the continuous-building benchmark
    /// (ADR-041).
    gauges: Gauges,
}

impl Wayfinding {
    /// Fields of an empty world (revision 0, nothing reachable) and no
    /// open request; adoption as soon as a result is ready.
    #[must_use]
    pub fn new(build: &Build) -> Self {
        Self::with_fields(FieldSet::empty(&build.grid().snapshot(), 0))
    }

    fn with_fields(fields: FieldSet) -> Self {
        let gauges = Gauges::default();
        let fields = Self::counted(fields, &gauges);
        Self {
            fields,
            adoption: Adoption::new(AdoptionMode::WhenReady),
            pending_input: None,
            recompute_started: None,
            last_recompute: None,
            gauges,
        }
    }

    /// `fields`, counted as a flow field set if they are not yet.
    fn counted(fields: FieldSet, gauges: &Gauges) -> FieldSet {
        if fields.is_tracked() {
            fields
        } else {
            let token = gauges.field_sets.acquire();
            fields.tracked(token)
        }
    }

    /// The job of a recomputation of `input`: counted as a recomputation in
    /// flight and as a snapshot copy until it is done, and its result as a
    /// flow field set from the start of its computation.
    fn job(&self, input: FieldInput) -> impl FnOnce(u32) -> FieldSet + Send + 'static {
        let job = self.gauges.jobs.acquire();
        let snapshot = self.gauges.snapshots.acquire();
        let sets = Arc::clone(&self.gauges.field_sets);
        move |revision| {
            let _counted = (job, snapshot);
            let set = sets.acquire();
            FieldSet::compute(&input, revision).tracked(set)
        }
    }

    /// Input of a computation of the flow fields: grid and targets of
    /// `build` now.
    #[must_use]
    pub fn input(build: &Build) -> FieldInput {
        FieldInput {
            grid: build.grid().snapshot(),
            targets: build.targets().to_vec(),
        }
    }

    /// The flow fields in use.
    #[must_use]
    pub fn fields(&self) -> &FieldSet {
        &self.fields
    }

    /// Revision of the flow fields in use.
    #[must_use]
    pub fn fields_revision(&self) -> u32 {
        self.fields.revision()
    }

    /// Takes over a new set of flow fields as a whole and returns the old
    /// one, so that the caller can drop its large buffers off the tick.
    pub fn adopt_fields(&mut self, fields: FieldSet) -> FieldSet {
        let fields = Self::counted(fields, &self.gauges);
        std::mem::replace(&mut self.fields, fields)
    }

    /// Starts a session: adoption with `mode`, no open request, no
    /// measurement.
    pub(crate) fn reset(&mut self, mode: AdoptionMode) {
        self.adoption = Adoption::new(mode);
        self.pending_input = None;
        self.recompute_started = None;
        self.last_recompute = None;
    }

    /// Computes the flow fields of `build` now, on `pool`, with revision 0.
    pub(crate) fn rebuild_now(&mut self, build: &Build, pool: &ThreadPool) {
        let input = Self::input(build);
        let fields = pool.install(|| FieldSet::compute(&input, 0));
        self.fields = Self::counted(fields, &self.gauges);
    }

    /// Starts recomputing the flow fields of `build` on `background` at
    /// `tick` and returns the revision. A newer request replaces an older
    /// one.
    pub fn request(&mut self, build: &Build, background: &ThreadPool, tick: Tick) -> u32 {
        let input = Self::input(build);
        self.pending_input = Some((input.clone(), self.gauges.snapshots.acquire()));
        let job = self.job(input);
        let revision = self.adoption.request(background, tick, job);
        self.recompute_started = Some((revision, Instant::now()));
        revision
    }

    /// `true` while a recomputation runs or waits for adoption.
    #[must_use]
    pub fn is_recomputing(&self) -> bool {
        self.adoption.is_pending()
    }

    /// Duration of the last adopted recomputation in this session.
    #[must_use]
    pub fn last_recompute(&self) -> Option<RecomputeStats> {
        self.last_recompute
    }

    /// Counts of recomputations in flight, flow field sets and snapshot
    /// copies, with their peaks since the module was created (ADR-041).
    #[must_use]
    pub fn gauges(&self) -> GaugeReading {
        self.gauges.read()
    }

    /// Stage of the module in a step (ADR-039): requests a recomputation for
    /// every [`InternalEvent::GridChanged`], then adopts a result that is
    /// due. Runs in every step, also while the game is paused. Returns the
    /// revision of adopted flow fields.
    pub(crate) fn step(
        &mut self,
        build: &Build,
        events: &InternalEvents,
        background: &ThreadPool,
        tick: Tick,
    ) -> Option<u32> {
        for event in events.for_stage(Stage::Wayfinding) {
            match event {
                InternalEvent::GridChanged => {
                    self.request(build, background, tick);
                }
            }
        }
        let (revision, fields) = self.adoption.poll(tick)?;
        self.pending_input = None;
        // The old set holds large buffers; freeing them is left to a
        // background thread so that the tick is not delayed.
        let old = self.adopt_fields(fields);
        background.spawn(move || drop(old));
        if let Some((started, at)) = self.recompute_started
            && started == revision
        {
            self.last_recompute = Some(RecomputeStats {
                revision,
                millis: u32::try_from(at.elapsed().as_millis()).unwrap_or(u32::MAX),
            });
        }
        Some(revision)
    }

    /// Feeds the flow fields in use into `hasher`.
    pub fn hash_into(&self, hasher: &mut StateHasher) {
        self.fields.hash_into(hasher);
    }

    /// State of the module for a save game, taken in one tick.
    #[must_use]
    pub fn snapshot(&self) -> WayfindingSnapshot {
        WayfindingSnapshot {
            fields_revision: self.fields.revision(),
            fields: self.fields.source().clone(),
            next_revision: self.adoption.next_revision(),
            pending: self
                .adoption
                .pending()
                .zip(self.pending_input.as_ref().map(|(input, _)| input.clone()))
                .map(|((revision, due), input)| (revision, due, input)),
        }
    }

    /// Structure of a loaded save game; the flow fields are computed by
    /// [`RestoredWayfinding::into_wayfinding`].
    ///
    /// # Errors
    ///
    /// `invalidState` with `blockIndex` or `table` for a broken block table,
    /// `limitExceeded` for too many targets.
    pub(crate) fn restore(
        saved: SavedWayfinding,
        map: MapSize,
        blocks: &Blocks,
    ) -> Result<RestoredWayfinding, Reason> {
        let fields = FieldInput {
            grid: blocks.snapshot(map, &saved.fields.grid)?,
            targets: restore_targets(saved.fields.targets)?,
        };
        let pending = match saved.pending {
            Some(p) => Some((
                p.revision,
                p.due,
                FieldInput {
                    grid: blocks.snapshot(map, &p.input.grid)?,
                    targets: restore_targets(p.input.targets)?,
                },
            )),
            None => None,
        };
        Ok(RestoredWayfinding {
            fields_revision: saved.fields_revision,
            fields,
            next_revision: saved.next_revision,
            pending,
        })
    }
}

/// State of the module taken for a save game.
#[derive(Clone)]
pub struct WayfindingSnapshot {
    fields_revision: u32,
    fields: FieldInput,
    next_revision: u32,
    pending: Option<(u32, Tick, FieldInput)>,
}

impl WayfindingSnapshot {
    pub(crate) fn to_saved(&self, blocks: &mut BlockPool) -> SavedWayfinding {
        let fields = SavedInput {
            grid: blocks.table(&self.fields.grid),
            targets: saved_targets(&self.fields.targets),
        };
        let pending = self
            .pending
            .as_ref()
            .map(|(revision, due, input)| SavedPending {
                revision: *revision,
                due: *due,
                input: SavedInput {
                    grid: blocks.table(&input.grid),
                    targets: saved_targets(&input.targets),
                },
            });
        SavedWayfinding {
            fields_revision: self.fields_revision,
            fields,
            next_revision: self.next_revision,
            pending,
        }
    }
}

/// Part of the module in a save game (S-5): not the flow fields, but the
/// input of the fields in use and of an open recomputation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct SavedWayfinding {
    /// Revision of the flow fields in use.
    pub fields_revision: u32,
    /// Input of the flow fields in use (grid and targets at that revision).
    pub fields: SavedInput,
    pub next_revision: u32,
    pub pending: Option<SavedPending>,
}

/// Input of a flow field computation: a block table of the save game's
/// block store and the targets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct SavedInput {
    pub grid: Vec<u32>,
    pub targets: Vec<SavedTarget>,
}

/// A recomputation that was running when the game was saved.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct SavedPending {
    pub revision: u32,
    /// Adoption tick in the verification mode.
    pub due: Tick,
    pub input: SavedInput,
}

/// The module of a loaded save game before its flow fields are computed.
pub(crate) struct RestoredWayfinding {
    fields_revision: u32,
    fields: FieldInput,
    next_revision: u32,
    pending: Option<(u32, Tick, FieldInput)>,
}

impl RestoredWayfinding {
    /// Computes the flow fields in use on `background`.
    ///
    /// # Errors
    ///
    /// `invalidState` with `fieldsMap` if the input belongs to another map.
    pub fn into_fields(self, map: MapSize, background: &ThreadPool) -> Result<Prepared, Reason> {
        if self.fields.grid.map() != map {
            return Err(invalid("fieldsMap"));
        }
        let fields = background.install(|| FieldSet::compute(&self.fields, self.fields_revision));
        Ok(Prepared {
            wayfinding: Wayfinding::with_fields(fields),
            next_revision: self.next_revision,
            pending: self.pending,
        })
    }
}

/// The module of a loaded save game with its flow fields; the open
/// recomputation starts again with [`Prepared::resume`] once the whole
/// world is checked.
pub(crate) struct Prepared {
    wayfinding: Wayfinding,
    next_revision: u32,
    pending: Option<(u32, Tick, FieldInput)>,
}

impl Prepared {
    /// The module with adoption in `mode`; a recomputation that was running
    /// when the game was saved starts again on `background` with its
    /// revision and adoption tick.
    pub fn resume(self, mode: AdoptionMode, background: &ThreadPool) -> Wayfinding {
        let mut wayfinding = self.wayfinding;
        wayfinding.adoption = Adoption::resumed(mode, self.next_revision);
        if let Some((revision, due, input)) = self.pending {
            wayfinding.pending_input = Some((input.clone(), wayfinding.gauges.snapshots.acquire()));
            let job = wayfinding.job(input);
            wayfinding
                .adoption
                .resume_request(background, revision, due, job);
            wayfinding.recompute_started = Some((revision, Instant::now()));
        }
        wayfinding
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::build::grid::{Cell, Zone};
    use crate::build::target::Target;
    use crate::geometry::{CellPos, CellRect};
    use crate::wayfinding::flowfield::UNREACHABLE;

    fn pool(threads: usize) -> ThreadPool {
        rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build()
            .unwrap()
    }

    fn pos(x: u16, y: u16) -> CellPos {
        CellPos { x, y, floor: 0 }
    }

    /// A 40-cell corridor with a target at each end.
    fn corridor() -> Build {
        let mut build = Build::new(MapSize::new(64, 8).unwrap());
        build
            .grid_mut()
            .fill(
                CellRect {
                    floor: 0,
                    x0: 0,
                    y0: 3,
                    x1: 40,
                    y1: 4,
                },
                Cell::floor(Zone::Landside),
            )
            .unwrap();
        build
            .add_target(Target::passengers(vec![pos(0, 3)]))
            .unwrap();
        build
            .add_target(Target::passengers(vec![pos(39, 3)]))
            .unwrap();
        build
    }

    #[test]
    fn a_new_field_set_is_taken_over_as_a_whole() {
        let pool = pool(2);
        let mut build = corridor();
        let mut wayfinding = Wayfinding::new(&build);
        wayfinding.rebuild_now(&build, &pool);
        assert_eq!(wayfinding.fields_revision(), 0);
        // Wall off the west half and recompute for revision 5.
        build.grid_mut().set(pos(10, 3), Cell::VOID).unwrap();
        let input = Wayfinding::input(&build);
        let old = wayfinding.adopt_fields(pool.install(|| FieldSet::compute(&input, 5)));
        assert_eq!(old.revision(), 0);
        assert_eq!(wayfinding.fields_revision(), 5);
        let fields = wayfinding.fields();
        assert_eq!(fields.dir(1, pos(20, 3)), 0, "east end is reachable");
        assert_eq!(
            fields.dir(0, pos(20, 3)),
            UNREACHABLE,
            "west end is cut off"
        );
        assert_eq!(fields.dir(0, pos(9, 3)), 4);
        assert_eq!(fields.targets(), 2);
    }

    #[test]
    fn a_grid_change_is_recomputed_and_adopted_in_the_stage() {
        let pool = pool(2);
        let mut build = corridor();
        let mut wayfinding = Wayfinding::new(&build);
        wayfinding.reset(AdoptionMode::FixedTick { delay: 2 });
        wayfinding.rebuild_now(&build, &pool);
        let mut events = InternalEvents::default();
        build.grid_mut().set(pos(10, 3), Cell::VOID).unwrap();
        events.push(Stage::Commands, InternalEvent::GridChanged);
        assert_eq!(wayfinding.step(&build, &events, &pool, 7), None);
        assert!(wayfinding.is_recomputing());
        events.end_step();
        assert_eq!(wayfinding.step(&build, &events, &pool, 8), None);
        assert_eq!(wayfinding.step(&build, &events, &pool, 9), Some(1));
        assert_eq!(wayfinding.fields_revision(), 1);
        assert_eq!(wayfinding.last_recompute().unwrap().revision, 1);
        assert!(!wayfinding.is_recomputing());
        // A later stage's event reaches the module only in the next step.
        let mut later = InternalEvents::default();
        later.push(Stage::Persons, InternalEvent::GridChanged);
        assert_eq!(wayfinding.step(&build, &later, &pool, 10), None);
        assert!(!wayfinding.is_recomputing());
        later.end_step();
        wayfinding.step(&build, &later, &pool, 11);
        assert!(wayfinding.is_recomputing());
    }
}
