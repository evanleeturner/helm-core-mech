// SPDX-License-Identifier: MIT
//! Falsifier 4, the pointer: under a lock a motion moves the kept point by
//! itself, the clip holds it, and the stick at that point is the game
//! file's; unlocked, the point follows the host's pointer. The client is
//! the game's, 640 by 480 at the screen's origin (`n1b`'s window, no frame
//! drawn), so a client point is its screen point. No run has reached play
//! (wall 1): each expected stick is worked from the machine code's `shl
//! 0x10; cdq; idiv` (`s15-stick-idiv.txt`), never harvested.
use window::{Event, Rect};
use window_input::Keys;

use crate::input::stick;

/// The game's client, 640 by 480.
const CLIENT: (i32, i32) = (0x280, 0x1e0);

/// The keys as play leaves them: locked, clipped to the client, the point
/// centred as `fun_1005ed0c`'s `SetCursorPos(w / 2, h / 2)` puts it.
fn in_play() -> Keys {
    let mut keys = Keys::new(CLIENT);
    keys.clip = Some(Rect::sized(0, 0, CLIENT.0, CLIENT.1));
    keys.locked = true;
    keys.point = (CLIENT.0 / 2, CLIENT.1 / 2);
    keys
}

/// The centre reads 0 on both axes.
#[test]
fn the_centre_reads_zero() {
    assert_eq!(stick((0x140, 0xf0), CLIENT), Some([0, 0]));
}

/// The left and top edges read -`0x10000`. Divided as `u32`, as the port
/// did before `8c2f575`, x = 0 read 6645350.
#[test]
fn the_left_and_top_edges_read_minus_one() {
    assert_eq!(stick((0, 0), CLIENT), Some([-0x10000, -0x10000]));
}

/// `idiv` truncates toward 0: at 1, 1 the axes read -65262 and -65331,
/// where a floor would read -65263 and -65332.
#[test]
fn the_divide_truncates_toward_zero() {
    assert_eq!(stick((1, 1), CLIENT), Some([-65262, -65331]));
}

/// The last point inside reads just under `0x10000`.
#[test]
fn the_last_point_reads_under_one() {
    assert_eq!(stick((0x27f, 0x1df), CLIENT), Some([65262, 65331]));
}

/// A point outside the client writes nothing.
#[test]
fn a_point_outside_reads_none() {
    for point in [(0x280, 0), (-1, 0), (0, 0x1e0), (0, -1)] {
        assert_eq!(stick(point, CLIENT), None, "{point:?}");
    }
}

/// Locked, the host's pointer moves nothing; a motion moves the kept
/// point by itself, and the stick reads that point.
#[test]
fn a_locked_motion_moves_the_kept_point() {
    let mut keys = in_play();
    keys.apply(Event::Pointer {
        window: 0,
        x: 10,
        y: 10,
    });
    assert_eq!(keys.point, (0x140, 0xf0));
    keys.apply(Event::Motion { dx: -40, dy: 24 });
    assert_eq!(keys.point, (0x118, 0x108));
    assert_eq!(stick(keys.point, CLIENT), Some([6553, -8192]));
}

/// The clip holds the point at the client's last column and row; the
/// stick reads the edge there.
#[test]
fn the_clip_holds_the_point() {
    let mut keys = in_play();
    keys.apply(Event::Motion {
        dx: -1000,
        dy: 1000,
    });
    assert_eq!(keys.point, (0, 0x1df));
    assert_eq!(stick(keys.point, CLIENT), Some([65262, -0x10000]));
}

/// Unlocked, the point follows the host's pointer and a motion moves
/// nothing.
#[test]
fn an_unlocked_point_follows_the_pointer() {
    let mut keys = Keys::new(CLIENT);
    keys.apply(Event::Pointer {
        window: 0,
        x: 100,
        y: 50,
    });
    keys.apply(Event::Motion { dx: 5, dy: 5 });
    assert_eq!(keys.point, (100, 50));
    assert_eq!(stick(keys.point, CLIENT), Some([-51882, -45056]));
}
