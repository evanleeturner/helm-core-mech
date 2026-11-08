// SPDX-License-Identifier: MIT
//! The bind, the half that judges: the rig's swap script's `sections`, `file_bytes`,
//! `header_key`, `iat_slot`, the tables' rows, `check_slots`, `point_slots` and `rebased`,
//! with no debugger. What it reads and writes comes through 2 seams, [`Image`] and
//! [`Memory`], so each judgment runs on the host against bytes a test holds; the process's
//! own memory behind them is the Windows side's.
//!
//! No game is named here: slots, exports and sites are the tables' and `mech-rows`'. A
//! refusal comes before any write, as the script's did: [`bind_table`] reads a whole table
//! first, and a bad line or a slot holding no import stops it with nothing written.
use core::fmt;

use mech_rows::sites::Patched;

/// `IMAGE_SCN_MEM_SHARED`: a page every process of the file shares, so a write here would
/// reach past this process. Replaces the `s` the script read in `/proc`'s maps.
const MEM_SHARED: u32 = 0x1000_0000;
/// A loader maps whole pages, so a section's last page is its own to the page's end: where
/// `wail32.dll`'s site 4 stands, `0x1dffc`, with `.data` running to `0x1d904`. `/proc`'s maps,
/// which the script read, know no finer than this.
const PAGE: u32 = 0x1000;
/// Room for the longest site's row, 62 bytes. The swap has no allocator, so rows move on the
/// stack; a row past this refuses rather than cut.
const ROW_BYTES: usize = 64;
/// Room for the sites and the guards a plan holds, on the stack for the same reason.
const MOST_SITES: usize = 8;
const MOST_GUARDS: usize = 8;

/// An address as the logs and the script's words show it.
pub(crate) struct Hex(pub(crate) u32);

impl fmt::Display for Hex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:x}", self.0)
    }
}

/// A row of bytes as the logs show it.
struct Bytes<'a>(&'a [u8]);

impl fmt::Display for Bytes<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.iter().try_for_each(|byte| write!(f, "{byte:02x}"))
    }
}

/// Why a bind stopped before writing.
#[derive(Debug, PartialEq, Eq)]
pub enum Refusal<'a> {
    /// A table's line without its fields.
    BadLine(&'a str),
    /// A slot holding 0, unbound or unreadable, or an address in its own file.
    NotAnImport {
        slot: u32,
        label: &'a str,
        value: u32,
    },
    /// An address the swap set names and its table lacks.
    NotInTable(&'a str),
    /// A DLL holding sites is not mapped: the script's `VERDICT P`.
    NotMapped(&'a str),
    /// A thread stands inside a guard: `VERDICT P`.
    ThreadInGuard { pc: u32 },
    /// A site's live bytes are not its old row: `VERDICT P`.
    OldRowUnequal { site: u8 },
    /// A site on a shared page, or in no section: `VERDICT SHARED`.
    SharedPage { site: u8 },
    /// A site's new row did not read back, its write done: `VERDICT Q`.
    ReadBack { site: u8 },
    /// A row this plan has no room for, or an old row and a new row of unequal lengths.
    BadSite { site: u8 },
    /// The other threads could not all be stopped, so no site is judged.
    ThreadsNotStopped(&'a str),
    /// A pieces setting that is none of [`pieces`]' words.
    BadPieces(&'a str),
}

impl fmt::Display for Refusal<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadLine(line) => write!(f, "a table's line lacks its fields: {line}"),
            Self::NotAnImport { slot, label, value } => write!(
                f,
                "layer slot {slot:#x} ({label}) holds {value:#x}, not an import outside its file"
            ),
            Self::NotInTable(address) => write!(f, "not in the table: {address}"),
            Self::NotMapped(dll) => write!(f, "{dll} holds sites and is not mapped"),
            Self::ThreadInGuard { pc } => write!(f, "a thread at {pc:#x} is inside a guard"),
            Self::OldRowUnequal { site } => write!(f, "site {site} is unequal to its old row"),
            Self::SharedPage { site } => write!(f, "site {site} is on a shared or unmapped page"),
            Self::ReadBack { site } => write!(f, "site {site} read back unequal to its new row"),
            Self::BadSite { site } => write!(f, "site {site}'s rows do not fit the plan"),
            Self::ThreadsNotStopped(why) => write!(f, "threads not stopped: {why}"),
            Self::BadPieces(value) => {
                write!(f, "pieces is unset, off, or off,<export>,...; not {value:?}")
            }
        }
    }
}

/// A PE image read by RVA: a file through its section table, or a mapped module directly.
pub trait Image {
    /// The bytes at `rva`, at most `size`, fewer at the image's end; `None` outside it.
    fn at(&self, rva: u32, size: usize) -> Option<&[u8]>;
}

/// A DLL or EXE as its file holds it.
pub struct File<'a>(pub &'a [u8]);

/// One row of a file's section table.
#[derive(Debug, PartialEq, Eq)]
pub struct Section<'a> {
    pub name: &'a [u8],
    pub va: u32,
    pub vsize: u32,
    pub raw: u32,
    pub rsize: u32,
    pub flags: u32,
}

fn le16(bytes: &[u8], at: usize) -> Option<u16> {
    Some(u16::from_le_bytes(bytes.get(at..at + 2)?.try_into().ok()?))
}

fn le32(bytes: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_le_bytes(bytes.get(at..at + 4)?.try_into().ok()?))
}

fn cut(bytes: &[u8], start: usize, size: usize) -> Option<&[u8]> {
    bytes.get(start..start.saturating_add(size).min(bytes.len()))
}

/// The file's sections, in the table's order; `None` when its headers do not read.
#[must_use]
pub fn sections(image: &[u8]) -> Option<impl Iterator<Item = Section<'_>>> {
    let pe = le32(image, 0x3c)? as usize;
    let count = le16(image, pe + 6)?;
    let optional = usize::from(le16(image, pe + 20)?);
    Some((0..usize::from(count)).filter_map(move |i| {
        let at = pe + 24 + optional + 40 * i;
        let name = image.get(at..at + 8)?;
        Some(Section {
            name: &name[..name.iter().position(|b| *b == 0).unwrap_or(8)],
            vsize: le32(image, at + 8)?,
            va: le32(image, at + 12)?,
            rsize: le32(image, at + 16)?,
            raw: le32(image, at + 20)?,
            flags: le32(image, at + 36)?,
        })
    }))
}

