// SPDX-License-Identifier: MIT
//! The listener's judging half: a table of function entries, the stub each one gets, the
//! jump that reaches it, and the mapped file a flight's calls land in. All of it is bytes
//! and arithmetic, so it runs and is tested on the host; `listen_live` is the half that
//! touches a process.
//!
//! The game's own code keeps doing the work. A row steals the first bytes of a function,
//! at least a jump's worth, to an instruction boundary the table's script has already
//! judged. The jump goes to a stub that counts the call, in `ring` mode records it, runs
//! the stolen bytes from their new home and jumps back past them.
//!
//! **A row holds an address and a length, never the file's bytes.** The stolen bytes are
//! read from the live image at the bind and stay in process memory.
use core::fmt;

use crate::bind::{Hex, Memory};

/// `jmp rel32`: what a row must steal at least.
pub const JUMP_BYTES: usize = 5;
/// The longest head a row may steal. The tables' longest is 13.
pub const MOST_STOLEN: usize = 16;
/// `rel32` fields a head can hold, each 5 bytes at the least.
pub const MOST_REL32: usize = MOST_STOLEN / JUMP_BYTES;
/// Everything a stub adds around the stolen bytes: `pushfd`, 2 counter steps, `push row`,
/// `call`, `popfd` and the jump back.
pub const STUB_FRAME: usize = 1 + 8 + 8 + 5 + 5 + 1 + JUMP_BYTES;
/// The room to give one stub, whatever its row.
pub const STUB_ROOM: usize = STUB_FRAME + MOST_STOLEN;

const BREAKPOINT: u8 = 0xCC;
const PAGE: u32 = 0x1000;

/// The mapped file's first word, so a reader knows what it opened.
pub const MAGIC: u32 = u32::from_le_bytes(*b"LSN1");
/// Words in the header, then the rows, the fault slots and the ring, in that order.
pub const HEADER_BYTES: u32 = 0x1000;
/// Row numbers fit the ring's 16-bit field.
pub const MOST_ROWS: u32 = 1 << 16;
/// A row in the file: its address, its length, then its 64-bit count.
pub const ROW_BYTES: u32 = 16;
/// Where a row's count sits inside it.
pub const ROW_COUNT: u32 = 8;
pub const FAULT_SLOTS: u32 = 1 << 10;
pub const FAULT_BYTES: u32 = 128;
/// Words a fault holds before its stack words.
pub const FAULT_HEAD_WORDS: usize = 16;
/// Stack words a fault holds.
pub const FAULT_STACK_WORDS: usize = 16;
/// A power of two, so a ticket masks to a slot.
pub const RING_RECORDS: u32 = 1 << 22;
/// A record's 10 words: `row | tid << 16`, the caller, `eax`, `ecx`, `edx`, 4 stack words,
/// and the time stamp's low half, written last.
pub const RECORD_BYTES: u32 = 40;
/// Files the header has room for.
pub const MOST_TABLES: u32 = 8;
/// One table's words in the header: see [`table_word`].
pub const TABLE_BYTES: u32 = 64;
/// The ring's next ticket, alone on its cache line.
pub const HEAD_AT: u32 = 0x40;
/// Faults seen, slots or not, alone on its cache line.
pub const FAULTS_SEEN_AT: u32 = 0x80;
/// Tables in use.
pub const TABLES_AT: u32 = 0xC0;
/// Rows in use.
pub const ROWS_USED_AT: u32 = 0xC4;
/// The first table's words.
pub const TABLE_AT: u32 = 0x100;

pub const ROWS_AT: u32 = HEADER_BYTES;
pub const FAULTS_AT: u32 = ROWS_AT + MOST_ROWS * ROW_BYTES;
pub const RING_AT: u32 = FAULTS_AT + FAULT_SLOTS * FAULT_BYTES;
/// The whole mapped file.
pub const MAP_BYTES: u32 = RING_AT + RING_RECORDS * RECORD_BYTES;

