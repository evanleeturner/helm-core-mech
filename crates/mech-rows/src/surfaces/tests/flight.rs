// SPDX-License-Identifier: MIT
//! The flight's lines harvested: SF1's `f2`, `logbook-f2.log` in SF1's records, kept whole as
//! `flight-f2.log` and played on the game file's row. Wine's trace (the parent) holds entries
//! alone; these hold what the real `DirectDraw` answered and wrote through the real tables
//! (`mech_layer::surfaces`), so each stand-in answer is held against the real one.
//!
//! The last `imp.call` is the 7th `Flip`; then W5, Wine's own copy faulting in `ucrtbase`. A
//! line is written when its method returns, so a call that never returned has none: these
//! lines do not say which call W5 fell in. The game reads `lPitch` and `Lock`'s `lpSurface` of
//! a description and nothing else (`s1-readback.md`): those are held. The words no caller
//! reads are named in [`UNREAD_DIFFERS`], not made equal.
use std::collections::{BTreeMap, BTreeSet};

use draw::easel::Easel;
use draw_surfaces::sdk::{
    DESC_BYTES, DESC_CAPS, DESC_FLAGS, DESC_PITCH, DESC_SIZE, DESC_SURFACE, DESC_WORDS, Desc,
};
use draw_surfaces::surfaces::Surfaces;
use window::desk::Desk;

use super::{BANKSWITCHED, Board, GAME, Tally, hex};

/// The lines, whole.
const FLIGHT: &str = include_str!("../flight-f2.log");
/// `MERCSW.DLL`'s image, its own header's base and size: a `from` outside is Wine's own line.
const GAME_FILE: std::ops::Range<u32> = 0x1000_0000..0x1000_0000 + GAME_FILE_BYTES;
const GAME_FILE_BYTES: u32 = 0x22_9000;
/// Each `(surface, word, real, ours)` the stand-in's description differs in, none read: the
/// real back sets `DDSD_BACKBUFFERCOUNT` over a count of 0; the real primary adds `VISIBLE`
/// and `SYSTEMMEMORY`.
const UNREAD_DIFFERS: &[(&str, usize, u32, u32)] = &[
    ("back", DESC_FLAGS, 0x102f, 0x100f),
    ("primary", DESC_CAPS, 0x8a38, 0x238),
];

/// One `imp.call` line, by `mech_layer::surfaces`' rule.
struct Call<'l> {
    name: &'l str,
    numbers: Vec<u32>,
    value: u32,
    wrote: Vec<u32>,
    from: u32,
}

/// The value after `name=` on a line; panics when a line has none.
fn field<'l>(line: &'l str, name: &str) -> &'l str {
    line.split(' ')
        .find_map(|part| part.strip_prefix(name)?.strip_prefix('='))
        .unwrap_or_else(|| panic!("{name} in {line}"))
}

/// A field's comma-separated hex words; `""` is none.
fn words(value: &str) -> Vec<u32> {
    value
        .trim_matches('"')
        .split(',')
        .filter(|word| !word.is_empty())
        .map(hex)
        .collect()
}

fn calls() -> Vec<Call<'static>> {
    FLIGHT
        .lines()
        .filter(|line| line.contains(" imp.call "))
        .map(|line| Call {
            name: field(line, "c"),
            numbers: words(field(line, "a")),
            value: hex(field(line, "v")),
            wrote: words(field(line, "o")),
            from: hex(field(line, "from")),
        })
        .collect()
}

/// `CreatePalette`'s colours, the `draw.table` lines in order, each opening at its own index.
fn colours() -> Vec<u32> {
    let mut all = Vec::new();
    let lines = FLIGHT.lines().filter(|line| line.contains(" draw.table "));
    for line in lines {
        assert_eq!(field(line, "c"), "IDirectDraw::CreatePalette");
        assert_eq!(hex(field(line, "i")) as usize, all.len());
        all.extend(words(field(line, "a")));
    }
    all
}

/// A logged rectangle's four words.
fn area(numbers: &[u32]) -> [i32; 4] {
    let edge = |index: usize| i32::try_from(numbers[index]).expect("an edge of a 320-by-200 mode");
    [edge(0), edge(1), edge(2), edge(3)]
}

/// One run of the lines on a live object, each real pointer tied to the stand-in's number.
struct Flight<'r> {
    surfaces: Surfaces<'r, &'r Tally, Board>,
    kept: BTreeMap<u32, u32>,
    primary: u32,
    /// The real `Lock`'s answer until its `Unlock`.
    locked: Option<u32>,
    /// Each real pixel address and ours: a flip moves both, and they must move as one.
    lent: BTreeMap<u32, u32>,
    lent_back: BTreeMap<u32, u32>,
    differs: BTreeSet<(&'static str, usize, u32, u32)>,
}

