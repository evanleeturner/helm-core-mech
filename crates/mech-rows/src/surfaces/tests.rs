// SPDX-License-Identifier: MIT
//! The tests harvested: T1a-17's kept Wine trace, cut to the calls the game file
//! itself made (`s2-trace-calls.tsv` in SF1's records), played on the game file's
//! row. `trace-cut.tsv` is its first 264 lines, every name the game calls among
//! them; `SF1_TRACE` names the whole for the ignored test, 18,484 lines.
//!
//! Wine's trace holds no returns. That the game went on is what says each answer
//! was 0; what its callers read back (`s1-readback.md`) is held here, and a `Lock`'s
//! address against the `data` Wine logged. Its flips are of a black screen: pixels
//! are `draw_surfaces`' own tests', planted there. Two kinds of line are Wine's own
//! and skipped: a flip's `Release`s of a surface to any count but 0, and a palette's
//! `Release`s, which sit inside a surface's cleanup. And 57 of the 18,484 kept lines
//! are one call printed twice (`doubled-lines.txt`; the raw trace's inner line tells
//! them from a true second call): each method's lines less these equal its
//! usual next line's count, `Lock` then `surface_lock` 1,718, and `Lock` and `Unlock`
//! then balance. Wine's addresses and ours agree at each of the 1,718 `Unlock`s; the
//! mechanism is unread. [`replay`] plays one; [`flight`] plays the flight's lines.
use std::cell::Cell;
use std::collections::{BTreeMap, BTreeSet};

use draw::easel::{Back, Easel, Shown};
use draw_surfaces::sdk::{
    DD_OK, DDERR_SURFACELOST, DDSCAPS_BACKBUFFER, DDSCAPS_FLIP, DESC_BACK_COUNT, DESC_BYTES,
    DESC_CAPS, DESC_FLAGS, DESC_PITCH, DESC_SIZE, DESC_SURFACE, DESC_WORDS, Desc,
};
use draw_surfaces::surfaces::Surfaces;
use window::desk::Desk;
use window::{Event, Rect, Screen};

use super::{CHAIN, GAME, MODES, PALETTE_FLAGS, SHELL};

mod flight;

/// The cut, in the trace's order.
const CUT: &str = include_str!("trace-cut.tsv");
/// The kept lines' numbers Wine printed twice: the raw trace tells them from a true
/// second call by the inner line between, which the kept lines do not hold.
const DOUBLED: &str = include_str!("doubled-lines.txt");
/// Wine's numbers for the primary and the back the game file holds.
const WINE_PRIMARY: &str = "01421968";
const WINE_BACK: &str = "01421FC8";
/// The second `GetAttachedSurface`'s out word, the one asking `DDSCAPS_FLIP`.
const THIRD_POINTER: &str = "100B9F80";
/// `GetCaps`' bit a caller refuses.
const BANKSWITCHED: u32 = 0x0800_0000;
/// `dwSize` as the callers write it.
const CAPS_BYTES: u32 = 0xac;
/// The 320-by-200 route's description (`s1-readback.md`): the trace shows its
/// address alone.
const FLAGS_BACK_COUNT: u32 = 0x20;
const CAPS_CHAIN: u32 = 0x218;
const PITCH: u32 = 320;
/// The game's one window, a number the fronts carry.
const WINDOW: u32 = 0x2005e;

struct Board;

impl Screen for Board {
    fn size(&self) -> (i32, i32) {
        (320, 200)
    }
    fn place(&self, _: u32, _: Rect, _: bool) {}
    fn gone(&self, _: u32) {}
    fn pump(&self, _: &mut dyn FnMut(Event)) {}
    fn now(&self) -> u32 {
        0
    }
}

/// Counts what it is handed; the page's own replacing is `draw`'s test.
#[derive(Default)]
struct Tally(Cell<u32>);

impl Back for &Tally {
    fn present(&self, _: &Shown) {
        self.0.set(self.0.get() + 1);
    }
}

