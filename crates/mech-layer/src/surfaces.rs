// SPDX-License-Identifier: MIT
//! The `DirectDraw` wrap (SF1, `JOB.md` call 3): the real library answers each
//! of mercs's 21 names, and each writes one `imp.call` line with its return and
//! what it wrote, which the kept trace of Wine's entries does not hold.
//!
//! One export, `GAME_DirectDrawCreate`, in the game file's table. The other 20
//! are methods, reached through tables and no import, so [`wrap`] rewrites the
//! real tables' slots in place: the object's at `DirectDrawCreate`'s return, the
//! surface's at the first `CreateSurface`, the palette's at the first
//! `CreatePalette`, each once. The object keeps Wine's own table, so a surface
//! handed back to Wine (`Blt`'s source, `SetPalette`) is never a stranger to a
//! check Wine makes on it. That is caution, not a measured need: Wine's source
//! is not on this box.
//!
//! Each slot is a naked entry for the `from`. A `from` outside the game file is
//! Wine calling itself: kept on the line, and the replay does not take it.
//!
//! A line's `a` follows [`crate::draw`]'s rule, with `this` first. `o` is what
//! the call wrote, on `DD_OK` alone: `DirectDraw` answers 0 for success, the
//! opposite of the drawing calls. Slot offsets and argument counts are `ddraw.h`'s,
//! the same numbers `draw_surfaces`' table tests pin.
use crate::draw::table;
use crate::window::{Numbers, record};
use core::ffi::c_void;
use core::ptr::{with_exposed_provenance, with_exposed_provenance_mut};
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

unsafe extern "system" {
    fn DirectDrawCreate(guid: u32, out: u32, outer: u32) -> u32;
    fn VirtualProtect(address: *mut c_void, size: usize, protect: u32, old: *mut u32) -> i32;
}

const DD_OK: u32 = 0;
/// `DDPCAPS_8BIT`: a palette of 256 entries, both files' one ask.
const DDPCAPS_8BIT: u32 = 4;
const PALETTE_ENTRIES: u32 = 256;
const PAGE_READWRITE: u32 = 4;
/// `sizeof(DDSURFACEDESC)`, its `dwSize`.
const DESC_BYTES: u32 = 0x6c;
/// `DDBLTFX`'s `dwFillColor`, by word.
const FX_FILL: usize = 20;
/// The most words a line shows of one `DDCAPS`, which says its own size.
const CAPS_WORDS: usize = 96;

/// One `imp.call` line, `o` the call's written words, the last error kept.
macro_rules! called {
    ($c:literal, $numbers:expr, $value:expr, $from:expr, $wrote:expr) => {
        kept!(log::debug!(c = $c, a:% = Numbers($numbers), v:% = mech_trace::__shown!($value),
            o:% = Numbers($wrote), from:% = mech_trace::__shown!($from); "imp.call"))
    };
}

/// The real method a slot held: stdcall, one 32-bit word an argument.
macro_rules! real {
    ($old:ident($($argument:expr),+)) => {{
        // SAFETY: `wrap` stored this slot's real method before pointing the slot
        // here, and the slot's argument count is `ddraw.h`'s.
        let method: extern "system" fn($(real!(@word $argument)),+) -> u32 = unsafe {
            core::mem::transmute(with_exposed_provenance::<c_void>(
                $old.load(Ordering::Acquire) as usize,
            ))
        };
        method($($argument),+)
    }};
    (@word $argument:expr) => { u32 };
}

/// A slot's real method's home and its naked entry.
macro_rules! method {
    ($old:ident, $entry:ident, $inner:path, $count:literal) => {
        static $old: AtomicU32 = AtomicU32::new(0);
        entry!($entry, $inner, $count);
    };
}

/// A method of `this` alone that writes nothing.
macro_rules! alone {
    ($inner:ident, $old:ident, $c:literal) => {
        extern "system" fn $inner(from: u32, this: u32) -> u32 {
            let value = real!($old(this));
            called!($c, &[this], value, from, &[]);
            value
        }
    };
}

/// One slot to wrap: its byte offset, where the real method goes, its entry.
struct Slot {
    offset: u32,
    old: &'static AtomicU32,
    entry: unsafe extern "C" fn(),
}

/// The most slots a table wraps here: the surface's 11.
const MOST_SLOTS: usize = 11;

