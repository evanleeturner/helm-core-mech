// SPDX-License-Identifier: MIT
//! What the starter reads, apart from Windows so the host can test it: its 2 arguments, and
//! where a program's first instruction will stand.
#![no_std]

const WORD_BYTES: usize = 4;
/// The DOS header's field holding the PE header's place.
const PE_FIELD: usize = 60;
const PE_MAGIC: &[u8] = b"PE\0\0";
/// From the PE header: the optional header's magic, the entry's offset and the image's base.
const OPTIONAL_MAGIC: usize = 24;
const ENTRY_OFFSET: usize = 40;
const IMAGE_BASE: usize = 52;
/// The optional header's magic for a 32-bit file, as it lies in the file.
const PE32: &[u8] = &[0x0b, 0x01];
/// How many words a command line holds here: the starter's own name and its 2 arguments.
const WORDS: usize = 3;

/// A 32-bit PE's first instruction in memory, its preferred base plus its entry's offset; none
/// when the bytes are no such file. A program not loaded at that base never stands there, so
/// the wait for the park ends in a named stop.
#[must_use]
pub fn entry_address(head: &[u8]) -> Option<u32> {
    let word = |at: usize| -> Option<u32> {
        let bytes = head.get(at..at + WORD_BYTES)?;
        Some(u32::from_le_bytes(bytes.try_into().ok()?))
    };
    if head.get(..2)? != b"MZ" {
        return None;
    }
    let pe = word(PE_FIELD)? as usize;
    if head.get(pe..pe + PE_MAGIC.len())? != PE_MAGIC {
        return None;
    }
    if head.get(pe + OPTIONAL_MAGIC..pe + OPTIONAL_MAGIC + PE32.len())? != PE32 {
        return None;
    }
    word(pe + IMAGE_BASE)?.checked_add(word(pe + ENTRY_OFFSET)?)
}

/// The command line's second and third words, each ending in its NUL, ready for Windows. The
/// quotes around a path with a space come off, in place. None unless there are 3 words exactly.
#[must_use]
pub fn two_arguments(line: &mut [u8]) -> Option<(&[u8], &[u8])> {
    let mut spans = [(0, 0); WORDS];
    let mut words = 0;
    let mut read = 0;
    let mut write = 0;
    loop {
        while matches!(line.get(read), Some(b' ' | b'\t')) {
            read += 1;
        }
        if matches!(line.get(read), None | Some(0)) {
            break;
        }
        if words == WORDS {
            return None;
        }
        let start = write;
        let mut quoted = false;
        while let Some(&byte) = line.get(read) {
            if byte == 0 || (!quoted && matches!(byte, b' ' | b'\t')) {
                break;
            }
            read += 1;
            if byte == b'"' {
                quoted = !quoted;
            } else {
                line[write] = byte;
                write += 1;
            }
        }
        // The separator is read past first: `write` may stand on it, and its NUL goes there.
        let more = line.get(read).is_some_and(|byte| *byte != 0);
        if more {
            read += 1;
        }
        *line.get_mut(write)? = 0;
        write += 1;
        spans[words] = (start, write);
        words += 1;
        if !more {
            break;
        }
    }
    if words != WORDS {
        return None;
    }
    let line = &*line;
    Some((&line[spans[1].0..spans[1].1], &line[spans[2].0..spans[2].1]))
}

#[cfg(test)]
mod tests {
    use super::*;

    const PE_AT: usize = 0x80;

    fn pe32(base: u32, entry: u32) -> [u8; 256] {
        let mut head = [0; 256];
        head[..2].copy_from_slice(b"MZ");
        head[PE_FIELD..PE_FIELD + 4].copy_from_slice(&(PE_AT as u32).to_le_bytes());
        head[PE_AT..PE_AT + 4].copy_from_slice(PE_MAGIC);
        head[PE_AT + OPTIONAL_MAGIC..PE_AT + OPTIONAL_MAGIC + 2].copy_from_slice(PE32);
        head[PE_AT + ENTRY_OFFSET..PE_AT + ENTRY_OFFSET + 4].copy_from_slice(&entry.to_le_bytes());
        head[PE_AT + IMAGE_BASE..PE_AT + IMAGE_BASE + 4].copy_from_slice(&base.to_le_bytes());
        head
    }

    #[test]
    fn the_entry_is_the_base_plus_its_offset() {
        assert_eq!(entry_address(&pe32(0x40_0000, 0x1080)), Some(0x40_1080));
    }

    #[test]
    fn anything_else_has_no_entry() {
        let mut no_mz = pe32(0x40_0000, 0x1080);
        no_mz[0] = 0;
        assert_eq!(entry_address(&no_mz), None);
        let mut no_pe = pe32(0x40_0000, 0x1080);
        no_pe[PE_AT] = 0;
        assert_eq!(entry_address(&no_pe), None);
        let mut pe64 = pe32(0x40_0000, 0x1080);
        pe64[PE_AT + OPTIONAL_MAGIC + 1] = 2;
        assert_eq!(entry_address(&pe64), None);
        assert_eq!(entry_address(&pe32(0x40_0000, 0x1080)[..PE_AT + 8]), None);
        assert_eq!(entry_address(&pe32(u32::MAX, 1)), None);
    }

    /// The bytes as Windows hands them: a NUL last, its length counted.
    fn line_of<const N: usize>(words: &[u8]) -> [u8; N] {
        let mut line = [0; N];
        line[..words.len()].copy_from_slice(words);
        line
    }

    #[test]
    fn plain_words_are_cut_with_their_nuls() {
        let mut line: [u8; 40] = line_of(br"me.exe C:\G\GAME.EXE Z:\swap.dll");
        let (target, library) = two_arguments(&mut line).expect("3 words");
        assert_eq!(target, b"C:\\G\\GAME.EXE\0");
        assert_eq!(library, b"Z:\\swap.dll\0");
    }

    #[test]
    fn a_line_exactly_full_keeps_its_last_nul() {
        let words = br"me.exe a.exe b.dll";
        let mut line: [u8; 19] = line_of(words);
        assert_eq!(words.len() + 1, line.len());
        assert_eq!(two_arguments(&mut line), Some((&b"a.exe\0"[..], &b"b.dll\0"[..])));
    }

    #[test]
    fn quotes_keep_a_space_and_come_off() {
        let mut line: [u8; 60] = line_of(br#""C:\my dir\me.exe"  "C:\Program Files\g.exe"	z.dll "#);
        let (target, library) = two_arguments(&mut line).expect("3 words");
        assert_eq!(target, b"C:\\Program Files\\g.exe\0");
        assert_eq!(library, b"z.dll\0");
    }

    #[test]
    fn only_three_words_will_do() {
        let mut two: [u8; 20] = line_of(b"me.exe a.exe");
        assert_eq!(two_arguments(&mut two), None);
        let mut four: [u8; 30] = line_of(b"me.exe a.exe b.dll c");
        assert_eq!(two_arguments(&mut four), None);
        let mut none: [u8; 4] = line_of(b"");
        assert_eq!(two_arguments(&mut none), None);
    }
}
