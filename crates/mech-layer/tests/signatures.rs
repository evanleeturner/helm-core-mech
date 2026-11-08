// SPDX-License-Identifier: MIT
//! `through.rs`'s table of declared counts against the declarations it is read from:
//! `slots.rs`'s rows and `input.rs`'s `GetAsyncKeyState`. A count that differs, a label no
//! declaration holds, or an `_AIL_` row left out of the table fails.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

fn source(name: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src").join(name);
    std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
}

/// Each label `slots.rs` and `input.rs` declare, and its count of arguments: one ident each,
/// after the row's name for `linked!` and `wail!`, a `wail!` row's label under `WAIL32!`.
fn declared() -> BTreeMap<String, usize> {
    let mut table = BTreeMap::new();
    for line in source("slots.rs").lines() {
        let (prefix, skip) = if line.starts_with("linked!(") {
            ("", 1)
        } else if line.starts_with("wail!(") {
            ("WAIL32!", 1)
        } else if line.starts_with("resolved!(") {
            ("", 0)
        } else {
            continue;
        };
        let parts: Vec<&str> = line.split('"').collect();
        let [.., label, rest] = parts.as_slice() else {
            panic!("a row with no label: {line}");
        };
        let rest = rest
            .trim_start_matches(',')
            .trim_end()
            .trim_end_matches(");");
        let count = rest.split_whitespace().skip(skip).count();
        let label = format!("{prefix}{label}");
        if let Some(before) = table.insert(label.clone(), count) {
            assert_eq!(before, count, "{label} declared twice, apart");
        }
    }
    let input = source("input.rs");
    let keys = input
        .lines()
        .find_map(|line| line.strip_prefix("entry!(GAME_GetAsyncKeyState, "))
        .expect("input.rs declares GAME_GetAsyncKeyState");
    let count = keys
        .rsplit(", ")
        .next()
        .and_then(|count| count.trim_end_matches(");").parse().ok())
        .expect("its count");
    table.insert("USER32!GetAsyncKeyState".to_owned(), count);
    table
}

/// `through.rs`'s `SIGNATURES` rows, as its text holds them.
fn signatures() -> Vec<(String, usize)> {
    let through = source("through.rs");
    let start = through
        .find("const SIGNATURES")
        .expect("through.rs holds SIGNATURES");
    let body = &through[start..];
    let body = &body[..body.find("];").expect("SIGNATURES ends")];
    body.lines()
        .filter_map(|line| {
            let row = line.trim().strip_prefix("(\"")?.strip_suffix("),")?;
            let (label, count) = row.split_once("\", ")?;
            Some((label.to_owned(), count.parse().expect("a count")))
        })
        .collect()
}

#[test]
fn every_count_is_the_declared_one() {
    let declared = declared();
    let table = signatures();
    assert!(!table.is_empty(), "SIGNATURES read empty");
    let apart: Vec<_> = table
        .iter()
        .filter(|(label, count)| declared.get(label) != Some(count))
        .map(|(label, count)| (label, count, declared.get(label)))
        .collect();
    assert!(
        apart.is_empty(),
        "rows apart from their declarations: {apart:?}"
    );
}

#[test]
fn every_ail_row_is_in_the_table() {
    let table: BTreeSet<String> = signatures().into_iter().map(|(label, _)| label).collect();
    let missing: Vec<String> = declared()
        .into_keys()
        .filter(|label| label.contains("!_AIL_") && !table.contains(label))
        .collect();
    assert!(
        missing.is_empty(),
        "_AIL_ rows missing from SIGNATURES: {missing:?}"
    );
}
