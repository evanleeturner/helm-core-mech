// SPDX-License-Identifier: MIT
//! What the in-process bind fires on: the 3 files it knows by their
//! headers, and the 4 sites it writes in memory at the game file's load, so the timer, the
//! wave callback and the CD check run patched from their first instruction. Nothing is
//! written on disk.
//!
//! The rows are the rig's patch file's, its `T1P_` lines. **Not an older 3-site form plus a
//! fourth:** the older form's site 2 has a different new row, and only the 4-site form writes
//! site 4's dword, which its own site 2 reads. Addresses here are
//! at each DLL's preferred base; the bind follows the DLL to where it is loaded.

/// One run of bytes rewritten in a DLL's image.
pub struct Site {
    /// The number the patch file and the logs call it.
    pub n: u8,
    pub at: u32,
    pub old: &'static [u8],
    pub new: &'static [u8],
}

/// A DLL's sites, and the spans no thread may stand in while they are written, both ends
/// inside the span. Site 4 is data, so no guard holds it.
pub struct Patched {
    pub dll: &'static str,
    pub base: u32,
    pub sites: &'static [Site],
    pub guards: &'static [(u32, u32)],
}

const WAIL32: Patched = Patched {
    dll: "wail32.dll",
    base: 0x2000_0000,
    sites: &[
        Site {
            n: 1,
            at: 0x2000_1301,
            old: &[
                0x8b, 0x0d, 0xf8, 0xb7, 0x01, 0x20, 0x8b, 0x44, 0x08, 0x08, 0x50, 0xa1, 0x08, 0xb8,
                0x01, 0x20, 0xc1, 0xe0, 0x02, 0x8d, 0x04, 0x80,
            ],
            new: &[
                0x8b, 0x0d, 0xf8, 0xb7, 0x01, 0x20, 0xff, 0x74, 0x08, 0x08, 0xff, 0x54, 0x08, 0x04,
                0x8d, 0x65, 0xf4, 0xe9, 0x2c, 0xff, 0xff, 0xff,
            ],
        },
        Site {
            n: 2,
            at: 0x2000_8e73,
            old: &[
                0xa1, 0x98, 0xc5, 0x01, 0x20, 0x50, 0xff, 0x15, 0x6c, 0xe1, 0x01, 0x20, 0x83, 0xf8,
                0xff, 0x0f, 0x85, 0x05, 0x00, 0x00, 0x00, 0xe9, 0xd0, 0x00, 0x00, 0x00, 0x8b, 0x45,
                0x0c, 0x25, 0xff, 0xff, 0x00, 0x00, 0x3d, 0xbd, 0x03, 0x00, 0x00, 0x0f, 0x84, 0x11,
                0x00, 0x00, 0x00, 0xa1, 0x98, 0xc5, 0x01, 0x20, 0x50, 0xff, 0x15, 0xf4, 0xe1, 0x01,
                0x20, 0xe9, 0xac, 0x00, 0x00, 0x00,
            ],
            new: &[
                0x64, 0xa1, 0x24, 0x00, 0x00, 0x00, 0x66, 0x81, 0x7d, 0x0c, 0xbb, 0x03, 0x75, 0x07,
                0xa3, 0xfc, 0xdf, 0x01, 0x20, 0xeb, 0x1f, 0x66, 0x81, 0x7d, 0x0c, 0xbd, 0x03, 0x75,
                0x17, 0x3b, 0x05, 0xfc, 0xdf, 0x01, 0x20, 0x74, 0x19, 0xa1, 0x98, 0xc5, 0x01, 0x20,
                0x50, 0xff, 0x15, 0x6c, 0xe1, 0x01, 0x20, 0x40, 0x75, 0x0a, 0xe9, 0xb1, 0x00, 0x00,
                0x00, 0x90, 0x90, 0x90, 0x90, 0x90,
            ],
        },
        Site {
            n: 4,
            at: 0x2001_dffc,
            old: &[0, 0, 0, 0],
            new: &[0, 0, 0, 0],
        },
    ],
    guards: &[(0x2000_1301, 0x2000_1316), (0x2000_8e6d, 0x2000_8f63)],
};

const MERCSW: Patched = Patched {
    dll: "mercsw.dll",
    base: 0x1000_0000,
    sites: &[Site {
        n: 3,
        at: 0x1006_ad0a,
        old: &[0x0f, 0x85, 0x0c, 0x00, 0x00, 0x00],
        new: &[0x90, 0xe9, 0x0c, 0x00, 0x00, 0x00],
    }],
    guards: &[(0x1006_ad0a, 0x1006_ad0f)],
};

/// A file the bind knows: its header's fields name it whatever else shares its base, its
/// table says which of its slots go where, and `sites` go in at its load.
pub struct BoundFile {
    pub file: &'static str,
    pub table: &'static str,
    pub base: u32,
    pub pe: u32,
    pub stamp: u32,
    pub size: u32,
    pub sites: &'static [Patched],
    /// The table of this file's functions that `HC5_SWAP` may jump to their rebuilt copies,
    /// read from `HC5_BIND`'s folder at this file's load; `None` for a file with none.
    pub swaps: Option<&'static str>,
}

/// `MW2SHELL.DLL` and `MERCSW.DLL` map at one base in turn, so only the headers tell them
/// apart. The sites wait for `MERCSW.DLL`: site 3 is in it, and its imports map `wail32.dll`.
pub const FILES: &[BoundFile] = &[
    BoundFile {
        file: "MERCSWIN.EXE",
        table: "launcher-table.tsv",
        base: 0x40_0000,
        pe: 0x80,
        stamp: 0x322d_c8ee,
        size: 0xe000,
        sites: &[],
        swaps: None,
    },
    BoundFile {
        file: "MW2SHELL.DLL",
        table: "layer-table.tsv",
        base: 0x1000_0000,
        pe: 0x80,
        stamp: 0x323d_eb8e,
        size: 0xe_a000,
        sites: &[],
        swaps: Some("swap-table.tsv"),
    },
    BoundFile {
        file: "MERCSW.DLL",
        table: "game-table.tsv",
        base: 0x1000_0000,
        pe: 0x80,
        stamp: 0x323d_0ef1,
        size: 0x22_9000,
        sites: &[WAIL32, MERCSW],
        swaps: None,
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    fn patched() -> impl Iterator<Item = &'static Patched> {
        FILES.iter().flat_map(|file| file.sites)
    }

    #[test]
    fn each_old_row_has_the_length_of_its_new_row() {
        for row in patched() {
            for site in row.sites {
                assert_eq!(site.old.len(), site.new.len(), "site {}", site.n);
            }
        }
    }

    #[test]
    fn the_four_sites_are_each_there_once() {
        let mut numbers: [u8; 4] = [0; 4];
        let mut count = 0;
        for site in patched().flat_map(|row| row.sites) {
            numbers[count] = site.n;
            count += 1;
        }
        numbers.sort_unstable();
        assert_eq!(numbers, [1, 2, 3, 4]);
    }

    #[test]
    fn each_code_site_ends_inside_a_guard_and_the_data_site_in_none() {
        for row in patched() {
            for site in row.sites {
                let last = site.at + site.old.len() as u32 - 1;
                let guarded = row
                    .guards
                    .iter()
                    .any(|(low, high)| *low <= site.at && last <= *high);
                assert_eq!(guarded, site.n != 4, "site {}", site.n);
            }
        }
    }
}
