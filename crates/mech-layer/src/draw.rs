// SPDX-License-Identifier: MIT
//! The drawing calls (DB1, `JOB.md` step 2): mercs's 23 GDI32 and paint
//! names of the drawing stand-in's step 1, each the original's, logged
//! with `from`, the caller, so `draw`'s fronts are checked against what
//! the files asked and read back.
//!
//! 41 exports, one a name a file imports: the shell's 19 `HC6_`, the game
//! file's 18 `GAME_`, the launcher's 4 `LAUNCHER_`. Each is a naked entry
//! that reads the address the call returns to first, and shows its records
//! as [`crate::window`]'s lines do, with that module's readers. The
//! shell's 4 rows HC6 logged with no caller (`BitBlt`, `SetPaletteEntries`,
//! `RealizePalette`, `SetDIBColorTable`) moved here from
//! [`crate::imports`], their exports' names unchanged.
//!
//! A line's `a` holds numbers: a `BITMAPINFOHEADER` a call is handed shows
//! its 10 words after the call's own numbers, never its address. `o` the
//! record a call wrote: `CreateDIBSection`'s bits pointer, `BeginPaint`'s
//! paint record, empty on a 0 answer. A palette's entries go on `draw.table`
//! lines after the call's line, 16 entries a line, each entry's 32 bits as
//! the caller's memory holds them: `SetPaletteEntries`, `SetDIBColorTable`,
//! `CreatePalette`, `CreateDIBSection`'s table (usage 1 a table of 16-bit
//! indexes, read as 32-bit words) and `GetSystemPaletteEntries`' written
//! entries. `SetDIBitsToDevice` and `StretchDIBits` show the header alone:
//! they run once a frame, and the table is `CreateDIBSection`'s to show.
//!
//! The dump (falsifier 3): with `DRAW_DUMP` set to a folder, `BitBlt`'s
//! source writes its bits and its colour table there, `bitblt-<n>.bits`
//! and `.table`, `n` the call's number in the run, when the picture differs
//! from the last one written, 8 pictures at most; each a `draw.dump` line.
//! Unset, the row costs one environment read.
use crate::window::{Numbers, record, wrote};
use core::ffi::{c_char, c_void};
use core::ptr::{null_mut, with_exposed_provenance};
use core::sync::atomic::{AtomicU32, Ordering};

unsafe extern "system" {
    fn CreatePalette(palette: u32) -> u32;
    fn RealizePalette(dc: u32) -> u32;
    fn SelectPalette(dc: u32, palette: u32, background: u32) -> u32;
    fn SetDIBColorTable(dc: u32, start: u32, count: u32, colors: u32) -> u32;
    fn SetPaletteEntries(palette: u32, start: u32, count: u32, entries: u32) -> u32;
    fn GetSystemPaletteEntries(dc: u32, start: u32, count: u32, entries: u32) -> u32;
    fn StretchBlt(
        dc: u32,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        source: u32,
        sx: u32,
        sy: u32,
        swidth: u32,
        sheight: u32,
        rop: u32,
    ) -> u32;
    fn CreateDIBSection(
        dc: u32,
        info: u32,
        usage: u32,
        bits: u32,
        section: u32,
        offset: u32,
    ) -> u32;
    fn BitBlt(
        dc: u32,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        source: u32,
        sx: u32,
        sy: u32,
        rop: u32,
    ) -> u32;
    fn SetDIBitsToDevice(
        dc: u32,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        sx: u32,
        sy: u32,
        start: u32,
        lines: u32,
        bits: u32,
        info: u32,
        usage: u32,
    ) -> u32;
    fn StretchDIBits(
        dc: u32,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        sx: u32,
        sy: u32,
        swidth: u32,
        sheight: u32,
        bits: u32,
        info: u32,
        usage: u32,
        rop: u32,
    ) -> u32;
    fn CreateCompatibleDC(dc: u32) -> u32;
    fn DeleteDC(dc: u32) -> u32;
    fn DeleteObject(object: u32) -> u32;
    fn SelectObject(dc: u32, object: u32) -> u32;
    fn GetDeviceCaps(dc: u32, index: u32) -> u32;
    fn GetStockObject(index: u32) -> u32;
    fn GetDC(window: u32) -> u32;
    fn ReleaseDC(window: u32, dc: u32) -> u32;
    fn UpdateWindow(window: u32) -> u32;
    fn BeginPaint(window: u32, paint: u32) -> u32;
    fn EndPaint(window: u32, paint: u32) -> u32;
    fn ValidateRect(window: u32, rectangle: u32) -> u32;

    // The dump's own, no file's: they answer no row and write no line.
    fn GetCurrentObject(dc: u32, kind: u32) -> u32;
    fn GetObjectA(object: u32, size: u32, into: *mut u32) -> u32;
    fn GetDIBColorTable(dc: u32, start: u32, count: u32, into: *mut u32) -> u32;
    fn GetEnvironmentVariableA(name: *const u8, into: *mut u8, size: u32) -> u32;
    fn CreateFileA(
        name: *const c_char,
        access: u32,
        share: u32,
        security: *mut c_void,
        disposition: u32,
        flags: u32,
        template: *mut c_void,
    ) -> *mut c_void;
    fn WriteFile(
        file: *mut c_void,
        buffer: *const c_void,
        bytes: u32,
        written: *mut u32,
        overlapped: *mut c_void,
    ) -> i32;
    fn CloseHandle(object: *mut c_void) -> i32;
}

