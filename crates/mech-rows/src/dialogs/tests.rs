// SPDX-License-Identifier: MIT
//! Falsifiers 2 and 3: the relay logs `r1`, `r2` and the flight `w1`
//! (`s2_harvest.py`'s rows, copied whole from WD1's records), played on the
//! rows. The file is planted to the shape the real one read as (`first-read.txt`,
//! WD1's records): its ids, states, options and styles, no template's bytes and no
//! game's words. Wine's window `2005e` is the desk's own handle here; its other
//! handles are mapped.
//! The front does not wait, so `DialogBoxParamA`'s line follows the init's calls,
//! where the log's call order puts it first.
use core::cell::Cell;
use core::ops::Range;
use std::collections::HashMap;

use kernel_process::{plant, resource};
use window::Rect;
use window::desk::{Desk, Window};
use window::front::Front as Windows;
use window::page::Page;
use window_dialogs::front::Front;
use window_dialogs::tables::Tables;
use window_dialogs::{Resources, plant as templates};

use super::movie_box;
use super::{BoxCalls, CANCEL, LAUNCHER, MENU_IDS, MOVIE_320, MOVIE_BOX, OK, SHELL, SHELL_MENU};
use crate::test_log::debug_lines;

const KEPT: &str = include_str!("harvest-kept.tsv");
const W1: &str = include_str!("harvest-w1.tsv");
const INSTANCE: u32 = 0x1000_0000;
const OWNER: u32 = 0x2_005e;
const PROCEDURE: Range<u32> = 0x1001_69f0..0x1001_6b50;
const WM_INITDIALOG: u32 = 0x110;
const WM_COMMAND: u32 = 0x111;
const RETURN: u8 = 0x0d;
const ESCAPE: u8 = 0x1b;

struct Disc(Vec<u8>);

impl Resources for Disc {
    fn resource(&self, instance: u32, kind: u32, id: u32) -> Option<&[u8]> {
        (instance == INSTANCE)
            .then(|| resource::find(&self.0, kind, id))
            .flatten()
    }
}

/// Menu `0x68` and box `0x8a` row for row as the real file holds them, the words
/// left empty; the bar's `0x10`, `0x11` and `0x90` open its 3 popups.
fn disc() -> Disc {
    let menu = templates::menu(&[
        (0x10, 0, ""),
        (0x01, 0x9c41, ""),
        (0x01, 0x9c99, ""),
        (0x02, 0x9c97, ""),
        (0x00, 0, ""),
        (0x80, 0x9c43, ""),
        (0x11, 0, ""),
        (0x01, 0x9c94, ""),
        (0x01, 0x9c4b, ""),
        (0x81, 0x9c96, ""),
        (0x90, 0, ""),
        (0x00, 0x9c95, ""),
        (0x00, 0, ""),
        (0x81, 0x9c9a, ""),
    ]);
    let controls = [
        (1, 0x80, 0x5001_0001, ""),
        (2, 0x80, 0x5001_0000, ""),
        (0xffff, 0x82, 0x5002_0001, ""),
        (0x3e8, 0x80, 0x5001_0003, ""),
        (0x3e9, 0x80, 0x5001_0003, ""),
        (0xffff, 0x82, 0x5002_0001, ""),
    ];
    let boxed = templates::dialog(0x90c8_00c0, "", &controls);
    Disc(plant::image(&[
        (resource::MENU, SHELL_MENU, &menu),
        (resource::DIALOG, MOVIE_BOX, &boxed),
    ]))
}

struct Seam<'a> {
    front: &'a Front<'a, Page, Disc>,
    launcher: Front<'a, Page, Disc>,
    windows: Windows<'a, Page>,
}

impl BoxCalls for Seam<'_> {
    fn check_dlg_button(&self, window: u32, id: u32, check: u32) -> u32 {
        self.front.check_dlg_button(window, id, check)
    }
    fn get_dlg_item(&self, window: u32, id: u32) -> u32 {
        self.front.get_dlg_item(window, id)
    }
    fn set_focus(&self, window: u32) -> u32 {
        self.windows.set_focus(window)
    }
    fn is_dlg_button_checked(&self, window: u32, id: u32) -> u32 {
        self.front.is_dlg_button_checked(window, id)
    }
    fn end_dialog(&self, window: u32, value: u32) -> u32 {
        self.front.end_dialog(window, value)
    }
}

struct Row<'a> {
    kind: &'a str,
    name: &'a str,
    args: Vec<u32>,
    result: u32,
    ret: u32,
}

fn number(cell: &str) -> u32 {
    u32::from_str_radix(cell, 16).expect("invariant: the harvest writes hex")
}