/// A word's place in a header's table.
#[derive(Clone, Copy)]
pub enum TableWord {
    Stamp,
    Size,
    Base,
    FirstRow,
    Rows,
    Loads,
    Listed,
    Found,
    Placed,
    Stubs,
    Written,
    Held,
    Moved,
    Parked,
}

/// The header offset of `word` in table `index`.
#[must_use]
pub const fn table_word(index: u32, word: TableWord) -> u32 {
    TABLE_AT + index * TABLE_BYTES + 4 * word as u32
}

/// Why a table will not be used. Each stops the bind by name: a row written from a table
/// the bind doubts could land anywhere in the game.
#[derive(Debug, PartialEq, Eq)]
pub enum Refusal<'a> {
    BadLine(&'a str),
    /// A head past the jump's room or the stub's.
    BadLength {
        address: u32,
        length: usize,
    },
    /// A `rel32` field that does not sit whole inside its head.
    BadField {
        address: u32,
        offset: usize,
    },
    /// An address outside the module, so the table is another file's or the file moved.
    OutsideModule(u32),
    /// A row not above the last one's end: the table is out of order or 2 heads overlap.
    Overlap(u32),
    TooManyRows,
}

impl fmt::Display for Refusal<'_> {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadLine(line) => write!(out, "a bad table line: {line}"),
            Self::BadLength { address, length } => {
                write!(out, "the head at {} is {length} bytes long", Hex(*address))
            }
            Self::BadField { address, offset } => {
                write!(out, "the head at {} has a field at {offset}", Hex(*address))
            }
            Self::OutsideModule(address) => {
                write!(out, "the head at {} is outside its module", Hex(*address))
            }
            Self::Overlap(address) => {
                write!(out, "the head at {} overlaps the one before", Hex(*address))
            }
            Self::TooManyRows => out.write_str("more rows than the file has room for"),
        }
    }
}

/// What a row does at each call. `Count` is one counter step; `Ring` adds a record, so a
/// hot function can stay `Count` by a table edit, with no build.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Count,
    Ring,
}

/// Which pass found the row. A `found` row has no listing behind it, so what it counts is
/// not called a function's count without that word beside it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Source {
    Listed,
    Found,
}

/// One table line: `address`, `length`, `mode`, the `rel32` offsets, `source`, tab-separated.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Row {
    pub address: u32,
    pub length: usize,
    pub mode: Mode,
    pub source: Source,
    /// The head's `rel32` fields, their offsets; the first [`Self::fields`] of them count.
    pub rel32: [u8; MOST_REL32],
    pub fields: usize,
}

impl Row {
    /// The offsets of the head's `rel32` fields.
    #[must_use]
    pub fn offsets(&self) -> &[u8] {
        &self.rel32[..self.fields]
    }
}

/// One line read, its fields checked against each other but not against the module.
pub fn parse_row(line: &str) -> Result<Row, Refusal<'_>> {
    let bad = || Refusal::BadLine(line);
    let mut fields = line.split('\t');
    let mut next = || fields.next().ok_or_else(bad);
    let address = u32::from_str_radix(next()?, 16).map_err(|_| bad())?;
    let length: usize = next()?.parse().map_err(|_| bad())?;
    let mode = match next()? {
        "count" => Mode::Count,
        "ring" => Mode::Ring,
        _ => return Err(bad()),
    };
    let offsets = next()?;
    let source = match next()? {
        "listed" => Source::Listed,
        "found" => Source::Found,
        _ => return Err(bad()),
    };
    if !(JUMP_BYTES..=MOST_STOLEN).contains(&length) {
        return Err(Refusal::BadLength { address, length });
    }
    let mut row = Row {
        address,
        length,
        mode,
        source,
        rel32: [0; MOST_REL32],
        fields: 0,
    };
    if offsets != "-" {
        for offset in offsets.split(',') {
            let offset: usize = offset.parse().map_err(|_| bad())?;
            if offset + 4 > length {
                return Err(Refusal::BadField { address, offset });
            }
            *row.rel32.get_mut(row.fields).ok_or_else(bad)? = offset as u8;
            row.fields += 1;
        }
    }
    Ok(row)
}