impl Image for File<'_> {
    /// Below the first section an RVA is its file offset: the headers, which `file_bytes`
    /// never had to answer because the script read them by offset.
    fn at(&self, rva: u32, size: usize) -> Option<&[u8]> {
        let mut first = u32::MAX;
        for section in sections(self.0)? {
            first = first.min(section.va);
            if section.va <= rva && rva - section.va < section.vsize.min(section.rsize) {
                let start = (section.raw as usize).saturating_add((rva - section.va) as usize);
                return cut(self.0, start, size);
            }
        }
        if rva < first {
            cut(self.0, rva as usize, size)
        } else {
            None
        }
    }
}

fn word(image: &impl Image, rva: u32) -> Option<u32> {
    Some(u32::from_le_bytes(image.at(rva, 4)?.try_into().ok()?))
}

/// The header fields a mapped copy keeps, which name a file whatever its base.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct HeaderKey {
    pub pe: u32,
    pub stamp: u32,
    pub size: u32,
}

/// `e_lfanew`, `TimeDateStamp` and `SizeOfImage`: is this mapped module that file?
pub fn header_key(image: &impl Image) -> Option<HeaderKey> {
    let pe = word(image, 0x3c)?;
    Some(HeaderKey {
        pe,
        stamp: word(image, pe + 8)?,
        size: word(image, pe + 24 + 56)?,
    })
}

fn text(image: &impl Image, rva: u32) -> Option<&[u8]> {
    let raw = image.at(rva, 256)?;
    Some(&raw[..raw.iter().position(|b| *b == 0).unwrap_or(raw.len())])
}

/// The slot `lib!name` is called through, at the file's preferred base; `None` if the file
/// does not import it. The library's name matches in any case and the function's exactly,
/// as the loader's do. A library with no lookup table is walked by its slots, so this finds
/// nothing in a mapped copy of one: its loader has overwritten them.
pub fn iat_slot(image: &impl Image, lib: &str, name: &str) -> Option<u32> {
    let pe = word(image, 0x3c)?;
    let directory = word(image, pe + 24 + 96 + 8)?;
    let base = word(image, pe + 24 + 28)?;
    let mut at = directory;
    loop {
        let lookup = word(image, at)?;
        let dll = word(image, at + 12)?;
        let first = word(image, at + 16)?;
        if dll == 0 {
            return None;
        }
        if text(image, dll)?.eq_ignore_ascii_case(lib.as_bytes()) {
            let walk = if lookup == 0 { first } else { lookup };
            let mut i = 0;
            loop {
                let entry = word(image, walk + 4 * i)?;
                if entry == 0 {
                    break;
                }
                if entry & 0x8000_0000 == 0 && text(image, entry + 2)? == name.as_bytes() {
                    let slot = base + first + 4 * i;
                    log::debug!(lib = lib, name = name, slot:% = Hex(slot); "bind.import");
                    return Some(slot);
                }
                i += 1;
            }
        }
        at += 20;
    }
}

fn hex(field: &str) -> Option<u32> {
    u32::from_str_radix(field.trim_start_matches("0x"), 16).ok()
}

/// One line of a layer table: `slot`, `export`, the export's address, `DLL!NAME`.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SlotRow<'a> {
    pub slot: u32,
    pub export: &'a str,
    pub target: u32,
    pub label: &'a str,
}

/// A layer table's rows; a line without its 4 fields is its own refusal.
pub fn slot_rows(table: &str) -> impl Iterator<Item = Result<SlotRow<'_>, Refusal<'_>>> {
    table.lines().filter(|l| !l.trim().is_empty()).map(|line| {
        let mut fields = line.split('\t');
        let row = (|| {
            Some(SlotRow {
                slot: hex(fields.next()?)?,
                export: fields.next()?,
                target: hex(fields.next()?)?,
                label: fields.next()?,
            })
        })();
        row.filter(|_| fields.next().is_none()).ok_or(Refusal::BadLine(line))
    })
}

/// One line of the swap table: the shell's address, the export, the export's address.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SwapRow<'a> {
    pub address: u32,
    pub export: &'a str,
    pub target: u32,
}

fn swap_rows(table: &str) -> impl Iterator<Item = Result<SwapRow<'_>, Refusal<'_>>> {
    table.lines().filter(|l| !l.trim().is_empty()).map(|line| {
        let mut fields = line.split('\t');
        let row = (|| {
            Some(SwapRow {
                address: hex(fields.next()?)?,
                export: fields.next()?,
                target: hex(fields.next()?)?,
            })
        })();
        row.filter(|_| fields.next().is_none()).ok_or(Refusal::BadLine(line))
    })
}

/// Hands `each` the swap set's rows and says how many: `none`, `all`, or a comma list kept
/// in its own order. The whole table and every named address are judged before the first
/// row is handed on, so a refusal leaves nothing half done.
pub fn for_chosen<'a>(
    table: &'a str,
    chosen: &'a str,
    mut each: impl FnMut(SwapRow<'a>),
) -> Result<u32, Refusal<'a>> {
    if chosen == "none" {
        return Ok(0);
    }
    for row in swap_rows(table) {
        row?;
    }
    let find = |word: &'a str| {
        let address = hex(word).ok_or(Refusal::NotInTable(word))?;
        swap_rows(table)
            .flatten()
            .find(|row| row.address == address)
            .ok_or(Refusal::NotInTable(word))
    };
    let mut count = 0;
    if chosen == "all" {
        for row in swap_rows(table).flatten() {
            each(row);
            count += 1;
        }
        return Ok(count);
    }
    for word in chosen.split(',') {
        find(word)?;
    }
    for word in chosen.split(',') {
        each(find(word)?);
        count += 1;
    }
    Ok(count)
}

/// The process's memory, or a test's.
pub trait Memory {
    /// Fills `out` from `at`; false when it does not read.
    fn read(&self, at: u32, out: &mut [u8]) -> bool;
    /// Writes `bytes` at `at`, its page's protection handled; false when refused.
    fn write(&mut self, at: u32, bytes: &[u8]) -> bool;
}