/// Points `slots` of the table `interface` holds at their entries, once a table
/// (`done`), keeping the real methods first. A refused `VirtualProtect` leaves
/// the table whole and the methods unlogged, said on the line.
///
/// # Safety
/// `interface` is a live `DirectDraw` interface whose table has these slots.
unsafe fn wrap(c: &'static str, interface: u32, slots: &[Slot], done: &AtomicBool) {
    if done.swap(true, Ordering::AcqRel) {
        return;
    }
    // SAFETY: an interface's first word is its table, by this function's contract.
    let table = unsafe { record::<1>(interface) }[0];
    let span = slots.iter().map(|slot| slot.offset + 4).max().unwrap_or(0);
    let at = with_exposed_provenance_mut::<c_void>(table as usize);
    let mut kept_protection = 0;
    // SAFETY: the table's own pages; `kept_protection` is a live local.
    let answer =
        unsafe { VirtualProtect(at, span as usize, PAGE_READWRITE, &raw mut kept_protection) };
    let mut olds = [0_u32; MOST_SLOTS];
    if answer != 0 {
        for (slot, old) in slots.iter().zip(olds.iter_mut()) {
            let word = with_exposed_provenance_mut::<u32>((table + slot.offset) as usize);
            // SAFETY: a slot of the table, made writable above; each is a 32-bit
            // word on this target, and the game's threads read it after this returns.
            unsafe {
                *old = word.read();
                slot.old.store(*old, Ordering::Release);
                word.write(slot.entry as usize as u32);
            }
        }
        let mut ignored = 0;
        // SAFETY: as the call above, the protection put back.
        unsafe { VirtualProtect(at, span as usize, kept_protection, &raw mut ignored) };
    }
    let shown = slots.len().min(MOST_SLOTS);
    let answered = answer as u32;
    kept!(
        log::debug!(c = c, t:% = mech_trace::__shown!(table), a:% = Numbers(&olds[..shown]),
        v:% = mech_trace::__shown!(answered); "surfaces.wrap")
    );
}

/// What a call wrote, for the line's `o`: nothing unless it answered `DD_OK`.
fn on_ok(value: u32, words: &[u32]) -> &[u32] {
    if value == DD_OK { words } else { &[] }
}

/// A `DDSURFACEDESC`'s 27 words at `address`; all 0 for null or a size no caller
/// was measured giving.
///
/// # Safety
/// `address` is 0 or the caller's record, its first word its size.
unsafe fn description(address: u32) -> [u32; 27] {
    // SAFETY: the record's first word, by this function's contract.
    if unsafe { record::<1>(address) }[0] != DESC_BYTES {
        return [0; 27];
    }
    // SAFETY: the record says it holds these 27 words.
    unsafe { record(address) }
}

/// The words of a `DDCAPS` at `address`, the first its own size; how many are
/// held. None for null.
///
/// # Safety
/// `address` is 0 or the caller's record, its first word its size in bytes.
unsafe fn caps(address: u32, into: &mut [u32]) -> usize {
    // SAFETY: the record's first word, by this function's contract.
    let size = unsafe { record::<1>(address) }[0] as usize / 4;
    let count = size.min(into.len());
    for (index, word) in into.iter_mut().take(count).enumerate() {
        // SAFETY: inside the record, by its own size.
        *word = unsafe { record::<1>(address.wrapping_add(index as u32 * 4)) }[0];
    }
    count
}

extern "system" fn direct_draw_create(from: u32, guid: u32, out: u32, outer: u32) -> u32 {
    // SAFETY: the caller's arguments, passed on whole.
    let value = unsafe { DirectDrawCreate(guid, out, outer) };
    // SAFETY: the caller's out pointer.
    let object = unsafe { record::<1>(out) };
    if value == DD_OK {
        // SAFETY: the real call just made this object.
        unsafe { wrap("IDirectDraw", object[0], &OBJECT, &OBJECT_DONE) };
    }
    called!(
        "DDRAW!DirectDrawCreate",
        &[guid, outer],
        value,
        from,
        on_ok(value, &object)
    );
    value
}
entry!(GAME_DirectDrawCreate, direct_draw_create, 3);

