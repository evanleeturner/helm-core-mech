// SPDX-License-Identifier: MIT
//! `bind`'s readers on the player's own files, held to what the rig's swap script read from the
//! same bytes. `bind`'s unit tests stand on a PE they build; only this says it reads the
//! game's.
//!
//! Neither the files nor anything harvested from them lives in this tree, so the test is
//! ignored unless asked for: `BIND_GAME` names the install's folder, `BIND_LINK` the swap
//! build's folder, `BIND_HARVEST` the script's rows and `BIND_PATCH` the rig's patch file.
//!
//! ```text
//! cargo test -p mech_swap --test bind_game_files -- --ignored --nocapture
//! ```
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::PathBuf;

use mech_rows::sites::FILES;
use mech_swap::bind::{
    File, HeaderKey, Loaded, Memory, Pointed, bind_table, for_chosen, header_key, iat_slot,
    plan_sites, rebased, sections, slot_rows,
};

const IGNORED: &str = "needs the player's own files: BIND_GAME, BIND_LINK, BIND_HARVEST";

fn folder(name: &str) -> PathBuf {
    PathBuf::from(env::var_os(name).unwrap_or_else(|| panic!("{IGNORED}: {name} unset")))
}

fn hex(field: &str) -> u32 {
    u32::from_str_radix(field, 16).unwrap_or_else(|_| panic!("not hex: {field}"))
}

fn bytes(field: &str) -> Vec<u8> {
    (0..field.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&field[i..i + 2], 16).expect("a harvest's row is hex"))
        .collect()
}

/// The harvest's rows of one kind, each cut at its tabs, the kind gone.
fn rows<'a>(harvest: &'a str, kind: &'a str) -> impl Iterator<Item = Vec<&'a str>> {
    harvest
        .lines()
        .filter_map(move |line| line.strip_prefix(kind)?.strip_prefix('\t'))
        .map(|line| line.split('\t').collect())
}

/// The install's file, read once.
fn file<'a>(files: &'a mut BTreeMap<String, Vec<u8>>, name: &str) -> &'a [u8] {
    files.entry(name.to_owned()).or_insert_with(|| {
        fs::read(folder("BIND_GAME").join(name)).unwrap_or_else(|e| panic!("{name}: {e}"))
    })
}

/// A memory of the slots a table names, each holding what a loader would leave there.
#[derive(Default)]
struct Slots(BTreeMap<u32, u8>);

impl Memory for Slots {
    fn read(&self, at: u32, out: &mut [u8]) -> bool {
        out.iter_mut()
            .zip(at..)
            .all(|(byte, place)| self.0.get(&place).map(|got| *byte = *got).is_some())
    }

    fn write(&mut self, at: u32, bytes: &[u8]) -> bool {
        for (byte, place) in bytes.iter().zip(at..) {
            self.0.insert(place, *byte);
        }
        true
    }
}

#[test]
#[ignore = "needs the player's own files: BIND_GAME, BIND_LINK, BIND_HARVEST"]
fn the_readers_equal_the_script_on_the_game_files() {
    let harvest = fs::read_to_string(folder("BIND_HARVEST")).expect("the harvest reads");
    let mut files = BTreeMap::new();
    let mut count = 0;
    for row in rows(&harvest, "key") {
        let image = File(file(&mut files, row[0]));
        let want = HeaderKey {
            pe: hex(row[1]),
            stamp: hex(row[2]),
            size: hex(row[3]),
        };
        assert_eq!(header_key(&image), Some(want), "{}", row[0]);
        count += 1;
    }
    println!("header keys equal: {count}");
    let mut count = 0;
    for name in rows(&harvest, "key").map(|row| row[0]) {
        let image = file(&mut files, name);
        let got: Vec<_> = sections(image).expect("the headers read").collect();
        let want: Vec<_> = rows(&harvest, "section")
            .filter(|row| row[0] == name)
            .collect();
        assert_eq!(got.len(), want.len(), "{name}");
        for (got, want) in got.iter().zip(&want) {
            assert_eq!(got.name, want[1].as_bytes(), "{name}");
            assert_eq!(
                (got.va, got.vsize, got.raw, got.rsize, got.flags),
                (
                    hex(want[2]),
                    hex(want[3]),
                    hex(want[4]),
                    hex(want[5]),
                    hex(want[6])
                ),
                "{name} {}",
                want[1]
            );
            count += 1;
        }
    }
    println!("section rows equal: {count}");
    let (mut found, mut absent) = (0, 0);
    for row in rows(&harvest, "import") {
        let image = File(file(&mut files, row[0]));
        let got = iat_slot(&image, row[1], row[2]);
        let want = Some(hex(row[3])).filter(|slot| *slot != 0);
        assert_eq!(got, want, "{} {}!{}", row[0], row[1], row[2]);
        if want.is_some() {
            found += 1;
        } else {
            absent += 1;
        }
    }
    println!("imports equal: {found} found, {absent} refused alike");
}

