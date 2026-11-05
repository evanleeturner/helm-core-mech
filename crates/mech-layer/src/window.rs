// SPDX-License-Identifier: MIT
//! The window calls (WN1, `JOB.md` step 2): mercs's 24 USER32 names of
//! the window stand-in's step 1, each the original's, logged with `from`,
//! the caller, so `window`'s fronts are checked against what the files
//! asked and read back.
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
use core::ffi::CStr;
use core::fmt;
use core::ptr::with_exposed_provenance;
use kernel_process::Text;

unsafe extern "system" {
    fn CreateWindowExA(
        extended: u32,
        class: u32,
        title: u32,
        style: u32,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        parent: u32,
        menu: u32,
        instance: u32,
        parameter: u32,
    ) -> u32;
    fn GetWindowLongA(window: u32, index: u32) -> u32;
    fn SetWindowLongA(window: u32, index: u32, value: u32) -> u32;
    fn DestroyWindow(window: u32) -> u32;
    fn LoadIconA(instance: u32, name: u32) -> u32;
    fn RegisterClassA(class: u32) -> u32;
    fn DefWindowProcA(window: u32, message: u32, w: u32, l: u32) -> u32;
    fn TranslateMessage(message: u32) -> u32;
    fn DispatchMessageA(message: u32) -> u32;
    fn PeekMessageA(message: u32, window: u32, low: u32, high: u32, remove: u32) -> u32;
    fn PostQuitMessage(code: u32);
    fn WaitMessage() -> u32;
    fn GetMessageTime() -> u32;
    fn SendMessageA(window: u32, message: u32, w: u32, l: u32) -> u32;
    fn PostMessageA(window: u32, message: u32, w: u32, l: u32) -> u32;
    fn ShowWindow(window: u32, command: u32) -> u32;
    fn AdjustWindowRect(rectangle: u32, style: u32, menu: u32) -> u32;
    fn SetWindowPos(
        window: u32,
        after: u32,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        flags: u32,
    ) -> u32;
    fn GetWindowRect(window: u32, rectangle: u32) -> u32;
    fn ClientToScreen(window: u32, point: u32) -> u32;
    fn ScreenToClient(window: u32, point: u32) -> u32;
    fn SetFocus(window: u32) -> u32;
    fn FindWindowA(class: u32, title: u32) -> u32;
    fn GetSystemMetrics(index: u32) -> u32;
}

/// A name a call is handed, as `s` and `with` show it.
enum Name<'a> {
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
unsafe fn name<'a>(address: u32) -> Name<'a> {
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
unsafe fn record<const N: usize>(address: u32) -> [u32; N] {
    if address == 0 {
        return [0; N];
    }
    // SAFETY: the caller's record, read where it lies.
    unsafe { with_exposed_provenance::<[u32; N]>(address as usize).read_unaligned() }
}

/// Numbers as a line shows them, comma-joined: `a`'s, or `o`'s record.
struct Numbers<'a>(&'a [u32]);

impl fmt::Display for Numbers<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.iter().enumerate().try_for_each(|(index, number)| {
            if index > 0 {
                f.write_str(",")?;
            }
            mech_trace::__shown!(*number).fmt(f)
        })
    }
}

/// What a call wrote at its record, the line's `o`: `N` numbers, or
/// nothing on a 0 answer.
struct Wrote<const N: usize>(Option<[u32; N]>);

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
unsafe fn wrote<const N: usize>(value: u32, address: u32) -> Wrote<N> {
    // SAFETY: the caller's record, read on a nonzero answer alone.
    Wrote((value != 0).then(|| unsafe { record(address) }))
}

#[allow(
    clippy::too_many_arguments,
    reason = "CreateWindowExA's own 12, and the caller first"
)]
extern "system" fn create_window(
    from: u32,
    extended: u32,
    class: u32,
    title: u32,
    style: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    parent: u32,
    menu: u32,
    instance: u32,
    parameter: u32,
) -> u32 {
    // SAFETY: the caller's arguments, passed on whole.
    let value = unsafe {
        CreateWindowExA(
            extended, class, title, style, x, y, width, height, parent, menu, instance, parameter,
        )
    };
    // SAFETY: the caller's class and title, each a number or a text.
    let (class, title) = unsafe { (name(class), name(title)) };
    kept!(log::debug!(c = "USER32!CreateWindowExA",
        a:% = Numbers(&[extended, style, x, y, width, height, parent, menu, instance, parameter]),
        s:% = class, with:% = title, v:% = mech_trace::__shown!(value),
        from:% = mech_trace::__shown!(from); "imp.call"));
    value
}