extern "system" fn set_cooperative_level(from: u32, this: u32, window: u32, flags: u32) -> u32 {
    let value = real!(OLD_LEVEL(this, window, flags));
    called!(
        "IDirectDraw::SetCooperativeLevel",
        &[this, window, flags],
        value,
        from,
        &[]
    );
    value
}

extern "system" fn set_display_mode(
    from: u32,
    this: u32,
    width: u32,
    height: u32,
    bits: u32,
) -> u32 {
    let value = real!(OLD_MODE(this, width, height, bits));
    called!(
        "IDirectDraw::SetDisplayMode",
        &[this, width, height, bits],
        value,
        from,
        &[]
    );
    value
}

/// `GetCaps`: `o` the driver's record then the emulator's, each opening with its size.
extern "system" fn object_get_caps(from: u32, this: u32, driver: u32, emulated: u32) -> u32 {
    let value = real!(OLD_CAPS(this, driver, emulated));
    let mut words = [0_u32; 2 * CAPS_WORDS];
    let (first, second) = words.split_at_mut(CAPS_WORDS);
    // SAFETY: the caller's records, or null.
    let (ones, twos) = unsafe { (caps(driver, first), caps(emulated, second)) };
    words.copy_within(CAPS_WORDS..CAPS_WORDS + twos, ones);
    called!(
        "IDirectDraw::GetCaps",
        &[this],
        value,
        from,
        on_ok(value, &words[..ones + twos])
    );
    value
}

/// `CreateSurface`: `a` holds the description asked, `o` the surface made.
extern "system" fn create_surface(from: u32, this: u32, asked: u32, out: u32, outer: u32) -> u32 {
    let value = real!(OLD_CREATE_SURFACE(this, asked, out, outer));
    // SAFETY: the caller's out pointer.
    let surface = unsafe { record::<1>(out) };
    if value == DD_OK {
        // SAFETY: the real call just made this surface.
        unsafe { wrap("IDirectDrawSurface", surface[0], &SURFACE, &SURFACE_DONE) };
    }
    let mut numbers = [0_u32; 29];
    numbers[0] = this;
    numbers[1] = outer;
    // SAFETY: the caller's record, or null.
    numbers[2..].copy_from_slice(&unsafe { description(asked) });
    called!(
        "IDirectDraw::CreateSurface",
        &numbers,
        value,
        from,
        on_ok(value, &surface)
    );
    value
}

/// `CreatePalette`: `o` the palette made, the colours on `draw.table` lines.
extern "system" fn create_palette(
    from: u32,
    this: u32,
    flags: u32,
    entries: u32,
    out: u32,
    outer: u32,
) -> u32 {
    let value = real!(OLD_CREATE_PALETTE(this, flags, entries, out, outer));
    // SAFETY: the caller's out pointer.
    let palette = unsafe { record::<1>(out) };
    if value == DD_OK {
        // SAFETY: the real call just made this palette.
        unsafe { wrap("IDirectDrawPalette", palette[0], &PALETTE, &PALETTE_DONE) };
    }
    called!(
        "IDirectDraw::CreatePalette",
        &[this, flags, outer],
        value,
        from,
        on_ok(value, &palette)
    );
    let count = if flags & DDPCAPS_8BIT == 0 {
        0
    } else {
        PALETTE_ENTRIES
    };
    // SAFETY: the caller's colours: 256 entries under this flag.
    kept!(unsafe { table("IDirectDraw::CreatePalette", 0, count, entries) });
    value
}

alone!(
    restore_display_mode,
    OLD_RESTORE_MODE,
    "IDirectDraw::RestoreDisplayMode"
);
alone!(object_release, OLD_OBJECT_RELEASE, "IDirectDraw::Release");

/// `SetEntries`: the colours handed on `draw.table` lines.
extern "system" fn set_entries(
    from: u32,
    this: u32,
    flags: u32,
    start: u32,
    count: u32,
    entries: u32,
) -> u32 {
    let value = real!(OLD_SET_ENTRIES(this, flags, start, count, entries));
    called!(
        "IDirectDrawPalette::SetEntries",
        &[this, flags, start, count],
        value,
        from,
        &[]
    );
    // SAFETY: the caller's `count` colours.
    kept!(unsafe { table("IDirectDrawPalette::SetEntries", start, count, entries) });
    value
}

alone!(
    palette_release,
    OLD_PALETTE_RELEASE,
    "IDirectDrawPalette::Release"
);

