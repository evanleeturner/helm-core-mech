// SPDX-License-Identifier: MIT
//! The text calls (KX1, `JOB.md` step 4): the 13 KERNEL32 code page,
//! conversion, character and locale names, each the original's, logged as
//! `kernel_text`'s front logs them, its fields in the front's order, then
//! `o`, what the call wrote into its buffer, and `from`, the caller.
//!
//! 35 exports, one a name a file imports: the launcher's 9 `LAUNCHER_`
//! (no `CompareString`, no `GetLocaleInfo`), the shell's 13 `HC6_`, the
//! game file's 13 `GAME_`. Each is a naked entry that reads the address
//! the call returns to first, as [`crate::process`]' entries do.
//!
//! A count below 0 is the NUL-ended text, the NUL shown for the
//! conversions, `GetStringType` and `LCMapString`, which take it, and left
//! out for `CompareString`, which stops at it. A null text shows empty.
use core::fmt;
use core::ptr::with_exposed_provenance;
use core::slice;
use kernel_text::table::Row;
use kernel_text::{Bytes, Units};
use mech_rows::text::{GAME, LAUNCHER, SHELL};
use mech_trace::Args;

use crate::left;

unsafe extern "system" {
    fn GetACP() -> u32;
    fn GetOEMCP() -> u32;
    fn GetCPInfo(code_page: u32, info: u32) -> i32;
    fn MultiByteToWideChar(
        code_page: u32,
        flags: u32,
        bytes: u32,
        count: i32,
        units: u32,
        size: i32,
    ) -> i32;
    fn WideCharToMultiByte(
        code_page: u32,
        flags: u32,
        units: u32,
        count: i32,
        bytes: u32,
        size: i32,
        fallback: u32,
        used: u32,
    ) -> i32;
    fn GetStringTypeA(locale: u32, kind: u32, bytes: u32, count: i32, classes: u32) -> i32;
    fn GetStringTypeW(kind: u32, units: u32, count: i32, classes: u32) -> i32;
    fn LCMapStringA(locale: u32, flags: u32, bytes: u32, count: i32, out: u32, size: i32) -> i32;
    fn LCMapStringW(locale: u32, flags: u32, units: u32, count: i32, out: u32, size: i32) -> i32;
    fn CompareStringA(
        locale: u32,
        flags: u32,
        first: u32,
        first_count: i32,
        second: u32,
        second_count: i32,
    ) -> i32;
    fn CompareStringW(
        locale: u32,
        flags: u32,
        first: u32,
        first_count: i32,
        second: u32,
        second_count: i32,
    ) -> i32;
    fn GetLocaleInfoA(locale: u32, kind: u32, out: u32, size: i32) -> i32;
    fn GetLocaleInfoW(locale: u32, kind: u32, out: u32, size: i32) -> i32;
}

/// A file the rows serve, by its row in `mech_rows::text`.
trait File {
    const ROW: &'static Row;
}

/// `mercswin`, the launcher.
enum Launcher {}
/// `mw2shell`, the shell.
enum Shell {}
/// `mercsw`, the game file.
enum Game {}

impl File for Launcher {
    const ROW: &'static Row = &LAUNCHER;
}

impl File for Shell {
    const ROW: &'static Row = &SHELL;
}

impl File for Game {
    const ROW: &'static Row = &GAME;
}

/// The text at `address`: `count` units, or for a count below 0 the units
/// before the first 0, that 0 kept when `nul` says; empty for a null one.
///
/// # Safety
/// `address` is 0 or a text of `count` units, NUL-ended for a count below
/// 0, as Win32 takes it.
unsafe fn given<'a, T: Copy + Default + PartialEq>(address: u32, count: i32, nul: bool) -> &'a [T] {
    if address == 0 {
        return &[];
    }
    let start = with_exposed_provenance::<T>(address as usize);
    let length = usize::try_from(count).unwrap_or_else(|_| {
        let mut length = 0;
        // SAFETY: the caller's NUL-ended text, read up to its NUL.
        while unsafe { start.add(length).read() } != T::default() {
            length += 1;
        }
        length + usize::from(nul)
    });
    // SAFETY: the caller's `length` units at `address`.
    unsafe { slice::from_raw_parts(start, length) }
}

