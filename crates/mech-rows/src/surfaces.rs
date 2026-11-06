// SPDX-License-Identifier: MIT
//! mercs's `DirectDraw` rows (SF1, rulings 1433, 1434): what the shell and the
//! game file ask of `draw_surfaces`. Past LAUNCH the game file is reached; the
//! menu path never reaches `DirectDraw`, so the kept trace is the game file's
//! alone and the shell's row is the reading of its callers, unmeasured. A mode
//! no row lists is refused: the callers' others are unread, and the flight adds
//! them.
//!
//! The 21 names and the files calling each, as the exports hold them (`mw2shell`
//! the shell, `mercsw` the game file):
//!
//! | Name | Files |
//! | --- | --- |
//! | `DirectDrawCreate` | mw2shell, mercsw |
//! | `IDirectDraw::SetCooperativeLevel` | mw2shell, mercsw |
//! | `IDirectDraw::SetDisplayMode` | mw2shell, mercsw |
//! | `IDirectDraw::GetCaps` | mw2shell, mercsw |
//! | `IDirectDraw::CreateSurface` | mw2shell, mercsw |
//! | `IDirectDraw::CreatePalette` | mw2shell, mercsw |
//! | `IDirectDraw::RestoreDisplayMode` | mw2shell |
//! | `IDirectDraw::Release` | mw2shell, mercsw |
//! | `IDirectDrawPalette::SetEntries` | mercsw |
//! | `IDirectDrawPalette::Release` | mw2shell, mercsw |
//! | `IDirectDrawSurface::GetAttachedSurface` | mw2shell, mercsw |
//! | `IDirectDrawSurface::GetSurfaceDesc` | mw2shell, mercsw |
//! | `IDirectDrawSurface::Lock` | mw2shell, mercsw |
//! | `IDirectDrawSurface::Unlock` | mw2shell, mercsw |
//! | `IDirectDrawSurface::IsLost` | mw2shell, mercsw |
//! | `IDirectDrawSurface::Restore` | mw2shell, mercsw |
//! | `IDirectDrawSurface::Flip` | mw2shell, mercsw |
//! | `IDirectDrawSurface::Blt` | mw2shell, mercsw |
//! | `IDirectDrawSurface::BltFast` | mercsw |
//! | `IDirectDrawSurface::SetPalette` | mw2shell, mercsw |
//! | `IDirectDrawSurface::Release` | mw2shell, mercsw |
//!
//! `tests` replays the kept trace and the flight's lines on these rows.
use draw_surfaces::table::Row;

/// The mode the trace ran: 320 by 200, 8 bits.
const MODES: &[(u32, u32, u32)] = &[(0x140, 0xc8, 8)];
/// `DDPCAPS_8BIT | DDPCAPS_ALLOW256`, both files' one ask.
const PALETTE_FLAGS: u32 = 0x44;
/// The backs both files ask, retrying with 1 when refused.
const CHAIN: u32 = 2;

/// The shell, its teardown's level 8 besides the game's 0x51.
pub static SHELL: Row = Row {
    file: "mw2shell",
    levels: &[0x51, 8],
    modes: MODES,
    chain: CHAIN,
    palette_flags: PALETTE_FLAGS,
};

/// The game file.
pub static GAME: Row = Row {
    file: "mercsw",
    levels: &[0x51],
    modes: MODES,
    chain: CHAIN,
    palette_flags: PALETTE_FLAGS,
};

#[cfg(test)]
mod tests;