/// One `imp.call` line, `a` and `v` and `from`, the thread's last error kept.
macro_rules! called {
    ($c:literal, $numbers:expr, $value:expr, $from:expr) => {
        kept!(log::debug!(c = $c, a:% = Numbers($numbers),
            v:% = mech_trace::__shown!($value), from:% = mech_trace::__shown!($from); "imp.call"))
    };
}

/// A table's entries a line.
const TABLE_LINE: u32 = 16;

/// The `draw.table` lines of `count` 32-bit entries at `address`, numbered
/// from `start`; none for a null table.
///
/// # Safety
/// `address` is 0 or the caller's table of `count` entries.
pub(crate) unsafe fn table(c: &'static str, start: u32, count: u32, address: u32) {
    if address == 0 {
        return;
    }
    let mut done = 0;
    while done < count {
        let length = (count - done).min(TABLE_LINE);
        let mut line = [0_u32; TABLE_LINE as usize];
        for (index, entry) in line.iter_mut().take(length as usize).enumerate() {
            let at = address.wrapping_add((done + index as u32) * 4);
            // SAFETY: inside the caller's table, by this function's contract.
            *entry = unsafe { record::<1>(at) }[0];
        }
        let first = start + done;
        log::debug!(c = c, i:% = mech_trace::__shown!(first),
            a:% = Numbers(&line[..length as usize]); "draw.table");
        done += length;
    }
}

/// `CreatePalette`: `a` the `LOGPALETTE`'s first word, its version and its
/// count, the entries on `draw.table` lines.
extern "system" fn create_palette(from: u32, palette: u32) -> u32 {
    // SAFETY: the caller's argument, passed on whole.
    let value = unsafe { CreatePalette(palette) };
    // SAFETY: the caller's record, its first word its version and count.
    let head = unsafe { record::<1>(palette) }[0];
    called!("GDI32!CreatePalette", &[head], value, from);
    // SAFETY: the record's entries follow its first word, `head >> 16` of them.
    kept!(unsafe {
        table(
            "GDI32!CreatePalette",
            0,
            head >> 16,
            palette.wrapping_add(4),
        )
    });
    value
}

extern "system" fn realize_palette(from: u32, dc: u32) -> u32 {
    // SAFETY: the caller's argument, passed on whole.
    let value = unsafe { RealizePalette(dc) };
    called!("GDI32!RealizePalette", &[dc], value, from);
    value
}

extern "system" fn select_palette(from: u32, dc: u32, palette: u32, background: u32) -> u32 {
    // SAFETY: the caller's arguments, passed on whole.
    let value = unsafe { SelectPalette(dc, palette, background) };
    called!(
        "GDI32!SelectPalette",
        &[dc, palette, background],
        value,
        from
    );
    value
}