/// What a call wrote at `address`: `length` units, empty for no buffer.
///
/// # Safety
/// `address` is 0 or a buffer Win32 wrote `length` units into.
unsafe fn wrote<'a, T>(address: u32, length: usize) -> &'a [T] {
    if address == 0 {
        return &[];
    }
    // SAFETY: the units Win32 wrote, as the caller counts them.
    unsafe { slice::from_raw_parts(with_exposed_provenance(address as usize), length) }
}

/// The units a call wrote by its answer: the smaller of `value` and the
/// buffer's `size`, 0 for a 0 answer or no buffer.
fn written(value: i32, size: i32) -> usize {
    usize::try_from(value.min(size)).unwrap_or(0)
}

/// Character classes as the line shows them: hex words, comma-joined.
struct Classes<'a>(&'a [u16]);

impl fmt::Display for Classes<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.iter().enumerate().try_for_each(|(index, class)| {
            if index > 0 {
                f.write_str(",")?;
            }
            write!(f, "{class:04x}")
        })
    }
}

/// The flag `WideCharToMultiByte` wrote at its used-default pointer,
/// empty for a null one.
struct Used(Option<i32>);

impl fmt::Display for Used {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            Some(flag) => mech_trace::__shown!(flag).fmt(f),
            None => Ok(()),
        }
    }
}

extern "system" fn ansi_code_page<F: File>(from: u32) -> u32 {
    // SAFETY: takes no arguments.
    let value = unsafe { GetACP() };
    kept!(
        log::debug!(c = F::ROW.calls.ansi_code_page, v:% = mech_trace::__shown!(value),
        from:% = mech_trace::__shown!(from); "imp.call")
    );
    value
}

extern "system" fn oem_code_page<F: File>(from: u32) -> u32 {
    // SAFETY: takes no arguments.
    let value = unsafe { GetOEMCP() };
    kept!(
        log::debug!(c = F::ROW.calls.oem_code_page, v:% = mech_trace::__shown!(value),
        from:% = mech_trace::__shown!(from); "imp.call")
    );
    value
}

extern "system" fn code_page_info<F: File>(from: u32, code_page: u32, info: u32) -> i32 {
    // SAFETY: the caller's arguments, passed on whole.
    let value = unsafe { GetCPInfo(code_page, info) };
    let error = left();
    // SAFETY: the 18-byte record Win32 wrote, nothing on a 0 answer.
    let record: &[u8] = unsafe { wrote(info, if value == 0 { 0 } else { 18 }) };
    kept!(
        log::debug!(c = F::ROW.calls.code_page_info, a:% = Args([mech_trace::__shown!(code_page)]),
        o:% = Bytes(record), v:% = mech_trace::__shown!(value), e:% = mech_trace::__shown!(error),
        from:% = mech_trace::__shown!(from); "imp.call")
    );
    value
}

extern "system" fn to_wide<F: File>(
    from: u32,
    code_page: u32,
    flags: u32,
    bytes: u32,
    count: i32,
    units: u32,
    size: i32,
) -> i32 {
    // SAFETY: the caller's arguments, passed on whole.
    let value = unsafe { MultiByteToWideChar(code_page, flags, bytes, count, units, size) };
    let error = left();
    // SAFETY: the caller's text, as the call took it.
    let text: &[u8] = unsafe { given(bytes, count, true) };
    // SAFETY: the units the call wrote by its answer.
    let out: &[u16] = unsafe { wrote(units, written(value, size)) };
    kept!(log::debug!(c = F::ROW.calls.to_wide, a:% = Args([
            mech_trace::__shown!(code_page),
            mech_trace::__shown!(flags),
            mech_trace::__shown!(size),
        ]), s:% = Bytes(text), o:% = Units(out),
        v:% = mech_trace::__shown!(value), e:% = mech_trace::__shown!(error),
        from:% = mech_trace::__shown!(from); "imp.call"));
    value
}