#[test]
#[ignore = "needs the player's own files: BIND_GAME, BIND_LINK, BIND_HARVEST"]
fn the_tables_read_as_the_script_read_them() {
    let harvest = fs::read_to_string(folder("BIND_HARVEST")).expect("the harvest reads");
    let link = folder("BIND_LINK");
    let mut total = 0;
    for name in ["layer-table.tsv", "game-table.tsv", "launcher-table.tsv"] {
        let table = fs::read_to_string(link.join(name)).expect("a layer table reads");
        let want: Vec<_> = rows(&harvest, "row").filter(|row| row[0] == name).collect();
        let got: Vec<_> = slot_rows(&table)
            .map(|row| row.expect("a whole line"))
            .collect();
        assert_eq!(got.len(), want.len(), "{name}");
        for (got, want) in got.iter().zip(&want) {
            assert_eq!(
                (got.slot, got.export, got.target, got.label),
                (hex(want[1]), want[2], hex(want[3]), want[4]),
                "{name}"
            );
            total += 1;
        }
    }
    println!("layer rows equal: {total}");
    let table = fs::read_to_string(link.join("swap-table.tsv")).expect("the swap table reads");
    let mut want = rows(&harvest, "swap");
    let mut seen = 0;
    let handed = for_chosen(&table, "all", |row| {
        let next = want.next().expect("a row the harvest has");
        assert_eq!(
            (row.address, row.export, row.target),
            (hex(next[0]), next[1], hex(next[2])),
        );
        seen += 1;
    })
    .expect("no refusal");
    assert_eq!((handed, want.next()), (seen, None));
    println!("swap rows equal: {seen}");
}

#[test]
#[ignore = "needs the player's own files: BIND_GAME, BIND_LINK, BIND_HARVEST"]
fn rebased_equals_the_script_on_the_patch_rows() {
    let harvest = fs::read_to_string(folder("BIND_HARVEST")).expect("the harvest reads");
    let mut count = 0;
    for row in rows(&harvest, "rebased") {
        let mut got = bytes(row[3]);
        let moved = rebased(&mut got, hex(row[0]), hex(row[1]), hex(row[2]));
        assert_eq!((got, moved), (bytes(row[4]), hex(row[5])), "{row:?}");
        count += 1;
    }
    println!("patch rows equal: {count}");
}

/// On recorded tables: each file's whole table binds at each of 2 loads,
/// each load's slots starting unbound again, as a file mapped afresh leaves them.
#[test]
#[ignore = "needs the player's own files: BIND_GAME, BIND_LINK, BIND_HARVEST"]
fn each_load_binds_its_whole_table_again() {
    let harvest = fs::read_to_string(folder("BIND_HARVEST")).expect("the harvest reads");
    let link = folder("BIND_LINK");
    let tables = [
        ("MW2SHELL.DLL", "layer-table.tsv"),
        ("MERCSW.DLL", "game-table.tsv"),
        ("MERCSWIN.EXE", "launcher-table.tsv"),
    ];
    for (name, table) in tables {
        let key = rows(&harvest, "key")
            .find(|row| row[0] == name)
            .expect("a key row");
        let (size, base) = (hex(key[3]), hex(key[4]));
        let table = fs::read_to_string(link.join(table)).expect("a table reads");
        for load in 1..=2 {
            let mut memory = Slots::default();
            let mut count = 0;
            for row in slot_rows(&table) {
                let row = row.expect("a whole line");
                assert!(memory.write(row.slot, &0x7b00_0000u32.to_le_bytes()));
                count += 1;
            }
            let pointed = bind_table(&table, &mut memory, base, base + size).expect("no refusal");
            assert_eq!(
                pointed,
                Pointed {
                    written: count,
                    failed: 0,
                    first_failed: None
                },
                "{name}"
            );
            for row in slot_rows(&table).flatten() {
                let mut got = [0; 4];
                assert!(memory.read(row.slot, &mut got));
                assert_eq!(u32::from_le_bytes(got), row.target, "{name} {}", row.label);
            }
            println!("{name} load {load}: {count} of {count} slots");
        }
    }
}

