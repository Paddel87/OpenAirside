//! Construction workers of the module "Personen" (ADR-045): persons of
//! group 9 that belong to a delivery point, fetch a load of material
//! there, carry it to the nearest construction site, work it off from a
//! cell beside the footprint of the order and walk back.
//!
//! Workers follow the set of the construction workers ("Wegfindung"):
//! field [`SITES`] on the way to a site, the field of their delivery point
//! on the way back. The work they do reaches the orders of "Welt & Bau"
//! as an internal event in the next step.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::{DIAGONAL, WALK_SPEED};
use crate::build::Build;
use crate::build::access::AccessProfile;
use crate::build::orders::{MAX_DELIVERY_POINTS, MAX_WORKERS, TICKS_PER_MINUTE};
use crate::geometry::{CellPos, MapSize};
use crate::hash::StateHasher;
use crate::internal::{InternalEvent, InternalEvents, Stage};
use crate::pool::Kind;
use crate::wayfinding::flowfield::{ARRIVED, DOWN, UNREACHABLE};
use crate::wayfinding::{Fields, SITES};

/// Group of the construction workers in the state buffer S-1 (ADR-045).
pub const GROUP_WORKER: u8 = 9;
/// Work one load of material is enough for: 15 game minutes of one
/// construction worker, in ticks (ADR-045, game value until 3.6).
pub const LOAD: u32 = 15 * TICKS_PER_MINUTE;
/// Most construction workers of a world.
pub const MAX_ALL_WORKERS: usize = MAX_DELIVERY_POINTS * MAX_WORKERS as usize;

/// What a construction worker does; its number is the state of the person
/// in S-1 (ADR-045).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum WorkerState {
    /// On the way with material to a construction site (or waiting at the
    /// delivery point while there is none).
    Carrying = 0,
    /// Working on an order from a cell beside its footprint.
    Working = 1,
    /// On the way back to the delivery point.
    Returning = 2,
}

/// One construction worker; stored as it is in a save game.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub(crate) struct Worker {
    x: u16,
    y: u16,
    /// Step along the field: direction 0 … 7, `ARRIVED` or `UNREACHABLE`
    /// (workers stay on the ground floor).
    heading: u8,
    /// Share of the current step already walked, 0 … 1.
    progress: f32,
    /// A [`WorkerState`] as number.
    state: u8,
    /// Object handle of the delivery point the worker belongs to.
    delivery: u32,
    /// Handle of the order the worker works on (state `Working`).
    order: u32,
    /// Work left in the load it carries, in ticks (0 … [`LOAD`]).
    load: u32,
}

/// What a loaded construction worker is checked against.
pub(crate) struct WorkerContext {
    pub map: MapSize,
    /// Object handles of the delivery points of the world.
    pub delivery: Vec<u32>,
}

impl Kind for Worker {
    const MAX: usize = MAX_ALL_WORKERS;
    const LIMIT: &'static str = "workers";
    const CHECK: &'static str = "worker";
    type Context = WorkerContext;

    fn is_valid(&self, context: &WorkerContext) -> bool {
        let heading_ok = self.heading < 8 || matches!(self.heading, ARRIVED | UNREACHABLE);
        context.map.contains(self.pos())
            && heading_ok
            && self.state <= WorkerState::Returning as u8
            && self.load <= LOAD
            && (0.0..=1.0).contains(&self.progress)
            && context.delivery.contains(&self.delivery)
    }

    fn hash_into(&self, hasher: &mut StateHasher) {
        hasher.u32(u32::from(self.x));
        hasher.u32(u32::from(self.y));
        hasher.bytes(&[self.heading, self.state]);
        hasher.f32(self.progress);
        hasher.u32(self.delivery);
        hasher.u32(self.order);
        hasher.u32(self.load);
    }
}

impl Worker {
    pub(crate) fn pos(&self) -> CellPos {
        CellPos {
            x: self.x,
            y: self.y,
            floor: 0,
        }
    }

    /// The worker's state in S-1.
    pub(crate) fn state(&self) -> u8 {
        self.state
    }

    /// Visible position and heading, as for other persons.
    pub(crate) fn heading(&self) -> u8 {
        self.heading
    }

    pub(crate) fn progress(&self) -> f32 {
        self.progress
    }

    pub(crate) fn delivery(&self) -> u32 {
        self.delivery
    }

    /// The workers of a new delivery point: on its cell with the smallest
    /// coordinates, each with a load, deciding in the next step.
    pub(crate) fn new_at(pos: CellPos, delivery: u32) -> Self {
        Self {
            x: pos.x,
            y: pos.y,
            heading: UNREACHABLE,
            progress: 0.0,
            state: WorkerState::Carrying as u8,
            delivery,
            order: 0,
            load: LOAD,
        }
    }
}