impl Flight<'_> {
    fn ours(&self, real: u32) -> u32 {
        *self
            .kept
            .get(&real)
            .unwrap_or_else(|| panic!("{real:x}, a pointer no line returned"))
    }

    /// Plays one line and answers the stand-in's return.
    fn step(&mut self, call: &Call<'_>) -> u32 {
        let numbers = &call.numbers;
        match call.name {
            "DDRAW!DirectDrawCreate" => self.surfaces.direct_draw_create(numbers[0], numbers[1]),
            "IDirectDraw::SetCooperativeLevel" => {
                self.surfaces.set_cooperative_level(numbers[1], numbers[2])
            }
            "IDirectDraw::SetDisplayMode" => self
                .surfaces
                .set_display_mode(numbers[1], numbers[2], numbers[3]),
            "IDirectDraw::GetCaps" => self.get_caps(call),
            "IDirectDraw::CreateSurface" => self.create_surface(call),
            "IDirectDraw::CreatePalette" => self.create_palette(call),
            "IDirectDrawSurface::GetAttachedSurface" => self.get_attached(call),
            "IDirectDrawSurface::GetSurfaceDesc" => self.get_desc(call),
            "IDirectDrawSurface::SetPalette" => self
                .surfaces
                .set_palette(self.ours(numbers[0]), self.ours(numbers[1])),
            "IDirectDrawSurface::IsLost" => self.surfaces.is_lost(self.ours(numbers[0])),
            "IDirectDrawSurface::Blt" => self.blt(numbers),
            "IDirectDrawSurface::Flip" => {
                self.surfaces
                    .flip(self.ours(numbers[0]), numbers[1], numbers[2])
            }
            "IDirectDrawSurface::Lock" => self.lock(call),
            "IDirectDrawSurface::Unlock" => self.unlock(numbers),
            other => panic!("a name this replay does not know: {other}"),
        }
    }

    /// The driver's record's size and the one bit a caller reads.
    fn get_caps(&self, call: &Call<'_>) -> u32 {
        let mut driver = [call.wrote[0], BANKSWITCHED];
        let answer = self.surfaces.get_caps(Some(&mut driver), None);
        assert_eq!(driver[0], call.wrote[0]);
        assert_eq!(driver[1] & BANKSWITCHED, call.wrote[1] & BANKSWITCHED);
        answer
    }

    /// `numbers`: `this`, the outer, the asked description's 27 words.
    fn create_surface(&mut self, call: &Call<'_>) -> u32 {
        let mut asked: Desc = [0; DESC_WORDS];
        asked.copy_from_slice(&call.numbers[2..]);
        let mut out = 0;
        let answer = self.surfaces.create_surface(&asked, &mut out);
        self.primary = call.wrote[0];
        self.kept.insert(call.wrote[0], out);
        answer
    }

    fn create_palette(&mut self, call: &Call<'_>) -> u32 {
        let mut out = 0;
        let answer = self
            .surfaces
            .create_palette(call.numbers[1], &colours(), &mut out);
        self.kept.insert(call.wrote[0], out);
        answer
    }

    /// Both real answers are one surface; the stand-in's must be one too.
    fn get_attached(&mut self, call: &Call<'_>) -> u32 {
        let mut out = 0;
        let answer = self.surfaces.get_attached_surface(
            self.ours(call.numbers[0]),
            call.numbers[1],
            &mut out,
        );
        assert_eq!(*self.kept.entry(call.wrote[0]).or_insert(out), out);
        answer
    }

    /// Holds the pitch, the word the caller keeps; the other words go to `differs`.
    fn get_desc(&mut self, call: &Call<'_>) -> u32 {
        let this = call.numbers[0];
        let mut desc: Desc = [0; DESC_WORDS];
        desc[DESC_SIZE] = DESC_BYTES;
        let answer = self.surfaces.get_surface_desc(self.ours(this), &mut desc);
        assert_eq!(desc[DESC_PITCH], call.wrote[DESC_PITCH]);
        let surface = if this == self.primary {
            "primary"
        } else {
            "back"
        };
        let pairs = call.wrote.iter().zip(desc).enumerate();
        for (word, (real, ours)) in pairs.filter(|(_, (real, ours))| **real != *ours) {
            self.differs.insert((surface, word, *real, ours));
        }
        answer
    }

    /// `numbers`: `this`, source, flags, fill, has-dest, 4, has-source, 4.
    fn blt(&self, numbers: &[u32]) -> u32 {
        assert_eq!((numbers[1], numbers[9]), (0, 0), "every fill has no source");
        let to = (numbers[4] != 0).then(|| area(&numbers[5..9]));
        self.surfaces
            .blt(self.ours(numbers[0]), to, (0, None), numbers[2], numbers[3])
    }

    /// `numbers`: `this`, flags, event, and a rectangle only when asked for one.
    fn lock(&mut self, call: &Call<'_>) -> u32 {
        assert_eq!(call.numbers.len(), 3, "every Lock asks the whole surface");
        let mut desc: Desc = [0; DESC_WORDS];
        let answer =
            self.surfaces
                .lock(self.ours(call.numbers[0]), None, &mut desc, call.numbers[1]);
        let (real, ours) = (call.wrote[DESC_SURFACE], desc[DESC_SURFACE]);
        assert_eq!(desc[DESC_PITCH], call.wrote[DESC_PITCH]);
        assert_eq!(*self.lent.entry(real).or_insert(ours), ours);
        assert_eq!(*self.lent_back.entry(ours).or_insert(real), real);
        assert_eq!(self.locked.replace(real), None, "a Lock before an Unlock");
        answer
    }

    /// `numbers`: `this`, the pointer the real `Lock` answered.
    fn unlock(&mut self, numbers: &[u32]) -> u32 {
        assert_eq!(self.locked.take(), Some(numbers[1]));
        self.surfaces.unlock(self.ours(numbers[0]))
    }
}

