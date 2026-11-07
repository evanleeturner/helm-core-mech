// SPDX-License-Identifier: MIT
//! Falsifier 3, the tests harvested: `n1b`'s 32 window lines, the layer's
//! rows under Wine from the main menu to the game file's first frames
//! (`records/2026-11-04-lane-c/helm-window-wn1/logbook-n1b.log`, counted
//! in `s2-counts.txt`), played on the rows less their `from` and `tid`.
//! Wine's window `2005e` is the desk's own handle here. The procedures
//! are the original's code under Wine; here a test procedure stands in,
//! its answers the logged `DefWindowProcA` lines' (wall 5). A number here
//! no `n1b` line holds is a bug; where the front answers otherwise than
//! Wine, the test names the choice.
use core::cell::Cell;

use window::desk::Desk;
use window::front::{Create, Front};
use window::{Event, Message, Rect, Screen};

use crate::test_log::debug_lines;
use crate::window::{GAME, LAUNCHER, SHELL};

/// `n1b`'s screen, 640 by 480 (the game's `GetSystemMetrics` lines), its
/// clock set by the test.
#[derive(Default)]
struct Board {
    clock: Cell<u32>,
}

impl Screen for Board {
    fn size(&self) -> (i32, i32) {
        (0x280, 0x1e0)
    }

    fn place(&self, _: u32, _: Rect, _: bool) {}

    fn gone(&self, _: u32) {}

    fn pump(&self, _: &mut dyn FnMut(Event)) {}

    fn now(&self) -> u32 {
        self.clock.get()
    }
}

/// The launcher's window as `n1b` found it: a popup, shown
/// (`0x90000000`), its class the launcher's. Made before the lines are
/// read: the launcher made it before the swap attached.
fn launcher_window(desk: &Desk<Board>) -> u32 {
    let launcher = Front::new(&LAUNCHER, desk);
    assert_eq!(
        launcher.register_class(0, 0x0040_12b0, 0, 0, b"MercsWin"),
        0xc000
    );
    launcher.create_window(&Create {
        ex_style: 0,
        class: b"MercsWin",
        title: b"Mercenaries",
        style: 0x9000_0000,
        place: [0, 0, 0x280, 0x1e0],
        parent: 0,
        menu: 0,
        param: 0,
    })
}

/// The lines with Wine's window named as the desk's.
fn flown(lines: &[&str], window: u32) -> Vec<String> {
    let own = format!("{window:x}");
    lines
        .iter()
        .map(|line| line.replace("2005e", &own))
        .collect()
}

/// The shell's 9 lines, 06:11:09.301 to 06:12:13.392: its pump
/// dispatches the menu's mouse records and the 2 messages it posted, its
/// procedure answering 0 to each. The pump's `PeekMessageA` holds Wine's
/// address from before the swap (lead 2), so `n1b` has no line of it:
/// its lines here are left out. The mouse records are Wine's input, step
/// 2's, queued here as `n1b` dispatched them. A posted message's pointer
/// is step 2's too: the desk queues 0, 0 where Wine wrote `207`, `127`
/// and `1f4`, `199`, which no procedure is handed.
#[test]
fn the_shell_pump_replays() {
    const LINES: [&str; 9] = [
        "imp.call c=USER32!DispatchMessageA a=2005e,200,0,1270207,20d52a7e,207,127 v=0 by=window",
        "imp.call c=USER32!DispatchMessageA a=2005e,201,1,1270207,20d52ae2,207,127 v=0 by=window",
        "imp.call c=USER32!PostMessageA a=2005e,c029,c02b,0 v=1 by=window",
        "imp.call c=USER32!DispatchMessageA a=2005e,c029,c02b,0,20d52e9a,0,0 v=0 by=window",
        "imp.call c=USER32!DispatchMessageA a=2005e,202,0,1270207,20d530be,207,127 v=0 by=window",
        "imp.call c=USER32!DispatchMessageA a=2005e,200,0,19901f4,20d61ffb,1f4,199 v=0 by=window",
        "imp.call c=USER32!DispatchMessageA a=2005e,201,1,19901f4,20d62060,1f4,199 v=0 by=window",
        "imp.call c=USER32!PostMessageA a=2005e,c011,c029,0 v=1 by=window",
        "imp.call c=USER32!DispatchMessageA a=2005e,c011,c029,0,20d624d2,0,0 v=0 by=window",
    ];
    let desk = Desk::new(Board::default());
    let window = launcher_window(&desk);
    let shell = Front::new(&SHELL, &desk);
    let input = |message, w, l, time, x, y| {
        desk.state().queue.push_back(Message {
            window,
            message,
            w,
            l,
            time,
            x,
            y,
        });
    };
    let turn = || {
        let message = shell.peek_message(0, 0, 0, 1).expect("a message waits");
        assert_eq!(shell.dispatch_message(&message, |_, _| 0), 0);
    };
    let post = |message, w, time| {
        desk.screen().clock.set(time);
        assert_eq!(shell.post_message(window, message, w, 0), 1);
    };
    let lines = debug_lines(|| {
        input(0x200, 0, 0x0127_0207, 0x20d5_2a7e, 0x207, 0x127);
        turn();
        input(0x201, 1, 0x0127_0207, 0x20d5_2ae2, 0x207, 0x127);
        turn();
        post(0xc029, 0xc02b, 0x20d5_2e9a);
        turn();
        input(0x202, 0, 0x0127_0207, 0x20d5_30be, 0x207, 0x127);
        turn();
        input(0x200, 0, 0x0199_01f4, 0x20d6_1ffb, 0x1f4, 0x199);
        turn();
        input(0x201, 1, 0x0199_01f4, 0x20d6_2060, 0x1f4, 0x199);
        turn();
        post(0xc011, 0xc029, 0x20d6_24d2);
        turn();
    });
    let dispatched: Vec<String> = lines
        .into_iter()
        .filter(|line| !line.contains("PeekMessageA"))
        .collect();
    assert_eq!(dispatched, flown(&LINES, window));
}

