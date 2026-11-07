// SPDX-License-Identifier: MIT
//! The text calls: the 13 KERNEL32 code page,
//! conversion, character and locale names, each answered by `kernel_text`'s
//! front in the file's row, Wine not asked. A row turns Win32's addresses and
//! counts into the front's slices and writes nothing of its own: the front
//! writes the call's one `imp.call` line, `by=kernel-text`, as the file rows'
//! front does. So `o`, what the original wrote, and `from`, the caller, left
//! the line with the original.
//!
//! 35 exports, one a name a file imports: the launcher's 9 `LAUNCHER_`
//! (no `CompareString`, no `GetLocaleInfo`), the shell's 13 `HC6_`, the
//! game file's 13 `GAME_`. Each is a naked entry that hands the row the
//! address the call returns to first, as [`crate::process`]' entries do; no
//! row here reads it.
//!
//! A count below 0 is the NUL-ended text, the NUL kept for the conversions,
//! `GetStringType` and `LCMapString`, which take it, and left out for
//! `CompareString`, which stops at it. A null text reads as empty. A null
//! buffer or a size of 0 or below only counts, where Win32 refuses a null
//! buffer with a size; no measured call passes one.
use core::ptr::{with_exposed_provenance, with_exposed_provenance_mut};
use core::slice;
use kernel_text::table::CodePage;
use kernel_text::{Front, Locale, pages};
use mech_rows::text::{GAME, LAUNCHER, SHELL};

/// The tables are the portable back's; a refusal's error and the line go
/// around the thread's own last error, which the sink's `WriteFile` would
/// otherwise change under the caller.
struct Kept;

impl Locale for Kept {
    fn code_page(&self, number: u32) -> Option<&CodePage> {
        pages::ALL.iter().find(|page| page.number == number)
    }

    fn fail(&self, code: u32) {
        // SAFETY: sets the thread's own last error.
        unsafe { crate::SetLastError(code) };
    }

    fn quiet(&self, line: impl FnOnce()) {
        kept!(line());
    }
}

/// A file the rows serve, by its row in `mech_rows::text`.
trait File {
    const FRONT: Front<Kept>;
}

/// `mercswin`, the launcher.
enum Launcher {}
/// `mw2shell`, the shell.
enum Shell {}
/// `mercsw`, the game file.
enum Game {}

impl File for Launcher {
    const FRONT: Front<Kept> = Front::new(&LAUNCHER, Kept);
}

impl File for Shell {
    const FRONT: Front<Kept> = Front::new(&SHELL, Kept);
}

impl File for Game {
    const FRONT: Front<Kept> = Front::new(&GAME, Kept);
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

/// The caller's buffer of `size` units, `None` where the call only counts.
///
/// # Safety
/// `address` is 0 or a buffer of `size` units the caller gave to be written.
unsafe fn buffer<'a, T>(address: u32, size: i32) -> Option<&'a mut [T]> {
    let length = usize::try_from(size).ok().filter(|length| *length > 0)?;
    if address == 0 {
        return None;
    }
    // SAFETY: the caller's `length` units at `address`.
    Some(unsafe {
        slice::from_raw_parts_mut(with_exposed_provenance_mut(address as usize), length)
    })
}

/// A class word's room for each of `count` units, empty for a null buffer.
///
/// # Safety
/// `address` is 0 or a buffer of `count` words, as `GetStringType` takes it.
unsafe fn class_words<'a>(address: u32, count: usize) -> &'a mut [u16] {
    let size = i32::try_from(count).unwrap_or(i32::MAX);
    // SAFETY: as this function's own contract.
    unsafe { buffer(address, size) }.unwrap_or_default()
}

extern "system" fn ansi_code_page<F: File>(_from: u32) -> u32 {
    F::FRONT.ansi_code_page()
}

extern "system" fn oem_code_page<F: File>(_from: u32) -> u32 {
    F::FRONT.oem_code_page()
}

extern "system" fn code_page_info<F: File>(_from: u32, code_page: u32, info: u32) -> i32 {
    let Some(record) = F::FRONT.code_page_info(code_page) else {
        return 0;
    };
    let out = with_exposed_provenance_mut::<u8>(info as usize);
    // SAFETY: the caller's `CPINFO`, 18 bytes: the size, 2 default bytes, 12
    // lead bytes.
    unsafe {
        out.cast::<u32>().write_unaligned(record.max_char_size);
        out.add(4)
            .copy_from_nonoverlapping(record.default.as_ptr(), 2);
        out.add(6)
            .copy_from_nonoverlapping(record.lead_bytes.as_ptr(), 12);
    }
    1
}

extern "system" fn to_wide<F: File>(
    _from: u32,
    code_page: u32,
    flags: u32,
    bytes: u32,
    count: i32,
    units: u32,
    size: i32,
) -> i32 {
    // SAFETY: the caller's text and buffer, as Win32 takes them.
    let (text, out) = unsafe { (given(bytes, count, true), buffer(units, size)) };
    F::FRONT.to_wide(code_page, flags, text, out)
}

#[allow(
    clippy::too_many_arguments,
    reason = "WideCharToMultiByte's own 8, and the caller first"
)]
extern "system" fn to_bytes<F: File>(
    _from: u32,
    code_page: u32,
    flags: u32,
    units: u32,
    count: i32,
    bytes: u32,
    size: i32,
    fallback: u32,
    used: u32,
) -> i32 {
    // SAFETY: the caller's text and buffer, as Win32 takes them.
    let (text, out) = unsafe { (given(units, count, true), buffer(bytes, size)) };
    // SAFETY: the caller's default character, one byte as a single-byte code
    // page reads it, or null.
    let default =
        (fallback != 0).then(|| unsafe { with_exposed_provenance::<u8>(fallback as usize).read() });
    let (value, defaulted) = F::FRONT.to_bytes(code_page, flags, text, out, default);
    if used != 0 {
        // SAFETY: the caller's flag, given to be written.
        unsafe { with_exposed_provenance_mut::<i32>(used as usize).write(i32::from(defaulted)) };
    }
    value
}

