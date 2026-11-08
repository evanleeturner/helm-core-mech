// SPDX-License-Identifier: MIT
//! The window calls: mercs's 24 USER32 names, each answered
//! by `window`'s direct back, [`user32`], from call 1. Each row is a thin call;
//! the line it writes, `by=window`, is the crate's. The rows before it logged these with
//! `from`, the caller, and the crate's lines keep that form, so a reader of an
//! older log reads this one.
//!
//! 46 exports, one a name a file imports: the shell's 19 `HC6_`, the game
//! file's 18 `GAME_`, the launcher's 9 `LAUNCHER_`. Each is a naked entry
//! that reads the address the call returns to first, as
//! [`crate::registry`]'s entries do. The shell's 5 rows HC6 logged with no
//! caller (`DefWindowProcA`, `TranslateMessage`, `DispatchMessageA`,
//! `GetSystemMetrics`, `PostMessageA`) moved here from [`crate::imports`],
//! their exports' names unchanged.
//!
//! A line's `a` holds numbers: a record a call is handed (a message, a
//! rectangle, a point, a class) shows its fields, never its address; `o`
//! the record the call wrote, empty on a 0 answer. A class, title, icon
//! or menu name shows as text, `s` the class or icon, `with` the title or
//! menu; one given as a number below `0x10000` (an atom or a resource id)
//! shows `#` and the number, a null one nothing. A message's `l` and
//! `CreateWindowExA`'s last argument are the numbers handed, a pointer for
//! some messages (`WM_COPYDATA`'s record): what lies behind is not read.
//!
//! The readers below stay for this crate's other rows, which still write
//! their own lines.
use ::window::user32;
use core::ffi::CStr;
use core::fmt;
use core::ptr::with_exposed_provenance;
use kernel_process::Text;

/// A name a call is handed, as `s` and `with` show it.
pub(crate) enum Name<'a> {
    /// A NUL-ended text, the NUL left out; empty for a null name.
    Text(&'a [u8]),
    /// An atom or a resource id: a number below `0x10000`.
    Number(u32),
}

impl fmt::Display for Name<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Name::Text(text) => Text(text).fmt(f),
            Name::Number(number) => {
                f.write_str("#")?;
                mech_trace::__shown!(*number).fmt(f)
            }
        }
    }
}

/// The name at `address`: nothing for null, a number below `0x10000`.
///
/// # Safety
/// `address` is below `0x10000` or a NUL-ended text, as Win32 takes it.
pub(crate) unsafe fn name<'a>(address: u32) -> Name<'a> {
    match address {
        0 => Name::Text(&[]),
        1..0x10000 => Name::Number(address),
        // SAFETY: the caller's NUL-ended text.
        _ => Name::Text(
            unsafe { CStr::from_ptr(with_exposed_provenance(address as usize)) }.to_bytes(),
        ),
    }
}

/// The `N` numbers of the record at `address`, in its fields' order; all
/// 0 for a null one.
///
/// # Safety
/// `address` is 0 or `4 * N` bytes the caller handed in.
pub(crate) unsafe fn record<const N: usize>(address: u32) -> [u32; N] {
    if address == 0 {
        return [0; N];
    }
    // SAFETY: the caller's record, read where it lies.
    unsafe { with_exposed_provenance::<[u32; N]>(address as usize).read_unaligned() }
}

pub(crate) use crate::numbers::Numbers;

/// What a call wrote at its record, the line's `o`: `N` numbers, or
/// nothing on a 0 answer.
pub(crate) struct Wrote<const N: usize>(Option<[u32; N]>);

impl<const N: usize> fmt::Display for Wrote<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0
            .as_ref()
            .map_or(Ok(()), |numbers| Numbers(numbers).fmt(f))
    }
}

/// The record at `address` after a call answered `value`; none on 0.
///
/// # Safety
/// `address` is 0 or `4 * N` bytes the caller handed in.
pub(crate) unsafe fn wrote<const N: usize>(value: u32, address: u32) -> Wrote<N> {
    // SAFETY: the caller's record, read on a nonzero answer alone.
    Wrote((value != 0).then(|| unsafe { record(address) }))
}

/// A row: the file's call, passed to `window`'s front whole.
macro_rules! row {
    ($name:ident, $($argument:ident),*) => {
        #[allow(clippy::too_many_arguments, reason = "the import's own, and the caller first")]
        extern "system" fn $name(from: u32, $($argument: u32),*) -> u32 {
            // SAFETY: the file's arguments as its import takes them, passed on whole.
            unsafe { user32::$name(from, $($argument),*) }
        }
    };
}