/// Every row of a table in file order. Blank lines are not rows.
pub fn rows(table: &str) -> impl Iterator<Item = Result<Row, Refusal<'_>>> {
    table.lines().filter(|line| !line.is_empty()).map(parse_row)
}

/// What a table holds, once every line of it has been judged.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TableCount {
    pub rows: u32,
    pub listed: u32,
    pub found: u32,
}

/// The whole table judged against the module it would be written into, before any of it is.
pub fn check_table(table: &str, base: u32, size: u32) -> Result<TableCount, Refusal<'_>> {
    let mut count = TableCount::default();
    let mut end = base;
    for row in rows(table) {
        let row = row?;
        let high = u64::from(base) + u64::from(size);
        let reach = u64::from(row.address) + row.length as u64;
        if row.address < base || reach > high {
            return Err(Refusal::OutsideModule(row.address));
        }
        if row.address < end {
            return Err(Refusal::Overlap(row.address));
        }
        end = row.address + row.length as u32;
        count.rows += 1;
        match row.source {
            Source::Listed => count.listed += 1,
            Source::Found => count.found += 1,
        }
    }
    if count.rows > MOST_ROWS {
        return Err(Refusal::TooManyRows);
    }
    Ok(count)
}

/// One stub's inputs. Every address is absolute, this being a 32-bit process and the stub's
/// counter and ring routine known only at the bind.
pub struct Stub<'a> {
    /// Where this stub will sit, which every `rel32` is measured from.
    pub at: u32,
    pub row: &'a Row,
    /// The head as the live image holds it.
    pub stolen: &'a [u8],
    /// The row's 64-bit count.
    pub count: u32,
    pub number: u32,
    /// The routine a `ring` row calls, its contract in `listen_live`.
    pub record: u32,
}

/// Writes one stub into `out`, which holds [`STUB_ROOM`], and answers its length:
///
/// ```text
/// pushfd
/// lock add dword [count], 1
/// lock adc dword [count+4], 0
/// push number                   ; these 2 in ring mode only
/// call record
/// popfd
/// <the stolen bytes, each rel32 field fixed>
/// jmp entry+length
/// ```
///
/// The counter's 2 steps and the record touch the flags, so `popfd` comes before the stolen
/// bytes, which then run with every register and flag as the caller left them.
#[must_use]
pub fn write_stub(stub: &Stub<'_>, out: &mut [u8]) -> usize {
    let mut at = 0;
    let mut put = |bytes: &[u8]| {
        out[at..at + bytes.len()].copy_from_slice(bytes);
        at += bytes.len();
        at
    };
    put(&[0x9C]);
    put(&[0xF0, 0x83, 0x05]);
    put(&stub.count.to_le_bytes());
    put(&[0x01]);
    put(&[0xF0, 0x83, 0x15]);
    put(&(stub.count + 4).to_le_bytes());
    put(&[0x00]);
    if stub.row.mode == Mode::Ring {
        put(&[0x68]);
        put(&stub.number.to_le_bytes());
        let after_call = stub.at + put(&[0xE8]) as u32 + 4;
        put(&stub.record.wrapping_sub(after_call).to_le_bytes());
    }
    let copy = put(&[0x9D]);
    put(stub.stolen);
    let moved = stub.row.address.wrapping_sub(stub.at + copy as u32);
    let back = stub.row.address + stub.row.length as u32;
    let after_jump = stub.at + put(&[0xE9]) as u32 + 4;
    let length = put(&back.wrapping_sub(after_jump).to_le_bytes());
    for offset in stub.row.offsets() {
        let field = copy + usize::from(*offset);
        let mut old = [0; 4];
        old.copy_from_slice(&out[field..field + 4]);
        let fixed = u32::from_le_bytes(old).wrapping_add(moved);
        out[field..field + 4].copy_from_slice(&fixed.to_le_bytes());
    }
    length
}