#[allow(
    clippy::too_many_arguments,
    reason = "WideCharToMultiByte's own 8, and the caller first"
)]
extern "system" fn to_bytes<F: File>(
    from: u32,
    code_page: u32,
    flags: u32,
    units: u32,
    count: i32,
    bytes: u32,
    size: i32,
    fallback: u32,
    used: u32,
) -> i32 {
    // SAFETY: the caller's arguments, passed on whole.
    let value =
        unsafe { WideCharToMultiByte(code_page, flags, units, count, bytes, size, fallback, used) };
    let error = left();
    // SAFETY: the caller's text, as the call took it.
    let text: &[u16] = unsafe { given(units, count, true) };
    // SAFETY: the bytes the call wrote by its answer.
    let out: &[u8] = unsafe { wrote(bytes, written(value, size)) };
    // SAFETY: the caller's default character, one byte as a single-byte
    // code page reads it, or null.
    let character: &[u8] = unsafe { given(fallback, 1, false) };
    // SAFETY: the caller's flag, which the call wrote, or null.
    let flag = (used != 0).then(|| unsafe { with_exposed_provenance::<i32>(used as usize).read() });
    kept!(log::debug!(c = F::ROW.calls.to_bytes, a:% = Args([
            mech_trace::__shown!(code_page),
            mech_trace::__shown!(flags),
            mech_trace::__shown!(size),
        ]), s:% = Units(text), with:% = Bytes(character), o:% = Bytes(out), r:% = Used(flag),
        v:% = mech_trace::__shown!(value), e:% = mech_trace::__shown!(error),
        from:% = mech_trace::__shown!(from); "imp.call"));
    value
}

extern "system" fn string_type<F: File>(
    from: u32,
    locale: u32,
    kind: u32,
    bytes: u32,
    count: i32,
    classes: u32,
) -> i32 {
    // SAFETY: the caller's arguments, passed on whole.
    let value = unsafe { GetStringTypeA(locale, kind, bytes, count, classes) };
    let error = left();
    // SAFETY: the caller's text, as the call took it.
    let text: &[u8] = unsafe { given(bytes, count, true) };
    // SAFETY: a class word for each byte the call read, nothing on a 0
    // answer.
    let out: &[u16] = unsafe { wrote(classes, if value == 0 { 0 } else { text.len() }) };
    kept!(log::debug!(c = F::ROW.calls.string_type, a:% = Args([
            mech_trace::__shown!(locale),
            mech_trace::__shown!(kind),
        ]), s:% = Bytes(text), o:% = Classes(out),
        v:% = mech_trace::__shown!(value), e:% = mech_trace::__shown!(error),
        from:% = mech_trace::__shown!(from); "imp.call"));
    value
}

extern "system" fn string_type_w<F: File>(
    from: u32,
    kind: u32,
    units: u32,
    count: i32,
    classes: u32,
) -> i32 {
    // SAFETY: the caller's arguments, passed on whole.
    let value = unsafe { GetStringTypeW(kind, units, count, classes) };
    let error = left();
    // SAFETY: the caller's text, as the call took it.
    let text: &[u16] = unsafe { given(units, count, true) };
    // SAFETY: a class word for each unit the call read, nothing on a 0
    // answer.
    let out: &[u16] = unsafe { wrote(classes, if value == 0 { 0 } else { text.len() }) };
    kept!(log::debug!(c = F::ROW.calls.string_type_w, a:% = Args([
            mech_trace::__shown!(0_u32),
            mech_trace::__shown!(kind),
        ]), s:% = Units(text), o:% = Classes(out),
        v:% = mech_trace::__shown!(value), e:% = mech_trace::__shown!(error),
        from:% = mech_trace::__shown!(from); "imp.call"));
    value
}

