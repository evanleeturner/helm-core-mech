// SPDX-License-Identifier: MIT
//! Falsifier 4, the pointer: under a lock a motion moves the kept point by
//! itself, the clip holds it, and the stick at that point is the game
//! file's; unlocked, the point follows the host's pointer. The client is
//! the game's, 640 by 480 at the screen's origin (`n1b`'s window, no frame
//! drawn), so a client point is its screen point. No run has reached play
//! (wall 1): each expected stick is worked from the machine code's `shl
//! 0x10; cdq; idiv` (`s15-stick-idiv.txt`), never harvested.
//!
//! Falsifier 7, the rows: the shell's row answers `k3`'s polls in the
//! layer's own words (`records/2026-11-05-lane-a/helm-window-input-wi1/`,
//! `s11-counts-k3.txt`), played less their `from` and `tid`.
//!
//! Falsifier 5, the remap rows: each chord a browser keeps has one row, its
//! key bound in both tables (`s1-tables.txt`: `57`, `54`, `4e` and `74`),
//! and no row is pressed with a chord a browser keeps.
//!
//! The held bit: each file's test of a held Ctrl, Alt or Shift reads
//! `GetKeyState`'s `0x1000`, each button's `GetAsyncKeyState`'s `0x8000`
//! (`s24-held-bit.txt`).
use core::cell::RefCell;

use window::desk::Desk;
use window::{Event, Rect, Screen};
use window_input::front::Front;
use window_input::remap::{BROWSER_KEPT, Chord, read};
use window_input::{Input, Keys};

use crate::input::{GAME, REMAP, SHELL, stick};
use crate::test_log::debug_lines;

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

/// The shell's screen as `k3` drew it, the game's 640 by 480; its pump
/// hands the events a test queued.
#[derive(Default)]
struct Board {
    events: RefCell<Vec<Event>>,
}

impl Screen for Board {
    fn size(&self) -> (i32, i32) {
        CLIENT
    }

    fn place(&self, _: u32, _: Rect, _: bool) {}

    fn gone(&self, _: u32) {}

    fn pump(&self, into: &mut dyn FnMut(Event)) {
        for event in self.events.take() {
            into(event);
        }
    }

    fn now(&self) -> u32 {
        0
    }
}

/// The shell's polls at the main menu, each a line `k3` logged: the
/// pointer at the screen's centre (`GetCursorPos` from `10055c38`), the 3
/// buttons up (`10055ca1`, `10055cb4`, `10055cc7`), Alt's character none
/// (`MapVirtualKeyA` from `1005b84c`), and Ctrl, Alt and Shift as Alt's
/// first press left them (`1005b86a`, `1005b878`, `1005b886`).
#[test]
fn the_shells_row_answers_k3s_polls() {
    let desk = Desk::new(Input::new(Board::default()));
    let shell = Front::new(&SHELL, &desk);
    desk.screen().back().events.borrow_mut().extend([
        Event::Pointer {
            window: 0,
            x: 0x140,
            y: 0xf0,
        },
        Event::Key {
            window: 0,
            key: 0x12,
            down: true,
        },
    ]);
    desk.pump();
    let lines = debug_lines(|| {
        assert_eq!(shell.get_cursor_pos(), (0x140, 0xf0));
        for button in [1, 2, 4] {
            assert_eq!(shell.get_async_key_state(button), 0);
        }
        assert_eq!(shell.map_virtual_key(0x12, 2), 0);
        assert_eq!(shell.get_key_state(0x11), 0);
        assert_eq!(shell.get_key_state(0x12), 0xff81);
        assert_eq!(shell.get_key_state(0x10), 0);
    });
    assert_eq!(
        lines,
        [
            "imp.call c=USER32!GetCursorPos o=140,f0 v=1",
            "imp.call c=USER32!GetAsyncKeyState a=1 v=0",
            "imp.call c=USER32!GetAsyncKeyState a=2 v=0",
            "imp.call c=USER32!GetAsyncKeyState a=4 v=0",
            "imp.call c=USER32!MapVirtualKeyA a=12,2 v=0",
            "imp.call c=USER32!GetKeyState a=11 v=0",
            "imp.call c=USER32!GetKeyState a=12 v=ff81",
            "imp.call c=USER32!GetKeyState a=10 v=0",
        ]
    );
}