/// `SetDIBColorTable`: the colours handed on `draw.table` lines.
extern "system" fn set_dib_color_table(
    from: u32,
    dc: u32,
    start: u32,
    count: u32,
    colors: u32,
) -> u32 {
    // SAFETY: the caller's arguments, passed on whole.
    let value = unsafe { SetDIBColorTable(dc, start, count, colors) };
    called!("GDI32!SetDIBColorTable", &[dc, start, count], value, from);
    // SAFETY: the caller's `count` colours.
    kept!(unsafe { table("GDI32!SetDIBColorTable", start, count, colors) });
    value
}

/// `SetPaletteEntries`: the entries handed on `draw.table` lines.
extern "system" fn set_palette_entries(
    from: u32,
    palette: u32,
    start: u32,
    count: u32,
    entries: u32,
) -> u32 {
    // SAFETY: the caller's arguments, passed on whole.
    let value = unsafe { SetPaletteEntries(palette, start, count, entries) };
    called!(
        "GDI32!SetPaletteEntries",
        &[palette, start, count],
        value,
        from
    );
    // SAFETY: the caller's `count` entries.
    kept!(unsafe { table("GDI32!SetPaletteEntries", start, count, entries) });
    value
}

/// `GetSystemPaletteEntries`: the entries written on `draw.table` lines,
/// `v` of them; a null pointer asks only the size, and has none.
extern "system" fn get_system_palette_entries(
    from: u32,
    dc: u32,
    start: u32,
    count: u32,
    entries: u32,
) -> u32 {
    // SAFETY: the caller's arguments, passed on whole.
    let value = unsafe { GetSystemPaletteEntries(dc, start, count, entries) };
    called!(
        "GDI32!GetSystemPaletteEntries",
        &[dc, start, count],
        value,
        from
    );
    // SAFETY: the call wrote `value` entries at the caller's pointer.
    kept!(unsafe { table("GDI32!GetSystemPaletteEntries", start, value, entries) });
    value
}

#[expect(
    clippy::too_many_arguments,
    reason = "StretchBlt's own 11, and the caller first"
)]
extern "system" fn stretch_blt(
    from: u32,
    dc: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    source: u32,
    sx: u32,
    sy: u32,
    swidth: u32,
    sheight: u32,
    rop: u32,
) -> u32 {
    // SAFETY: the caller's arguments, passed on whole.
    let value = unsafe {
        StretchBlt(
            dc, x, y, width, height, source, sx, sy, swidth, sheight, rop,
        )
    };
    called!(
        "GDI32!StretchBlt",
        &[
            dc, x, y, width, height, source, sx, sy, swidth, sheight, rop
        ],
        value,
        from
    );
    value
}

/// `CreateDIBSection`: `a` the context, the usage, the section, the offset
/// and the header's 10 words; `o` the bits pointer it wrote; the table
/// behind the header on `draw.table` lines.
extern "system" fn create_dib_section(
    from: u32,
    dc: u32,
    info: u32,
    usage: u32,
    bits: u32,
    section: u32,
    offset: u32,
) -> u32 {
    // SAFETY: the caller's header, read before the call.
    let h = unsafe { record::<10>(info) };
    // SAFETY: the caller's arguments, passed on whole.
    let value = unsafe { CreateDIBSection(dc, info, usage, bits, section, offset) };
    // SAFETY: the caller's out-pointer, read on a nonzero answer alone.
    let got = unsafe { wrote::<1>(value, bits) };
    let numbers = [
        dc, usage, section, offset, h[0], h[1], h[2], h[3], h[4], h[5], h[6], h[7], h[8], h[9],
    ];
    kept!(
        log::debug!(c = "GDI32!CreateDIBSection", a:% = Numbers(&numbers), o:% = got,
        v:% = mech_trace::__shown!(value), from:% = mech_trace::__shown!(from); "imp.call")
    );
    let depth = h[3] >> 16;
    let entries = match h[8] {
        0 if depth <= 8 => 1_u32 << depth,
        used => used,
    };
    let words = if usage == 1 {
        entries.div_ceil(2)
    } else {
        entries
    };
    // SAFETY: the table lies behind the header, its size the header's first word.
    kept!(unsafe { table("GDI32!CreateDIBSection", 0, words, info.wrapping_add(h[0])) });
    value
}

