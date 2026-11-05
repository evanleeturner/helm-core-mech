// SPDX-License-Identifier: MIT
//! The tests harvested: `d1`'s lines, the layer's rows under Wine from the main
//! menu to the game file's first frames (`logbook-d1.log` in DB1's records, the
//! lines kept in `s6-replay-lines.txt`), played on the rows less their `from`
//! and `tid`. Wine's window `2005e` is only a number here: `GetDC` needs no
//! window on the desk. A number here no `d1` line holds is a bug.
//!
//! Only the lines are the original's. The shell's section reached no layer
//! line, so its header here is the game file's. The 32 of `BITSPIXEL` reached
//! no line either: the pair's test holds the row to its reading, not to a run.
use window::desk::Desk;
use window::{Event, Rect, Screen};

use draw::easel::{Back, Easel, Shown};
use draw::front::{Front, Header};
use draw::table::Calls;

use super::{BITSPIXEL, CALLS, GAME, MODE, NUMCOLORS, ROWS, SHELL};
use crate::test_log::debug_lines;

/// The game file's palette, in order; its `draw.table` lines left out.
const GAME_PALETTE: [&str; 7] = [
    "19:54:31.241 D imp.call c=USER32!GetDC a=2005e v=b010050 from=100452f7 tid=158",
    "19:54:31.241 D imp.call c=GDI32!CreatePalette a=1000300 v=408005d from=1004599e tid=158",
    "19:54:31.241 D imp.call c=GDI32!SelectPalette a=b010050,408005d,0 v=188002f from=100459b7 tid=158",
    "19:54:31.241 D imp.call c=GDI32!RealizePalette a=b010050 v=0 from=100459e9 tid=158",
    "19:54:31.242 D imp.call c=GDI32!SetDIBitsToDevice a=b010050,0,0,280,1e0,0,0,0,1e0,1,28,280,fffffe20,80001,0,0,0,0,0,0 v=1e0 from=100455f2 tid=158",
    "19:54:31.242 D imp.call c=GDI32!SetPaletteEntries a=408005d,0,100 v=100 from=100459dd tid=158",
    "19:54:31.242 D imp.call c=GDI32!RealizePalette a=b010050 v=0 from=100459e9 tid=158",
];

/// The game file's first colour read.
const GAME_COLOUR_READ: [&str; 2] = [
    "19:54:31.243 D imp.call c=USER32!GetDC a=2005e v=7010045 from=10036f14 tid=158",
    "19:54:31.243 D imp.call c=GDI32!GetDeviceCaps a=7010045,18 v=ffffffff from=10036f1b tid=158",
];

/// The shell's 6 distinct `BitBlt` lines, the first of each.
const SHELL_BLITS: [&str; 6] = [
    "19:53:23.753 D imp.call c=GDI32!BitBlt a=b010050,1d,1ba,db,f,241005d,1d,1ba,cc0020 v=1 from=1000e025 tid=158",
    "19:53:23.810 D imp.call c=GDI32!BitBlt a=b010050,1b5,116,a8,25,241005d,1b5,116,cc0020 v=1 from=1000e025 tid=158",
    "19:53:49.821 D imp.call c=GDI32!BitBlt a=b010050,19,1a,246,1b9,241005d,19,1a,cc0020 v=1 from=1000e025 tid=158",
    "19:53:49.998 D imp.call c=GDI32!BitBlt a=b010050,1b9,1a,a0,a0,241005d,1b9,1a,cc0020 v=1 from=1000e025 tid=158",
    "19:54:26.913 D imp.call c=GDI32!BitBlt a=b010050,1ac,1a,b3,190,241005d,1ac,1a,cc0020 v=1 from=1000e025 tid=158",
    "19:54:27.329 D imp.call c=GDI32!BitBlt a=b010050,20,36,ca,13e,241005d,20,36,cc0020 v=1 from=1000e025 tid=158",
];

/// `d1`'s window, a number the fronts carry.
const WINDOW: u32 = 0x2005e;
/// The header of `d1`'s `SetDIBitsToDevice` lines: top-down, 8 bits.
const HEADER: Header = Header([0x28, 0x280, 0xffff_fe20, 0x8_0001, 0, 0, 0, 0, 0, 0]);
/// A palette's 256 entries as the game file hands them, each `4000000`.
const ENTRIES: [u32; 256] = [0x0400_0000; 256];