/// The mask `fun_1005b830` and `fun_10070381` OR into a queued key: Ctrl
/// `0x100`, Alt `0x400` and Shift `0x200`, each on its `GetKeyState`
/// answer's `0x1000`, the machine code's `test ah,0x10`.
fn held<S: Screen>(front: &Front<'_, S>) -> u16 {
    [(0x11, 0x100), (0x12, 0x400), (0x10, 0x200)]
        .into_iter()
        .filter(|&(key, _)| front.get_key_state(key) & 0x1000 != 0)
        .fold(0, |mask, (_, bit)| mask | bit)
}

/// The held bit: the shell and the game file test a held Ctrl, Alt or
/// Shift by bit `0x1000` of `GetKeyState` (`1005b86a`, `1005b878` and
/// `1005b886` in the shell), which the front's `0xff80` holds and its
/// toggle bit never sets; the 3 buttons' polls (`fun_10055bf0`,
/// `fun_1005ed99`) test `GetAsyncKeyState`'s `0x8000` alone. 6 calls of
/// each in the two files, every key's `0x1000` test on `GetKeyState`
/// (`s24-held-bit.txt`). The presses are the test's: play is unreached
/// (wall 1), and `k3` logged the shell's Alt alone, `ff81` and `ff80`.
#[test]
fn a_held_key_reads_the_bit_each_test_reads() {
    for row in [&SHELL, &GAME] {
        let desk = Desk::new(Input::new(Board::default()));
        let front = Front::new(row, &desk);
        let hand = |events: &[Event]| {
            desk.screen().back().events.borrow_mut().extend(events);
            desk.pump();
        };
        let key = |key, down| Event::Key {
            window: 0,
            key,
            down,
        };
        hand(&[key(0x11, true)]);
        assert_eq!(held(&front), 0x100, "{}", row.file);
        hand(&[key(0x12, true), key(0x10, true)]);
        assert_eq!(held(&front), 0x700, "{}", row.file);
        hand(&[key(0x11, false), key(0x12, false), key(0x10, false)]);
        assert_eq!(held(&front), 0, "{}", row.file);
        hand(&[Event::Button {
            window: 0,
            key: 1,
            down: true,
        }]);
        assert_eq!(
            front.get_async_key_state(1) & 0x8000,
            0x8000,
            "{}",
            row.file
        );
        assert_eq!(front.get_async_key_state(2) & 0x8000, 0, "{}", row.file);
    }
}

#[test]
fn each_browser_kept_chord_has_one_row() {
    for chord in BROWSER_KEPT {
        let rows = REMAP.iter().filter(|row| row.kept == *chord).count();
        assert_eq!(rows, 1, "{chord:?}");
    }
    assert_eq!(REMAP.len(), BROWSER_KEPT.len());
}

#[test]
fn no_row_is_pressed_with_a_browser_kept_chord() {
    for (n, row) in REMAP.iter().enumerate() {
        assert!(!BROWSER_KEPT.contains(&row.pressed), "{row:?}");
        assert!(
            REMAP[n + 1..]
                .iter()
                .all(|other| other.pressed != row.pressed),
            "{row:?}"
        );
    }
}

#[test]
fn the_page_reads_the_kept_chord_in_the_pressed_ones_place() {
    assert_eq!(read(REMAP, Chord::alt(b'W')), Chord::ctrl(b'W'));
    assert_eq!(read(REMAP, Chord::alt(b'N')), Chord::ctrl(b'N'));
    assert_eq!(read(REMAP, Chord::alt(0x74)), Chord::key(0x74));
    assert_eq!(read(REMAP, Chord::key(b'W')), Chord::key(b'W'));
    assert_eq!(read(REMAP, Chord::alt(b'A')), Chord::alt(b'A'));
}
