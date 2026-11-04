// SPDX-License-Identifier: MIT
//! The fronts on a row of no game: the table's own round trip and the
//! refusals. The harvested census rows replay in the game's rows crate;
//! Wine's 256-byte answers check the table once the probe's fixture lands.

use super::*;
use core::cell::Cell;
use table::{Calls, Errors};

thread_local! {
    static ERROR: Cell<u32> = const { Cell::new(0) };
}

fn keep(code: u32) {
    ERROR.with(|error| error.set(code));
}

const CALLS: Calls = Calls {
    ansi_code_page: "acp",
    oem_code_page: "oemcp",
    code_page_info: "cpinfo",
    to_wide: "to_wide",
    to_bytes: "to_bytes",
    string_type: "type",
    string_type_w: "type_w",
    map_string: "map",
    map_string_w: "map_w",
    compare: "compare",
    compare_w: "compare_w",
    locale_info: "info",
    locale_info_w: "info_w",
};

static ROW: Row = Row {
    file: "test",
    calls: CALLS,
    ansi: 1252,
    oem: 437,
    locale: 0x409,
    locale_info: &[(0x1004, "1252")],
    errors: Errors {
        invalid_parameter: 87,
        insufficient_buffer: 122,
        invalid_flags: 1004,
    },
};

fn front() -> Front<Portable> {
    Front::new(&ROW, Portable::new(keep))
}

#[test]
fn every_byte_goes_there_and_back() {
    let bytes: [u8; 256] = core::array::from_fn(|byte| u8::try_from(byte).unwrap_or(0));
    let mut units = [0; 256];
    let mut back = [0; 256];
    assert_eq!(front().to_wide(0, 0, &bytes, Some(&mut units)), 256);
    assert_eq!(units[0x80], 0x20AC);
    assert_eq!(units[0xE9], 0x00E9);
    assert_eq!(
        front().to_bytes(0, 0, &units, Some(&mut back), None),
        (256, false)
    );
    assert_eq!(back, bytes);
}

#[test]
fn a_unit_the_page_lacks_answers_the_default() {
    let mut out = [0; 2];
    assert_eq!(
        front().to_bytes(1252, 0, &[0x0100, 0x41], Some(&mut out), None),
        (2, true)
    );
    assert_eq!(out, [b'?', b'A']);
}

#[test]
fn refusals_leave_the_rows_errors() {
    keep(0);
    assert_eq!(front().to_wide(437, 0, b"a", None), 0);
    assert_eq!(ERROR.with(Cell::get), 87);
    assert_eq!(front().to_bytes(0, 0x400, &[0x41], None, None), (0, false));
    assert_eq!(ERROR.with(Cell::get), 1004);
    assert_eq!(front().to_wide(0, 0, b"abc", Some(&mut [0; 2])), 0);
    assert_eq!(ERROR.with(Cell::get), 122);
    assert_eq!(front().compare(0, 0x1000, b"a", b"a"), 0);
    assert_eq!(ERROR.with(Cell::get), 1004);
}

#[test]
fn the_info_the_cases_and_the_classes() {
    let info = front()
        .code_page_info(0)
        .map(|info| (info.max_char_size, info.default));
    assert_eq!(info, Some((1, [b'?', 0])));
    let mut out = [0; 4];
    assert_eq!(
        front().map_string(0, UPPERCASE, b"a\xe9\xff1", Some(&mut out)),
        4
    );
    assert_eq!(&out, b"A\xc9\x9f1");
    let mut classes = [0; 3];
    assert_eq!(
        front().string_type_w(1, &[0x41, 0x20, 0x37], &mut classes),
        1
    );
    assert_eq!(classes, [0x181, 0x048, 0x084]);
    let mut text = [0; 5];
    assert_eq!(front().locale_info(0, 0x8000_1004, Some(&mut text)), 5);
    assert_eq!(&text, b"1252\0");
}