/// The bits written, the calls seen and the last picture's sum (the dump).
static WRITTEN: AtomicU32 = AtomicU32::new(0);
static SEEN: AtomicU32 = AtomicU32::new(0);
static LAST_SUM: AtomicU32 = AtomicU32::new(0);

/// The pictures one run writes at most.
const DUMPS: u32 = 8;
/// A dump file's path, as Windows' own limit.
const PATH: usize = 260;
/// The folder's room in [`PATH`], the file's name taking the rest.
const FOLDER: usize = 220;

/// `folder\bitblt-<n, 8 hex digits><suffix>`, NUL-terminated.
fn dump_path(folder: &[u8], n: u32, suffix: &[u8]) -> [u8; PATH] {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut path = [0_u8; PATH];
    let mut at = 0;
    let mut put = |byte: u8| {
        path[at] = byte;
        at += 1;
    };
    folder.iter().copied().for_each(&mut put);
    b"\\bitblt-".iter().copied().for_each(&mut put);
    (0..8)
        .rev()
        .for_each(|nibble| put(DIGITS[(n >> (nibble * 4)) as usize & 15]));
    suffix.iter().copied().for_each(&mut put);
    path
}

/// Writes `bytes` to the NUL-terminated `path`, over any file there.
///
/// # Safety
/// `path` holds a NUL.
unsafe fn write_file(path: &[u8; PATH], bytes: &[u8]) {
    const GENERIC_WRITE: u32 = 0x4000_0000;
    const CREATE_ALWAYS: u32 = 2;
    const FILE_ATTRIBUTE_NORMAL: u32 = 0x80;
    // SAFETY: the path holds a NUL, by this function's contract.
    let file = unsafe {
        CreateFileA(
            path.as_ptr().cast(),
            GENERIC_WRITE,
            0,
            null_mut(),
            CREATE_ALWAYS,
            FILE_ATTRIBUTE_NORMAL,
            null_mut(),
        )
    };
    if file as usize == usize::MAX {
        return;
    }
    let mut written = 0_u32;
    // SAFETY: the file just opened, the caller's bytes, the count on this stack.
    let _ = unsafe {
        WriteFile(
            file,
            bytes.as_ptr().cast(),
            bytes.len() as u32,
            &raw mut written,
            null_mut(),
        )
    };
    // SAFETY: the file just opened, closed once.
    let _ = unsafe { CloseHandle(file) };
}

/// FNV-1a, 32 bits: tells one picture from the last, nothing more; the
/// record's SHA-256 is taken from the files.
fn picture_sum(bytes: &[u8]) -> u32 {
    bytes.iter().fold(0x811c_9dc5, |sum, byte| {
        (sum ^ u32::from(*byte)).wrapping_mul(0x0100_0193)
    })
}