/// What a run came to: each name's lines, the real addresses, the differs, the presents.
struct Flown {
    counts: BTreeMap<&'static str, u32>,
    addresses: usize,
    differs: BTreeSet<(&'static str, usize, u32, u32)>,
    presents: u32,
}

/// Runs every line on the game file's row, each return held against the real one.
fn fly() -> Flown {
    let tally = Tally::default();
    let easel = Easel::new(&tally, 64, 32);
    let desk = Desk::new(Board);
    let mut run = Flight {
        surfaces: Surfaces::new(&GAME, &easel, &desk),
        kept: BTreeMap::new(),
        primary: 0,
        locked: None,
        lent: BTreeMap::new(),
        lent_back: BTreeMap::new(),
        differs: BTreeSet::new(),
    };
    let mut counts = BTreeMap::new();
    for call in calls() {
        assert!(GAME_FILE.contains(&call.from), "{:x}", call.from);
        assert_eq!(run.step(&call), call.value, "{}", call.name);
        *counts.entry(call.name).or_default() += 1;
    }
    assert_eq!(run.locked, None, "the last Lock met its Unlock");
    let (addresses, differs) = (run.lent.len(), run.differs);
    Flown {
        counts,
        addresses,
        differs,
        presents: tally.0.get(),
    }
}

/// The import and 13 methods, every real return and read word the stand-in's; each present
/// comes of a `Flip` or the primary's `SetPalette`.
#[test]
fn the_flight_replays_every_real_answer() {
    let flown = fly();
    let count = |name: &str| flown.counts.get(name).copied().unwrap_or(0);
    assert_eq!(flown.counts.values().sum::<u32>(), 70);
    assert_eq!(flown.counts.len(), 14, "{:?}", flown.counts);
    assert_eq!(
        flown.presents,
        count("IDirectDrawSurface::Flip") + count("IDirectDrawSurface::SetPalette")
    );
}

/// Three real pixel addresses, each always the same one of ours: the stand-in's flip turns the
/// chain as the real one did.
#[test]
fn the_flight_lends_the_real_chain() {
    assert_eq!(fly().addresses, 3);
}

/// The words of a description no caller reads, named so a change among them is seen.
#[test]
fn the_unread_words_that_differ_are_named() {
    let named: BTreeSet<_> = UNREAD_DIFFERS.iter().copied().collect();
    assert_eq!(fly().differs, named);
}

/// Each table rewritten once, the object's 7 slots, the surface's 11, the palette's 2.
#[test]
fn the_flight_wrapped_each_table_once() {
    let wrapped: Vec<(&str, usize)> = FLIGHT
        .lines()
        .filter(|line| line.contains(" surfaces.wrap "))
        .map(|line| {
            assert_eq!(hex(field(line, "v")), 1);
            (field(line, "c"), words(field(line, "a")).len())
        })
        .collect();
    let expected = [
        ("IDirectDraw", 7),
        ("IDirectDrawSurface", 11),
        ("IDirectDrawPalette", 2),
    ];
    assert_eq!(wrapped, expected);
}