/// Writes and reads back; false unless every byte came back.
fn written(memory: &mut impl Memory, at: u32, bytes: &[u8]) -> bool {
    memory.write(at, bytes)
        && bytes.iter().zip(at..).all(|(want, place)| {
            let mut got = [0];
            memory.read(place, &mut got) && got[0] == *want
        })
}

/// The table's first bad line, or first slot holding 0 or an address in its own file
/// `[low, high)`: such a slot is unbound, so what the layer would answer has no original.
pub fn check_slots<'a>(
    table: &'a str,
    memory: &impl Memory,
    low: u32,
    high: u32,
) -> Result<u32, Refusal<'a>> {
    let mut count = 0;
    for row in slot_rows(table) {
        let row = row?;
        let mut raw = [0; 4];
        let value = if memory.read(row.slot, &mut raw) { u32::from_le_bytes(raw) } else { 0 };
        log::debug!(
            slot:% = Hex(row.slot), label = row.label, export = row.export, value:% = Hex(value);
            "bind.slot.live"
        );
        if value == 0 || (low <= value && value < high) {
            return Err(Refusal::NotAnImport {
                slot: row.slot,
                label: row.label,
                value,
            });
        }
        count += 1;
    }
    Ok(count)
}

/// What [`point_slots`] did: no list, the swap having no allocator; each failure has its
/// own ERROR line, and the first is kept for the bind's one line.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub struct Pointed {
    pub written: u32,
    pub failed: u32,
    pub first_failed: Option<u32>,
}

/// Each slot pointed at its export and read back: [`point_routed`] with every piece on.
pub fn point_slots(table: &str, memory: &mut impl Memory) -> Pointed {
    point_routed(table, memory, &Routes::ALL, |_, _, _| None).pointed
}

/// One file's table, the whole of a load's bind: judged first, then written. Run at each
/// load, never once: `MERCSWIN.EXE`'s 2 files map at one base in turn, so the slots a first
/// bind wrote are gone with its file and the next file's are unbound again.
pub fn bind_table<'a>(
    table: &'a str,
    memory: &mut impl Memory,
    low: u32,
    high: u32,
) -> Result<Pointed, Refusal<'a>> {
    bind_routed(table, memory, low, high, &Routes::ALL, |_, _, _| None).map(|r| r.pointed)
}

/// Which pieces answer the calls. `All` is every row at its export, the build as it always
/// was. `Off` sends a row's call back to the original it replaced, but for the rows a start
/// cannot go without and the exports `back` names, a comma list put back one by one.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Pieces<'a> {
    All,
    Off { back: &'a str },
}

/// The setting, read once at the bind. Unset is `All`; `off`, or `off,` and a comma list of
/// exports, each named and with no space, is `Off`; anything else refuses, so a mistyped
/// setting never runs as one it did not name.
pub fn pieces(value: Option<&str>) -> Result<Pieces<'_>, Refusal<'_>> {
    let Some(value) = value else {
        return Ok(Pieces::All);
    };
    if value == "off" {
        return Ok(Pieces::Off { back: "" });
    }
    let named = |name: &str| !name.is_empty() && !name.contains(char::is_whitespace);
    match value.strip_prefix("off,") {
        Some(back) if back.split(',').all(named) => Ok(Pieces::Off { back }),
        _ => Err(Refusal::BadPieces(value)),
    }
}

/// Where one row's slot is pointed.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Route {
    /// At the row's export, ours.
    Export,
    /// At a pass-through that logs the call and goes on to the original, answering nothing.
    Through,
    /// At the one row whose answer stays ours while the original does the work.
    Height,
}

impl Route {
    fn word(self) -> &'static str {
        match self {
            Self::Export => "export",
            Self::Through => "through",
            Self::Height => "height",
        }
    }
}

/// The setting with what it cannot turn off: `kept`, the exports a start needs, and
/// `height`, the export whose answer stays ours while the original does the work.
#[derive(Debug, Clone, Copy)]
pub struct Routes<'a> {
    pub pieces: Pieces<'a>,
    pub kept: &'a [&'a str],
    pub height: &'a str,
}

impl Routes<'_> {
    /// Every row at its export.
    pub const ALL: Self = Self {
        pieces: Pieces::All,
        kept: &[],
        height: "",
    };

    /// Where the row with `export` goes. A name put back or kept wins over the height row,
    /// so putting the height export back restores its front whole.
    #[must_use]
    pub fn route(&self, export: &str) -> Route {
        let Pieces::Off { back } = self.pieces else {
            return Route::Export;
        };
        let put_back = !back.is_empty() && back.split(',').any(|name| name == export);
        if put_back || self.kept.contains(&export) {
            Route::Export
        } else if !self.height.is_empty() && export == self.height {
            Route::Height
        } else {
            Route::Through
        }
    }
}

/// What [`point_routed`] did: the slots written and failed, and of those written, how many
/// went to their exports, to the pass-through, and to the height row.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub struct Routed {
    pub pointed: Pointed,
    pub export: u32,
    pub through: u32,
    pub height: u32,
}

/// Each slot pointed where `setting` sends its row, and read back. An export row takes its
/// export; any other reads the slot's live value, the original, and takes what `elsewhere`
/// hands back for it. `None` from `elsewhere` fails that row alone, as a write that does not
/// read back does: it goes on past a failure, as the script did, so the line counts them
/// all. A bad line counts as failed, never skipped.
pub fn point_routed<'a>(
    table: &'a str,
    memory: &mut impl Memory,
    setting: &Routes<'_>,
    mut elsewhere: impl FnMut(Route, &'a str, u32) -> Option<u32>,
) -> Routed {
    let mut routed = Routed::default();
    for row in slot_rows(table) {
        let Ok(row) = row else {
            routed.pointed.failed += 1;
            continue;
        };
        let route = setting.route(row.export);
        let target = match route {
            Route::Export => Some(row.target),
            Route::Through | Route::Height => {
                let mut raw = [0; 4];
                memory
                    .read(row.slot, &mut raw)
                    .then(|| u32::from_le_bytes(raw))
                    .and_then(|live| elsewhere(route, row.label, live))
            }
        };
        match target {
            Some(target) if written(memory, row.slot, &target.to_le_bytes()) => {
                routed.pointed.written += 1;
                *match route {
                    Route::Export => &mut routed.export,
                    Route::Through => &mut routed.through,
                    Route::Height => &mut routed.height,
                } += 1;
                if route == Route::Export {
                    log::debug!(
                        slot:% = Hex(row.slot), label = row.label, export = row.export,
                        target:% = Hex(target);
                        "bind.slot.pointed"
                    );
                } else {
                    log::debug!(
                        slot:% = Hex(row.slot), label = row.label, export = row.export,
                        route = route.word(), target:% = Hex(target);
                        "bind.slot.routed"
                    );
                }
            }
            _ => {
                routed.pointed.failed += 1;
                routed.pointed.first_failed.get_or_insert(row.slot);
                log::error!(slot:% = Hex(row.slot), label = row.label; "bind.slot.failed");
            }
        }
    }
    routed
}

