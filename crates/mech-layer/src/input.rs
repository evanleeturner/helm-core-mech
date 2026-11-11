// SPDX-License-Identifier: MIT
//! The input calls: mercs's 8 USER32 names, each answered
//! by `window-input`'s direct back, [`user32`], from call 1. Each row is a thin
//! call; the line it writes, `by=window-input`, is the crate's. The rows before it logged these
//! with `from`, the caller, and the crate's lines keep that form.
//!
//! 15 exports, one a name a file imports: the shell's 7 `HC6_`, the game
//! file's 7 `GAME_`, the launcher's 1 `LAUNCHER_` (`LoadCursorA` alone).
//! Each is a naked entry that reads the address the call returns to first.
//!
//! The pointer's own three, `GetCursorPos`, `SetCursorPos` and
//! `ClipCursor`, are `told` rows: each tells the game its screen first, by
//! [`crate::window`]'s `told`, as that module's pointer rows do, so the
//! point it reads or gives is the told screen's from the first call, and
//! `window-input`'s back maps it to and from the real one. `ShowCursor`,
//! `LoadCursorA` and the keys are told nothing.
use ::window_input::user32;

/// A row: the file's call, passed to `window-input`'s front whole; a
/// `told` row tells the game its screen first.
macro_rules! row {
    (told $name:ident, $($argument:ident),*) => {
        extern "system" fn $name(from: u32, $($argument: u32),*) -> u32 {
            crate::window::told();
            // SAFETY: the file's arguments as its import takes them, passed on whole.
            unsafe { user32::$name(from, $($argument),*) }
        }
    };
    ($name:ident, $($argument:ident),*) => {
        extern "system" fn $name(from: u32, $($argument: u32),*) -> u32 {
            // SAFETY: the file's arguments as its import takes them, passed on whole.
            unsafe { user32::$name(from, $($argument),*) }
        }
    };
}

/// A row whose front takes no address, so it needs no promise; a `told`
/// row tells the game its screen first.
macro_rules! safe_row {
    (told $name:ident, $($argument:ident),*) => {
        extern "system" fn $name(from: u32, $($argument: u32),*) -> u32 {
            crate::window::told();
            user32::$name(from, $($argument),*)
        }
    };
    ($name:ident, $($argument:ident),*) => {
        extern "system" fn $name(from: u32, $($argument: u32),*) -> u32 {
            user32::$name(from, $($argument),*)
        }
    };
}

row!(told get_cursor_pos, point);
row!(load_cursor, instance, cursor);
row!(told clip_cursor, rectangle);
safe_row!(told set_cursor_pos, x, y);
safe_row!(show_cursor, show);
safe_row!(get_async_key_state, key);
safe_row!(get_key_state, key);
safe_row!(map_virtual_key, code, kind);

entry!(HC6_GetCursorPos, get_cursor_pos, 1);
entry!(HC6_SetCursorPos, set_cursor_pos, 2);
entry!(HC6_ShowCursor, show_cursor, 1);
entry!(HC6_ClipCursor, clip_cursor, 1);
entry!(HC6_GetAsyncKeyState, get_async_key_state, 1);
entry!(HC6_GetKeyState, get_key_state, 1);
entry!(HC6_MapVirtualKeyA, map_virtual_key, 2);

entry!(GAME_GetCursorPos, get_cursor_pos, 1);
entry!(GAME_SetCursorPos, set_cursor_pos, 2);
entry!(GAME_ShowCursor, show_cursor, 1);
entry!(GAME_ClipCursor, clip_cursor, 1);
entry!(GAME_GetAsyncKeyState, get_async_key_state, 1);
entry!(GAME_GetKeyState, get_key_state, 1);
entry!(GAME_MapVirtualKeyA, map_virtual_key, 2);

entry!(LAUNCHER_LoadCursorA, load_cursor, 2);