/// `GetAttachedSurface`: `a` holds the caps asked, `o` the surface found.
extern "system" fn get_attached_surface(from: u32, this: u32, asked: u32, out: u32) -> u32 {
    let value = real!(OLD_ATTACHED(this, asked, out));
    // SAFETY: the caller's caps record and out pointer.
    let (wanted, surface) = unsafe { (record::<1>(asked)[0], record::<1>(out)) };
    called!(
        "IDirectDrawSurface::GetAttachedSurface",
        &[this, wanted],
        value,
        from,
        on_ok(value, &surface)
    );
    value
}

extern "system" fn get_surface_desc(from: u32, this: u32, desc: u32) -> u32 {
    let value = real!(OLD_DESC(this, desc));
    // SAFETY: the caller's record.
    let words = unsafe { description(desc) };
    called!(
        "IDirectDrawSurface::GetSurfaceDesc",
        &[this],
        value,
        from,
        on_ok(value, &words)
    );
    value
}

/// `Lock`: `a` a rectangle's 4 numbers last, none for a null; `o` the description.
extern "system" fn lock(from: u32, this: u32, rect: u32, desc: u32, flags: u32, event: u32) -> u32 {
    let value = real!(OLD_LOCK(this, rect, desc, flags, event));
    // SAFETY: the caller's rectangle, or null.
    let area = unsafe { record::<4>(rect) };
    // SAFETY: the caller's record.
    let words = unsafe { description(desc) };
    let numbers = [this, flags, event, area[0], area[1], area[2], area[3]];
    let shown = if rect == 0 { 3 } else { numbers.len() };
    called!(
        "IDirectDrawSurface::Lock",
        &numbers[..shown],
        value,
        from,
        on_ok(value, &words)
    );
    value
}

extern "system" fn unlock(from: u32, this: u32, pointer: u32) -> u32 {
    let value = real!(OLD_UNLOCK(this, pointer));
    called!(
        "IDirectDrawSurface::Unlock",
        &[this, pointer],
        value,
        from,
        &[]
    );
    value
}

alone!(is_lost, OLD_IS_LOST, "IDirectDrawSurface::IsLost");
alone!(restore, OLD_RESTORE, "IDirectDrawSurface::Restore");

extern "system" fn flip(from: u32, this: u32, over: u32, flags: u32) -> u32 {
    let value = real!(OLD_FLIP(this, over, flags));
    called!(
        "IDirectDrawSurface::Flip",
        &[this, over, flags],
        value,
        from,
        &[]
    );
    value
}

/// `Blt`: 14 numbers whichever is null, so a line parses by place; see `a`'s book entry.
extern "system" fn blt(
    from: u32,
    this: u32,
    area: u32,
    source: u32,
    source_area: u32,
    flags: u32,
    effects: u32,
) -> u32 {
    let value = real!(OLD_BLT(this, area, source, source_area, flags, effects));
    // SAFETY: the caller's rectangles and effects record, or null.
    let (to, of, fill) = unsafe {
        (
            record::<4>(area),
            record::<4>(source_area),
            record::<{ FX_FILL + 1 }>(effects)[FX_FILL],
        )
    };
    let numbers = [
        this,
        source,
        flags,
        fill,
        u32::from(area != 0),
        to[0],
        to[1],
        to[2],
        to[3],
        u32::from(source_area != 0),
        of[0],
        of[1],
        of[2],
        of[3],
    ];
    called!("IDirectDrawSurface::Blt", &numbers, value, from, &[]);
    value
}

/// `BltFast`: `a` a rectangle's 4 numbers last, none for a null.
extern "system" fn blt_fast(
    from: u32,
    this: u32,
    x: u32,
    y: u32,
    source: u32,
    area: u32,
    how: u32,
) -> u32 {
    let value = real!(OLD_BLT_FAST(this, x, y, source, area, how));
    // SAFETY: the caller's rectangle, or null.
    let of = unsafe { record::<4>(area) };
    let numbers = [this, x, y, source, how, of[0], of[1], of[2], of[3]];
    let shown = if area == 0 { 5 } else { numbers.len() };
    called!(
        "IDirectDrawSurface::BltFast",
        &numbers[..shown],
        value,
        from,
        &[]
    );
    value
}

