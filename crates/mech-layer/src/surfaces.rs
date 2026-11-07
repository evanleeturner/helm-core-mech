// SPDX-License-Identifier: MIT
//! `DirectDrawCreate`'s 2 rows: `draw_surfaces` answers from the
//! first call, over the easel [`crate::draw`] holds, so a GDI call and a
//! `DirectDraw` call see one picture. The other 20 names are methods a file
//! reaches through `draw_surfaces::com`'s tables and no import, so they have no
//! row here; each writes its own `imp.call` line, `by=draw-surfaces`.
//!
//! One switch, `HC5_MODE_REFUSED`: set to anything, a file's row holds no mode and
//! `SetDisplayMode` answers `DDERR_UNSUPPORTED`, the number Wine gave on a flight, so
//! the file takes the route it took there and only who answers changes. Read once
//! a file, at its first `DirectDrawCreate`, and named on the `surfaces.row` line.
//!
//! The shell and the game file both prefer `0x10000000` and are never mapped
//! together, so each has its own object over the one easel.
use ::draw::user32::{Real, Window};
use core::cell::OnceCell;
use core::ptr::with_exposed_provenance_mut;
use draw_surfaces::com::Com;
use draw_surfaces::sdk::DDERR_UNSUPPORTED;
use draw_surfaces::surfaces::Surfaces;
use draw_surfaces::table::Row;
use mech_rows::surfaces::{GAME, GAME_MODE_REFUSED, SHELL, SHELL_MODE_REFUSED};

unsafe extern "system" {
    fn GetEnvironmentVariableA(name: *const u8, buffer: *mut u8, size: u32) -> u32;
}

/// One file's object, made at its first `DirectDrawCreate`.
struct Object(OnceCell<Com<Window, Real>>);

// SAFETY: [`create`] reads it only after `crate::draw::easel` answered, which it
// does on the easel's own thread and no other.
unsafe impl Sync for Object {}

static SHELL_OBJECT: Object = Object(OnceCell::new());
static GAME_OBJECT: Object = Object(OnceCell::new());

/// Whether the switch is set; a value too long for the buffer still answers nonzero.
fn mode_refused() -> bool {
    let mut value = [0_u8; 2];
    // SAFETY: a name ending in NUL, and this function's own 2 bytes.
    let found =
        unsafe { GetEnvironmentVariableA(b"HC5_MODE_REFUSED\0".as_ptr(), value.as_mut_ptr(), 2) };
    found != 0
}

/// The row: `DDERR_UNSUPPORTED` with one `draw.second.thread` line on a thread
/// that is not the easel's.
fn create(
    object: &'static Object,
    open: &'static Row,
    refused: &'static Row,
    (from, guid, out, outer): (u32, u32, u32, u32),
) -> u32 {
    let Some(easel) = crate::draw::easel("DirectDrawCreate", from) else {
        return DDERR_UNSUPPORTED;
    };
    let com = object.0.get_or_init(|| {
        let row = if mode_refused() { refused } else { open };
        let modes = row.modes.len();
        kept!(
            log::info!(file = row.file, modes:% = mech_trace::__shown!(modes),
            from:% = mech_trace::__shown!(from), by = "mech-layer"; "surfaces.row")
        );
        Com::new(Surfaces::new(row, easel, &crate::draw::REAL))
    });
    // SAFETY: `out` is the file's own word, as `DirectDrawCreate` promises, or null,
    // which the front refuses.
    unsafe { com.direct_draw_create(guid, with_exposed_provenance_mut(out as usize), outer) }
}

extern "system" fn shell_create(from: u32, guid: u32, out: u32, outer: u32) -> u32 {
    create(
        &SHELL_OBJECT,
        &SHELL,
        &SHELL_MODE_REFUSED,
        (from, guid, out, outer),
    )
}
entry!(HC6_DirectDrawCreate, shell_create, 3);

extern "system" fn game_create(from: u32, guid: u32, out: u32, outer: u32) -> u32 {
    create(
        &GAME_OBJECT,
        &GAME,
        &GAME_MODE_REFUSED,
        (from, guid, out, outer),
    )
}
entry!(GAME_DirectDrawCreate, game_create, 3);