extern "system" fn map_string<F: File>(
    from: u32,
    locale: u32,
    flags: u32,
    bytes: u32,
    count: i32,
    out: u32,
    size: i32,
) -> i32 {
    // SAFETY: the caller's arguments, passed on whole.
    let value = unsafe { LCMapStringA(locale, flags, bytes, count, out, size) };
    let error = left();
    // SAFETY: the caller's text, as the call took it.
    let text: &[u8] = unsafe { given(bytes, count, true) };
    // SAFETY: the bytes the call wrote by its answer.
    let mapped: &[u8] = unsafe { wrote(out, written(value, size)) };
    kept!(log::debug!(c = F::ROW.calls.map_string, a:% = Args([
            mech_trace::__shown!(locale),
            mech_trace::__shown!(flags),
            mech_trace::__shown!(size),
        ]), s:% = Bytes(text), o:% = Bytes(mapped),
        v:% = mech_trace::__shown!(value), e:% = mech_trace::__shown!(error),
        from:% = mech_trace::__shown!(from); "imp.call"));
    value
}

extern "system" fn map_string_w<F: File>(
    from: u32,
    locale: u32,
    flags: u32,
    units: u32,
    count: i32,
    out: u32,
    size: i32,
) -> i32 {
    // SAFETY: the caller's arguments, passed on whole.
    let value = unsafe { LCMapStringW(locale, flags, units, count, out, size) };
    let error = left();
    // SAFETY: the caller's text, as the call took it.
    let text: &[u16] = unsafe { given(units, count, true) };
    // SAFETY: the units the call wrote by its answer.
    let mapped: &[u16] = unsafe { wrote(out, written(value, size)) };
    kept!(log::debug!(c = F::ROW.calls.map_string_w, a:% = Args([
            mech_trace::__shown!(locale),
            mech_trace::__shown!(flags),
            mech_trace::__shown!(size),
        ]), s:% = Units(text), o:% = Units(mapped),
        v:% = mech_trace::__shown!(value), e:% = mech_trace::__shown!(error),
        from:% = mech_trace::__shown!(from); "imp.call"));
    value
}

extern "system" fn compare<F: File>(
    from: u32,
    locale: u32,
    flags: u32,
    first: u32,
    first_count: i32,
    second: u32,
    second_count: i32,
) -> i32 {
    // SAFETY: the caller's arguments, passed on whole.
    let value = unsafe { CompareStringA(locale, flags, first, first_count, second, second_count) };
    let error = left();
    // SAFETY: the caller's two texts, as the call took them.
    let (one, two): (&[u8], &[u8]) = unsafe {
        (
            given(first, first_count, false),
            given(second, second_count, false),
        )
    };
    kept!(log::debug!(c = F::ROW.calls.compare, a:% = Args([
            mech_trace::__shown!(locale),
            mech_trace::__shown!(flags),
        ]), s:% = Bytes(one), with:% = Bytes(two),
        v:% = mech_trace::__shown!(value), e:% = mech_trace::__shown!(error),
        from:% = mech_trace::__shown!(from); "imp.call"));
    value
}

extern "system" fn compare_w<F: File>(
    from: u32,
    locale: u32,
    flags: u32,
    first: u32,
    first_count: i32,
    second: u32,
    second_count: i32,
) -> i32 {
    // SAFETY: the caller's arguments, passed on whole.
    let value = unsafe { CompareStringW(locale, flags, first, first_count, second, second_count) };
    let error = left();
    // SAFETY: the caller's two texts, as the call took them.
    let (one, two): (&[u16], &[u16]) = unsafe {
        (
            given(first, first_count, false),
            given(second, second_count, false),
        )
    };
    kept!(log::debug!(c = F::ROW.calls.compare_w, a:% = Args([
            mech_trace::__shown!(locale),
            mech_trace::__shown!(flags),
        ]), s:% = Units(one), with:% = Units(two),
        v:% = mech_trace::__shown!(value), e:% = mech_trace::__shown!(error),
        from:% = mech_trace::__shown!(from); "imp.call"));
    value
}

extern "system" fn locale_info<F: File>(
    from: u32,
    locale: u32,
    kind: u32,
    out: u32,
    size: i32,
) -> i32 {
    // SAFETY: the caller's arguments, passed on whole.
    let value = unsafe { GetLocaleInfoA(locale, kind, out, size) };
    let error = left();
    // SAFETY: the bytes the call wrote by its answer.
    let info: &[u8] = unsafe { wrote(out, written(value, size)) };
    kept!(log::debug!(c = F::ROW.calls.locale_info, a:% = Args([
            mech_trace::__shown!(locale),
            mech_trace::__shown!(kind),
        ]), o:% = Bytes(info),
        v:% = mech_trace::__shown!(value), e:% = mech_trace::__shown!(error),
        from:% = mech_trace::__shown!(from); "imp.call"));
    value
}