/// The launcher's 7 lines: the shell's `SendMessageA` of `0x41e` runs
/// the launcher's procedure, whose `DefWindowProcA` line lands first, as
/// the layer logs a call when it returns (lead 3); then Wine's messages
/// as a window is placed and restyled (`0x46`, `0x83`, `0x47`, `0x7c`,
/// `0x7d`), each answered 0. The stand-in sends none of them itself (the
/// crate's walls): they are played here as the procedure's own calls.
#[test]
fn the_launcher_procedure_replays() {
    const LINES: [&str; 8] = [
        "imp.call c=USER32!DefWindowProcA a=2005e,41e,0,0 v=0 by=window",
        "imp.call c=USER32!SendMessageA a=2005e,41e,0,0 v=0 by=window",
        "imp.call c=USER32!DefWindowProcA a=2005e,46,0,31fa58 v=0 by=window",
        "imp.call c=USER32!DefWindowProcA a=2005e,83,1,31fa24 v=0 by=window",
        "imp.call c=USER32!DefWindowProcA a=2005e,47,0,31fa58 v=0 by=window",
        "imp.call c=USER32!DefWindowProcA a=2005e,7c,fffffff0,31f7e4 v=0 by=window",
        "imp.call c=USER32!DefWindowProcA a=2005e,7d,fffffff0,31f7e4 v=0 by=window",
        "imp.call c=USER32!DefWindowProcA a=2005e,46,0,31f7b4 v=0 by=window",
    ];
    let desk = Desk::new(Board::default());
    let window = launcher_window(&desk);
    let launcher = Front::new(&LAUNCHER, &desk);
    let shell = Front::new(&SHELL, &desk);
    let lines = debug_lines(|| {
        let answer = shell.send_message(window, 0x41e, 0, 0, |procedure, message| {
            assert_eq!(procedure, 0x0040_12b0);
            launcher.def_window_proc(message.window, message.message, message.w, message.l)
        });
        assert_eq!(answer, 0);
        for (message, w, l) in [
            (0x46, 0, 0x0031_fa58),
            (0x83, 1, 0x0031_fa24),
            (0x47, 0, 0x0031_fa58),
            (0x7c, 0xffff_fff0, 0x0031_f7e4),
            (0x7d, 0xffff_fff0, 0x0031_f7e4),
            (0x46, 0, 0x0031_f7b4),
        ] {
            assert_eq!(launcher.def_window_proc(window, message, w, l), 0);
        }
    });
    assert_eq!(lines, flown(&LINES, window));
}