/// The value after `name ` in a line's arguments, as Wine prints it.
fn field<'l>(arguments: &'l str, name: &str) -> &'l str {
    let trimmed = arguments.trim_end_matches('.');
    let mut found = trimmed.split(", ").filter_map(|part| part.split_once(' '));
    found
        .find(|(key, _)| *key == name)
        .map_or_else(|| panic!("{name} in {arguments}"), |(_, value)| value)
}

fn hex(value: &str) -> u32 {
    u32::from_str_radix(value.trim_start_matches("0x"), 16).expect("Wine prints hex here")
}

fn decimal(value: &str) -> u32 {
    value.parse().expect("Wine prints decimal here")
}

/// `(0,0)-(320,200)`'s four, `(null)` none.
fn rect(value: &str) -> Option<[i32; 4]> {
    if value == "(null)" {
        return None;
    }
    let numbers: Vec<i32> = value
        .split(|c: char| !c.is_ascii_digit())
        .filter(|part| !part.is_empty())
        .map(|part| part.parse().expect("a rectangle's edge"))
        .collect();
    Some([numbers[0], numbers[1], numbers[2], numbers[3]])
}

/// One run of the trace's lines on a live object.
struct Replay<'r> {
    surfaces: Surfaces<'r, &'r Tally, Board>,
    primary: u32,
    back: u32,
    palette: u32,
    /// The last `Lock`'s answer, which the next `Unlock`'s `data` names.
    locked: u32,
    /// Each of Wine's pixel addresses and ours, this init's: the two must move as
    /// one, and a new surface begins a new pair.
    wine: BTreeMap<u32, u32>,
    ours: BTreeMap<u32, u32>,
    names: BTreeSet<String>,
    counts: BTreeMap<String, u32>,
}