/// `d1`'s screen, 640 by 480; these lines move no window and read no clock.
struct Board;

impl Screen for Board {
    fn size(&self) -> (i32, i32) {
        (0x280, 0x1e0)
    }

    fn place(&self, _: u32, _: Rect, _: bool) {}

    fn gone(&self, _: u32) {}

    fn pump(&self, _: &mut dyn FnMut(Event)) {}

    fn now(&self) -> u32 {
        0
    }
}

/// A back that shows nothing: under Wine the pictures stayed Wine's, and a `d1`
/// line holds no pixel.
struct Still;

impl Back for Still {
    fn present(&self, _: &Shown) {}
}

fn rig() -> (Easel<Still>, Desk<Board>) {
    (Easel::new(Still, MODE.0, MODE.1), Desk::new(Board))
}

/// `line` as a front writes it: after its level, less `from` and `tid`, each
/// Wine handle the front's.
fn flown(line: &str, handles: &[(&str, u32)]) -> String {
    let (_, body) = line
        .split_once(" D ")
        .expect("invariant: a logged line holds its level");
    let kept: Vec<&str> = body
        .split(' ')
        .filter(|word| !word.starts_with("from=") && !word.starts_with("tid="))
        .collect();
    let mut line = kept.join(" ");
    for (wine, ours) in handles {
        line = line.replace(wine, &format!("{ours:x}"));
    }
    line
}

/// The numbers of `line`'s `a`.
fn numbers(line: &str) -> Vec<u32> {
    let (_, after) = line
        .split_once(" a=")
        .expect("invariant: a logged call holds its numbers");
    let (a, _) = after
        .split_once(' ')
        .expect("invariant: an answer follows the numbers");
    a.split(',')
        .map(|number| u32::from_str_radix(number, 16).expect("invariant: `a` is hexadecimal"))
        .collect()
}

/// The 16 `draw.table` lines `d1` logged behind `call`: 256 entries of
/// `4000000`, 16 a line, numbered `0` to `f0`.
fn palette_table(call: &str) -> Vec<String> {
    let words = ["4000000"; 16].join(",");
    (0..16_u32)
        .map(|line| format!("draw.table c={call} i={:x} a={words}", line * 16))
        .collect()
}

/// The game file's 7 palette lines, whole and in order, with the 32 table
/// lines behind `CreatePalette` and `SetPaletteEntries`.
#[test]
fn game_palette_replays_d1() {
    let (easel, desk) = rig();
    let front = Front::new(&GAME, &easel, &desk);
    let bits = vec![0_u8; 0x280 * 0x1e0];
    let (mut dc, mut palette, mut old) = (0, 0, 0);
    let lines = debug_lines(|| {
        dc = front.get_dc(WINDOW);
        palette = front.create_palette(0x100_0300, &ENTRIES);
        old = front.select_palette(dc, palette, 0);
        front.realize_palette(dc);
        front.set_dib_bits_to_device(dc, 0, 0, 0x280, 0x1e0, 0, 0, 0, 0x1e0, 1, HEADER, &bits);
        front.set_palette_entries(palette, 0, 0x100, &ENTRIES);
        front.realize_palette(dc);
    });

    let handles = [("b010050", dc), ("408005d", palette), ("188002f", old)];
    let mut want = Vec::new();
    for line in GAME_PALETTE {
        want.push(flown(line, &handles));
        for call in ["GDI32!CreatePalette", "GDI32!SetPaletteEntries"] {
            if line.contains(call) {
                want.extend(palette_table(call));
            }
        }
    }
    assert_eq!(lines, want);
}

/// The game file's first colour read: `NUMCOLORS` answers -1 (`SimMain`'s first
/// read; its second reached no line, `s6-route-10036b60.md`).
#[test]
fn game_colour_read_replays_d1() {
    let (easel, desk) = rig();
    let front = Front::new(&GAME, &easel, &desk);
    let mut dc = 0;
    let lines = debug_lines(|| {
        dc = front.get_dc(WINDOW);
        front.get_device_caps(dc, NUMCOLORS);
    });
    let handles = [("7010045", dc)];
    assert_eq!(lines, GAME_COLOUR_READ.map(|line| flown(line, &handles)));
}