fn rows<'a>(tsv: &'a str, key: &str) -> Vec<Row<'a>> {
    let cells = tsv.lines().map(|line| line.split('\t').collect::<Vec<_>>());
    cells
        .filter(|cell| cell[0] == key)
        .map(|cell| Row {
            kind: cell[4],
            name: cell[5],
            args: cell[6].split(',').map(number).collect(),
            result: number(cell[7]),
            ret: number(cell[8]),
        })
        .collect()
}

fn imp_calls(call: impl FnOnce()) -> Vec<String> {
    let mut lines = debug_lines(call);
    lines.retain(|line| line.starts_with("imp.call"));
    lines
}

fn own(map: &HashMap<u32, u32>, value: u32) -> u32 {
    map.get(&value).copied().unwrap_or(value)
}

fn wanted(row: &Row<'_>, map: &HashMap<u32, u32>) -> String {
    let args: Vec<String> = row
        .args
        .iter()
        .map(|a| format!("{:x}", own(map, *a)))
        .collect();
    let (name, args) = (row.name, args.join(","));
    // The box's focus goes through the window front, whose line it is.
    let by = if name == "SetFocus" {
        "window"
    } else {
        "window-dialogs"
    };
    format!(
        "imp.call c=USER32!{name} a={args} v={:x} by={by}",
        own(map, row.result)
    )
}

/// One program with the owner window, then `test` over its fronts.
fn stage<T>(test: impl FnOnce(&Desk<Page>, &Tables, &Front<'_, Page, Disc>, &Seam<'_>) -> T) -> T {
    let (desk, tables, disc) = (Desk::new(Page::new(0x280, 0x1e0)), Tables::new(), disc());
    desk.state().windows.push(Window {
        handle: OWNER,
        class: Vec::new(),
        procedure: 0,
        rect: Rect::default(),
        style: 0,
        ex_style: 0,
        user: 0,
        longs: Vec::new(),
        title: Vec::new(),
        parent: 0,
        shown: true,
    });
    let shell = Front::new(&SHELL, &desk, &tables, &disc);
    let seam = Seam {
        front: &shell,
        launcher: Front::new(&LAUNCHER, &desk, &tables, &disc),
        windows: Windows::new(&crate::window::SHELL, &desk),
    };
    test(&desk, &tables, &shell, &seam)
}

/// A call outside the box, as its row names it.
fn call(front: &Front<'_, Page, Disc>, row: &Row<'_>, map: &HashMap<u32, u32>) -> u32 {
    let arg = |at: usize| own(map, row.args[at]);
    match row.name {
        "SetMenu" => front.set_menu(arg(0), arg(1)),
        "LoadMenuA" => front.load_menu(arg(0), arg(1)),
        "EnableMenuItem" => front.enable_menu_item(arg(0), arg(1), arg(2)),
        "CheckMenuItem" => front.check_menu_item(arg(0), arg(1), arg(2)),
        "DrawMenuBar" => front.draw_menu_bar(arg(0)),
        name => panic!("a row no front serves: {name}"),
    }
}

/// Plays the rows of `key`, each call's line and answer the log's; the setting
/// after, from 1, as the logs' first open read it not 0.
fn replay(tsv: &str, key: &str) -> u32 {
    stage(|desk, tables, shell, seam| {
        let all = rows(tsv, key);
        let setting = Cell::new(1);
        let mut map = HashMap::new();
        let mut at = 0;
        let body = |from: usize| {
            let inside = |row: &&Row<'_>| row.kind == "api" && PROCEDURE.contains(&row.ret);
            all[from..]
                .iter()
                .filter(|row| {
                    row.kind == "api" || row.name == "WM_COMMAND" || row.name == "WM_INITDIALOG"
                })
                .take_while(inside)
                .collect::<Vec<_>>()
        };
        while at < all.len() {
            let row = &all[at];
            at += 1;
            match (row.kind, row.name) {
                ("proc", "WM_COMMAND") => {
                    let window = own(&map, row.args[0]);
                    let id = u16::try_from(row.args[1]).expect("invariant: an id's a low word");
                    let lines = imp_calls(|| {
                        assert!(match u32::from(id) {
                            OK => shell.key(window, RETURN),
                            CANCEL => shell.key(window, ESCAPE),
                            _ => shell.press(window, id),
                        });
                        let sent = desk.state().queue.pop_back().expect("invariant: it queued");
                        let answer = movie_box(seam, &setting, sent.window, sent.message, sent.w);
                        assert_eq!(answer, row.result);
                    });
                    let want: Vec<String> = body(at).iter().map(|row| wanted(row, &map)).collect();
                    assert_eq!(lines, want, "{key} WM_COMMAND {id:x}");
                }
                ("api", "DialogBoxParamA") => {
                    let from = at
                        + all[at..]
                            .iter()
                            .position(|r| r.name == "WM_INITDIALOG")
                            .expect("invariant: init follows");
                    let init = &all[from];
                    let arguments = <[_; 5]>::try_from(&row.args[..]).expect("5 arguments");
                    let mut lines = imp_calls(|| {
                        let answer = shell.dialog_box_param(arguments, |_, sent| {
                            movie_box(seam, &setting, sent.window, sent.message, sent.w)
                        });
                        assert_eq!(answer, row.result);
                    });
                    // `r1`'s relay list held no `SetFocus` (the sheet): `w1` holds its 2 lines.
                    if key == "r1" {
                        lines.retain(|line| !line.contains("!SetFocus"));
                    }
                    let (handle, first) = {
                        let state = tables.state();
                        let open = state.boxes.last().expect("invariant: the box stays open");
                        (open.handle, open.controls[0].handle)
                    };
                    map.insert(init.args[0], handle);
                    map.insert(init.args[1], first);
                    let mut want: Vec<String> =
                        body(from + 1).iter().map(|row| wanted(row, &map)).collect();
                    want.push(wanted(row, &map));
                    assert_eq!(lines, want, "{key} the opening");
                    assert_eq!(
                        desk.state().focus,
                        first,
                        "GetDlgItem's handle, SetFocus took it"
                    );
                }
                ("api", _) if !PROCEDURE.contains(&row.ret) => {
                    let front = if row.ret < INSTANCE {
                        &seam.launcher
                    } else {
                        shell
                    };
                    let mut answer = 0;
                    let lines = imp_calls(|| answer = call(front, row, &map));
                    if row.name == "LoadMenuA" {
                        map.insert(row.result, answer);
                    } else {
                        assert_eq!(answer, row.result, "{key} {}", row.name);
                    }
                    assert_eq!(lines, [wanted(row, &map)]);
                }
                _ => {}
            }
        }
        setting.get()
    })
}