/// `mech_rows`' `FILES` were typed by hand from the harvest's `key` rows.
#[test]
#[ignore = "needs the script's rows: BIND_HARVEST"]
fn the_file_rows_equal_the_harvests_keys() {
    let harvest = fs::read_to_string(folder("BIND_HARVEST")).expect("the harvest reads");
    assert_eq!(rows(&harvest, "key").count(), FILES.len());
    for row in FILES {
        let key = rows(&harvest, "key")
            .find(|key| key[0] == row.file)
            .expect("a key row");
        assert_eq!(
            (row.pe, row.stamp, row.size, row.base),
            (hex(key[1]), hex(key[2]), hex(key[3]), hex(key[4])),
            "{}",
            row.file
        );
    }
}

/// The patch file's `NAME=value` lines, quotes off; its `#` lines are words.
fn patch_values(text: &str) -> BTreeMap<&str, &str> {
    text.lines()
        .filter(|line| !line.starts_with('#'))
        .filter_map(|line| line.split_once('='))
        .map(|(name, value)| (name, value.trim_matches('\'')))
        .collect()
}

/// A patch file's row of bytes, spaces between them.
fn spaced_bytes(field: &str) -> Vec<u8> {
    field
        .split(' ')
        .map(|byte| u8::from_str_radix(byte, 16).expect("a patch row is hex"))
        .collect()
}

/// `mech_rows`' sites and guards were typed by hand from the rig's patch file.
#[test]
#[ignore = "needs the rig's patch file: BIND_PATCH"]
fn the_sites_equal_the_patch_file() {
    let text = fs::read_to_string(folder("BIND_PATCH")).expect("the patch file reads");
    let values = patch_values(&text);
    let patched: Vec<_> = FILES.iter().flat_map(|file| file.sites).collect();
    let mut seen = 0;
    for row in &patched {
        for site in row.sites {
            let n = site.n;
            let value = |name: &str| values[format!("T1P_{name}_{n}").as_str()];
            let or_shared = |name: &str| {
                let own = format!("T1P_{name}_{n}");
                let shared = format!("T1P_{name}");
                values.get(own.as_str()).copied().unwrap_or(values[shared.as_str()])
            };
            assert_eq!(site.at, hex(value("AT")), "site {n}");
            assert_eq!(site.old, spaced_bytes(value("OLD")), "site {n}");
            assert_eq!(site.new, spaced_bytes(value("NEW")), "site {n}");
            assert_eq!(row.dll, or_shared("DLL"), "site {n}");
            assert_eq!(row.base, hex(or_shared("BASE")), "site {n}");
            seen += 1;
        }
    }
    assert_eq!(seen.to_string(), values["T1P_SITES"]);
    let mut typed: Vec<_> = patched.iter().flat_map(|row| row.guards).copied().collect();
    let mut written: Vec<_> = values["T1P_GUARD"]
        .split(' ')
        .map(|span| span.split_once('-').expect("a guard is low-high"))
        .map(|(low, high)| (hex(low), hex(high)))
        .collect();
    typed.sort_unstable();
    written.sort_unstable();
    assert_eq!(typed, written);
    println!("{seen} sites, {} guards, as the patch file", typed.len());
}

/// The judgment a flight first stopped at, on the player's own headers: every site is on an
/// unshared page of its DLL. The test above reads the rows and never reached this.
#[test]
#[ignore = "needs the player's own files: BIND_GAME"]
fn every_site_plans_on_the_game_files() {
    let mut seen = 0;
    for row in FILES.iter().flat_map(|file| file.sites) {
        let name = row.dll.to_ascii_uppercase();
        let headers = fs::read(folder("BIND_GAME").join(&name))
            .unwrap_or_else(|e| panic!("{name}: {e}"));
        let planned = plan_sites(std::slice::from_ref(row), |_| {
            Some(Loaded {
                base: row.base,
                size: 0x10_0000,
                headers: &headers,
            })
        });
        assert_eq!(planned.map(|_| ()), Ok(()), "{name}");
        seen += row.sites.len();
    }
    println!("{seen} sites planned");
}