extern "system" fn set_palette(from: u32, this: u32, palette: u32) -> u32 {
    let value = real!(OLD_SET_PALETTE(this, palette));
    called!(
        "IDirectDrawSurface::SetPalette",
        &[this, palette],
        value,
        from,
        &[]
    );
    value
}

alone!(
    surface_release,
    OLD_SURFACE_RELEASE,
    "IDirectDrawSurface::Release"
);

method!(OLD_LEVEL, DD_SetCooperativeLevel, set_cooperative_level, 3);
method!(OLD_MODE, DD_SetDisplayMode, set_display_mode, 4);
method!(OLD_CAPS, DD_GetCaps, object_get_caps, 3);
method!(OLD_CREATE_SURFACE, DD_CreateSurface, create_surface, 4);
method!(OLD_CREATE_PALETTE, DD_CreatePalette, create_palette, 5);
method!(
    OLD_RESTORE_MODE,
    DD_RestoreDisplayMode,
    restore_display_mode,
    1
);
method!(OLD_OBJECT_RELEASE, DD_Release, object_release, 1);
method!(OLD_SET_ENTRIES, DD_SetEntries, set_entries, 5);
method!(OLD_PALETTE_RELEASE, DD_PaletteRelease, palette_release, 1);
method!(OLD_ATTACHED, DD_GetAttachedSurface, get_attached_surface, 3);
method!(OLD_DESC, DD_GetSurfaceDesc, get_surface_desc, 2);
method!(OLD_LOCK, DD_Lock, lock, 5);
method!(OLD_UNLOCK, DD_Unlock, unlock, 2);
method!(OLD_IS_LOST, DD_IsLost, is_lost, 1);
method!(OLD_RESTORE, DD_Restore, restore, 1);
method!(OLD_FLIP, DD_Flip, flip, 3);
method!(OLD_BLT, DD_Blt, blt, 6);
method!(OLD_BLT_FAST, DD_BltFast, blt_fast, 6);
method!(OLD_SET_PALETTE, DD_SetPalette, set_palette, 2);
method!(OLD_SURFACE_RELEASE, DD_SurfaceRelease, surface_release, 1);

static OBJECT_DONE: AtomicBool = AtomicBool::new(false);
static SURFACE_DONE: AtomicBool = AtomicBool::new(false);
static PALETTE_DONE: AtomicBool = AtomicBool::new(false);

/// One `Slot`, by the method's offset in `ddraw.h`.
macro_rules! slot {
    ($offset:literal, $old:ident, $entry:ident) => {
        Slot {
            offset: $offset,
            old: &$old,
            entry: $entry,
        }
    };
}

static OBJECT: [Slot; 7] = [
    slot!(0x08, OLD_OBJECT_RELEASE, DD_Release),
    slot!(0x14, OLD_CREATE_PALETTE, DD_CreatePalette),
    slot!(0x18, OLD_CREATE_SURFACE, DD_CreateSurface),
    slot!(0x2c, OLD_CAPS, DD_GetCaps),
    slot!(0x4c, OLD_RESTORE_MODE, DD_RestoreDisplayMode),
    slot!(0x50, OLD_LEVEL, DD_SetCooperativeLevel),
    slot!(0x54, OLD_MODE, DD_SetDisplayMode),
];

static PALETTE: [Slot; 2] = [
    slot!(0x08, OLD_PALETTE_RELEASE, DD_PaletteRelease),
    slot!(0x18, OLD_SET_ENTRIES, DD_SetEntries),
];

static SURFACE: [Slot; MOST_SLOTS] = [
    slot!(0x08, OLD_SURFACE_RELEASE, DD_SurfaceRelease),
    slot!(0x14, OLD_BLT, DD_Blt),
    slot!(0x1c, OLD_BLT_FAST, DD_BltFast),
    slot!(0x2c, OLD_FLIP, DD_Flip),
    slot!(0x30, OLD_ATTACHED, DD_GetAttachedSurface),
    slot!(0x58, OLD_DESC, DD_GetSurfaceDesc),
    slot!(0x60, OLD_IS_LOST, DD_IsLost),
    slot!(0x64, OLD_LOCK, DD_Lock),
    slot!(0x6c, OLD_RESTORE, DD_Restore),
    slot!(0x7c, OLD_SET_PALETTE, DD_SetPalette),
    slot!(0x80, OLD_UNLOCK, DD_Unlock),
];