row!(
    create_window,
    extended,
    class,
    title,
    style,
    x,
    y,
    width,
    height,
    parent,
    menu,
    instance,
    parameter
);
row!(set_window_long, window, index, long);
row!(destroy_window, window);
row!(load_icon, instance, icon);
row!(register_class, class);
row!(def_window_proc, window, message, w, l);
row!(translate_message, message);
row!(dispatch_message, message);
row!(peek_message, message, window, low, high, remove);
row!(send_message, window, message, w, l);
row!(post_message, window, message, w, l);
row!(show_window, window, command);
row!(adjust_window_rect, rectangle, style, menu);
row!(set_window_pos, window, after, x, y, width, height, flags);
row!(get_window_rect, window, rectangle);
row!(client_to_screen, window, point);
row!(screen_to_client, window, point);
row!(set_focus, window);
row!(find_window, class, title);

extern "system" fn wait_message(from: u32) -> u32 {
    // SAFETY: the file's own wait, in the loop that holds nothing across it.
    unsafe { user32::wait_message(from) }
}

extern "system" fn get_window_long(from: u32, window: u32, index: u32) -> u32 {
    user32::get_window_long(from, window, index)
}

extern "system" fn post_quit_message(from: u32, code: u32) {
    user32::post_quit_message(from, code);
}

extern "system" fn get_message_time(from: u32) -> u32 {
    user32::get_message_time(from)
}

extern "system" fn get_system_metrics(from: u32, index: u32) -> u32 {
    user32::get_system_metrics(from, index)
}

entry!(HC6_CreateWindowExA, create_window, 12);
entry!(HC6_GetWindowLongA, get_window_long, 2);
entry!(HC6_SetWindowLongA, set_window_long, 3);
entry!(HC6_DefWindowProcA, def_window_proc, 4);
entry!(HC6_TranslateMessage, translate_message, 1);
entry!(HC6_DispatchMessageA, dispatch_message, 1);
entry!(HC6_PeekMessageA, peek_message, 5);
entry!(HC6_PostQuitMessage, post_quit_message, 1);
entry!(HC6_WaitMessage, wait_message, 0);
entry!(HC6_SendMessageA, send_message, 4);
entry!(HC6_PostMessageA, post_message, 4);
entry!(HC6_ShowWindow, show_window, 2);
entry!(HC6_AdjustWindowRect, adjust_window_rect, 3);
entry!(HC6_SetWindowPos, set_window_pos, 7);
entry!(HC6_GetWindowRect, get_window_rect, 2);
entry!(HC6_ClientToScreen, client_to_screen, 2);
entry!(HC6_ScreenToClient, screen_to_client, 2);
entry!(HC6_SetFocus, set_focus, 1);
entry!(HC6_GetSystemMetrics, get_system_metrics, 1);

entry!(GAME_CreateWindowExA, create_window, 12);
entry!(GAME_GetWindowLongA, get_window_long, 2);
entry!(GAME_SetWindowLongA, set_window_long, 3);
entry!(GAME_DefWindowProcA, def_window_proc, 4);
entry!(GAME_TranslateMessage, translate_message, 1);
entry!(GAME_DispatchMessageA, dispatch_message, 1);
entry!(GAME_PeekMessageA, peek_message, 5);
entry!(GAME_PostQuitMessage, post_quit_message, 1);
entry!(GAME_WaitMessage, wait_message, 0);
entry!(GAME_GetMessageTime, get_message_time, 0);
entry!(GAME_SendMessageA, send_message, 4);
entry!(GAME_ShowWindow, show_window, 2);
entry!(GAME_AdjustWindowRect, adjust_window_rect, 3);
entry!(GAME_SetWindowPos, set_window_pos, 7);
entry!(GAME_GetWindowRect, get_window_rect, 2);
entry!(GAME_ClientToScreen, client_to_screen, 2);
entry!(GAME_ScreenToClient, screen_to_client, 2);
entry!(GAME_GetSystemMetrics, get_system_metrics, 1);

entry!(LAUNCHER_CreateWindowExA, create_window, 12);
entry!(LAUNCHER_DestroyWindow, destroy_window, 1);
entry!(LAUNCHER_LoadIconA, load_icon, 2);
entry!(LAUNCHER_RegisterClassA, register_class, 1);
entry!(LAUNCHER_DefWindowProcA, def_window_proc, 4);
entry!(LAUNCHER_ShowWindow, show_window, 2);
entry!(LAUNCHER_AdjustWindowRect, adjust_window_rect, 3);
entry!(LAUNCHER_FindWindowA, find_window, 2);
entry!(LAUNCHER_GetSystemMetrics, get_system_metrics, 1);