impl Replay<'_> {
    fn surface(&self, iface: &str) -> u32 {
        match iface {
            WINE_PRIMARY => self.primary,
            WINE_BACK => self.back,
            other => panic!("a surface the game never holds: {other}"),
        }
    }

    /// Plays one line, holding what its caller reads back. Wine's own lines pass.
    fn step(&mut self, line: &str) {
        let mut columns = line.split('\t').skip(2);
        let name = columns.next().expect("a line's call");
        let arguments = columns.next().expect("a line's arguments");
        let answer = match name {
            "ddraw_palette_Release" => return,
            "ddraw_surface1_Release" if !arguments.contains("refcount to 0") => return,
            "DirectDrawCreate" => self.surfaces.direct_draw_create(0, 0),
            "ddraw1_SetCooperativeLevel" => {
                let flags = hex(field(arguments, "flags"));
                let window = hex(field(arguments, "window"));
                assert_eq!(window, WINDOW);
                self.surfaces.set_cooperative_level(window, flags)
            }
            "ddraw1_SetDisplayMode" => self.surfaces.set_display_mode(
                decimal(field(arguments, "width")),
                decimal(field(arguments, "height")),
                decimal(field(arguments, "bpp")),
            ),
            "ddraw1_GetCaps" => self.get_caps(),
            "ddraw1_CreateSurface" => self.create_surface(),
            "ddraw1_CreatePalette" => {
                let flags = hex(field(arguments, "flags"));
                self.surfaces
                    .create_palette(flags, &[0; 256], &mut self.palette)
            }
            "ddraw_palette_SetEntries" => {
                let start = decimal(field(arguments, "start"));
                let count = decimal(field(arguments, "count")) as usize;
                self.surfaces
                    .set_entries(self.palette, 0, start, &vec![0; count])
            }
            "ddraw_surface1_GetAttachedSurface" => self.get_attached(arguments),
            "ddraw_surface1_GetSurfaceDesc" => self.get_desc(arguments),
            "ddraw_surface1_IsLost" => {
                let answer = self
                    .surfaces
                    .is_lost(self.surface(field(arguments, "iface")));
                assert_ne!(answer, DDERR_SURFACELOST);
                answer
            }
            "ddraw_surface1_Blt" => self.blt(arguments),
            "ddraw_surface1_BltFast" => self.blt_fast(arguments),
            "ddraw_surface1_Flip" => self.surfaces.flip(
                self.surface(field(arguments, "iface")),
                hex(field(arguments, "src")),
                hex(field(arguments, "flags")),
            ),
            "ddraw_surface1_Lock" => self.lock(arguments),
            "ddraw_surface1_Unlock" => self.unlock(arguments),
            "ddraw_surface1_SetPalette" => {
                let surface = self.surface(field(arguments, "iface"));
                self.surfaces.set_palette(surface, self.palette)
            }
            "ddraw_surface1_Release" => {
                let id = arguments.split(' ').nth(1).expect("the surface");
                let left = self.surfaces.release_surface(self.surface(id));
                assert_eq!(left, 0, "{line}");
                left
            }
            other => panic!("a call this replay does not know: {other}"),
        };
        assert_eq!(answer, DD_OK, "{line}");
        self.names.insert(name.to_owned());
        *self.counts.entry(name.to_owned()).or_default() += 1;
    }

    fn get_caps(&self) -> u32 {
        let mut driver = [CAPS_BYTES, BANKSWITCHED];
        let answer = self.surfaces.get_caps(Some(&mut driver), None);
        assert_eq!(
            driver[1] & BANKSWITCHED,
            0,
            "the caller would refuse the mode"
        );
        answer
    }

    fn create_surface(&mut self) -> u32 {
        let mut desc: Desc = [0; DESC_WORDS];
        desc[DESC_SIZE] = DESC_BYTES;
        desc[DESC_FLAGS] = FLAGS_BACK_COUNT;
        desc[DESC_BACK_COUNT] = CHAIN;
        desc[DESC_CAPS] = CAPS_CHAIN;
        self.wine.clear();
        self.ours.clear();
        self.surfaces.create_surface(&desc, &mut self.primary)
    }

    /// Wine answered both caps with one surface, the game's back and its third
    /// pointer alike (`s1-readback.md`).
    fn get_attached(&mut self, arguments: &str) -> u32 {
        let third = field(arguments, "attachment") == THIRD_POINTER;
        let caps = if third {
            DDSCAPS_FLIP
        } else {
            DDSCAPS_BACKBUFFER
        };
        let mut out = 0;
        let answer = self
            .surfaces
            .get_attached_surface(self.primary, caps, &mut out);
        if third {
            assert_eq!(out, self.back, "both caps answer the one interface");
        } else {
            self.back = out;
        }
        answer
    }

    /// The attached surface's pitch is what the caller keeps.
    fn get_desc(&self, arguments: &str) -> u32 {
        let surface = self.surface(field(arguments, "iface"));
        let mut desc: Desc = [0; DESC_WORDS];
        desc[DESC_SIZE] = DESC_BYTES;
        let answer = self.surfaces.get_surface_desc(surface, &mut desc);
        assert_eq!(desc[DESC_PITCH], PITCH);
        answer
    }

    fn blt(&self, arguments: &str) -> u32 {
        let target = self.surface(field(arguments, "iface"));
        let to = rect(field(arguments, "dst_rect"));
        let flags = hex(field(arguments, "flags"));
        assert_eq!(field(arguments, "src_surface"), "00000000");
        self.surfaces.blt(target, to, (0, None), flags, 0)
    }

    fn blt_fast(&self, arguments: &str) -> u32 {
        let target = self.surface(field(arguments, "iface"));
        let source = self.surface(field(arguments, "src_surface"));
        let at = (
            target,
            decimal(field(arguments, "dst_x")),
            decimal(field(arguments, "dst_y")),
        );
        let from = rect(field(arguments, "src_rect"));
        self.surfaces
            .blt_fast(at, source, from, hex(field(arguments, "flags")))
    }

    fn lock(&mut self, arguments: &str) -> u32 {
        let surface = self.surface(field(arguments, "iface"));
        assert_eq!(field(arguments, "rect"), "(null)");
        let mut desc: Desc = [0; DESC_WORDS];
        let flags = hex(field(arguments, "flags"));
        let answer = self.surfaces.lock(surface, None, &mut desc, flags);
        assert_eq!(desc[DESC_PITCH], PITCH);
        self.locked = desc[DESC_SURFACE];
        answer
    }

    /// `data` is the pointer Wine's `Lock` answered: a flip moved the memory under
    /// it, so each of Wine's addresses must always be the same one of ours.
    fn unlock(&mut self, arguments: &str) -> u32 {
        let data = hex(field(arguments, "data"));
        assert_eq!(*self.wine.entry(data).or_insert(self.locked), self.locked);
        assert_eq!(*self.ours.entry(self.locked).or_insert(data), data);
        self.surfaces
            .unlock(self.surface(field(arguments, "iface")))
    }
}