/// [`bind_table`] with `routes`: judged first by the same [`check_slots`], then written.
pub fn bind_routed<'a>(
    table: &'a str,
    memory: &mut impl Memory,
    low: u32,
    high: u32,
    routes: &Routes<'_>,
    elsewhere: impl FnMut(Route, &'a str, u32) -> Option<u32>,
) -> Result<Routed, Refusal<'a>> {
    check_slots(table, memory, low, high)?;
    Ok(point_routed(table, memory, routes, elsewhere))
}

/// `row` with each 32-bit word naming its own DLL's image moved to the loaded base; how many
/// moved. A word found steps 4 bytes and a miss 1, so unaligned words in code are found, and
/// a byte is never part of 2 moved words.
pub fn rebased(row: &mut [u8], preferred: u32, size: u32, actual: u32) -> u32 {
    let (mut moved, mut i) = (0, 0);
    while i + 4 <= row.len() {
        let value = u32::from_le_bytes([row[i], row[i + 1], row[i + 2], row[i + 3]]);
        if preferred <= value && value - preferred < size {
            let word = value - preferred + actual;
            row[i..i + 4].copy_from_slice(&word.to_le_bytes());
            log::debug!(place = i, from:% = Hex(value), to:% = Hex(word); "bind.rebased");
            moved += 1;
            i += 4;
        } else {
            i += 1;
        }
    }
    moved
}

/// A DLL as the process holds it: where it is, how far it runs, and its header page, which
/// holds the section table.
#[derive(Clone, Copy)]
pub struct Loaded<'a> {
    pub base: u32,
    pub size: u32,
    pub headers: &'a [u8],
}

#[derive(Clone, Copy)]
struct Moved {
    n: u8,
    at: u32,
    len: usize,
    old: [u8; ROW_BYTES],
    new: [u8; ROW_BYTES],
}

impl Moved {
    fn old_row(&self) -> &[u8] {
        &self.old[..self.len]
    }

    fn new_row(&self) -> &[u8] {
        &self.new[..self.len]
    }
}

/// The sites and guards at their loaded addresses, made while every thread still runs, so
/// [`apply_sites`] has only to compare and write.
pub struct Plan {
    sites: [Moved; MOST_SITES],
    site_count: usize,
    guards: [(u32, u32); MOST_GUARDS],
    guard_count: usize,
}

/// Each row and guard followed to its DLL's loaded base, the static judgments made: a DLL
/// unmapped, a site on a shared page. `find` answers for a DLL by name; the DLL moves
/// because another prefers its base (measured on a flight: `wail32.dll` at `0x15e0000`).
pub fn plan_sites<'a, 'h>(
    rows: &'a [Patched],
    find: impl Fn(&str) -> Option<Loaded<'h>>,
) -> Result<Plan, Refusal<'a>> {
    let empty = Moved {
        n: 0,
        at: 0,
        len: 0,
        old: [0; ROW_BYTES],
        new: [0; ROW_BYTES],
    };
    let mut plan = Plan {
        sites: [empty; MOST_SITES],
        site_count: 0,
        guards: [(0, 0); MOST_GUARDS],
        guard_count: 0,
    };
    for row in rows {
        let loaded = find(row.dll).ok_or(Refusal::NotMapped(row.dll))?;
        log::info!(
            dll = row.dll, prefers:% = Hex(row.base), size:% = Hex(loaded.size),
            loaded:% = Hex(loaded.base); "bind.sites.dll"
        );
        let moved = |address: u32| address - row.base + loaded.base;
        for site in row.sites {
            let len = site.old.len();
            if len != site.new.len() || len > ROW_BYTES || plan.site_count == MOST_SITES {
                return Err(Refusal::BadSite { site: site.n });
            }
            let rva = site.at - row.base;
            let holding = sections(loaded.headers).into_iter().flatten().find(|section| {
                section.va <= rva && rva - section.va < section.vsize.next_multiple_of(PAGE)
            });
            if holding.is_none_or(|section| section.flags & MEM_SHARED != 0) {
                return Err(Refusal::SharedPage { site: site.n });
            }
            let mut entry = Moved {
                n: site.n,
                at: moved(site.at),
                len,
                ..empty
            };
            entry.old[..len].copy_from_slice(site.old);
            entry.new[..len].copy_from_slice(site.new);
            rebased(&mut entry.old[..len], row.base, loaded.size, loaded.base);
            rebased(&mut entry.new[..len], row.base, loaded.size, loaded.base);
            plan.sites[plan.site_count] = entry;
            plan.site_count += 1;
        }
        for (low, high) in row.guards {
            if plan.guard_count == MOST_GUARDS {
                return Err(Refusal::BadSite { site: 0 });
            }
            plan.guards[plan.guard_count] = (moved(*low), moved(*high));
            plan.guard_count += 1;
        }
    }
    Ok(plan)
}

/// The moment itself, every other thread stopped: a thread in a guard or a site unequal to
/// its old row refuses with nothing written, then each new row is written and read back.
///
/// **It logs nothing and calls no heap.** A stopped thread may hold the sink's lock or the
/// heap's, and this thread would wait on it for good. [`report_sites`] speaks after they run.
pub fn apply_sites(
    plan: &Plan,
    pcs: &[u32],
    memory: &mut impl Memory,
) -> Result<(), Refusal<'static>> {
    let guards = &plan.guards[..plan.guard_count];
    let sites = &plan.sites[..plan.site_count];
    for pc in pcs {
        if guards.iter().any(|(low, high)| low <= pc && pc <= high) {
            return Err(Refusal::ThreadInGuard { pc: *pc });
        }
    }
    for site in sites {
        let mut live = [0; ROW_BYTES];
        if !memory.read(site.at, &mut live[..site.len]) || &live[..site.len] != site.old_row() {
            return Err(Refusal::OldRowUnequal { site: site.n });
        }
    }
    for site in sites {
        if !written(memory, site.at, site.new_row()) {
            return Err(Refusal::ReadBack { site: site.n });
        }
    }
    Ok(())
}

