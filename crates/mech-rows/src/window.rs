// SPDX-License-Identifier: MIT
//! mercs's window rows (WN1, ruling 1413): the launcher's, the shell's and
//! the game file's window and message calls, as `window`'s seam serves
//! them. The run `n1b` (WN1 step 2, the layer's rows under Wine from the
//! main menu to the game file's first frames) logged 32 lines of 7 names;
//! the other 17 names carry the plan's answer, unmeasured (wall 3): the
//! launcher made its window before the swap attached, and the game file
//! died 0.5 s after it took the window, its loop never run. The tests
//! replaying `n1b` live in `tests`.
//!
//! One window serves all three files: the launcher's, its class's
//! procedure the launcher's own, which hands each message to the loaded
//! file's `ShellWindowProc` or `SimWindowProc` (KP1's name table) and
//! falls through to `DefWindowProcA`.
//!
//! The 24 names and the files importing each (`mercswin` the launcher,
//! `mw2shell` the shell, `mercsw` the game file):
//!
//! | Name | Files |
//! | --- | --- |
//! | `AdjustWindowRect` | all three |
//! | `ClientToScreen` | mw2shell, mercsw |
//! | `CreateWindowExA` | all three |
//! | `DefWindowProcA` | all three |
//! | `DestroyWindow` | mercswin |
//! | `DispatchMessageA` | mw2shell, mercsw |
//! | `FindWindowA` | mercswin |
//! | `GetMessageTime` | mercsw |
//! | `GetSystemMetrics` | all three |
//! | `GetWindowLongA` | mw2shell, mercsw |
//! | `GetWindowRect` | mw2shell, mercsw |
//! | `LoadIconA` | mercswin |
//! | `PeekMessageA` | mw2shell, mercsw |
//! | `PostMessageA` | mw2shell |
//! | `PostQuitMessage` | mw2shell, mercsw |
//! | `RegisterClassA` | mercswin |
//! | `ScreenToClient` | mw2shell, mercsw |
//! | `SendMessageA` | mw2shell, mercsw |
//! | `SetFocus` | mw2shell |
//! | `SetWindowLongA` | mw2shell, mercsw |
//! | `SetWindowPos` | mw2shell, mercsw |
//! | `ShowWindow` | all three |
//! | `TranslateMessage` | mw2shell, mercsw |
//! | `WaitMessage` | mw2shell, mercsw |
use window::table::{Calls, Row};

#[cfg(test)]
mod tests;

/// Each call's name, the layer's spelling.
const CALLS: Calls = Calls {
    register_class: "USER32!RegisterClassA",
    create_window: "USER32!CreateWindowExA",
    destroy_window: "USER32!DestroyWindow",
    find_window: "USER32!FindWindowA",
    load_icon: "USER32!LoadIconA",
    show_window: "USER32!ShowWindow",
    set_window_pos: "USER32!SetWindowPos",
    adjust_window_rect: "USER32!AdjustWindowRect",
    get_window_rect: "USER32!GetWindowRect",
    get_window_long: "USER32!GetWindowLongA",
    set_window_long: "USER32!SetWindowLongA",
    client_to_screen: "USER32!ClientToScreen",
    screen_to_client: "USER32!ScreenToClient",
    set_focus: "USER32!SetFocus",
    get_system_metrics: "USER32!GetSystemMetrics",
    def_window_proc: "USER32!DefWindowProcA",
    peek_message: "USER32!PeekMessageA",
    wait_message: "USER32!WaitMessage",
    translate_message: "USER32!TranslateMessage",
    dispatch_message: "USER32!DispatchMessageA",
    send_message: "USER32!SendMessageA",
    post_message: "USER32!PostMessageA",
    post_quit_message: "USER32!PostQuitMessage",
    get_message_time: "USER32!GetMessageTime",
};

/// The metrics past the screen's size, Windows 95's standard scheme:
/// `SM_CYCAPTION` (4) 19, `SM_CXBORDER` (5) 1, `SM_CXDLGFRAME` (7) 3,
/// `SM_CYMENU` (15) 19, `SM_CXFRAME` (32) 4. `n1b` measured 15 alone,
/// `0x13` to the shell's `ShellWindowProc` (`f=10014570`, 2 `call.ret`
/// lines); the other 4 are the plan's answer, which `AdjustWindowRect`
/// reads and no run reached (wall 3).
const METRICS: &[(i32, i32)] = &[(4, 19), (5, 1), (7, 3), (15, 0x13), (32, 4)];

/// The launcher, `mercswin`.
pub const LAUNCHER: Row = Row {
    file: "mercswin",
    calls: &CALLS,
    metrics: METRICS,
};

/// The shell, `mw2shell`.
pub const SHELL: Row = Row {
    file: "mw2shell",
    ..LAUNCHER
};

/// The game file, `mercsw`.
pub const GAME: Row = Row {
    file: "mercsw",
    ..LAUNCHER
};

/// Every file the window seam serves.
pub const ROWS: &[&Row] = &[&LAUNCHER, &SHELL, &GAME];