fn is_doubled(doubled: &BTreeSet<&str>, line: &str) -> bool {
    line.split('\t')
        .next()
        .is_some_and(|number| doubled.contains(number))
}

/// Runs `lines` on the game file's row: the names played, the counts, the presents.
fn replay<'l>(
    lines: impl Iterator<Item = &'l str>,
) -> (BTreeSet<String>, BTreeMap<String, u32>, u32) {
    let tally = Tally::default();
    let easel = Easel::new(&tally, 64, 32);
    let desk = Desk::new(Board);
    let mut run = Replay {
        surfaces: Surfaces::new(&GAME, &easel, &desk),
        primary: 0,
        back: 0,
        palette: 0,
        locked: 0,
        wine: BTreeMap::new(),
        ours: BTreeMap::new(),
        names: BTreeSet::new(),
        counts: BTreeMap::new(),
    };
    let doubled: BTreeSet<&str> = DOUBLED.split_whitespace().collect();
    for line in lines.filter(|line| !is_doubled(&doubled, line)) {
        run.step(line);
    }
    let (names, counts) = (run.names, run.counts);
    (names, counts, tally.0.get())
}

/// Each present comes of a flip, a `SetEntries` on the primary's palette or a
/// `SetPalette`: the trace's fills and `BltFast`s are all on the back.
fn presents_expected(counts: &BTreeMap<String, u32>) -> u32 {
    let count = |name: &str| counts.get(name).copied().unwrap_or(0);
    count("ddraw_surface1_Flip")
        + count("ddraw_palette_SetEntries")
        + count("ddraw_surface1_SetPalette")
}

/// The cut: `DirectDrawCreate` and the 16 methods the game calls, every line 0.
#[test]
fn the_cut_replays_every_call_the_game_makes() {
    let (names, counts, presents) = replay(CUT.lines());
    let expected = [
        "DirectDrawCreate",
        "ddraw1_CreatePalette",
        "ddraw1_CreateSurface",
        "ddraw1_GetCaps",
        "ddraw1_SetCooperativeLevel",
        "ddraw1_SetDisplayMode",
        "ddraw_palette_SetEntries",
        "ddraw_surface1_Blt",
        "ddraw_surface1_BltFast",
        "ddraw_surface1_Flip",
        "ddraw_surface1_GetAttachedSurface",
        "ddraw_surface1_GetSurfaceDesc",
        "ddraw_surface1_IsLost",
        "ddraw_surface1_Lock",
        "ddraw_surface1_Release",
        "ddraw_surface1_SetPalette",
        "ddraw_surface1_Unlock",
    ];
    let names: Vec<&str> = names.iter().map(String::as_str).collect();
    assert_eq!(names, expected, "1 import and 16 methods");
    assert_eq!(presents, presents_expected(&counts));
}