/// Reacts to the delivery points of "Welt & Bau" (ADR-045): a new one
/// brings its construction workers, a demolished one takes them away. Runs
/// in every step, also while the game is paused.
pub(crate) fn react(workers: &mut Vec<Worker>, build: &Build, events: &InternalEvents) {
    for event in events.for_stage(Stage::Persons) {
        match event {
            InternalEvent::DeliveryPointBuilt { handle } => {
                let (Some(point), Some(object)) =
                    (build.delivery_point(handle), build.object(handle))
                else {
                    continue;
                };
                let at = CellPos {
                    x: object.x,
                    y: object.y,
                    floor: object.floor,
                };
                let room = MAX_ALL_WORKERS.saturating_sub(workers.len());
                let count = usize::from(point.workers).min(room);
                workers.extend(std::iter::repeat_n(Worker::new_at(at, handle), count));
            }
            InternalEvent::DeliveryPointRemoved { handle } => {
                workers.retain(|w| w.delivery != handle);
            }
            InternalEvent::GridChanged
            | InternalEvent::SitesChanged
            | InternalEvent::WorkDone { .. } => {}
        }
    }
}

/// Simulation step of the construction workers, in storage order: they
/// walk, work and choose their next way; the work done is laid down per
/// order, in the order of the handles.
pub(crate) fn advance(
    workers: &mut [Worker],
    build: &Build,
    fields: &Fields,
    events: &mut InternalEvents,
) {
    let mut work: BTreeMap<u32, u32> = BTreeMap::new();
    for w in workers.iter_mut() {
        step_worker(w, build, fields, &mut work);
    }
    for (order, ticks) in work {
        events.push(Stage::Persons, InternalEvent::WorkDone { order, ticks });
    }
}

fn walkable(build: &Build, pos: CellPos) -> bool {
    build
        .grid()
        .cell(pos)
        .is_some_and(|c| AccessProfile::Worker.rules().walkable(c.flags))
}

fn at_delivery(w: &Worker, build: &Build) -> bool {
    build.object(w.delivery).is_some_and(|o| {
        let r = o.rect();
        (r.x0..r.x1).contains(&w.x) && (r.y0..r.y1).contains(&w.y)
    })
}

/// Field the worker follows now, if the fields know it.
fn field_of(w: &Worker, fields: &Fields) -> Option<u8> {
    if w.state == WorkerState::Carrying as u8 {
        Some(SITES)
    } else {
        fields.delivery_field(w.delivery)
    }
}

fn step_worker(w: &mut Worker, build: &Build, fields: &Fields, work: &mut BTreeMap<u32, u32>) {
    // A cell that became impassable for workers (a wall or object was
    // finished over a passer-by): back to the delivery point.
    if !walkable(build, w.pos()) {
        if let Some(o) = build.object(w.delivery) {
            w.x = o.x;
            w.y = o.y;
        }
        w.state = WorkerState::Returning as u8;
        w.heading = UNREACHABLE;
        w.progress = 0.0;
        return;
    }
    if w.state == WorkerState::Working as u8 {
        if build.order(w.order).is_some() && w.load > 0 {
            w.load -= 1;
            *work.entry(w.order).or_default() += 1;
            if w.load == 0 {
                w.state = WorkerState::Returning as u8;
                w.heading = UNREACHABLE;
            }
        } else {
            // Done, cancelled or the load is used up: back with the rest.
            w.state = WorkerState::Returning as u8;
            w.heading = UNREACHABLE;
        }
        return;
    }
    if w.heading > DOWN {
        decide(w, build, fields);
        return;
    }
    let diagonal = matches!(w.heading, 1 | 3 | 5 | 7);
    w.progress += if diagonal {
        WALK_SPEED * DIAGONAL
    } else {
        WALK_SPEED
    };
    if w.progress >= 1.0 {
        if let Some(field) = field_of(w, fields)
            && let Some(next) = fields.workers().step_target(field, w.pos(), w.heading)
        {
            w.x = next.x;
            w.y = next.y;
        }
        w.progress = 0.0;
        decide(w, build, fields);
    }
}

/// Chooses the next step of a worker that stands or reached a cell
/// (ADR-045): at a construction site it starts working on the order there
/// or, without one, goes back with its load; at the delivery point it
/// takes a new load; without any site it waits at its delivery point.
fn decide(w: &mut Worker, build: &Build, fields: &Fields) {
    let set = fields.workers();
    let pos = w.pos();
    if w.state == WorkerState::Carrying as u8 {
        match set.dir(SITES, pos) {
            ARRIVED => {
                if let Some(order) = build.workplace_order(pos) {
                    w.state = WorkerState::Working as u8;
                    w.order = order.handle;
                    w.heading = ARRIVED;
                    return;
                }
                w.state = WorkerState::Returning as u8;
            }
            UNREACHABLE => {
                if at_delivery(w, build) {
                    w.heading = ARRIVED;
                    return;
                }
                w.state = WorkerState::Returning as u8;
            }
            d => {
                w.heading = d;
                return;
            }
        }
    }
    let Some(field) = fields.delivery_field(w.delivery) else {
        w.heading = UNREACHABLE;
        return;
    };
    match set.dir(field, pos) {
        ARRIVED => {
            w.load = LOAD;
            w.state = WorkerState::Carrying as u8;
            w.heading = ARRIVED;
        }
        d => w.heading = d,
    }
}

/// Feeds the workers into `hasher`; nothing for a world without them, so
/// that its hash stays as it was before ADR-045.
pub(crate) fn hash_into(workers: &[Worker], hasher: &mut StateHasher) {
    if workers.is_empty() {
        return;
    }
    hasher.u32(u32::try_from(workers.len()).unwrap_or(u32::MAX));
    for w in workers {
        w.hash_into(hasher);
    }
}