extern "system" fn get_window_long(from: u32, window: u32, index: u32) -> u32 {
    // SAFETY: the caller's arguments, passed on whole.
    let value = unsafe { GetWindowLongA(window, index) };
    kept!(
        log::debug!(c = "USER32!GetWindowLongA", a:% = Numbers(&[window, index]),
        v:% = mech_trace::__shown!(value), from:% = mech_trace::__shown!(from); "imp.call")
    );
    value
}

extern "system" fn set_window_long(from: u32, window: u32, index: u32, long: u32) -> u32 {
    // SAFETY: the caller's arguments, passed on whole.
    let value = unsafe { SetWindowLongA(window, index, long) };
    kept!(
        log::debug!(c = "USER32!SetWindowLongA", a:% = Numbers(&[window, index, long]),
        v:% = mech_trace::__shown!(value), from:% = mech_trace::__shown!(from); "imp.call")
    );
    value
}

extern "system" fn destroy_window(from: u32, window: u32) -> u32 {
    // SAFETY: the caller's argument, passed on whole.
    let value = unsafe { DestroyWindow(window) };
    kept!(
        log::debug!(c = "USER32!DestroyWindow", a:% = Numbers(&[window]),
        v:% = mech_trace::__shown!(value), from:% = mech_trace::__shown!(from); "imp.call")
    );
    value
}

extern "system" fn load_icon(from: u32, instance: u32, icon: u32) -> u32 {
    // SAFETY: the caller's arguments, passed on whole.
    let value = unsafe { LoadIconA(instance, icon) };
    // SAFETY: the caller's icon name, a resource id or a text.
    let icon = unsafe { name(icon) };
    kept!(
        log::debug!(c = "USER32!LoadIconA", a:% = Numbers(&[instance]), s:% = icon,
        v:% = mech_trace::__shown!(value), from:% = mech_trace::__shown!(from); "imp.call")
    );
    value
}

/// `RegisterClassA`: `a` the class record's first 8 fields (style,
/// procedure, the two extra counts, instance, icon, cursor, brush), `s`
/// its class name, `with` its menu name.
extern "system" fn register_class(from: u32, class: u32) -> u32 {
    // SAFETY: the caller's class record, 10 fields, read before the call.
    let fields = unsafe { record::<10>(class) };
    // SAFETY: the caller's argument, passed on whole.
    let value = unsafe { RegisterClassA(class) };
    // SAFETY: the record's class and menu names, each a number or a text.
    let (named, menu) = unsafe { (name(fields[9]), name(fields[8])) };
    kept!(
        log::debug!(c = "USER32!RegisterClassA", a:% = Numbers(&fields[..8]),
        s:% = named, with:% = menu, v:% = mech_trace::__shown!(value),
        from:% = mech_trace::__shown!(from); "imp.call")
    );
    value
}

extern "system" fn def_window_proc(from: u32, window: u32, message: u32, w: u32, l: u32) -> u32 {
    // SAFETY: the caller's arguments, passed on whole.
    let value = unsafe { DefWindowProcA(window, message, w, l) };
    kept!(
        log::debug!(c = "USER32!DefWindowProcA", a:% = Numbers(&[window, message, w, l]),
        v:% = mech_trace::__shown!(value), from:% = mech_trace::__shown!(from); "imp.call")
    );
    value
}

/// `TranslateMessage`: `a` the message record's 7 fields (window,
/// message, `w`, `l`, time, x, y), read before the call.
extern "system" fn translate_message(from: u32, message: u32) -> u32 {
    // SAFETY: the caller's message record, read before the call.
    let fields = unsafe { record::<7>(message) };
    // SAFETY: the caller's argument, passed on whole.
    let value = unsafe { TranslateMessage(message) };
    kept!(
        log::debug!(c = "USER32!TranslateMessage", a:% = Numbers(&fields),
        v:% = mech_trace::__shown!(value), from:% = mech_trace::__shown!(from); "imp.call")
    );
    value
}

