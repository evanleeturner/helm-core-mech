// SPDX-License-Identifier: MIT
//! The drawing calls: mercs's 23 `GDI32` and paint names,
//! each answered by [`::draw`]'s front from call 1, `by=draw` on its line and
//! `from`, the caller, beside it. The picture reaches the real window by
//! [`::draw::user32::Window`], `SetDIBitsToDevice` alone.
//!
//! 41 exports, one a name a file imports: the shell's 19 `HC6_`, the game
//! file's 18 `GAME_`, the launcher's 4 `LAUNCHER_`. Each is a naked entry that
//! reads the address the call returns to first. A row reads what the call
//! points at, hands numbers to the front, and writes back what it gave: the
//! front never sees a file's pointer. One row serves all 3 exports: `Front` reads
//! a [`::draw::table::Row`]'s calls and caps alone, the 3 files share both, and
//! `from` says whose.
//!
//! **One thread's.** [`Easel`] is not `Sync`, so [`front`] keeps it to the first
//! thread that draws: 306,418 of 306,418 drawing lines of one flight were one `tid`.
//! Any other gets one `draw.second.thread` line and the row's own answer as
//! drawn, the number the front's `draw.frozen` door gives. `GetDeviceCaps` and
//! `GetStockObject`, which have no such door, answer 0 there.
//!
//! **What goes on the line differs from `by=orig`'s:** `CreateDIBSection`'s
//! bits pointer and every handle are `draw`'s, so no call here reaches a Wine
//! context, and Smacker's own `GDI32` stays Wine's, by its own imports.
use crate::window::record;
use ::draw::easel::Easel;
use ::draw::front::{Front, Header};
use ::draw::user32::{Real, Window};
use core::cell::OnceCell;
use core::ptr::{with_exposed_provenance, with_exposed_provenance_mut};
use core::sync::atomic::{AtomicU32, Ordering};
use mech_rows::draw::{MODE, SHELL};

unsafe extern "system" {
    fn GetCurrentThreadId() -> u32;
}

/// The rows' state: one easel and the thread it belongs to, 0 until a first
/// row. A `OnceCell`, never an `Option` behind a `&mut`: `UpdateWindow` runs the
/// file's procedure inside the call, which comes back into a row here while the
/// first still holds its reference.
struct Shared {
    owner: AtomicU32,
    easel: OnceCell<Easel<Window>>,
}

// SAFETY: `front` reads `easel` on the thread `owner` names and no other, and
// `owner` is atomic.
unsafe impl Sync for Shared {}

static SHARED: Shared = Shared {
    owner: AtomicU32::new(0),
    easel: OnceCell::new(),
};
pub(crate) static REAL: Real = Real;