/// What goes over the head: a jump to the stub, and a breakpoint in each byte left, so a
/// jump into the range the table's script could not see stops by name at its own address
/// instead of running half an instruction.
#[must_use]
pub fn jump_over(entry: u32, stub: u32, length: usize) -> [u8; MOST_STOLEN] {
    let mut bytes = [BREAKPOINT; MOST_STOLEN];
    bytes[0] = 0xE9;
    let rel = stub.wrapping_sub(entry + JUMP_BYTES as u32);
    bytes[1..JUMP_BYTES].copy_from_slice(&rel.to_le_bytes());
    bytes[length..].fill(0);
    bytes
}

/// A row as the bind holds it between a file's loads, in process memory alone.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct Placed {
    pub address: u32,
    pub stub: u32,
    pub length: u32,
    /// The head as the file's first load held it.
    pub saved: [u8; MOST_STOLEN],
}

/// What one bind did with a file's rows.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Placement {
    /// Jumps written.
    pub written: u32,
    /// A jump already there: the file was loaded again without being unmapped.
    pub held: u32,
    /// Bytes neither the saved head nor the jump, so left alone.
    pub moved: u32,
    /// A stopped thread stands inside the head, so left alone.
    pub parked: u32,
}

/// The rows written, with every other thread stopped: no log line, no heap and no protection
/// change here, since a stopped thread may hold the lock any of them takes. `memory` must be
/// writable already: see [`open_pages`].
pub fn place_rows(placed: &[Placed], pcs: &[u32], memory: &mut impl Memory) -> Placement {
    let mut placement = Placement::default();
    for row in placed {
        let length = row.length as usize;
        let parked = pcs
            .iter()
            .any(|pc| (row.address..row.address + row.length).contains(pc));
        if parked {
            placement.parked += 1;
            continue;
        }
        let jump = jump_over(row.address, row.stub, length);
        let mut live = [0; MOST_STOLEN];
        if !memory.read(row.address, &mut live[..length]) {
            placement.moved += 1;
        } else if live[..length] == jump[..length] {
            placement.held += 1;
        } else if live[..length] == row.saved[..length]
            && memory.write(row.address, &jump[..length])
        {
            placement.written += 1;
        } else {
            placement.moved += 1;
        }
    }
    placement
}

/// A page's protection, set and answered as `VirtualProtect` does.
pub trait Protect {
    /// The page's protection before this call, or `None` if it would not change.
    fn set(&mut self, page: u32, protection: u32) -> Option<u32>;
}

/// Every page the rows touch made `protection`, each page's own old value kept in `opened`;
/// how many, or the page that would not open, with every page before it closed again.
///
/// **By page, because `VirtualProtect` answers only the first page's old value.** A file
/// that has unlocked part of its own text holds pages of 2 protections, and one restore
/// over a run of them would take away the right it gave itself.
pub fn open_pages(
    placed: &[Placed],
    protection: u32,
    protect: &mut impl Protect,
    opened: &mut [(u32, u32)],
) -> Result<usize, u32> {
    let mut count = 0;
    for row in placed {
        let first = row.address & !(PAGE - 1);
        let last = (row.address + row.length - 1) & !(PAGE - 1);
        let mut page = first;
        while page <= last {
            let seen = count > 0 && opened[count - 1].0 >= page;
            if !seen {
                let was = if count < opened.len() {
                    protect.set(page, protection)
                } else {
                    None
                };
                let Some(was) = was else {
                    close_pages(&opened[..count], protect);
                    return Err(page);
                };
                opened[count] = (page, was);
                count += 1;
            }
            page += PAGE;
        }
    }
    Ok(count)
}

/// Each page back to its own old value; how many would not go.
pub fn close_pages(opened: &[(u32, u32)], protect: &mut impl Protect) -> u32 {
    let failed = opened
        .iter()
        .filter(|(page, was)| protect.set(*page, *was).is_none())
        .count();
    failed as u32
}

