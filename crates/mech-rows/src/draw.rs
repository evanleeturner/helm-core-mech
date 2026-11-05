// SPDX-License-Identifier: MIT
//! mercs's drawing rows (DB1, ruling 1422): the launcher's, the shell's and
//! the game file's drawing and paint calls, as `draw`'s seam serves them. The
//! run `d1` (DB1 step 2, the layer's rows under Wine) logged 8 of the 23 names
//! on a layer line and `SetDIBColorTable` by a rebuilt function alone; the
//! other 14 carry the reading of `s1-readback.md`, unmeasured.
//!
//! The 23 names and the files importing each, as the layer's exports hold them
//! (`mercswin` the launcher, `mw2shell` the shell, `mercsw` the game file):
//!
//! | Name | Files |
//! | --- | --- |
//! | `BeginPaint` | mw2shell |
//! | `BitBlt` | mw2shell |
//! | `CreateCompatibleDC` | mw2shell, mercsw |
//! | `CreateDIBSection` | mw2shell, mercsw |
//! | `CreatePalette` | mw2shell, mercsw |
//! | `DeleteDC` | mw2shell, mercsw |
//! | `DeleteObject` | mw2shell, mercsw |
//! | `EndPaint` | mw2shell |
//! | `GetDC` | all three |
//! | `GetDeviceCaps` | all three |
//! | `GetStockObject` | mercswin |
//! | `GetSystemPaletteEntries` | mw2shell |
//! | `RealizePalette` | mw2shell, mercsw |
//! | `ReleaseDC` | mw2shell, mercsw |
//! | `SelectObject` | mw2shell, mercsw |
//! | `SelectPalette` | mw2shell, mercsw |
//! | `SetDIBColorTable` | mw2shell, mercsw |
//! | `SetDIBitsToDevice` | mercsw |
//! | `SetPaletteEntries` | mw2shell, mercsw |
//! | `StretchBlt` | mw2shell, mercsw |
//! | `StretchDIBits` | mercsw |
//! | `UpdateWindow` | all three |
//! | `ValidateRect` | mercsw |
use draw::table::{Calls, Route, Row};

/// Each call's name, the layer's spelling.
const CALLS: Calls = Calls {
    create_palette: "GDI32!CreatePalette",
    select_palette: "GDI32!SelectPalette",
    set_palette_entries: "GDI32!SetPaletteEntries",
    realize_palette: "GDI32!RealizePalette",
    set_dib_color_table: "GDI32!SetDIBColorTable",
    get_system_palette_entries: "GDI32!GetSystemPaletteEntries",
    create_dib_section: "GDI32!CreateDIBSection",
    bit_blt: "GDI32!BitBlt",
    set_dib_bits_to_device: "GDI32!SetDIBitsToDevice",
    stretch_dib_bits: "GDI32!StretchDIBits",
    stretch_blt: "GDI32!StretchBlt",
    create_compatible_dc: "GDI32!CreateCompatibleDC",
    delete_dc: "GDI32!DeleteDC",
    delete_object: "GDI32!DeleteObject",
    select_object: "GDI32!SelectObject",
    get_device_caps: "GDI32!GetDeviceCaps",
    get_stock_object: "GDI32!GetStockObject",
    get_dc: "USER32!GetDC",
    release_dc: "USER32!ReleaseDC",
    update_window: "USER32!UpdateWindow",
    begin_paint: "USER32!BeginPaint",
    end_paint: "USER32!EndPaint",
    validate_rect: "USER32!ValidateRect",
};

/// `GetDeviceCaps`' index for a pixel's bits.
const BITSPIXEL: u32 = 0xc;
/// `GetDeviceCaps`' index for a palette's colours, -1 past 8 bits.
const NUMCOLORS: u32 = 0x18;

/// `GetDeviceCaps`' answers (ruling 1426): a device deeper than 8 bits, not
/// the plan's palette device.
///
/// `NUMCOLORS` -1 is `d1`'s one line, `SimMain`'s first read at `10036f1b`.
/// Its second read, at `100372b1`, reached no line; its route was read, not
/// flown: `s6-route-10036b60.md`.
///
/// `BITSPIXEL` 32 is unmeasured: it reached no line in `d1`, and the 24 planes
/// of that run's Xvfb root do not prove it. It must agree with the -1: at 8
/// the shell's `1000e330` would open its system palette read with a count of
/// -1. Below 8 the launcher's `00401640` shows its display box.
const CAPS: &[(u32, i32)] = &[(BITSPIXEL, 32), (NUMCOLORS, -1)];

/// The picture every file's calls name, 640 by 480 at 8 bits: `d1`'s
/// `SetDIBitsToDevice` lines, 32 of 32.
pub const MODE: (u32, u32) = (0x280, 0x1e0);

/// The launcher, `mercswin`: its 4 names draw no picture.
pub const LAUNCHER: Row = Row {
    file: "mercswin",
    calls: &CALLS,
    caps: CAPS,
    route: Route::None,
};

/// The shell, `mw2shell`: it imports `BitBlt` and neither `Bits` name; `d1`
/// logged 263 of its layer lines and 3 from a rebuilt function.
pub const SHELL: Row = Row {
    file: "mw2shell",
    route: Route::Blit,
    ..LAUNCHER
};

/// The game file, `mercsw`: it imports both `Bits` names and no `BitBlt`; `d1`
/// logged 32 `SetDIBitsToDevice` lines, and `StretchDIBits` none.
pub const GAME: Row = Row {
    file: "mercsw",
    route: Route::Bits,
    ..LAUNCHER
};

/// Every file the drawing seam serves.
pub const ROWS: &[&Row] = &[&LAUNCHER, &SHELL, &GAME];
