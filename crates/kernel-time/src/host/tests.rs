// SPDX-License-Identifier: MIT
//! The host's clocks against the host's own `date` (wall 3): the zone's
//! bias in force against `date +%z`, the wall clock against `date +%s`.
//! The game clock is the host's instant, so a real wait moves it.
use super::Host;
use crate::date::UNIX_EPOCH;
use crate::{Clocks, InForce};
use std::process::Command;

fn date(format: &str) -> String {
    let output = Command::new("date")
        .arg(format)
        .output()
        .expect("date runs");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

#[test]
fn the_zone_in_force_is_dates() {
    let host = Host::new(u64::MAX);
    let (zone, in_force) = host.zone(host.wall());
    let east = -zone.bias_in(in_force);
    let sign = if east < 0 { '-' } else { '+' };
    let ours = format!("{sign}{:02}{:02}", east.abs() / 60, east.abs() % 60);
    assert_eq!(ours, date("+%z"), "{zone:?} {in_force:?}");
    if in_force == InForce::Unknown {
        assert_eq!(zone.daylight_date.0, [0; 8]);
    } else {
        assert_ne!(
            zone.daylight_date.0[1], 0,
            "a daylight zone names its month"
        );
    }
}

#[test]
fn the_wall_is_dates() {
    let host = Host::new(u64::MAX);
    let seconds = (host.wall() - UNIX_EPOCH) / 10_000_000;
    let theirs: u64 = date("+%s").parse().expect("seconds");
    assert!(seconds.abs_diff(theirs) <= 2, "{seconds} {theirs}");
}

#[test]
fn a_sleep_moves_the_clock() {
    let host = Host::new(u64::MAX);
    let before = host.micros();
    host.sleep(20);
    let after = host.micros();
    assert!(after - before >= 20_000, "{before} {after}");
}