/// Falsifier 3's dump of `BitBlt`'s source, when `DRAW_DUMP` names a folder
/// (the module's doc): a source that is no DIB section writes nothing.
fn dump(from: u32, source: u32) {
    const OBJ_BITMAP: u32 = 7;
    const BITMAP_BYTES: u32 = 24;
    let mut folder = [0_u8; FOLDER];
    // SAFETY: the name is NUL-terminated; the buffer is `FOLDER` long.
    let length = unsafe {
        GetEnvironmentVariableA(
            c"DRAW_DUMP".as_ptr().cast(),
            folder.as_mut_ptr(),
            FOLDER as u32,
        )
    } as usize;
    if length == 0 || length >= FOLDER {
        return;
    }
    let n = SEEN.fetch_add(1, Ordering::Relaxed);
    if WRITTEN.load(Ordering::Relaxed) >= DUMPS {
        return;
    }
    // SAFETY: the caller's context; a wrong one answers 0.
    let bitmap = unsafe { GetCurrentObject(source, OBJ_BITMAP) };
    let mut fields = [0_u32; 6];
    // SAFETY: `BITMAP` is 6 words, the buffer here; a wrong object answers 0.
    if unsafe { GetObjectA(bitmap, BITMAP_BYTES, fields.as_mut_ptr()) } == 0 || fields[5] == 0 {
        return;
    }
    let [_, width, height, pitch, depth, bits] = fields;
    // SAFETY: a DIB section's bits, `pitch` by `height` bytes, by its own record.
    let picture = unsafe {
        core::slice::from_raw_parts(
            with_exposed_provenance::<u8>(bits as usize),
            pitch.wrapping_mul(height) as usize,
        )
    };
    let sum = picture_sum(picture);
    if LAST_SUM.swap(sum, Ordering::Relaxed) == sum {
        return;
    }
    WRITTEN.fetch_add(1, Ordering::Relaxed);
    let mut colors = [0_u32; 256];
    // SAFETY: the buffer holds the 256 entries asked.
    let count = unsafe { GetDIBColorTable(source, 0, 256, colors.as_mut_ptr()) };
    // SAFETY: the same array, its `count` entries as bytes.
    let colors = unsafe {
        core::slice::from_raw_parts(colors.as_ptr().cast::<u8>(), count.min(256) as usize * 4)
    };
    // SAFETY: `dump_path` leaves the last bytes of its zeroed array NUL.
    unsafe {
        write_file(&dump_path(&folder[..length], n, b".bits"), picture);
        write_file(&dump_path(&folder[..length], n, b".table"), colors);
    }
    log::debug!(n:% = mech_trace::__shown!(n), a:% = Numbers(&[width, height, pitch, depth, bits, count]),
        v:% = mech_trace::__shown!(sum), from:% = mech_trace::__shown!(from); "draw.dump");
}

/// `BitBlt`: `v` the answer as it came, the shell's route comparing it with
/// the height (`s1-readback.md`); the dump runs here.
#[expect(
    clippy::too_many_arguments,
    reason = "BitBlt's own 9, and the caller first"
)]
extern "system" fn bit_blt(
    from: u32,
    dc: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    source: u32,
    sx: u32,
    sy: u32,
    rop: u32,
) -> u32 {
    // SAFETY: the caller's arguments, passed on whole.
    let value = unsafe { BitBlt(dc, x, y, width, height, source, sx, sy, rop) };
    called!(
        "GDI32!BitBlt",
        &[dc, x, y, width, height, source, sx, sy, rop],
        value,
        from
    );
    kept!(dump(from, source));
    value
}

/// `SetDIBitsToDevice`: `a` its numbers less the two pointers, then the
/// header's 10 words.
#[expect(
    clippy::too_many_arguments,
    reason = "SetDIBitsToDevice's own 12, and the caller first"
)]
extern "system" fn set_dib_bits_to_device(
    from: u32,
    dc: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    sx: u32,
    sy: u32,
    start: u32,
    lines: u32,
    bits: u32,
    info: u32,
    usage: u32,
) -> u32 {
    // SAFETY: the caller's header, read before the call.
    let h = unsafe { record::<10>(info) };
    // SAFETY: the caller's arguments, passed on whole.
    let value = unsafe {
        SetDIBitsToDevice(
            dc, x, y, width, height, sx, sy, start, lines, bits, info, usage,
        )
    };
    let numbers = [
        dc, x, y, width, height, sx, sy, start, lines, usage, h[0], h[1], h[2], h[3], h[4], h[5],
        h[6], h[7], h[8], h[9],
    ];
    called!("GDI32!SetDIBitsToDevice", &numbers, value, from);
    value
}

