//! Adoption of data computed in the background, such as flow fields
//! (ADR-010): in normal play a result is used as soon as it is ready; in the
//! verification mode it is used exactly at a fixed tick after the request,
//! so runs are bit-identical regardless of thread count and machine speed.

use std::sync::mpsc::{self, Receiver, TryRecvError};

use rayon::ThreadPool;

use crate::time::Tick;

/// When background results are taken over.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdoptionMode {
    /// As soon as the result is ready (normal play).
    WhenReady,
    /// Exactly `delay` ticks after the request; the simulation waits for the
    /// result if it is late (verification mode for tests, ADR-010).
    FixedTick {
        /// Ticks between request and adoption, at least 1.
        delay: u32,
    },
}

struct Pending<T> {
    revision: u32,
    due: Tick,
    result: Receiver<T>,
}

/// One background computation at a time; a newer request replaces an older
/// one that is still running.
pub struct Adoption<T> {
    mode: AdoptionMode,
    next_revision: u32,
    pending: Option<Pending<T>>,
}

impl<T: Send + 'static> Adoption<T> {
    /// Adoption with the given mode; the first request gets revision 1.
    #[must_use]
    pub fn new(mode: AdoptionMode) -> Self {
        Self {
            mode,
            next_revision: 1,
            pending: None,
        }
    }

    /// Starts `job` on `pool` at `tick`; the job receives the revision of
    /// the request, which is also returned.
    pub fn request<F>(&mut self, pool: &ThreadPool, tick: Tick, job: F) -> u32
    where
        F: FnOnce(u32) -> T + Send + 'static,
    {
        let revision = self.next_revision;
        self.next_revision = self.next_revision.wrapping_add(1).max(1);
        let (sender, result) = mpsc::sync_channel(1);
        pool.spawn(move || {
            // The receiver is gone if a newer request replaced this one.
            let _ = sender.send(job(revision));
        });
        let delay = match self.mode {
            AdoptionMode::WhenReady => 0,
            AdoptionMode::FixedTick { delay } => delay.max(1),
        };
        self.pending = Some(Pending {
            revision,
            due: tick.saturating_add(delay),
            result,
        });
        revision
    }

    /// Returns the result to adopt at `tick`, together with its revision.
    ///
    /// # Panics
    ///
    /// Panics if the background job panicked; the simulation cannot continue
    /// without its derived data.
    pub fn poll(&mut self, tick: Tick) -> Option<(u32, T)> {
        let pending = self.pending.as_ref()?;
        let value = match self.mode {
            AdoptionMode::WhenReady => match pending.result.try_recv() {
                Ok(value) => value,
                Err(TryRecvError::Empty) => return None,
                Err(TryRecvError::Disconnected) => panic!("background job failed"),
            },
            AdoptionMode::FixedTick { .. } => {
                if tick < pending.due {
                    return None;
                }
                pending.result.recv().expect("background job failed")
            }
        };
        let revision = pending.revision;
        self.pending = None;
        Some((revision, value))
    }

    /// `true` while a request has not been adopted.
    #[must_use]
    pub fn is_pending(&self) -> bool {
        self.pending.is_some()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Barrier};

    use super::*;

    fn pool() -> ThreadPool {
        rayon::ThreadPoolBuilder::new()
            .num_threads(2)
            .build()
            .unwrap()
    }

    #[test]
    fn fixed_tick_adopts_exactly_after_the_delay() {
        let pool = pool();
        let mut adoption = Adoption::new(AdoptionMode::FixedTick { delay: 5 });
        assert_eq!(adoption.poll(0), None);
        let revision = adoption.request(&pool, 10, |_| 42);
        assert_eq!(revision, 1);
        for tick in 10..15 {
            assert_eq!(adoption.poll(tick), None, "too early at {tick}");
        }
        assert_eq!(adoption.poll(15), Some((1, 42)));
        assert!(!adoption.is_pending());
    }

    #[test]
    fn fixed_tick_waits_for_a_late_result() {
        let pool = pool();
        let gate = Arc::new(Barrier::new(2));
        let job_gate = Arc::clone(&gate);
        let mut adoption = Adoption::new(AdoptionMode::FixedTick { delay: 0 });
        adoption.request(&pool, 0, move |_| {
            job_gate.wait();
            7
        });
        assert_eq!(adoption.poll(0), None, "delay 0 counts as 1");
        let opener = std::thread::spawn(move || gate.wait());
        assert_eq!(adoption.poll(1), Some((1, 7)));
        opener.join().unwrap();
    }

    #[test]
    fn when_ready_adopts_as_soon_as_possible_and_newer_requests_win() {
        let pool = pool();
        let gate = Arc::new(Barrier::new(2));
        let job_gate = Arc::clone(&gate);
        let mut adoption = Adoption::new(AdoptionMode::WhenReady);
        adoption.request(&pool, 0, move |_| {
            job_gate.wait();
            1
        });
        assert_eq!(adoption.poll(0), None);
        assert!(adoption.is_pending());
        let second = adoption.request(&pool, 1, |revision| revision * 10);
        gate.wait(); // let the replaced job finish
        let adopted = loop {
            if let Some(result) = adoption.poll(2) {
                break result;
            }
            std::thread::yield_now();
        };
        assert_eq!(adopted, (second, 20));
    }
}