/// What [`apply_sites`] could not say, once the threads run again: each thread's address and
/// each site's old, new and live bytes, whichever way it ended.
pub fn report_sites(plan: &Plan, pcs: &[u32], memory: &impl Memory) {
    for pc in pcs {
        log::debug!(pc:% = Hex(*pc); "bind.thread");
    }
    for (low, high) in &plan.guards[..plan.guard_count] {
        log::debug!(low:% = Hex(*low), high:% = Hex(*high); "bind.guard");
    }
    for site in &plan.sites[..plan.site_count] {
        let mut live = [0; ROW_BYTES];
        let read = memory.read(site.at, &mut live[..site.len]);
        log::debug!(
            site = site.n, at:% = Hex(site.at), old:% = Bytes(site.old_row()),
            new:% = Bytes(site.new_row()), live:% = Bytes(if read { &live[..site.len] } else { &[] });
            "bind.site"
        );
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::vec;
    use std::vec::Vec;

    use super::*;

    const BASE: u32 = 0x1000_0000;
    const PE_WORD: u32 = 0x40;
    const PE: usize = PE_WORD as usize;
    const RAW_WORD: u32 = 0x200;
    const RAW: usize = RAW_WORD as usize;
    const VA: u32 = 0x1000;

    fn put(image: &mut [u8], at: usize, value: u32) {
        image[at..at + 4].copy_from_slice(&value.to_le_bytes());
    }

    /// One `.idata` section: `KERNEL32.dll` with a lookup table holding a name, an ordinal
    /// and a name, and `USER32.dll` with none, walked by its slots.
    fn image() -> Vec<u8> {
        let mut image = vec![0u8; RAW + 0x800];
        put(&mut image, 0x3c, PE_WORD);
        image[PE + 6] = 1;
        put(&mut image, PE + 8, 0x3333_0001);
        image[PE + 20] = 0xe0;
        put(&mut image, PE + 24 + 28, BASE);
        put(&mut image, PE + 24 + 56, 0x9000);
        put(&mut image, PE + 24 + 96 + 8, VA);
        let table = PE + 24 + 0xe0;
        image[table..table + 6].copy_from_slice(b".idata");
        put(&mut image, table + 8, 0x800);
        put(&mut image, table + 12, VA);
        put(&mut image, table + 16, 0x800);
        put(&mut image, table + 20, RAW_WORD);
        put(&mut image, table + 36, 0xc000_0040);
        let file = |rva: u32| RAW + (rva - VA) as usize;
        for (at, lookup, dll, first) in [(VA, VA + 0x100, VA + 0x300, VA + 0x200), (VA + 20, 0, VA + 0x310, VA + 0x210)] {
            put(&mut image, file(at), lookup);
            put(&mut image, file(at + 12), dll);
            put(&mut image, file(at + 16), first);
        }
        for (at, value) in [
            (0x100, VA + 0x320),
            (0x104, 0x8000_0005),
            (0x108, VA + 0x340),
            (0x200, VA + 0x320),
            (0x204, 0x8000_0005),
            (0x208, VA + 0x340),
            (0x210, VA + 0x360),
        ] {
            put(&mut image, file(VA + at), value);
        }
        for (at, name) in [
            (0x300, &b"KERNEL32.dll"[..]),
            (0x310, b"USER32.dll"),
            (0x322, b"LoadLibraryA"),
            (0x342, b"GetProcAddress"),
            (0x362, b"PeekMessageA"),
        ] {
            let start = file(VA + at);
            image[start..start + name.len()].copy_from_slice(name);
        }
        image
    }

    #[test]
    fn sections_reads_the_table_and_rva_goes_through_it() {
        let image = image();
        let rows: Vec<_> = sections(&image).expect("headers").collect();
        assert_eq!(rows.len(), 1);
        assert_eq!((rows[0].name, rows[0].va, rows[0].raw), (&b".idata"[..], VA, RAW_WORD));
        assert_eq!(rows[0].flags, 0xc000_0040);
        let file = File(&image);
        assert_eq!(file.at(VA + 0x300, 5), Some(&b"KERNE"[..]));
        assert_eq!(file.at(VA + 0x800, 1), None, "past the section");
        assert_eq!(file.at(0x3c, 4), Some(&(PE_WORD).to_le_bytes()[..]), "the headers");
    }

    #[test]
    fn header_key_names_the_file() {
        let image = image();
        let key = header_key(&File(&image)).expect("headers");
        assert_eq!(
            key,
            HeaderKey { pe: PE_WORD, stamp: 0x3333_0001, size: 0x9000 }
        );
    }

    #[test]
    fn iat_slot_finds_by_name_past_an_ordinal_and_without_a_lookup_table() {
        let image = image();
        let file = File(&image);
        assert_eq!(iat_slot(&file, "KERNEL32.dll", "LoadLibraryA"), Some(BASE + VA + 0x200));
        assert_eq!(iat_slot(&file, "KERNEL32.dll", "GetProcAddress"), Some(BASE + VA + 0x208));
        assert_eq!(iat_slot(&file, "user32.DLL", "PeekMessageA"), Some(BASE + VA + 0x210));
        assert_eq!(iat_slot(&file, "KERNEL32.dll", "peekmessagea"), None);
        assert_eq!(iat_slot(&file, "KERNEL32.dll", "PeekMessageA"), None);
        assert_eq!(iat_slot(&file, "WAIL32.dll", "LoadLibraryA"), None);
    }

    /// Memory a test holds; writes at `frozen` are taken and do not land.
    #[derive(Default)]
    struct Fake {
        bytes: BTreeMap<u32, u8>,
        frozen: Option<u32>,
    }

    impl Fake {
        fn word(&mut self, at: u32, value: u32) {
            for (place, byte) in (at..).zip(value.to_le_bytes()) {
                self.bytes.insert(place, byte);
            }
        }
    }

    impl Memory for Fake {
        fn read(&self, at: u32, out: &mut [u8]) -> bool {
            (at..).zip(out.iter_mut()).all(|(place, byte)| {
                self.bytes.get(&place).map(|got| *byte = *got).is_some()
            })
        }

        fn write(&mut self, at: u32, bytes: &[u8]) -> bool {
            if self.frozen != Some(at) {
                for (place, byte) in (at..).zip(bytes) {
                    self.bytes.insert(place, *byte);
                }
            }
            true
        }
    }

    const TABLE: &str = "10001000\tL_A\t60000010\tKERNEL32!A\n10001004\tL_B\t60000020\tUSER32!B\n";

    fn bound() -> Fake {
        let mut memory = Fake::default();
        memory.word(0x1000_1000, 0x7b00_0010);
        memory.word(0x1000_1004, 0x7e00_0020);
        memory
    }

    fn slot(memory: &Fake, at: u32) -> u32 {
        let mut raw = [0; 4];
        assert!(memory.read(at, &mut raw));
        u32::from_le_bytes(raw)
    }

    #[test]
    fn a_bind_points_each_slot_and_says_so() {
        let mut memory = bound();
        let pointed = bind_table(TABLE, &mut memory, BASE, BASE + 0x9000).expect("no refusal");
        assert_eq!(pointed, Pointed { written: 2, failed: 0, first_failed: None });
        assert_eq!(slot(&memory, 0x1000_1000), 0x6000_0010);
        assert_eq!(slot(&memory, 0x1000_1004), 0x6000_0020);
    }

    #[test]
    fn a_slot_holding_no_import_refuses_with_nothing_written() {
        for bad in [0, BASE + 0x40] {
            let mut memory = bound();
            memory.word(0x1000_1004, bad);
            let refused = bind_table(TABLE, &mut memory, BASE, BASE + 0x9000);
            assert_eq!(
                refused,
                Err(Refusal::NotAnImport { slot: 0x1000_1004, label: "USER32!B", value: bad })
            );
            assert_eq!(slot(&memory, 0x1000_1000), 0x7b00_0010, "the first slot untouched");
        }
    }

    #[test]
    fn an_unreadable_slot_refuses_as_a_zero() {
        let mut memory = Fake::default();
        let refused = bind_table(TABLE, &mut memory, BASE, BASE + 0x9000);
        assert!(matches!(refused, Err(Refusal::NotAnImport { value: 0, .. })));
    }

    #[test]
    fn a_bad_line_refuses_with_nothing_written() {
        let table = "10001000\tL_A\t60000010\tKERNEL32!A\n10001004\tL_B\n";
        let mut memory = bound();
        let refused = bind_table(table, &mut memory, BASE, BASE + 0x9000);
        assert_eq!(refused, Err(Refusal::BadLine("10001004\tL_B")));
        assert_eq!(slot(&memory, 0x1000_1000), 0x7b00_0010);
    }

    #[test]
    fn a_write_that_does_not_read_back_is_counted_and_the_rest_go_on() {
        let mut memory = bound();
        memory.frozen = Some(0x1000_1000);
        let pointed = bind_table(TABLE, &mut memory, BASE, BASE + 0x9000).expect("no refusal");
        assert_eq!(
            pointed,
            Pointed { written: 1, failed: 1, first_failed: Some(0x1000_1000) }
        );
        assert_eq!(slot(&memory, 0x1000_1004), 0x6000_0020);
    }

    /// The second shell load has not flown, so held here on the host: 2 files mapping at one
    /// base in turn, each load finds its slots unbound again and takes a whole bind.
    #[test]
    fn each_load_takes_its_own_bind() {
        let mut memory = bound();
        for _ in 0..2 {
            let pointed = bind_table(TABLE, &mut memory, BASE, BASE + 0x9000).expect("a load");
            assert_eq!(pointed.written, 2);
            memory.word(0x1000_1000, 0x7b00_0010);
            memory.word(0x1000_1004, 0x7e00_0020);
        }
        let again = bind_table(TABLE, &mut memory, BASE, BASE + 0x9000).expect("a third load");
        assert_eq!(again.written, 2);
        assert_eq!(slot(&memory, 0x1000_1000), 0x6000_0010);
    }

    #[test]
    fn pieces_reads_unset_off_and_off_with_names_and_refuses_the_rest() {
        assert_eq!(pieces(None), Ok(Pieces::All));
        assert_eq!(pieces(Some("off")), Ok(Pieces::Off { back: "" }));
        assert_eq!(pieces(Some("off,A,B")), Ok(Pieces::Off { back: "A,B" }));
        for bad in ["", "on", "off,", "off,A,,B", "off, A", "OFF"] {
            assert_eq!(pieces(Some(bad)), Err(Refusal::BadPieces(bad)), "{bad:?}");
        }
    }

    #[test]
    fn every_piece_on_writes_as_the_bind_always_did_and_asks_nothing_elsewhere() {
        let mut plain = bound();
        let mut routed = bound();
        let pointed = bind_table(TABLE, &mut plain, BASE, BASE + 0x9000).expect("no refusal");
        let got = bind_routed(TABLE, &mut routed, BASE, BASE + 0x9000, &Routes::ALL, |_, _, _| {
            unreachable!("every row goes to its export")
        })
        .expect("no refusal");
        assert_eq!(got, Routed { pointed, export: 2, through: 0, height: 0 });
        assert_eq!(plain.bytes, routed.bytes);
    }

    const THREE: &str = "10001000\tL_A\t60000010\tKERNEL32!A\n\
                         10001004\tL_B\t60000020\tUSER32!B\n\
                         10001008\tL_C\t60000030\tGDI32!C\n";

    fn bound_three() -> Fake {
        let mut memory = bound();
        memory.word(0x1000_1008, 0x7d00_0030);
        memory
    }

    fn off(back: &str) -> Routes<'_> {
        Routes { pieces: Pieces::Off { back }, kept: &["L_A"], height: "L_B" }
    }

    #[test]
    fn off_keeps_a_kept_row_and_hands_the_rest_elsewhere_with_their_live_values() {
        let mut memory = bound_three();
        let mut asked = Vec::new();
        let routed = bind_routed(THREE, &mut memory, BASE, BASE + 0x9000, &off(""), |r, l, v| {
            asked.push((r, l, v));
            Some(if r == Route::Height { 0x5000_0004 } else { 0x5000_0008 })
        })
        .expect("no refusal");
        assert_eq!(
            asked,
            vec![
                (Route::Height, "USER32!B", 0x7e00_0020),
                (Route::Through, "GDI32!C", 0x7d00_0030)
            ]
        );
        let pointed = Pointed { written: 3, failed: 0, first_failed: None };
        assert_eq!(routed, Routed { pointed, export: 1, through: 1, height: 1 });
        assert_eq!(slot(&memory, 0x1000_1000), 0x6000_0010);
        assert_eq!(slot(&memory, 0x1000_1004), 0x5000_0004);
        assert_eq!(slot(&memory, 0x1000_1008), 0x5000_0008);
    }

    #[test]
    fn a_name_put_back_answers_from_its_export_the_height_row_too() {
        let mut memory = bound_three();
        let routed = bind_routed(THREE, &mut memory, BASE, BASE + 0x9000, &off("L_C,L_B"), {
            |_, _, _| unreachable!("every row is kept or put back")
        })
        .expect("no refusal");
        assert_eq!((routed.export, routed.through, routed.height), (3, 0, 0));
        assert_eq!(slot(&memory, 0x1000_1004), 0x6000_0020);
        assert_eq!(slot(&memory, 0x1000_1008), 0x6000_0030);
    }

    #[test]
    fn a_row_with_nowhere_to_go_fails_alone_and_the_rest_go_on() {
        let mut memory = bound_three();
        let routed = bind_routed(THREE, &mut memory, BASE, BASE + 0x9000, &off(""), |r, _, v| {
            (r == Route::Height).then_some(v)
        })
        .expect("no refusal");
        let pointed = Pointed { written: 2, failed: 1, first_failed: Some(0x1000_1008) };
        assert_eq!(routed, Routed { pointed, export: 1, through: 0, height: 1 });
        assert_eq!(slot(&memory, 0x1000_1008), 0x7d00_0030, "the failed slot untouched");
    }

    const SWAPS: &str = "100026c1\tHC5_100026c1\t60001000\n100042c0\tHC5_100042c0\t60002000\n";

    fn chosen(table: &'static str, set: &'static str) -> Result<Vec<u32>, Refusal<'static>> {
        let mut seen = Vec::new();
        for_chosen(table, set, |row| seen.push(row.address)).map(|count| {
            assert_eq!(count as usize, seen.len());
            seen
        })
    }

    #[test]
    fn the_swap_set_is_none_all_or_a_list_in_its_own_order() {
        assert_eq!(chosen(SWAPS, "none"), Ok(vec![]));
        assert_eq!(chosen(SWAPS, "all"), Ok(vec![0x1000_26c1, 0x1000_42c0]));
        assert_eq!(chosen(SWAPS, "100042c0,100026c1"), Ok(vec![0x1000_42c0, 0x1000_26c1]));
    }

    #[test]
    fn a_named_address_not_in_the_table_hands_on_nothing() {
        let mut seen = 0;
        let refused = for_chosen(SWAPS, "100026c1,deadbeef", |_| seen += 1);
        assert_eq!(refused, Err(Refusal::NotInTable("deadbeef")));
        assert_eq!(seen, 0);
    }

    #[test]
    fn rebased_moves_each_word_of_its_image_found_unaligned() {
        // `call [0x2001dffc]` then a word outside the image and one in it at an odd place.
        let mut row = vec![0xff, 0x15, 0xfc, 0xdf, 0x01, 0x20, 0, 0, 0, 0x30, 0x90, 0x10, 0, 0, 0x20];
        let moved = rebased(&mut row, 0x2000_0000, 0x10_0000, 0x2100_0000);
        assert_eq!(moved, 2);
        assert_eq!(&row[2..6], &0x2101_dffcu32.to_le_bytes());
        assert_eq!(&row[6..10], &[0, 0, 0, 0x30], "outside the image");
        assert_eq!(&row[11..15], &0x2100_0010u32.to_le_bytes()[..]);
    }

    #[test]
    fn rebased_at_its_own_base_changes_nothing() {
        let before = [0xfc, 0xdf, 0x01, 0x20, 0x90];
        let mut row = before;
        assert_eq!(rebased(&mut row, 0x2000_0000, 0x10_0000, 0x2000_0000), 1);
        assert_eq!(row, before);
    }

    const DLL_BASE: u32 = 0x2000_0000;
    const DLL_LOADED: u32 = 0x015e_0000;
    const DLL_SIZE: u32 = 0x2_1000;
    const SITE_AT: u32 = DLL_LOADED + 0x1000;

    /// A row whose code names its own DLL, so the move shows in both rows.
    const ROWS: &[Patched] = &[Patched {
        dll: "wail32.dll",
        base: DLL_BASE,
        sites: &[mech_rows::sites::Site {
            n: 7,
            at: DLL_BASE + 0x1000,
            old: &[0xa1, 0xfc, 0x0f, 0x00, 0x20],
            new: &[0xa3, 0xfc, 0x0f, 0x00, 0x20],
        }],
        guards: &[(DLL_BASE + 0x1000, DLL_BASE + 0x1004)],
    }];

    /// `image`'s section table, its one section running from `VA` for `0x800`, under flags.
    fn headers(flags: u32) -> Vec<u8> {
        let mut bytes = image();
        put(&mut bytes, PE + 24 + 0xe0 + 36, flags);
        bytes
    }

    fn plan(headers: &[u8]) -> Result<Plan, Refusal<'static>> {
        plan_sites(ROWS, |dll| {
            (dll == "wail32.dll").then_some(Loaded {
                base: DLL_LOADED,
                size: DLL_SIZE,
                headers,
            })
        })
    }

    /// The site's old row as it stands at the moved DLL.
    fn at_the_site() -> Fake {
        let mut memory = Fake::default();
        for (place, byte) in (SITE_AT..).zip([0xa1, 0xfc, 0x0f, 0x5e, 0x01]) {
            memory.bytes.insert(place, byte);
        }
        memory
    }

    fn site_bytes(memory: &Fake) -> Vec<u8> {
        (SITE_AT..SITE_AT + 5).map(|at| memory.bytes[&at]).collect()
    }

    #[test]
    fn a_site_follows_its_dll_and_is_written_with_its_words_moved() {
        let plan = plan(&headers(0x6000_0020)).expect("a plan");
        let mut memory = at_the_site();
        assert_eq!(apply_sites(&plan, &[0x0040_1528], &mut memory), Ok(()));
        assert_eq!(site_bytes(&memory), [0xa3, 0xfc, 0x0f, 0x5e, 0x01]);
    }

    #[test]
    fn a_dll_unmapped_refuses() {
        let got = plan_sites(ROWS, |_| None);
        assert_eq!(got.err(), Some(Refusal::NotMapped("wail32.dll")));
    }

    #[test]
    fn a_site_on_a_shared_page_refuses() {
        assert_eq!(plan(&headers(0x7000_0020)).err(), Some(Refusal::SharedPage { site: 7 }));
    }

    /// A dword, as site 4 is: zero, written zero, no guard, on the last page of `headers`'
    /// section, which runs from `VA` for `0x800`.
    const LAST_PAGE: &[Patched] = &[Patched {
        dll: "wail32.dll",
        base: DLL_BASE,
        sites: &[mech_rows::sites::Site {
            n: 8,
            at: DLL_BASE + VA + 0xffc,
            old: &[0; 4],
            new: &[0; 4],
        }],
        guards: &[],
    }];

    /// The same dword on the page after that section.
    const NEXT_PAGE: &[Patched] = &[Patched {
        dll: "wail32.dll",
        base: DLL_BASE,
        sites: &[mech_rows::sites::Site {
            n: 8,
            at: DLL_BASE + VA + 0x1000,
            old: &[0; 4],
            new: &[0; 4],
        }],
        guards: &[],
    }];

    fn plan_of(rows: &'static [Patched], headers: &[u8]) -> Result<u8, Refusal<'static>> {
        let found = plan_sites(rows, |_| {
            Some(Loaded {
                base: DLL_LOADED,
                size: DLL_SIZE,
                headers,
            })
        });
        found.map(|plan| plan.sites[0].n)
    }

    /// Measured on a flight: `wail32.dll`'s `.data` runs to `0x1d904` and site 4 stands at `0x1dffc`.
    /// The loader maps whole pages, so the last page's tail is the section's, as the script
    /// read it in `/proc`'s maps.
    #[test]
    fn a_site_past_its_sections_size_on_its_last_page_is_planned() {
        assert_eq!(plan_of(LAST_PAGE, &headers(0x6000_0020)), Ok(8));
    }

    #[test]
    fn a_site_past_its_sections_last_page_refuses() {
        let got = plan_of(NEXT_PAGE, &headers(0x6000_0020));
        assert_eq!(got, Err(Refusal::SharedPage { site: 8 }));
    }

    #[test]
    fn a_site_in_no_section_refuses() {
        let got = plan_sites(ROWS, |_| {
            Some(Loaded {
                base: DLL_LOADED,
                size: DLL_SIZE,
                headers: &[],
            })
        });
        assert_eq!(got.err(), Some(Refusal::SharedPage { site: 7 }));
    }

    #[test]
    fn a_thread_in_a_moved_guard_refuses_with_nothing_written() {
        let plan = plan(&headers(0x6000_0020)).expect("a plan");
        let mut memory = at_the_site();
        let before = site_bytes(&memory);
        let last = DLL_LOADED + 0x1004;
        let got = apply_sites(&plan, &[0x0040_1528, last], &mut memory);
        assert_eq!(got, Err(Refusal::ThreadInGuard { pc: last }));
        assert_eq!(site_bytes(&memory), before);
    }

    #[test]
    fn a_thread_at_the_guards_preferred_address_is_not_in_it() {
        let plan = plan(&headers(0x6000_0020)).expect("a plan");
        let mut memory = at_the_site();
        assert_eq!(apply_sites(&plan, &[DLL_BASE + 0x1000], &mut memory), Ok(()));
    }

    #[test]
    fn an_old_row_unequal_refuses_with_nothing_written() {
        let plan = plan(&headers(0x6000_0020)).expect("a plan");
        let mut memory = at_the_site();
        memory.bytes.insert(SITE_AT, 0x90);
        let before = site_bytes(&memory);
        assert_eq!(apply_sites(&plan, &[], &mut memory), Err(Refusal::OldRowUnequal { site: 7 }));
        assert_eq!(site_bytes(&memory), before);
    }

    #[test]
    fn a_second_apply_refuses_by_its_old_row() {
        // A second game load with the DLL still mapped: named, and not yet flown.
        let plan = plan(&headers(0x6000_0020)).expect("a plan");
        let mut memory = at_the_site();
        assert_eq!(apply_sites(&plan, &[], &mut memory), Ok(()));
        assert_eq!(apply_sites(&plan, &[], &mut memory), Err(Refusal::OldRowUnequal { site: 7 }));
    }

    #[test]
    fn a_write_that_does_not_read_back_refuses() {
        let plan = plan(&headers(0x6000_0020)).expect("a plan");
        let mut memory = at_the_site();
        memory.frozen = Some(SITE_AT);
        assert_eq!(apply_sites(&plan, &[], &mut memory), Err(Refusal::ReadBack { site: 7 }));
    }

    #[test]
    fn a_row_past_the_room_refuses() {
        const LONG: [u8; ROW_BYTES + 1] = [0; ROW_BYTES + 1];
        let rows = [Patched {
            dll: "a.dll",
            base: 0,
            sites: &[mech_rows::sites::Site {
                n: 9,
                at: 0,
                old: &LONG,
                new: &LONG,
            }],
            guards: &[],
        }];
        let got = plan_sites(&rows, |_| {
            Some(Loaded {
                base: 0,
                size: 0,
                headers: &[],
            })
        });
        assert_eq!(got.err(), Some(Refusal::BadSite { site: 9 }));
    }

    #[test]
    fn the_games_rows_fit_the_plan() {
        for file in mech_rows::sites::FILES {
            let sites: usize = file.sites.iter().map(|row| row.sites.len()).sum();
            let guards: usize = file.sites.iter().map(|row| row.guards.len()).sum();
            assert!(sites <= MOST_SITES && guards <= MOST_GUARDS, "{}", file.file);
            for site in file.sites.iter().flat_map(|row| row.sites) {
                assert!(site.old.len() <= ROW_BYTES, "site {}", site.n);
            }
        }
    }
}