/// `DispatchMessageA`: `a` the message record's 7 fields, read before the
/// call, as the procedure it calls may reuse the record.
extern "system" fn dispatch_message(from: u32, message: u32) -> u32 {
    // SAFETY: the caller's message record, read before the call.
    let fields = unsafe { record::<7>(message) };
    // SAFETY: the caller's argument, passed on whole.
    let value = unsafe { DispatchMessageA(message) };
    kept!(
        log::debug!(c = "USER32!DispatchMessageA", a:% = Numbers(&fields),
        v:% = mech_trace::__shown!(value), from:% = mech_trace::__shown!(from); "imp.call")
    );
    value
}

/// `PeekMessageA`: `a` the window, the range and the flag; `o` the 7
/// fields of the message it wrote, none when it answered 0.
extern "system" fn peek_message(
    from: u32,
    message: u32,
    window: u32,
    low: u32,
    high: u32,
    remove: u32,
) -> u32 {
    // SAFETY: the caller's arguments, passed on whole.
    let value = unsafe { PeekMessageA(message, window, low, high, remove) };
    // SAFETY: the caller's message record.
    let got = unsafe { wrote::<7>(value, message) };
    kept!(
        log::debug!(c = "USER32!PeekMessageA", a:% = Numbers(&[window, low, high, remove]),
        o:% = got, v:% = mech_trace::__shown!(value), from:% = mech_trace::__shown!(from);
        "imp.call")
    );
    value
}

extern "system" fn post_quit_message(from: u32, code: u32) {
    // SAFETY: the caller's argument, passed on whole.
    unsafe { PostQuitMessage(code) };
    kept!(
        log::debug!(c = "USER32!PostQuitMessage", a:% = Numbers(&[code]),
        from:% = mech_trace::__shown!(from); "imp.call")
    );
}

extern "system" fn wait_message(from: u32) -> u32 {
    // SAFETY: takes no arguments.
    let value = unsafe { WaitMessage() };
    kept!(
        log::debug!(c = "USER32!WaitMessage", v:% = mech_trace::__shown!(value),
        from:% = mech_trace::__shown!(from); "imp.call")
    );
    value
}

extern "system" fn get_message_time(from: u32) -> u32 {
    // SAFETY: takes no arguments.
    let value = unsafe { GetMessageTime() };
    kept!(
        log::debug!(c = "USER32!GetMessageTime", v:% = mech_trace::__shown!(value),
        from:% = mech_trace::__shown!(from); "imp.call")
    );
    value
}

extern "system" fn send_message(from: u32, window: u32, message: u32, w: u32, l: u32) -> u32 {
    // SAFETY: the caller's arguments, passed on whole.
    let value = unsafe { SendMessageA(window, message, w, l) };
    kept!(
        log::debug!(c = "USER32!SendMessageA", a:% = Numbers(&[window, message, w, l]),
        v:% = mech_trace::__shown!(value), from:% = mech_trace::__shown!(from); "imp.call")
    );
    value
}

extern "system" fn post_message(from: u32, window: u32, message: u32, w: u32, l: u32) -> u32 {
    // SAFETY: the caller's arguments, passed on whole.
    let value = unsafe { PostMessageA(window, message, w, l) };
    kept!(
        log::debug!(c = "USER32!PostMessageA", a:% = Numbers(&[window, message, w, l]),
        v:% = mech_trace::__shown!(value), from:% = mech_trace::__shown!(from); "imp.call")
    );
    value
}

extern "system" fn show_window(from: u32, window: u32, command: u32) -> u32 {
    // SAFETY: the caller's arguments, passed on whole.
    let value = unsafe { ShowWindow(window, command) };
    kept!(
        log::debug!(c = "USER32!ShowWindow", a:% = Numbers(&[window, command]),
        v:% = mech_trace::__shown!(value), from:% = mech_trace::__shown!(from); "imp.call")
    );
    value
}

/// `AdjustWindowRect`: `a` the rectangle's 4 fields as handed, the style
/// and the menu flag; `o` the 4 it wrote, none when it answered 0.
extern "system" fn adjust_window_rect(from: u32, rectangle: u32, style: u32, menu: u32) -> u32 {
    // SAFETY: the caller's rectangle, read before the call.
    let [left, top, right, bottom] = unsafe { record::<4>(rectangle) };
    // SAFETY: the caller's arguments, passed on whole.
    let value = unsafe { AdjustWindowRect(rectangle, style, menu) };
    // SAFETY: the caller's rectangle.
    let got = unsafe { wrote::<4>(value, rectangle) };
    kept!(log::debug!(c = "USER32!AdjustWindowRect",
        a:% = Numbers(&[left, top, right, bottom, style, menu]), o:% = got,
        v:% = mech_trace::__shown!(value), from:% = mech_trace::__shown!(from); "imp.call"));
    value
}