/// The front for `from`'s call on this thread; `None`, as [`easel`] answers.
fn front(call: &'static str, from: u32) -> Option<Front<'static, Window, Real>> {
    easel(call, from).map(|easel| Front::new(&SHELL, easel, &REAL).from(from))
}

/// The easel, for `from`'s call on this thread; `None`, one `draw.second.thread`
/// line written, on a thread that is not its own. The first call builds the
/// screen, `MODE`'s size.
pub(crate) fn easel(call: &'static str, from: u32) -> Option<&'static Easel<Window>> {
    // SAFETY: reads this thread's own id.
    let tid = unsafe { GetCurrentThreadId() };
    match SHARED
        .owner
        .compare_exchange(0, tid, Ordering::AcqRel, Ordering::Acquire)
    {
        Err(owner) if owner != tid => {
            kept!(log::warn!(c = call, owner:% = mech_trace::__shown!(owner),
                from:% = mech_trace::__shown!(from), by = "mech-layer"; "draw.second.thread"));
            return None;
        }
        _ => {}
    }
    Some(
        SHARED
            .easel
            .get_or_init(|| Easel::new(Window::new(), MODE.0, MODE.1)),
    )
}

/// The real window a present paints from now on; 0, the whole screen, moves
/// nothing. Only after [`front`] answered, so the easel is this thread's.
fn aim(window: u32) {
    if window != 0
        && let Some(easel) = SHARED.easel.get()
    {
        easel.back().aim(window);
    }
}

/// The most entries a table holds: a palette's, and a section's.
const MOST: usize = 256;

/// Up to [`MOST`] of the `count` words at `address`, copied, since a
/// `LOGPALETTE`'s entries may sit on a 2-byte edge; none for a null one.
///
/// # Safety
/// `address` is 0 or the caller's table of `count` entries.
unsafe fn words(address: u32, count: u32, into: &mut [u32; MOST]) -> &[u32] {
    if address == 0 {
        return &[];
    }
    let count = (count as usize).min(MOST);
    for (at, word) in (0_u32..).zip(into.iter_mut().take(count)) {
        // SAFETY: inside the caller's table, by this function's contract.
        *word = unsafe { record::<1>(address.wrapping_add(at * 4)) }[0];
    }
    &into[..count]
}

/// `words` written at the caller's `address`, on whatever edge it lies.
///
/// # Safety
/// `address` is the caller's room for `words.len()` words.
unsafe fn give(address: u32, words: &[u32]) {
    let to = with_exposed_provenance_mut::<u32>(address as usize);
    for (at, word) in words.iter().enumerate() {
        // SAFETY: inside the caller's room, by this function's contract.
        unsafe { to.add(at).write_unaligned(*word) };
    }
}

/// `rows` rows of the 8-bit picture at `bits`, each row on a 4-byte edge;
/// empty for a null one or any other depth, which the front refuses by the
/// header anyway.
///
/// # Safety
/// `bits` is 0 or the caller's picture, at least that long.
unsafe fn picture<'a>(bits: u32, header: &[u32; 10], rows: u32) -> &'a [u8] {
    if bits == 0 || header[3] >> 16 != 8 {
        return &[];
    }
    let pitch = header[1].wrapping_add(3) & !3;
    // SAFETY: the caller's picture, by this function's contract.
    unsafe {
        core::slice::from_raw_parts(
            with_exposed_provenance::<u8>(bits as usize),
            pitch as usize * rows as usize,
        )
    }
}

/// A header's rows, whichever way they lie.
const fn rows(header: &[u32; 10]) -> u32 {
    (header[2] as i32).unsigned_abs()
}

/// A row that passes its numbers on as they came: `$second` its answer on a
/// thread that is not the easel's.
macro_rules! row {
    ($name:ident, $call:literal, $second:expr, $($argument:ident),*) => {
        #[allow(clippy::too_many_arguments, reason = "the import's own, and the caller first")]
        extern "system" fn $name(from: u32, $($argument: u32),*) -> u32 {
            front($call, from).map_or($second, |front| front.$name($($argument),*))
        }
    };
}

/// `CreatePalette`: `head` the `LOGPALETTE`'s first word, its version and its
/// count, `head >> 16` entries behind it.
extern "system" fn create_palette(from: u32, palette: u32) -> u32 {
    let Some(front) = front("GDI32!CreatePalette", from) else {
        return 0;
    };
    // SAFETY: the caller's record, its first word its version and count.
    let head = unsafe { record::<1>(palette) }[0];
    let mut entries = [0_u32; MOST];
    // SAFETY: the record's entries follow its first word, `head >> 16` of them.
    let entries = unsafe { words(palette.wrapping_add(4), head >> 16, &mut entries) };
    front.create_palette(head, entries)
}

row!(realize_palette, "GDI32!RealizePalette", 0, dc);
row!(
    select_palette,
    "GDI32!SelectPalette",
    0,
    dc,
    palette,
    background
);

extern "system" fn set_dib_color_table(
    from: u32,
    dc: u32,
    start: u32,
    count: u32,
    colors: u32,
) -> u32 {
    let Some(front) = front("GDI32!SetDIBColorTable", from) else {
        return count;
    };
    let mut quads = [0_u32; MOST];
    // SAFETY: the caller's `count` colours.
    let quads = unsafe { words(colors, count, &mut quads) };
    front.set_dib_color_table(dc, start, count, quads)
}

extern "system" fn set_palette_entries(
    from: u32,
    palette: u32,
    start: u32,
    count: u32,
    entries: u32,
) -> u32 {
    let Some(front) = front("GDI32!SetPaletteEntries", from) else {
        return count;
    };
    let mut handed = [0_u32; MOST];
    // SAFETY: the caller's `count` entries.
    let handed = unsafe { words(entries, count, &mut handed) };
    front.set_palette_entries(palette, start, count, handed)
}

/// `GetSystemPaletteEntries`: the entries written at the caller's pointer, `v`
/// of them; a null pointer asks only the size, and has none written.
extern "system" fn get_system_palette_entries(
    from: u32,
    dc: u32,
    start: u32,
    count: u32,
    entries: u32,
) -> u32 {
    let Some(front) = front("GDI32!GetSystemPaletteEntries", from) else {
        return 0;
    };
    let mut local = [0_u32; MOST];
    let written = front.get_system_palette_entries(dc, start, count, &mut local);
    if entries != 0 {
        // SAFETY: the caller's room for `count` entries, `written` at most that.
        unsafe { give(entries, &local[..written as usize]) };
    }
    written
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
    front("GDI32!StretchBlt", from).map_or(1, |front| {
        front.stretch_blt(
            dc,
            (x, y, width, height),
            source,
            (sx, sy, swidth, sheight),
            rop,
        )
    })
}

/// `CreateDIBSection`: the header and its table read here; the bits pointer
/// the front made written at the caller's `bits` on a nonzero answer. Usage 1's
/// table is 16-bit indexes, read as 32-bit words.
extern "system" fn create_dib_section(
    from: u32,
    dc: u32,
    info: u32,
    usage: u32,
    bits: u32,
    section: u32,
    offset: u32,
) -> u32 {
    let Some(front) = front("GDI32!CreateDIBSection", from) else {
        return 0;
    };
    // SAFETY: the caller's header.
    let h = unsafe { record::<10>(info) };
    let depth = h[3] >> 16;
    let entries = match h[8] {
        0 if depth <= 8 => 1_u32 << depth,
        used => used,
    };
    let count = if usage == 1 {
        entries.div_ceil(2)
    } else {
        entries
    };
    let mut table = [0_u32; MOST];
    // SAFETY: the table lies behind the header, its size the header's first word.
    let table = unsafe { words(info.wrapping_add(h[0]), count, &mut table) };
    let mut made = 0_u32;
    let handle = front.create_dib_section(dc, usage, section, offset, Header(h), table, &mut made);
    if handle != 0 && bits != 0 {
        // SAFETY: the caller's out-pointer, one word.
        unsafe { give(bits, &[made]) };
    }
    handle
}

/// Where `FUN_1000df80`'s `BitBlt` returns to: the shell's set-mode present.
const SET_MODE_PRESENT: u32 = 0x1000_dfae;

/// `BitBlt`: the front's answer, but for one caller.
///
/// **The one call the shell quits on.** Its set-mode routine tests the present
/// at `1000ddd4` and shuts the game down unless `BitBlt` answered the height,
/// and the front answers 1, as Wine does (measured on a flight:
/// `ExitProcess(2)`). So this row answers the height, for that return address
/// alone and only when the draw worked; `draw.answer` shows both numbers. It is
/// the shell's rule and no window's, so it stays here, not in `draw`.
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
    let value = front("GDI32!BitBlt", from).map_or(1, |front| {
        front.bit_blt(dc, x, y, width, height, source, sx, sy, rop)
    });
    answered(from, height, value)
}

/// The one caller's answer: `height` where the draw worked, else the draw's own `value`.
fn answered(from: u32, height: u32, value: u32) -> u32 {
    if from == SET_MODE_PRESENT && value != 0 {
        kept!(log::debug!(v:% = mech_trace::__shown!(value),
            answer:% = mech_trace::__shown!(height), from:% = mech_trace::__shown!(from),
            by = "mech-layer";
            "draw.answer"));
        return height;
    }
    value
}

/// Wine's `BitBlt`, as the height row's slot held it at the bind.
static ORIGINAL_BIT_BLT: AtomicU32 = AtomicU32::new(0);

/// The height row with the pieces off: keeps `original`, the slot's live `BitBlt`, and gives
/// the address of `ORIG_BitBlt` for the slot, so the original draws and [`bit_blt`]'s one
/// caller still gets the height. `None` for an address past 32 bits, which no x86 one is.
pub fn height_row(original: u32) -> Option<u32> {
    ORIGINAL_BIT_BLT.store(original, Ordering::Release);
    u32::try_from(ORIG_BitBlt as unsafe extern "C" fn() as usize).ok()
}

/// `BitBlt` with the pieces off: the original through the slot's own value, then
/// [`bit_blt`]'s answer for its one caller.
#[expect(
    clippy::too_many_arguments,
    reason = "BitBlt's own 9, and the caller first"
)]
extern "system" fn orig_bit_blt(
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
    type BitBlt = unsafe extern "system" fn(u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32;
    let original = ORIGINAL_BIT_BLT.load(Ordering::Acquire) as usize;
    // SAFETY: the bind stored the slot's live value, judged an import outside its file,
    // before it pointed the slot here: Wine's `BitBlt`, a stdcall of 9.
    let real = unsafe { core::mem::transmute::<usize, BitBlt>(original) };
    let value = imp_call!(
        "orig",
        "GDI32!BitBlt",
        [dc, x, y, width, height, source, sx, sy, rop],
        // SAFETY: the original, as above, handed the caller's own arguments.
        unsafe { real(dc, x, y, width, height, source, sx, sy, rop) }
    );
    answered(from, height, value)
}

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
    let Some(front) = front("GDI32!SetDIBitsToDevice", from) else {
        return lines;
    };
    // SAFETY: the caller's header.
    let h = unsafe { record::<10>(info) };
    // SAFETY: the caller's picture, `lines` rows of it at most the header's.
    let picture = unsafe { picture(bits, &h, lines.min(rows(&h))) };
    front.set_dib_bits_to_device(
        dc,
        x,
        y,
        width,
        height,
        sx,
        sy,
        start,
        lines,
        usage,
        Header(h),
        picture,
    )
}

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
    // SAFETY: the caller's header.
    let h = unsafe { record::<10>(info) };
    let Some(front) = front("GDI32!StretchDIBits", from) else {
        return rows(&h);
    };
    // SAFETY: the caller's picture, the header's rows of it.
    let picture = unsafe { picture(bits, &h, rows(&h)) };
    front.stretch_dib_bits(
        dc,
        (x, y, width, height),
        (sx, sy, swidth, sheight),
        usage,
        rop,
        Header(h),
        picture,
    )
}