/// The shell's 6 `BitBlt` rectangles, each answering 1, from a compatible
/// context holding a section. The section's calls are not `d1`'s: the shell's
/// reached no layer line.
#[test]
fn shell_bit_blts_replay_d1() {
    let (easel, desk) = rig();
    let front = Front::new(&SHELL, &easel, &desk);
    let dc = front.get_dc(WINDOW);
    let memory = front.create_compatible_dc(dc);
    let mut bits = 0;
    let section = front.create_dib_section(memory, 0, 0, 0, HEADER, &[], &mut bits);
    front.select_object(memory, section);

    let handles = [("b010050", dc), ("241005d", memory)];
    for line in SHELL_BLITS {
        let n = numbers(line);
        let lines = debug_lines(|| {
            let answer = front.bit_blt(dc, n[1], n[2], n[3], n[4], memory, n[6], n[7], n[8]);
            assert_eq!(answer, 1, "{line}");
        });
        assert_eq!(lines, [flown(line, &handles)]);
    }
}

/// The answers `d1`'s rebuilt functions logged on `call.ret` lines:
/// `SetDIBColorTable` and `SetPaletteEntries` `100`, `RealizePalette` 0, `BitBlt`
/// 1. Only the answers are those lines'; the arguments are this test's, the one
/// exception to the rule above.
#[test]
fn shell_answers_the_rebuilt_functions_lines() {
    let (easel, desk) = rig();
    let front = Front::new(&SHELL, &easel, &desk);
    let dc = front.get_dc(WINDOW);
    let memory = front.create_compatible_dc(dc);
    let mut bits = 0;
    let section = front.create_dib_section(memory, 0, 0, 0, HEADER, &[], &mut bits);
    front.select_object(memory, section);
    let palette = front.create_palette(0x100_0300, &ENTRIES);
    front.select_palette(dc, palette, 0);

    assert_eq!(front.set_dib_color_table(memory, 0, 0x100, &ENTRIES), 0x100);
    assert_eq!(
        front.set_palette_entries(palette, 0, 0x100, &ENTRIES),
        0x100
    );
    assert_eq!(front.realize_palette(dc), 0);
    assert_eq!(front.bit_blt(dc, 0, 0, 1, 1, memory, 0, 0, 0x00cc_0020), 1);
}

/// The pair, every row (ruling 1426): `BITSPIXEL` must agree with `NUMCOLORS`'
/// -1. At 8 the shell's `1000e330` would open its system palette read with a
/// count of -1; below 8 the launcher's `00401640` shows its display box. The
/// 32 is unmeasured, and this test does not measure it.
#[test]
fn every_row_answers_a_device_past_8_bits() {
    let (easel, desk) = rig();
    for row in ROWS {
        let front = Front::new(row, &easel, &desk);
        assert_eq!(front.get_device_caps(0, NUMCOLORS), 0xffff_ffff, "{row:?}");
        assert!(front.get_device_caps(0, BITSPIXEL) > 8, "{row:?}");
    }
}

/// Falsifier 1's half: each of the 23 names is spelled as the layer's own row
/// logs it. The pattern names every field, so a 24th fails to compile here.
#[test]
fn every_name_is_the_layers() {
    let layer = include_str!("../../../mech-layer/src/draw.rs");
    let Calls {
        create_palette,
        select_palette,
        set_palette_entries,
        realize_palette,
        set_dib_color_table,
        get_system_palette_entries,
        create_dib_section,
        bit_blt,
        set_dib_bits_to_device,
        stretch_dib_bits,
        stretch_blt,
        create_compatible_dc,
        delete_dc,
        delete_object,
        select_object,
        get_device_caps,
        get_stock_object,
        get_dc,
        release_dc,
        update_window,
        begin_paint,
        end_paint,
        validate_rect,
    } = CALLS;
    let names = [
        create_palette,
        select_palette,
        set_palette_entries,
        realize_palette,
        set_dib_color_table,
        get_system_palette_entries,
        create_dib_section,
        bit_blt,
        set_dib_bits_to_device,
        stretch_dib_bits,
        stretch_blt,
        create_compatible_dc,
        delete_dc,
        delete_object,
        select_object,
        get_device_caps,
        get_stock_object,
        get_dc,
        release_dc,
        update_window,
        begin_paint,
        end_paint,
        validate_rect,
    ];
    for name in names {
        assert!(layer.contains(&format!("\"{name}\"")), "{name}");
    }
}
