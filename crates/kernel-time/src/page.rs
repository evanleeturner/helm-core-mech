// SPDX-License-Identifier: MIT
//! The page's clocks: everything handed in. The frame loop moves the game
//! clock once a frame from the browser's own instant, and sets the wall
//! clock and the zone from `Date`; nothing here reads a clock, and
//! nothing blocks: `Sleep` returns at once, the wait being the frame
//! loop's yield to the browser. The host tests use it as their fixed
//! clock.
use crate::clock::Clock;
use crate::{Clocks, InForce, Zone};
use core::cell::Cell;

/// The clocks a page's owner feeds.
pub struct Page {
    clock: Cell<Clock>,
    wall: Cell<u64>,
    zone: Cell<(Zone, InForce)>,
    error: Cell<u32>,
}

impl Page {
    /// The game clock `clock`, the wall at file-time `wall`, the zone
    /// `zone` with `in_force` in force.
    #[must_use]
    pub const fn new(clock: Clock, wall: u64, zone: Zone, in_force: InForce) -> Self {
        Self {
            clock: Cell::new(clock),
            wall: Cell::new(wall),
            zone: Cell::new((zone, in_force)),
            error: Cell::new(0),
        }
    }

    fn moved(&self, step: impl FnOnce(&mut Clock)) {
        let mut clock = self.clock.get();
        step(&mut clock);
        self.clock.set(clock);
    }

    /// The owner's frame at the host's instant `host`, in microseconds.
    pub fn frame(&self, host: u64) {
        self.moved(|clock| clock.frame(host));
    }

    pub fn pause(&self) {
        self.moved(Clock::pause);
    }

    pub fn resume(&self) {
        self.moved(Clock::resume);
    }

    pub fn rebase(&self) {
        self.moved(Clock::rebase);
    }

    pub fn reset(&self) {
        self.moved(Clock::reset);
    }

    /// The wall clock, file-time ticks, UTC.
    pub fn set_wall(&self, ticks: u64) {
        self.wall.set(ticks);
    }

    pub fn set_zone(&self, zone: Zone, in_force: InForce) {
        self.zone.set((zone, in_force));
    }

    #[must_use]
    pub fn clock(&self) -> Clock {
        self.clock.get()
    }

    /// The last error kept.
    #[must_use]
    pub fn last_error(&self) -> u32 {
        self.error.get()
    }
}

impl Clocks for Page {
    fn micros(&self) -> u64 {
        self.clock.get().micros()
    }

    fn wall(&self) -> u64 {
        self.wall.get()
    }

    /// The zone handed in, whatever the instant.
    fn zone(&self, _at: u64) -> (Zone, InForce) {
        self.zone.get()
    }

    fn sleep(&self, _millis: u32) {}

    fn fail(&self, code: u32) {
        self.error.set(code);
    }
}
