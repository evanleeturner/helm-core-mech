// SPDX-License-Identifier: MIT
//! mercs's input rows (WI1, ruling 1417): the launcher's, the shell's and
//! the game file's keyboard and pointer calls, as `window_input`'s fronts
//! serve them. The run `k3` (WI1 step 2, the layer's rows under Wine
//! through the shell's menus by keys) logged 4 of the 8 names, the shell's
//! alone (`s11-counts-k3.txt`); the game file's 7 wait on play (wall 1),
//! the launcher's 1 on a run through the layer (wall 5), each the plan's
//! answer, unmeasured.
//!
//! The 8 names and the files importing each (`mercswin` the launcher,
//! `mw2shell` the shell, `mercsw` the game file):
//!
//! | Name | Files |
//! | --- | --- |
//! | `ClipCursor` | mw2shell, mercsw |
//! | `GetAsyncKeyState` | mw2shell, mercsw |
//! | `GetCursorPos` | mw2shell, mercsw |
//! | `GetKeyState` | mw2shell, mercsw |
//! | `LoadCursorA` | mercswin |
//! | `MapVirtualKeyA` | mw2shell, mercsw |
//! | `SetCursorPos` | mw2shell, mercsw |
//! | `ShowCursor` | mw2shell, mercsw |
//!
//! The stick: in play alone the game file's `fun_1005ed99` reads the
//! pointer as a joystick, each axis from -`0x10000` at the client's left
//! or top edge to just under `0x10000` at its right or bottom. Its formula
//! is the machine code's, `MERCSW.DLL` `1005ef3a` to `1005ef6c`; no run has
//! reached play (wall 1), so no number of it is harvested. The tests
//! replaying `k3` and the stick's live in `tests`.
use window_input::table::{Calls, Row};

#[cfg(test)]
mod tests;

/// Each call's name, the layer's spelling.
const CALLS: Calls = Calls {
    get_cursor_pos: "USER32!GetCursorPos",
    set_cursor_pos: "USER32!SetCursorPos",
    show_cursor: "USER32!ShowCursor",
    load_cursor: "USER32!LoadCursorA",
    clip_cursor: "USER32!ClipCursor",
    get_async_key_state: "USER32!GetAsyncKeyState",
    get_key_state: "USER32!GetKeyState",
    map_virtual_key: "USER32!MapVirtualKeyA",
};

/// The launcher, `mercswin`.
pub const LAUNCHER: Row = Row {
    file: "mercswin",
    calls: &CALLS,
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

/// Every file the input seam serves.
pub const ROWS: &[&Row] = &[&LAUNCHER, &SHELL, &GAME];

/// The stick the game file reads at the client point `point` on a client
/// `size` wide and high: `[y, x]`, the order `fun_1005ed99` writes them,
/// each axis `((2p - n) << 16) / n` divided signed and truncated toward 0,
/// as its `cdq; idiv`. `None` for a point outside the client, where the
/// game writes nothing.
#[must_use]
pub const fn stick(point: (i32, i32), size: (i32, i32)) -> Option<[i32; 2]> {
    let ((x, y), (width, height)) = (point, size);
    if x < 0 || x >= width || y < 0 || y >= height {
        return None;
    }
    Some([axis(y, height), axis(x, width)])
}

/// One axis: `p` on a side `n` long, `n` above `p` and so above 0.
const fn axis(p: i32, n: i32) -> i32 {
    p.wrapping_mul(2).wrapping_sub(n).wrapping_shl(16) / n
}