/// The game file's 15 lines, 06:12:17.777 to 06:12:18.001: the screen's
/// size twice, the launcher's popup restyled (`0x94000000` before: the
/// popup's `WS_CLIPSIBLINGS`, wall 2 of session 5) and placed at 640 by
/// 480, the size again; `SimWindowProc` handed `0x41f` falls through,
/// `WM_ACTIVATEAPP` it answers itself; then Wine's activation messages
/// (`0x86`, `6`, `8`, `0x281`) fall through, `WM_NCACTIVATE` answering 1.
#[test]
fn the_game_takes_the_window() {
    const LINES: [&str; 15] = [
        "imp.call c=USER32!GetSystemMetrics a=0 v=280 by=window",
        "imp.call c=USER32!GetSystemMetrics a=1 v=1e0 by=window",
        "imp.call c=USER32!GetSystemMetrics a=0 v=280 by=window",
        "imp.call c=USER32!GetSystemMetrics a=1 v=1e0 by=window",
        "imp.call c=USER32!SetWindowLongA a=2005e,fffffff0,90000000 v=94000000 by=window",
        "imp.call c=USER32!SetWindowPos a=2005e,fffffffe,0,0,280,1e0,10 v=1 by=window",
        "imp.call c=USER32!GetSystemMetrics a=0 v=280 by=window",
        "imp.call c=USER32!GetSystemMetrics a=1 v=1e0 by=window",
        "imp.call c=USER32!DefWindowProcA a=2005e,41f,0,0 v=0 by=window",
        "imp.call c=USER32!SendMessageA a=2005e,41f,0,0 v=0 by=window",
        "imp.call c=USER32!SendMessageA a=2005e,1c,1,0 v=0 by=window",
        "imp.call c=USER32!DefWindowProcA a=2005e,86,0,0 v=1 by=window",
        "imp.call c=USER32!DefWindowProcA a=2005e,6,0,0 v=0 by=window",
        "imp.call c=USER32!DefWindowProcA a=2005e,8,0,0 v=0 by=window",
        "imp.call c=USER32!DefWindowProcA a=2005e,281,0,c000000f v=0 by=window",
    ];
    let desk = Desk::new(Board::default());
    let window = launcher_window(&desk);
    let game = Front::new(&GAME, &desk);
    let sim_window_proc = |_: u32, message: &Message| match message.message {
        0x1c => 0,
        _ => game.def_window_proc(message.window, message.message, message.w, message.l),
    };
    let lines = debug_lines(|| {
        for _ in 0..2 {
            assert_eq!(game.get_system_metrics(0), 0x280);
            assert_eq!(game.get_system_metrics(1), 0x1e0);
        }
        assert_eq!(game.set_window_long(window, -16, 0x9000_0000), 0x9400_0000);
        let place = Rect::sized(0, 0, 0x280, 0x1e0);
        assert_eq!(game.set_window_pos(window, 0xffff_fffe, place, 0x10), 1);
        assert_eq!(game.get_system_metrics(0), 0x280);
        assert_eq!(game.get_system_metrics(1), 0x1e0);
        assert_eq!(game.send_message(window, 0x41f, 0, 0, sim_window_proc), 0);
        assert_eq!(game.send_message(window, 0x1c, 1, 0, sim_window_proc), 0);
        for (message, l, answer) in [(0x86, 0, 1), (6, 0, 0), (8, 0, 0), (0x281, 0xc000_000f, 0)] {
            assert_eq!(game.def_window_proc(window, message, 0, l), answer);
        }
    });
    assert_eq!(lines, flown(&LINES, window));
}

/// The shell's `ShellWindowProc` (`f=10014570`) asks `SM_CYMENU` past the
/// layer, 2 `call.ret` lines answering `0x13`; the line here is the
/// front's own.
#[test]
fn the_shell_menu_bar_is_19() {
    let desk = Desk::new(Board::default());
    let shell = Front::new(&SHELL, &desk);
    let lines = debug_lines(|| assert_eq!(shell.get_system_metrics(0xf), 0x13));
    assert_eq!(
        lines,
        ["imp.call c=USER32!GetSystemMetrics a=f v=13 by=window"]
    );
}

/// The launcher's start, which no run reached (wall 3): the plan's
/// answer. Its class registered, an icon a handle, its window made, found
/// by its class, shown and gone. A popup's rectangle grows by nothing.
#[test]
fn the_launcher_start_answers_the_plan() {
    let desk = Desk::new(Board::default());
    let launcher = Front::new(&LAUNCHER, &desk);
    assert_ne!(launcher.load_icon(0x0040_0000, 1), 0);
    let window = launcher_window(&desk);
    assert_ne!(window, 0);
    assert_eq!(launcher.find_window(Some(b"MERCSWIN"), None), window);
    assert_eq!(launcher.show_window(window, 1), 1);
    let rect = Rect::sized(0, 0, 0x280, 0x1e0);
    assert_eq!(launcher.adjust_window_rect(rect, 0x9000_0000, false), rect);
    assert_eq!(launcher.destroy_window(window), 1);
    assert_eq!(launcher.find_window(Some(b"MercsWin"), None), 0);
}