/// `StretchDIBits`: `a` its numbers less the two pointers, then the
/// header's 10 words.
#[expect(
    clippy::too_many_arguments,
    reason = "StretchDIBits' own 13, and the caller first"
)]
extern "system" fn stretch_dib_bits(
    from: u32,
    dc: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    sx: u32,
    sy: u32,
    swidth: u32,
    sheight: u32,
    bits: u32,
    info: u32,
    usage: u32,
    rop: u32,
) -> u32 {
    // SAFETY: the caller's header, read before the call.
    let h = unsafe { record::<10>(info) };
    // SAFETY: the caller's arguments, passed on whole.
    let value = unsafe {
        StretchDIBits(
            dc, x, y, width, height, sx, sy, swidth, sheight, bits, info, usage, rop,
        )
    };
    let numbers = [
        dc, x, y, width, height, sx, sy, swidth, sheight, usage, rop, h[0], h[1], h[2], h[3], h[4],
        h[5], h[6], h[7], h[8], h[9],
    ];
    called!("GDI32!StretchDIBits", &numbers, value, from);
    value
}

extern "system" fn create_compatible_dc(from: u32, dc: u32) -> u32 {
    // SAFETY: the caller's argument, passed on whole.
    let value = unsafe { CreateCompatibleDC(dc) };
    called!("GDI32!CreateCompatibleDC", &[dc], value, from);
    value
}

extern "system" fn delete_dc(from: u32, dc: u32) -> u32 {
    // SAFETY: the caller's argument, passed on whole.
    let value = unsafe { DeleteDC(dc) };
    called!("GDI32!DeleteDC", &[dc], value, from);
    value
}

extern "system" fn delete_object(from: u32, object: u32) -> u32 {
    // SAFETY: the caller's argument, passed on whole.
    let value = unsafe { DeleteObject(object) };
    called!("GDI32!DeleteObject", &[object], value, from);
    value
}

extern "system" fn select_object(from: u32, dc: u32, object: u32) -> u32 {
    // SAFETY: the caller's arguments, passed on whole.
    let value = unsafe { SelectObject(dc, object) };
    called!("GDI32!SelectObject", &[dc, object], value, from);
    value
}

/// `GetDeviceCaps`: `v` the answer a front's row must give at that index.
extern "system" fn get_device_caps(from: u32, dc: u32, index: u32) -> u32 {
    // SAFETY: the caller's arguments, passed on whole.
    let value = unsafe { GetDeviceCaps(dc, index) };
    called!("GDI32!GetDeviceCaps", &[dc, index], value, from);
    value
}

extern "system" fn get_stock_object(from: u32, index: u32) -> u32 {
    // SAFETY: the caller's argument, passed on whole.
    let value = unsafe { GetStockObject(index) };
    called!("GDI32!GetStockObject", &[index], value, from);
    value
}

extern "system" fn get_dc(from: u32, window: u32) -> u32 {
    // SAFETY: the caller's argument, passed on whole.
    let value = unsafe { GetDC(window) };
    called!("USER32!GetDC", &[window], value, from);
    value
}

extern "system" fn release_dc(from: u32, window: u32, dc: u32) -> u32 {
    // SAFETY: the caller's arguments, passed on whole.
    let value = unsafe { ReleaseDC(window, dc) };
    called!("USER32!ReleaseDC", &[window, dc], value, from);
    value
}

extern "system" fn update_window(from: u32, window: u32) -> u32 {
    // SAFETY: the caller's argument, passed on whole.
    let value = unsafe { UpdateWindow(window) };
    called!("USER32!UpdateWindow", &[window], value, from);
    value
}

/// `BeginPaint`: `o` the paint record's first 6 words: the context, the
/// erase flag, `rcPaint`'s 4 fields.
extern "system" fn begin_paint(from: u32, window: u32, paint: u32) -> u32 {
    // SAFETY: the caller's arguments, passed on whole.
    let value = unsafe { BeginPaint(window, paint) };
    // SAFETY: the caller's record, read on a nonzero answer alone.
    let got = unsafe { wrote::<6>(value, paint) };
    kept!(
        log::debug!(c = "USER32!BeginPaint", a:% = Numbers(&[window]), o:% = got,
        v:% = mech_trace::__shown!(value), from:% = mech_trace::__shown!(from); "imp.call")
    );
    value
}