#[test]
fn r1_replays_ok_keeping_the_setting() {
    assert_eq!(replay(KEPT, "r1"), 1);
}

#[test]
fn r2_replays_with_no_box() {
    assert_eq!(replay(KEPT, "r2"), 1);
}

#[test]
fn w1_replays_each_arm_ok_writing_0_and_escape_keeping_it() {
    assert_eq!(replay(W1, "w1"), 0);
}

#[test]
fn each_id_the_shell_asks_is_found_and_0x9c72_is_not() {
    stage(|_, _, shell, _| {
        let menu = shell.load_menu(INSTANCE, SHELL_MENU);
        for id in MENU_IDS {
            assert_ne!(
                shell.enable_menu_item(menu, u32::from(id), 0),
                u32::MAX,
                "{id:#x}"
            );
        }
        assert_eq!(shell.enable_menu_item(menu, 0x9c72, 0), u32::MAX);
    });
}

#[test]
fn every_other_message_the_logs_hold_answered_0() {
    for (tsv, key) in [(KEPT, "r1"), (W1, "w1")] {
        let all = rows(tsv, key);
        let other = all.iter().filter(|row| row.kind == "proc");
        let other = other.filter(|row| row.name != "WM_INITDIALOG" && row.name != "WM_COMMAND");
        assert!(other.clone().count() > 0);
        assert!(other.clone().all(|row| row.result == 0), "{key}");
    }
}

/// Opens the box with `setting`, runs `after`, and hands back the setting.
fn opened(
    setting: u32,
    after: impl FnOnce(&Front<'_, Page, Disc>, &Seam<'_>, &Cell<u32>, u32),
) -> u32 {
    stage(|_, tables, shell, seam| {
        let kept = Cell::new(setting);
        let opening = [INSTANCE, MOVIE_BOX, OWNER, 0x1001_69f0, 0];
        let answer = shell.dialog_box_param(opening, |_, sent| {
            assert_eq!(sent.message, WM_INITDIALOG);
            movie_box(seam, &kept, sent.window, sent.message, sent.w)
        });
        assert_eq!(answer, 0);
        let handle = tables.state().boxes[0].handle;
        after(shell, seam, &kept, handle);
        kept.get()
    })
}

#[test]
fn cancel_leaves_the_setting_and_closes() {
    let kept = opened(1, |shell, seam, kept, window| {
        assert_eq!(movie_box(seam, kept, window, WM_COMMAND, CANCEL), 1);
        assert_eq!(
            shell.is_dlg_button_checked(window, MOVIE_320),
            0,
            "the box is gone"
        );
    });
    assert_eq!(kept, 1);
}

#[test]
fn another_message_or_id_calls_nothing() {
    opened(1, |_, seam, kept, window| {
        let lines = imp_calls(|| {
            assert_eq!(movie_box(seam, kept, window, 0x30, 0), 0);
            assert_eq!(movie_box(seam, kept, window, WM_COMMAND, 0x77), 1);
        });
        assert!(lines.is_empty());
    });
}