row!(create_compatible_dc, "GDI32!CreateCompatibleDC", 0, dc);
row!(delete_dc, "GDI32!DeleteDC", 1, dc);
row!(delete_object, "GDI32!DeleteObject", 1, object);
row!(select_object, "GDI32!SelectObject", 0, dc, object);
row!(get_device_caps, "GDI32!GetDeviceCaps", 0, dc, index);
row!(get_stock_object, "GDI32!GetStockObject", 0, index);

/// `GetDC`: the file's window is where the picture goes from here on.
extern "system" fn get_dc(from: u32, window: u32) -> u32 {
    let Some(front) = front("USER32!GetDC", from) else {
        return 0;
    };
    let dc = front.get_dc(window);
    aim(window);
    dc
}

row!(release_dc, "USER32!ReleaseDC", 1, window, dc);
row!(update_window, "USER32!UpdateWindow", 1, window);

/// `BeginPaint`: the paint record's first 6 words, the context, the erase flag,
/// `rcPaint`'s 4 fields, written on a nonzero answer; the rest of the caller's
/// record is left as it was.
extern "system" fn begin_paint(from: u32, window: u32, paint: u32) -> u32 {
    let Some(front) = front("USER32!BeginPaint", from) else {
        return 0;
    };
    let mut got = [0_u32; 6];
    let dc = front.begin_paint(window, &mut got);
    if dc != 0 && paint != 0 {
        // SAFETY: the caller's paint record, longer than these 6 words.
        unsafe { give(paint, &got) };
    }
    aim(window);
    dc
}

extern "system" fn end_paint(from: u32, window: u32, paint: u32) -> u32 {
    let Some(front) = front("USER32!EndPaint", from) else {
        return 1;
    };
    // SAFETY: the caller's record; a null one reads as zeros.
    let got = unsafe { record::<6>(paint) };
    front.end_paint(window, &got)
}

/// `ValidateRect`: a null rectangle is the whole window.
extern "system" fn validate_rect(from: u32, window: u32, rectangle: u32) -> u32 {
    let Some(front) = front("USER32!ValidateRect", from) else {
        return 1;
    };
    // SAFETY: the caller's rectangle, read when it is not null.
    let rectangle = (rectangle != 0).then(|| unsafe { record::<4>(rectangle) });
    front.validate_rect(window, rectangle)
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
entry!(ORIG_BitBlt, orig_bit_blt, 9);
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