extern "system" fn string_type<F: File>(
    _from: u32,
    locale: u32,
    kind: u32,
    bytes: u32,
    count: i32,
    classes: u32,
) -> i32 {
    // SAFETY: the caller's text and a class word for each byte of it.
    let (text, out) = unsafe {
        let text: &[u8] = given(bytes, count, true);
        (text, class_words(classes, text.len()))
    };
    F::FRONT.string_type(locale, kind, text, out)
}

extern "system" fn string_type_w<F: File>(
    _from: u32,
    kind: u32,
    units: u32,
    count: i32,
    classes: u32,
) -> i32 {
    // SAFETY: the caller's text and a class word for each unit of it.
    let (text, out) = unsafe {
        let text: &[u16] = given(units, count, true);
        (text, class_words(classes, text.len()))
    };
    F::FRONT.string_type_w(kind, text, out)
}

extern "system" fn map_string<F: File>(
    _from: u32,
    locale: u32,
    flags: u32,
    bytes: u32,
    count: i32,
    out: u32,
    size: i32,
) -> i32 {
    // SAFETY: the caller's text and buffer, as Win32 takes them.
    let (text, out) = unsafe { (given(bytes, count, true), buffer(out, size)) };
    F::FRONT.map_string(locale, flags, text, out)
}

extern "system" fn map_string_w<F: File>(
    _from: u32,
    locale: u32,
    flags: u32,
    units: u32,
    count: i32,
    out: u32,
    size: i32,
) -> i32 {
    // SAFETY: the caller's text and buffer, as Win32 takes them.
    let (text, out) = unsafe { (given(units, count, true), buffer(out, size)) };
    F::FRONT.map_string_w(locale, flags, text, out)
}

extern "system" fn compare<F: File>(
    _from: u32,
    locale: u32,
    flags: u32,
    first: u32,
    first_count: i32,
    second: u32,
    second_count: i32,
) -> i32 {
    // SAFETY: the caller's two texts, as Win32 takes them.
    let (one, two) = unsafe {
        (
            given(first, first_count, false),
            given(second, second_count, false),
        )
    };
    F::FRONT.compare(locale, flags, one, two)
}

extern "system" fn compare_w<F: File>(
    _from: u32,
    locale: u32,
    flags: u32,
    first: u32,
    first_count: i32,
    second: u32,
    second_count: i32,
) -> i32 {
    // SAFETY: the caller's two texts, as Win32 takes them.
    let (one, two) = unsafe {
        (
            given(first, first_count, false),
            given(second, second_count, false),
        )
    };
    F::FRONT.compare_w(locale, flags, one, two)
}

extern "system" fn locale_info<F: File>(
    _from: u32,
    locale: u32,
    kind: u32,
    out: u32,
    size: i32,
) -> i32 {
    // SAFETY: the caller's buffer, as Win32 takes it.
    let out = unsafe { buffer(out, size) };
    F::FRONT.locale_info(locale, kind, out)
}

extern "system" fn locale_info_w<F: File>(
    _from: u32,
    locale: u32,
    kind: u32,
    out: u32,
    size: i32,
) -> i32 {
    // SAFETY: the caller's buffer, as Win32 takes it.
    let out = unsafe { buffer(out, size) };
    F::FRONT.locale_info_w(locale, kind, out)
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