extern "system" fn set_window_pos(
    from: u32,
    window: u32,
    after: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    flags: u32,
) -> u32 {
    // SAFETY: the caller's arguments, passed on whole.
    let value = unsafe { SetWindowPos(window, after, x, y, width, height, flags) };
    kept!(log::debug!(c = "USER32!SetWindowPos",
        a:% = Numbers(&[window, after, x, y, width, height, flags]),
        v:% = mech_trace::__shown!(value), from:% = mech_trace::__shown!(from); "imp.call"));
    value
}

/// `GetWindowRect`: `a` the window; `o` the rectangle's 4 fields it
/// wrote, none when it answered 0.
extern "system" fn get_window_rect(from: u32, window: u32, rectangle: u32) -> u32 {
    // SAFETY: the caller's arguments, passed on whole.
    let value = unsafe { GetWindowRect(window, rectangle) };
    // SAFETY: the caller's rectangle.
    let got = unsafe { wrote::<4>(value, rectangle) };
    kept!(
        log::debug!(c = "USER32!GetWindowRect", a:% = Numbers(&[window]), o:% = got,
        v:% = mech_trace::__shown!(value), from:% = mech_trace::__shown!(from); "imp.call")
    );
    value
}

/// `ClientToScreen`: `a` the window and the point as handed; `o` the
/// point it wrote, none when it answered 0.
extern "system" fn client_to_screen(from: u32, window: u32, point: u32) -> u32 {
    // SAFETY: the caller's point, read before the call.
    let [x, y] = unsafe { record::<2>(point) };
    // SAFETY: the caller's arguments, passed on whole.
    let value = unsafe { ClientToScreen(window, point) };
    // SAFETY: the caller's point.
    let got = unsafe { wrote::<2>(value, point) };
    kept!(
        log::debug!(c = "USER32!ClientToScreen", a:% = Numbers(&[window, x, y]), o:% = got,
        v:% = mech_trace::__shown!(value), from:% = mech_trace::__shown!(from); "imp.call")
    );
    value
}

/// `ScreenToClient`: as [`client_to_screen`], the other way.
extern "system" fn screen_to_client(from: u32, window: u32, point: u32) -> u32 {
    // SAFETY: the caller's point, read before the call.
    let [x, y] = unsafe { record::<2>(point) };
    // SAFETY: the caller's arguments, passed on whole.
    let value = unsafe { ScreenToClient(window, point) };
    // SAFETY: the caller's point.
    let got = unsafe { wrote::<2>(value, point) };
    kept!(
        log::debug!(c = "USER32!ScreenToClient", a:% = Numbers(&[window, x, y]), o:% = got,
        v:% = mech_trace::__shown!(value), from:% = mech_trace::__shown!(from); "imp.call")
    );
    value
}

extern "system" fn set_focus(from: u32, window: u32) -> u32 {
    // SAFETY: the caller's argument, passed on whole.
    let value = unsafe { SetFocus(window) };
    kept!(log::debug!(c = "USER32!SetFocus", a:% = Numbers(&[window]),
        v:% = mech_trace::__shown!(value), from:% = mech_trace::__shown!(from); "imp.call"));
    value
}

extern "system" fn find_window(from: u32, class: u32, title: u32) -> u32 {
    // SAFETY: the caller's arguments, passed on whole.
    let value = unsafe { FindWindowA(class, title) };
    // SAFETY: the caller's class and title, each a number or a text.
    let (class, title) = unsafe { (name(class), name(title)) };
    kept!(
        log::debug!(c = "USER32!FindWindowA", s:% = class, with:% = title,
        v:% = mech_trace::__shown!(value), from:% = mech_trace::__shown!(from); "imp.call")
    );
    value
}

extern "system" fn get_system_metrics(from: u32, index: u32) -> u32 {
    // SAFETY: the caller's argument, passed on whole.
    let value = unsafe { GetSystemMetrics(index) };
    kept!(
        log::debug!(c = "USER32!GetSystemMetrics", a:% = Numbers(&[index]),
        v:% = mech_trace::__shown!(value), from:% = mech_trace::__shown!(from); "imp.call")
    );
    value
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