/// `<stamp>-<size>.rows.tsv`: a file's table goes by what it is, not what it is called, so
/// a table can only be read for the file it was cut from. Answers the name's length.
#[must_use]
pub fn table_name(stamp: u32, size: u32, out: &mut [u8; 32]) -> usize {
    const TAIL: &[u8] = b".rows.tsv";
    hex_word(stamp, &mut out[..8]);
    out[8] = b'-';
    hex_word(size, &mut out[9..17]);
    out[17..17 + TAIL.len()].copy_from_slice(TAIL);
    17 + TAIL.len()
}

/// `listen-<pid>.map`, a file each process, so a child's rows cannot land in its parent's
/// counts. Answers the name's length.
#[must_use]
pub fn map_name(pid: u32, out: &mut [u8; 32]) -> usize {
    const HEAD: &[u8] = b"listen-";
    const TAIL: &[u8] = b".map";
    out[..HEAD.len()].copy_from_slice(HEAD);
    hex_word(pid, &mut out[HEAD.len()..HEAD.len() + 8]);
    out[HEAD.len() + 8..HEAD.len() + 8 + TAIL.len()].copy_from_slice(TAIL);
    HEAD.len() + 8 + TAIL.len()
}

/// `word` in 8 lowercase digits, the form a reader's script names files by.
fn hex_word(word: u32, out: &mut [u8]) {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    for (digit, place) in out.iter_mut().enumerate() {
        *place = DIGITS[(word >> (28 - 4 * digit)) as usize & 15];
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::format;

    use super::*;

    #[test]
    fn a_mapped_files_name_says_its_process() {
        let mut out = [0; 32];
        let length = map_name(0x2c, &mut out);
        assert_eq!(&out[..length], b"listen-0000002c.map");
    }

    const F: u32 = 0x1000_2000;
    const FRAME: &str = "10002000\t6\tcount\t-\tlisted";

    fn row(line: &str) -> Row {
        parse_row(line).expect("the planted line reads")
    }

    /// A stub's bytes, run by hand: its 2 counter steps, the record if any, and where the
    /// fixed copy of the head starts.
    fn stub_of(row: &Row, stolen: &[u8]) -> ([u8; STUB_ROOM], usize, u32) {
        let at = 0x4000_0000;
        let mut out = [0; STUB_ROOM];
        let stub = Stub {
            at,
            row,
            stolen,
            count: 0x5000_0008,
            number: 7,
            record: 0x6000_1000,
        };
        let length = write_stub(&stub, &mut out);
        (out, length, at)
    }

    #[test]
    fn a_table_line_reads_whole() {
        let read = row("10002000\t10\tring\t1,6\tfound");
        assert_eq!(read.address, F);
        assert_eq!(read.length, 10);
        assert_eq!(read.mode, Mode::Ring);
        assert_eq!(read.source, Source::Found);
        assert_eq!(read.offsets(), &[1, 6]);
    }

    #[test]
    fn the_cut_tables_line_form_reads() {
        let read = row(FRAME);
        assert_eq!(read.offsets(), &[] as &[u8]);
        assert_eq!(read.mode, Mode::Count);
    }

    #[test]
    fn a_line_short_a_field_is_a_bad_line() {
        let line = "10002000\t6\tcount\t-";
        assert_eq!(parse_row(line), Err(Refusal::BadLine(line)));
    }

    #[test]
    fn a_mode_or_source_not_named_is_a_bad_line() {
        for line in [
            "10002000\t6\tloud\t-\tlisted",
            "10002000\t6\tcount\t-\tguessed",
            "zzzz\t6\tcount\t-\tlisted",
        ] {
            assert_eq!(parse_row(line), Err(Refusal::BadLine(line)));
        }
    }

    #[test]
    fn a_stolen_length_short_of_the_jump_or_past_the_room_refuses() {
        for length in [0, 1, 4, 17, 300] {
            let line = format!("10002000\t{length}\tcount\t-\tlisted");
            assert_eq!(
                parse_row(&line),
                Err(Refusal::BadLength { address: F, length })
            );
        }
    }

    #[test]
    fn a_field_that_leaves_its_head_refuses() {
        assert_eq!(
            parse_row("10002000\t6\tcount\t3\tlisted"),
            Err(Refusal::BadField {
                address: F,
                offset: 3
            })
        );
    }

    #[test]
    fn a_table_outside_its_module_refuses() {
        assert_eq!(
            check_table(FRAME, 0x2000_0000, 0x1000),
            Err(Refusal::OutsideModule(F))
        );
        let high_end = "10002ffc\t6\tcount\t-\tlisted";
        assert_eq!(
            check_table(high_end, F, 0x1000),
            Err(Refusal::OutsideModule(0x1000_2ffc))
        );
    }

    #[test]
    fn overlapping_or_unordered_heads_refuse() {
        let overlap = "10002000\t6\tcount\t-\tlisted\n10002005\t5\tcount\t-\tlisted\n";
        assert_eq!(
            check_table(overlap, F, 0x1000),
            Err(Refusal::Overlap(0x1000_2005))
        );
        let backward = "10002010\t5\tcount\t-\tlisted\n10002000\t5\tcount\t-\tlisted\n";
        assert_eq!(check_table(backward, F, 0x1000), Err(Refusal::Overlap(F)));
    }

    #[test]
    fn a_table_is_counted_by_class() {
        let table = "10002000\t6\tcount\t-\tlisted\n10002006\t5\tcount\t-\tfound\n\n";
        assert_eq!(
            check_table(table, F, 0x1000),
            Ok(TableCount {
                rows: 2,
                listed: 1,
                found: 1
            })
        );
    }

    #[test]
    fn a_count_stub_counts_runs_the_head_and_jumps_back() {
        let stolen = [0x55, 0x8B, 0xEC, 0x83, 0xEC, 0x10];
        let (out, length, at) = stub_of(&row(FRAME), &stolen);
        let expected = [
            0x9C, // pushfd
            0xF0, 0x83, 0x05, 0x08, 0x00, 0x00, 0x50, 0x01, // lock add dword [count], 1
            0xF0, 0x83, 0x15, 0x0C, 0x00, 0x00, 0x50, 0x00, // lock adc dword [count+4], 0
            0x9D, // popfd
            0x55, 0x8B, 0xEC, 0x83, 0xEC, 0x10, // the head
            0xE9,
        ];
        assert_eq!(&out[..expected.len()], &expected);
        assert_eq!(length, expected.len() + 4);
        let rel = i32::from_le_bytes([out[25], out[26], out[27], out[28]]);
        let landed = (at + length as u32).wrapping_add(rel as u32);
        assert_eq!(landed, F + 6);
    }

    #[test]
    fn a_ring_stub_pushes_its_row_and_calls_the_record_with_the_flags_still_saved() {
        let read = row("10002000\t6\tring\t-\tlisted");
        let (out, _, at) = stub_of(&read, &[0x55, 0x8B, 0xEC, 0x83, 0xEC, 0x10]);
        assert_eq!(&out[17..22], &[0x68, 7, 0, 0, 0]);
        assert_eq!(out[22], 0xE8);
        let rel = u32::from_le_bytes([out[23], out[24], out[25], out[26]]);
        assert_eq!((at + 27).wrapping_add(rel), 0x6000_1000);
        assert_eq!(out[27], 0x9D, "popfd comes after the call");
    }

    #[test]
    fn a_moved_rel32_still_lands_where_the_original_did() {
        // A `call rel32` at the head's first byte, then a `push`: entry + 5 + 0x100.
        let read = row("10002000\t6\tcount\t1\tlisted");
        let stolen = [0xE8, 0x00, 0x01, 0x00, 0x00, 0x50];
        let (out, _, at) = stub_of(&read, &stolen);
        let copy = 18;
        assert_eq!(out[copy], 0xE8);
        let rel = u32::from_le_bytes([out[19], out[20], out[21], out[22]]);
        let landed = (at + copy as u32 + 5).wrapping_add(rel);
        assert_eq!(landed, F + 5 + 0x100);
        assert_eq!(
            out[copy + 5],
            0x50,
            "the bytes after the field are as stolen"
        );
    }

    #[test]
    fn a_rel32_in_a_head_is_red_if_the_table_says_the_wrong_offset() {
        // The same head with its offset one byte off: the call lands elsewhere, so the
        // stub's byte test above is what a wrong table would turn red.
        let read = row("10002000\t6\tcount\t2\tlisted");
        let stolen = [0xE8, 0x00, 0x01, 0x00, 0x00, 0x50];
        let (out, _, at) = stub_of(&read, &stolen);
        let rel = u32::from_le_bytes([out[19], out[20], out[21], out[22]]);
        assert_ne!((at + 23).wrapping_add(rel), F + 5 + 0x100);
    }

    #[test]
    fn the_jump_reaches_its_stub_and_breaks_on_every_byte_left() {
        let bytes = jump_over(F, 0x4000_0000, 7);
        assert_eq!(bytes[0], 0xE9);
        let rel = u32::from_le_bytes([bytes[1], bytes[2], bytes[3], bytes[4]]);
        assert_eq!((F + 5).wrapping_add(rel), 0x4000_0000);
        assert_eq!(&bytes[5..7], &[0xCC, 0xCC]);
    }

    #[test]
    fn the_room_holds_the_largest_stub() {
        let read = row("10002000\t16\tring\t-\tlisted");
        let (_, length, _) = stub_of(&read, &[0x90; MOST_STOLEN]);
        assert_eq!(length, STUB_ROOM);
    }

    #[derive(Default)]
    struct Fake {
        bytes: HashMap<u32, u8>,
        protection: HashMap<u32, u32>,
        refuse: Option<u32>,
    }

    impl Fake {
        fn holding(address: u32, bytes: &[u8]) -> Self {
            let mut fake = Self::default();
            for (at, byte) in (address..).zip(bytes) {
                fake.bytes.insert(at, *byte);
            }
            fake
        }
    }

    impl Memory for Fake {
        fn read(&self, at: u32, out: &mut [u8]) -> bool {
            for (place, byte) in (at..).zip(out.iter_mut()) {
                let Some(held) = self.bytes.get(&place) else {
                    return false;
                };
                *byte = *held;
            }
            true
        }

        fn write(&mut self, at: u32, bytes: &[u8]) -> bool {
            for (place, byte) in (at..).zip(bytes) {
                self.bytes.insert(place, *byte);
            }
            true
        }
    }

    impl Protect for Fake {
        fn set(&mut self, page: u32, protection: u32) -> Option<u32> {
            if self.refuse == Some(page) {
                return None;
            }
            self.protection.insert(page, protection)
        }
    }

    fn placed(address: u32, head: &[u8]) -> Placed {
        let mut saved = [0; MOST_STOLEN];
        saved[..head.len()].copy_from_slice(head);
        Placed {
            address,
            stub: 0x4000_0000,
            length: head.len() as u32,
            saved,
        }
    }

    const HEAD: [u8; 6] = [0x55, 0x8B, 0xEC, 0x83, 0xEC, 0x10];

    #[test]
    fn a_row_is_written_over_the_head_it_saved() {
        let mut memory = Fake::holding(F, &HEAD);
        let done = place_rows(&[placed(F, &HEAD)], &[], &mut memory);
        assert_eq!(done.written, 1);
        let mut live = [0; 6];
        assert!(memory.read(F, &mut live));
        assert_eq!(live, jump_over(F, 0x4000_0000, 6)[..6]);
    }

    #[test]
    fn a_second_bind_over_a_file_still_mapped_writes_nothing() {
        let mut memory = Fake::holding(F, &HEAD);
        place_rows(&[placed(F, &HEAD)], &[], &mut memory);
        let again = place_rows(&[placed(F, &HEAD)], &[], &mut memory);
        assert_eq!((again.written, again.held), (0, 1));
    }

    #[test]
    fn a_fresh_load_of_the_same_file_is_written_again() {
        let row = placed(F, &HEAD);
        let mut first = Fake::holding(F, &HEAD);
        place_rows(&[row], &[], &mut first);
        let mut fresh = Fake::holding(F, &HEAD);
        assert_eq!(place_rows(&[row], &[], &mut fresh).written, 1);
    }

    #[test]
    fn bytes_that_moved_between_loads_are_left_and_counted() {
        let mut memory = Fake::holding(F, &[0x90; 6]);
        let done = place_rows(&[placed(F, &HEAD)], &[], &mut memory);
        assert_eq!((done.written, done.moved), (0, 1));
        let mut live = [0; 6];
        assert!(memory.read(F, &mut live));
        assert_eq!(live, [0x90; 6]);
    }

    #[test]
    fn a_thread_stopped_inside_a_head_keeps_it() {
        let row = placed(F, &HEAD);
        for pc in [F, F + 5] {
            let mut memory = Fake::holding(F, &HEAD);
            let done = place_rows(&[row], &[pc], &mut memory);
            assert_eq!((done.written, done.parked), (0, 1));
        }
        let mut memory = Fake::holding(F, &HEAD);
        assert_eq!(place_rows(&[row], &[F + 6], &mut memory).written, 1);
    }

    #[test]
    fn each_page_goes_back_to_its_own_protection() {
        // The second page's protection is the one the game gave itself. One restore from the
        // first page's answer, `Live::write`'s way, would take it away.
        const READ_EXECUTE: u32 = 0x20;
        const WRITE: u32 = 4;
        const OPEN: u32 = 0x40;
        let rows = [placed(0x1000_0FFD, &HEAD), placed(0x1000_1100, &HEAD)];
        let mut fake = Fake::default();
        fake.protection.insert(0x1000_0000, READ_EXECUTE);
        fake.protection.insert(0x1000_1000, WRITE);
        let mut opened = [(0, 0); 8];
        let count = open_pages(&rows, OPEN, &mut fake, &mut opened).expect("the pages open");
        assert_eq!(count, 2, "a head across 2 pages opens both, and each once");
        assert_eq!(fake.protection[&0x1000_1000], OPEN);
        assert_eq!(close_pages(&opened[..count], &mut fake), 0);
        assert_eq!(fake.protection[&0x1000_0000], READ_EXECUTE);
        assert_eq!(fake.protection[&0x1000_1000], WRITE);
    }

    #[test]
    fn a_page_that_will_not_open_closes_the_ones_before_it() {
        const READ_EXECUTE: u32 = 0x20;
        let rows = [placed(F, &HEAD), placed(F + 0x1000, &HEAD)];
        let mut fake = Fake {
            refuse: Some(F + 0x1000),
            ..Fake::default()
        };
        fake.protection.insert(F, READ_EXECUTE);
        fake.protection.insert(F + 0x1000, READ_EXECUTE);
        let mut opened = [(0, 0); 8];
        assert_eq!(
            open_pages(&rows, 0x40, &mut fake, &mut opened),
            Err(F + 0x1000)
        );
        assert_eq!(fake.protection[&F], READ_EXECUTE);
    }

    #[test]
    fn a_table_goes_by_the_files_stamp_and_size() {
        let mut name = [0; 32];
        let length = table_name(0x32A5_1F00, 0x000D_B000, &mut name);
        assert_eq!(&name[..length], b"32a51f00-000db000.rows.tsv");
    }

    #[test]
    fn the_map_holds_its_parts_in_order_and_fits_a_32_bit_process() {
        assert!(table_word(MOST_TABLES - 1, TableWord::Parked) + 4 <= HEADER_BYTES);
        assert!(HEAD_AT + 4 <= FAULTS_SEEN_AT && FAULTS_SEEN_AT + 4 <= TABLES_AT);
        assert_eq!(ROWS_AT, HEADER_BYTES);
        assert!(RING_RECORDS.is_power_of_two());
        let head_room = FAULT_HEAD_WORDS + FAULT_STACK_WORDS;
        assert!(head_room * 4 <= FAULT_BYTES as usize);
        assert!(MAP_BYTES < 0x2000_0000);
    }
}