/// `EndPaint`: `a` the window and the paint record's first 6 words.
extern "system" fn end_paint(from: u32, window: u32, paint: u32) -> u32 {
    // SAFETY: the caller's record, read before the call.
    let p = unsafe { record::<6>(paint) };
    // SAFETY: the caller's arguments, passed on whole.
    let value = unsafe { EndPaint(window, paint) };
    called!(
        "USER32!EndPaint",
        &[window, p[0], p[1], p[2], p[3], p[4], p[5]],
        value,
        from
    );
    value
}

/// `ValidateRect`: `a` the window and the rectangle's 4 fields, the window
/// alone for a null one.
extern "system" fn validate_rect(from: u32, window: u32, rectangle: u32) -> u32 {
    // SAFETY: the caller's rectangle, read before the call.
    let r = unsafe { record::<4>(rectangle) };
    // SAFETY: the caller's arguments, passed on whole.
    let value = unsafe { ValidateRect(window, rectangle) };
    let numbers = [window, r[0], r[1], r[2], r[3]];
    let handed: &[u32] = if rectangle == 0 {
        &numbers[..1]
    } else {
        &numbers
    };
    called!("USER32!ValidateRect", handed, value, from);
    value
}

entry!(HC6_CreatePalette, create_palette, 1);
entry!(HC6_RealizePalette, realize_palette, 1);
entry!(HC6_SelectPalette, select_palette, 3);
entry!(HC6_SetDIBColorTable, set_dib_color_table, 4);
entry!(HC6_SetPaletteEntries, set_palette_entries, 4);
entry!(HC6_GetSystemPaletteEntries, get_system_palette_entries, 4);
entry!(HC6_StretchBlt, stretch_blt, 11);
entry!(HC6_CreateDIBSection, create_dib_section, 6);
entry!(HC6_BitBlt, bit_blt, 9);
entry!(HC6_CreateCompatibleDC, create_compatible_dc, 1);
entry!(HC6_DeleteDC, delete_dc, 1);
entry!(HC6_DeleteObject, delete_object, 1);
entry!(HC6_SelectObject, select_object, 2);
entry!(HC6_GetDeviceCaps, get_device_caps, 2);
entry!(HC6_GetDC, get_dc, 1);
entry!(HC6_UpdateWindow, update_window, 1);
entry!(HC6_ReleaseDC, release_dc, 2);
entry!(HC6_BeginPaint, begin_paint, 2);
entry!(HC6_EndPaint, end_paint, 2);

entry!(GAME_CreatePalette, create_palette, 1);
entry!(GAME_RealizePalette, realize_palette, 1);
entry!(GAME_SelectPalette, select_palette, 3);
entry!(GAME_SetDIBColorTable, set_dib_color_table, 4);
entry!(GAME_SetPaletteEntries, set_palette_entries, 4);
entry!(GAME_StretchBlt, stretch_blt, 11);
entry!(GAME_CreateDIBSection, create_dib_section, 6);
entry!(GAME_SetDIBitsToDevice, set_dib_bits_to_device, 12);
entry!(GAME_StretchDIBits, stretch_dib_bits, 13);
entry!(GAME_CreateCompatibleDC, create_compatible_dc, 1);
entry!(GAME_DeleteDC, delete_dc, 1);
entry!(GAME_DeleteObject, delete_object, 1);
entry!(GAME_SelectObject, select_object, 2);
entry!(GAME_GetDeviceCaps, get_device_caps, 2);
entry!(GAME_GetDC, get_dc, 1);
entry!(GAME_UpdateWindow, update_window, 1);
entry!(GAME_ReleaseDC, release_dc, 2);
entry!(GAME_ValidateRect, validate_rect, 2);

entry!(LAUNCHER_GetDeviceCaps, get_device_caps, 2);
entry!(LAUNCHER_GetStockObject, get_stock_object, 1);
entry!(LAUNCHER_GetDC, get_dc, 1);
entry!(LAUNCHER_UpdateWindow, update_window, 1);
