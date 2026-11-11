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
//! The game is told a 640 by 480 screen, `mech_rows::draw::MODE`, before
//! the first window it places, size it reads, or point it reads or gives:
//! the rows that do any of these (`create_window`, `set_window_pos`,
//! `get_window_rect`, `get_system_metrics`, and the pointer's
//! `peek_message`, `client_to_screen`, `screen_to_client`) tell it first,
//! by `user32::tell_screen`. Its windows then land on the real screen at
//! the whole-factor fit, and the sizes it reads answer the told screen's.
//! The points the game reads or gives on the pointer's rows (a mouse
//! message's, a window's corner) are the told screen's and Wine's the real
//! one's, mapped between them by `window`'s back. The pointer's own three
//! calls, in [`crate::input`], tell it by this module's `told` too. The
//! other rows are told nothing and hand their numbers on as they came.
//!
//! `DefWindowProcA` alone does one thing after its call: at
//! `WM_WINDOWPOSCHANGED`, once Wine has moved or sized a window, it has
//! [`crate::draw::present_again`] show the picture on that window's new
//! inside.
//!
//! `RegisterClassA` hands the real call a copy of the file's class with
//! its background brush 0, by [`crate::window_class`], and writes one
//! line naming the brush taken out: an erase the system sends while it
//! holds the window's messages then fills nothing, and the last picture
//! stays. The file's own record is not written.
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
use core::sync::atomic::{AtomicBool, Ordering};
use kernel_process::Text;
use mech_rows::draw::MODE;

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

/// Whether the game has been told its screen.
static TOLD: AtomicBool = AtomicBool::new(false);

/// Tells the game its screen, `MODE`'s 640 by 480, the first time a row
/// that places a window, reads a size, or reads or gives a point runs,
/// here or in [`crate::input`]. Two rows at once may both tell it: telling
/// the same screen twice changes nothing and writes no second line, and no
/// row goes on untold.
pub(crate) fn told() {
    if !TOLD.load(Ordering::Acquire) {
        user32::tell_screen(MODE.0, MODE.1);
        TOLD.store(true, Ordering::Release);
    }
}

/// A row: the file's call, passed to `window`'s front whole; a `told` row
/// tells the game its screen first.
macro_rules! row {
    (told $name:ident, $($argument:ident),*) => {
        #[allow(clippy::too_many_arguments, reason = "the import's own, and the caller first")]
        extern "system" fn $name(from: u32, $($argument: u32),*) -> u32 {
            told();
            // SAFETY: the file's arguments as its import takes them, passed on whole.
            unsafe { user32::$name(from, $($argument),*) }
        }
    };
    ($name:ident, $($argument:ident),*) => {
        #[allow(clippy::too_many_arguments, reason = "the import's own, and the caller first")]
        extern "system" fn $name(from: u32, $($argument: u32),*) -> u32 {
            // SAFETY: the file's arguments as its import takes them, passed on whole.
            unsafe { user32::$name(from, $($argument),*) }
        }
    };
}

row!(
    told create_window,
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
row!(translate_message, message);
row!(dispatch_message, message);
row!(told peek_message, message, window, low, high, remove);
row!(send_message, window, message, w, l);
row!(post_message, window, message, w, l);
row!(show_window, window, command);
row!(adjust_window_rect, rectangle, style, menu);
row!(told set_window_pos, window, after, x, y, width, height, flags);
row!(told get_window_rect, window, rectangle);
row!(told client_to_screen, window, point);
row!(told screen_to_client, window, point);
row!(set_focus, window);
row!(find_window, class, title);

/// `WM_WINDOWPOSCHANGED`, the message a window is sent once it has been
/// moved or sized.
const WINDOW_POS_CHANGED: u32 = 0x47;

/// The file's `DefWindowProcA`, passed on whole and its answer handed back.
/// After the answer to [`WINDOW_POS_CHANGED`] the picture is shown again on
/// the window, so a window Wine grew or shrank is painted at its new inside
/// now, not at the file's next paint, which may not come.
extern "system" fn def_window_proc(from: u32, window: u32, message: u32, w: u32, l: u32) -> u32 {
    // SAFETY: the file's arguments as its import takes them, passed on whole.
    let value = unsafe { user32::def_window_proc(from, window, message, w, l) };
    if message == WINDOW_POS_CHANGED {
        crate::draw::present_again(window);
    }
    value
}

/// The file's `RegisterClassA`, handed on with a copy of its class whose
/// background brush is 0, by [`crate::window_class::without_brush`]; the
/// file's own record is not written. A null class is passed on as it came.
extern "system" fn register_class(from: u32, class: u32) -> u32 {
    if class == 0 {
        // SAFETY: the file's null argument, passed on whole.
        return unsafe { user32::register_class(from, 0) };
    }
    // SAFETY: the file's class record, 10 fields, as `RegisterClassA` takes it.
    let (copy, brush) = crate::window_class::without_brush(unsafe { record(class) });
    kept!(log::debug!(brush:% = mech_trace::__shown!(brush),
        from:% = mech_trace::__shown!(from), by = "mech-layer";
        "window.brush"));
    // SAFETY: `copy` is a whole class record on this stack, alive across the call.
    unsafe { user32::register_class(from, copy.as_ptr().expose_provenance() as u32) }
}

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
    told();
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