extern "system" fn locale_info_w<F: File>(
    from: u32,
    locale: u32,
    kind: u32,
    out: u32,
    size: i32,
) -> i32 {
    // SAFETY: the caller's arguments, passed on whole.
    let value = unsafe { GetLocaleInfoW(locale, kind, out, size) };
    let error = left();
    // SAFETY: the units the call wrote by its answer.
    let info: &[u16] = unsafe { wrote(out, written(value, size)) };
    kept!(log::debug!(c = F::ROW.calls.locale_info_w, a:% = Args([
            mech_trace::__shown!(locale),
            mech_trace::__shown!(kind),
        ]), o:% = Units(info),
        v:% = mech_trace::__shown!(value), e:% = mech_trace::__shown!(error),
        from:% = mech_trace::__shown!(from); "imp.call"));
    value
}

entry!(LAUNCHER_GetACP, ansi_code_page::<Launcher>, 0);
entry!(LAUNCHER_GetOEMCP, oem_code_page::<Launcher>, 0);
entry!(LAUNCHER_GetCPInfo, code_page_info::<Launcher>, 2);
entry!(LAUNCHER_MultiByteToWideChar, to_wide::<Launcher>, 6);
entry!(LAUNCHER_WideCharToMultiByte, to_bytes::<Launcher>, 8);
entry!(LAUNCHER_GetStringTypeA, string_type::<Launcher>, 5);
entry!(LAUNCHER_GetStringTypeW, string_type_w::<Launcher>, 4);
entry!(LAUNCHER_LCMapStringA, map_string::<Launcher>, 6);
entry!(LAUNCHER_LCMapStringW, map_string_w::<Launcher>, 6);

entry!(HC6_GetACP, ansi_code_page::<Shell>, 0);
entry!(HC6_GetOEMCP, oem_code_page::<Shell>, 0);
entry!(HC6_GetCPInfo, code_page_info::<Shell>, 2);
entry!(HC6_MultiByteToWideChar, to_wide::<Shell>, 6);
entry!(HC6_WideCharToMultiByte, to_bytes::<Shell>, 8);
entry!(HC6_GetStringTypeA, string_type::<Shell>, 5);
entry!(HC6_GetStringTypeW, string_type_w::<Shell>, 4);
entry!(HC6_LCMapStringA, map_string::<Shell>, 6);
entry!(HC6_LCMapStringW, map_string_w::<Shell>, 6);
entry!(HC6_CompareStringA, compare::<Shell>, 6);
entry!(HC6_CompareStringW, compare_w::<Shell>, 6);
entry!(HC6_GetLocaleInfoA, locale_info::<Shell>, 4);
entry!(HC6_GetLocaleInfoW, locale_info_w::<Shell>, 4);

entry!(GAME_GetACP, ansi_code_page::<Game>, 0);
entry!(GAME_GetOEMCP, oem_code_page::<Game>, 0);
entry!(GAME_GetCPInfo, code_page_info::<Game>, 2);
entry!(GAME_MultiByteToWideChar, to_wide::<Game>, 6);
entry!(GAME_WideCharToMultiByte, to_bytes::<Game>, 8);
entry!(GAME_GetStringTypeA, string_type::<Game>, 5);
entry!(GAME_GetStringTypeW, string_type_w::<Game>, 4);
entry!(GAME_LCMapStringA, map_string::<Game>, 6);
entry!(GAME_LCMapStringW, map_string_w::<Game>, 6);
entry!(GAME_CompareStringA, compare::<Game>, 6);
entry!(GAME_CompareStringW, compare_w::<Game>, 6);
entry!(GAME_GetLocaleInfoA, locale_info::<Game>, 4);
entry!(GAME_GetLocaleInfoW, locale_info_w::<Game>, 4);