/// The whole kept trace, outside this repository, its counts `s2-trace-shapes.txt`'s.
#[test]
#[ignore = "reads the kept trace, named by SF1_TRACE, 18,484 lines of the records"]
fn the_whole_trace_replays() {
    let path = std::env::var("SF1_TRACE").expect("SF1_TRACE names s2-trace-calls.tsv");
    let text = std::fs::read_to_string(path).expect("the kept trace is readable");
    let doubled: BTreeSet<&str> = DOUBLED.split_whitespace().collect();
    let cut = text.lines().filter(|line| is_doubled(&doubled, line));
    assert_eq!(cut.count(), 57, "each listed line is in the kept trace");
    let (names, counts, presents) = replay(text.lines());
    assert_eq!(names.len(), 17);
    let count = |name: &str| counts.get(name).copied().unwrap_or(0);
    assert_eq!(
        count("ddraw_surface1_Flip"),
        1630,
        "1,632 lines, 2 printed twice"
    );
    assert_eq!(
        count("ddraw_surface1_Lock"),
        1718,
        "1,725 lines, 7 printed twice"
    );
    assert_eq!(
        count("ddraw_surface1_Unlock"),
        1718,
        "1,723 lines, 5 printed twice"
    );
    assert_eq!(
        count("ddraw_surface1_Blt"),
        1642,
        "1,645 lines, 3 printed twice"
    );
    assert_eq!(
        count("ddraw_surface1_BltFast"),
        1623,
        "1,626 lines, 3 printed twice"
    );
    assert_eq!(
        count("ddraw_surface1_IsLost"),
        5081,
        "5,103 lines, 22 printed twice"
    );
    assert_eq!(
        count("ddraw_surface1_GetSurfaceDesc"),
        9,
        "10 lines, 1 printed twice"
    );
    assert_eq!(count("ddraw_palette_SetEntries"), 91);
    assert_eq!(count("ddraw_surface1_SetPalette"), 3);
    assert_eq!(count("ddraw_surface1_Release"), 2);
    assert_eq!(presents, presents_expected(&counts));
    eprintln!("the whole trace: {counts:?}, {presents} presents");
}

/// The 4 no run of the game reached: `RestoreDisplayMode`, the object's `Release`,
/// `Restore`, the palette's `Release`. No line says what any answered: these hold
/// the plan's answer on the rows, read from callers who read nothing back.
#[test]
fn the_four_no_run_reached_answer_the_plan() {
    let tally = Tally::default();
    let easel = Easel::new(&tally, 320, 200);
    let desk = Desk::new(Board);
    let surfaces = Surfaces::new(&SHELL, &easel, &desk);
    assert_eq!(surfaces.direct_draw_create(0, 0), DD_OK);
    let mut asked: Desc = [0; DESC_WORDS];
    asked[DESC_SIZE] = DESC_BYTES;
    asked[DESC_FLAGS] = FLAGS_BACK_COUNT;
    asked[DESC_BACK_COUNT] = CHAIN;
    asked[DESC_CAPS] = CAPS_CHAIN;
    let (mut primary, mut palette) = (0, 0);
    assert_eq!(surfaces.create_surface(&asked, &mut primary), DD_OK);
    assert_eq!(
        surfaces.create_palette(PALETTE_FLAGS, &[0; 256], &mut palette),
        DD_OK
    );
    assert_eq!(surfaces.restore(primary), DD_OK);
    assert_eq!(surfaces.release_palette(palette), 0);
    assert_eq!(surfaces.restore_display_mode(), DD_OK);
    assert_eq!(
        surfaces.set_cooperative_level(WINDOW, 8),
        DD_OK,
        "the shell's teardown"
    );
    assert_eq!(surfaces.release(), 0);
}

/// The one mode measured, each file its levels: the game file's `0x51`, the shell's
/// `0x51` and teardown's 8.
#[test]
fn each_row_holds_what_its_callers_ask() {
    assert_eq!(MODES, &[(320, 200, 8)]);
    assert!(GAME.levels.contains(&0x51));
    assert!(SHELL.levels.contains(&0x51) && SHELL.levels.contains(&8));
    assert_eq!(GAME.palette_flags, 0x44);
    assert_eq!((GAME.file, SHELL.file), ("mercsw", "mw2shell"));
}
