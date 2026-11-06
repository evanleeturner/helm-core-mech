// SPDX-License-Identifier: MIT
//! mercs's box and menu rows (WD1, ruling 1444): the calls as `window_dialogs`'
//! seam serves them, the shell's template numbers, and the one box procedure
//! reached, `100169f0`, in Rust from its bytes. The relay logs `r1`, `r2` and
//! `w1`, every answer Wine's `user32`, are replayed in `tests`.
//!
//! Reached: `SetMenu` by the launcher and the shell, the other 9 by the shell.
//! Call: [`movie_box`] `(calls, setting, window, message, w) -> answer`.
use core::cell::Cell;

use window_dialogs::table::{Calls, Row};

#[cfg(test)]
mod tests;

/// Each call's name, the layer's spelling.
const CALLS: Calls = Calls {
    check_dlg_button: "USER32!CheckDlgButton",
    check_menu_item: "USER32!CheckMenuItem",
    dialog_box_param: "USER32!DialogBoxParamA",
    draw_menu_bar: "USER32!DrawMenuBar",
    enable_menu_item: "USER32!EnableMenuItem",
    end_dialog: "USER32!EndDialog",
    get_dlg_item: "USER32!GetDlgItem",
    is_dlg_button_checked: "USER32!IsDlgButtonChecked",
    load_menu: "USER32!LoadMenuA",
    set_menu: "USER32!SetMenu",
};

/// The launcher, `mercswin`.
pub const LAUNCHER: Row = Row {
    file: "mercswin",
    calls: &CALLS,
};

/// The shell, `mw2shell`.
pub const SHELL: Row = Row {
    file: "mw2shell",
    calls: &CALLS,
};

/// Every file the seam serves.
pub const ROWS: &[&Row] = &[&LAUNCHER, &SHELL];

/// The shell's menu bar and the movie box: their resource numbers.
pub const SHELL_MENU: u32 = 0x68;
pub const MOVIE_BOX: u32 = 0x8a;

/// The ids the shell asks of menu `0x68` by id. `0x9c72` is not among them: the
/// shell asks it too, and it lies on menu `0x67` alone, so the answer is -1.
pub const MENU_IDS: [u16; 7] = [0x9c41, 0x9c99, 0x9c97, 0x9c94, 0x9c4b, 0x9c96, 0x9c9a];

/// The box's controls: the 2 buttons and the 2 auto check boxes, one pressed
/// sets the other opposite. `w1` pressed `0x3e8` as 320 and `0x3e9` as 640.
pub const OK: u32 = 1;
pub const CANCEL: u32 = 2;
pub const MOVIE_320: u32 = 0x3e8;
pub const MOVIE_640: u32 = 0x3e9;

const WM_INITDIALOG: u32 = 0x110;
const WM_COMMAND: u32 = 0x111;

/// What `100169f0` calls, each answering as the front of its name does.
pub trait BoxCalls {
    fn check_dlg_button(&self, window: u32, id: u32, check: u32) -> u32;
    fn get_dlg_item(&self, window: u32, id: u32) -> u32;
    fn set_focus(&self, window: u32) -> u32;
    fn is_dlg_button_checked(&self, window: u32, id: u32) -> u32;
    fn end_dialog(&self, window: u32, value: u32) -> u32;
}

/// The check box pressed reads its new state; the other is set opposite.
fn pair(calls: &impl BoxCalls, window: u32, pressed: u32, other: u32) {
    let off = u32::from(calls.is_dlg_button_checked(window, pressed) != 1);
    calls.check_dlg_button(window, other, off);
}

/// `100169f0`, the movie box's procedure; `setting` is `DAT_10098220`.
/// Answers 0 to `WM_INITDIALOG`, having set the focus, 1 to every `WM_COMMAND`,
/// and 0 with no call to any other message. OK writes the setting from `0x3e8`;
/// Cancel leaves it. Reads `w`'s low word alone, and never `lParam`.
pub fn movie_box(
    calls: &impl BoxCalls,
    setting: &Cell<u32>,
    window: u32,
    message: u32,
    w: u32,
) -> u32 {
    match message {
        WM_INITDIALOG => {
            let shown = if setting.get() == 0 {
                MOVIE_640
            } else {
                MOVIE_320
            };
            calls.check_dlg_button(window, shown, 1);
            calls.set_focus(calls.get_dlg_item(window, OK));
            0
        }
        WM_COMMAND => {
            match w & 0xffff {
                OK => {
                    let chosen = calls.is_dlg_button_checked(window, MOVIE_320) == 1;
                    setting.set(u32::from(chosen));
                    calls.end_dialog(window, 0);
                }
                CANCEL => {
                    calls.end_dialog(window, 0);
                }
                MOVIE_320 => pair(calls, window, MOVIE_320, MOVIE_640),
                MOVIE_640 => pair(calls, window, MOVIE_640, MOVIE_320),
                _ => {}
            }
            1
        }
        _ => 0,
    }
}
