// SPDX-License-Identifier: MIT
//! The input calls (WI1, `JOB.md` step 2): mercs's 8 USER32 names of the
//! window stand-in's step 2, each the original's, logged with `from`, the
//! caller, so `window-input`'s fronts are checked against what the files
//! asked and read back.
//!
//! 15 exports, one a name a file imports: the shell's 7 `HC6_`, the game
//! file's 7 `GAME_`, the launcher's 1 `LAUNCHER_` (`LoadCursorA` alone).
//! Each is a naked entry that reads the address the call returns to
//! first, and shows its records and names as [`crate::window`]'s lines
//! do, with that module's readers.
//!
//! `GetKeyState` and `GetAsyncKeyState` answer a `SHORT`: `eax` goes back
//! whole, as the call left it, and `v` shows its low 16 bits, the value
//! the caller reads.
use crate::window::{Numbers, name, record, wrote};

unsafe extern "system" {
    fn GetCursorPos(point: u32) -> u32;
    fn SetCursorPos(x: u32, y: u32) -> u32;
    fn ShowCursor(show: u32) -> u32;
    fn LoadCursorA(instance: u32, cursor: u32) -> u32;
    fn ClipCursor(rectangle: u32) -> u32;
    fn GetAsyncKeyState(key: u32) -> u32;
    fn GetKeyState(key: u32) -> u32;
    fn MapVirtualKeyA(code: u32, kind: u32) -> u32;
}

/// `GetCursorPos`: `o` the point it wrote, none when it answered 0.
extern "system" fn get_cursor_pos(from: u32, point: u32) -> u32 {
    // SAFETY: the caller's argument, passed on whole.
    let value = unsafe { GetCursorPos(point) };
    // SAFETY: the caller's point.
    let got = unsafe { wrote::<2>(value, point) };
    kept!(log::debug!(c = "USER32!GetCursorPos", o:% = got,
        v:% = mech_trace::__shown!(value), from:% = mech_trace::__shown!(from); "imp.call"));
    value
}

extern "system" fn set_cursor_pos(from: u32, x: u32, y: u32) -> u32 {
    // SAFETY: the caller's arguments, passed on whole.
    let value = unsafe { SetCursorPos(x, y) };
    kept!(
        log::debug!(c = "USER32!SetCursorPos", a:% = Numbers(&[x, y]),
        v:% = mech_trace::__shown!(value), from:% = mech_trace::__shown!(from); "imp.call")
    );
    value
}

/// `ShowCursor`: `v` the display count it left.
extern "system" fn show_cursor(from: u32, show: u32) -> u32 {
    // SAFETY: the caller's argument, passed on whole.
    let value = unsafe { ShowCursor(show) };
    kept!(log::debug!(c = "USER32!ShowCursor", a:% = Numbers(&[show]),
        v:% = mech_trace::__shown!(value), from:% = mech_trace::__shown!(from); "imp.call"));
    value
}

/// `LoadCursorA`: `a` the instance, `s` the cursor's name, a resource id
/// shown `#` and the number.
extern "system" fn load_cursor(from: u32, instance: u32, cursor: u32) -> u32 {
    // SAFETY: the caller's arguments, passed on whole.
    let value = unsafe { LoadCursorA(instance, cursor) };
    // SAFETY: the caller's cursor name, a resource id or a text.
    let cursor = unsafe { name(cursor) };
    kept!(
        log::debug!(c = "USER32!LoadCursorA", a:% = Numbers(&[instance]), s:% = cursor,
        v:% = mech_trace::__shown!(value), from:% = mech_trace::__shown!(from); "imp.call")
    );
    value
}

/// `ClipCursor`: `a` the rectangle's 4 fields as handed, none for a null
/// one, the clip let go.
extern "system" fn clip_cursor(from: u32, rectangle: u32) -> u32 {
    // SAFETY: the caller's rectangle, read before the call.
    let fields = unsafe { record::<4>(rectangle) };
    // SAFETY: the caller's argument, passed on whole.
    let value = unsafe { ClipCursor(rectangle) };
    let handed: &[u32] = if rectangle == 0 { &[] } else { &fields };
    kept!(log::debug!(c = "USER32!ClipCursor", a:% = Numbers(handed),
        v:% = mech_trace::__shown!(value), from:% = mech_trace::__shown!(from); "imp.call"));
    value
}

/// `GetAsyncKeyState`: `v` the `SHORT`'s 16 bits; `eax` goes back whole.
extern "system" fn get_async_key_state(from: u32, key: u32) -> u32 {
    // SAFETY: the caller's argument, passed on whole.
    let value = unsafe { GetAsyncKeyState(key) };
    let short = value & 0xffff;
    kept!(
        log::debug!(c = "USER32!GetAsyncKeyState", a:% = Numbers(&[key]),
        v:% = mech_trace::__shown!(short), from:% = mech_trace::__shown!(from); "imp.call")
    );
    value
}

/// `GetKeyState`: as [`get_async_key_state`].
extern "system" fn get_key_state(from: u32, key: u32) -> u32 {
    // SAFETY: the caller's argument, passed on whole.
    let value = unsafe { GetKeyState(key) };
    let short = value & 0xffff;
    kept!(log::debug!(c = "USER32!GetKeyState", a:% = Numbers(&[key]),
        v:% = mech_trace::__shown!(short), from:% = mech_trace::__shown!(from); "imp.call"));
    value
}

extern "system" fn map_virtual_key(from: u32, code: u32, kind: u32) -> u32 {
    // SAFETY: the caller's arguments, passed on whole.
    let value = unsafe { MapVirtualKeyA(code, kind) };
    kept!(
        log::debug!(c = "USER32!MapVirtualKeyA", a:% = Numbers(&[code, kind]),
        v:% = mech_trace::__shown!(value), from:% = mech_trace::__shown!(from); "imp.call")
    );
    value
}

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
