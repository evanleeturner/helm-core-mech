# SPDX-License-Identifier: MIT
"""``names.py`` on the heap ports' harvested rows and the workspace's map: each name
shown is the map's, an address it lacks shows bare, a row still splits after, and a
rename is one edit to the map. Then on made-up rows and exports alone: two DLLs
loaded in turn at one base each name their own rows, and the build refuses a map
that would hide which name is meant."""

from __future__ import annotations

import json
import re
from pathlib import Path

import pytest
from lines import fields
from lines import parse
from names import annotate
from names import build
from names import Exports
from names import gather
from names import load
from names import MAP
from names import Reader

HARVEST = Path(__file__).resolve().parents[2] / "mech-game" / "harvest"
FIVE = (
    "heap_1001c303",
    "heap_1001c5d1",
    "heap_1001c8a3",
    "heap_1001c989",
    "heap_10086951",
)
SHOWN = re.compile(r"\A([0-9a-f]+)\(([^()]+)\)\Z")
GAME = "mercsw"  # the target the heap five's rows are code of


def bodies() -> list[str]:
    return [
        body
        for name in FIVE
        for body in (HARVEST / f"{name}.log").read_text(encoding="utf-8").splitlines()
    ]


def split(body: str) -> dict[str, str]:
    return fields(body.partition(" ")[2])


def test_each_shown_name_is_the_maps_and_the_row_still_splits() -> None:
    names = load()["targets"][GAME]
    named = 0
    for body in bodies():
        before, after = split(body), split(annotate(body, names))
        assert list(after) == list(before), body
        for key, table in (("f", "functions"), ("c", "functions"), ("g", "globals")):
            if key not in before:
                continue
            shown = SHOWN.match(after[key])
            if before[key] in names[table]:
                assert shown is not None and shown.groups() == (
                    before[key],
                    names[table][before[key]],
                ), body
                named += 1
            else:
                assert after[key] == before[key], body
    assert named > 0, "no row of the five shows a name"


def test_fn_in_arguments_are_named_by_position() -> None:
    names = load()["targets"][GAME]
    entries = [
        body for body in bodies() if body.startswith("fn.in ") and "a" in split(body)
    ]
    assert entries
    for body in entries:
        before, after = split(body), split(annotate(body, names))
        slot = names["parameters"].get(before["f"], {})
        for at, (word, shown) in enumerate(
            zip(before["a"].split(","), after["a"].split(","), strict=True), start=1
        ):
            assert shown == (f"{word}({slot[str(at)]})" if str(at) in slot else word), (
                body
            )


def test_a_rename_is_one_edit_to_the_map(tmp_path: Path) -> None:
    before = MAP.read_bytes()
    names = load()
    game = names["targets"][GAME]
    body = next(
        body
        for body in bodies()
        if body.startswith("fn.in ") and split(body)["f"] in game["functions"]
    )
    address = split(body)["f"]
    game["functions"][address] = "RenamedInTheCopy"
    copy = tmp_path / "names.json"
    copy.write_text(json.dumps(names), encoding="utf-8")
    shown = split(annotate(body, load(copy)["targets"][GAME]))["f"]
    assert shown == f"{address}(RenamedInTheCopy)"
    assert MAP.read_bytes() == before


def test_build_keeps_the_last_spelling_and_names_alone() -> None:
    def row(address: str, kind: str, name: str, *spellings: str) -> dict:
        steps = [
            {
                "spelling": s,
                "rung": 1,
                "approved": "2000-01-01",
                "list": "a-list",
                "evidence": "0" * 64,
            }
            for s in spellings
        ]
        return {
            "target": "game",
            "address": address,
            "kind": kind,
            "name": name,
            "steps": steps,
        }

    built = build(
        [
            row("10000000", "function", "FUN_10000000", "First", "Second"),
            row("10000000", "parameter", "param_2", "count"),
            row("10000000", "local", "local_8", "scratch"),
            row("10009000", "global", "DAT_10009000", "g_table"),
        ]
    )
    assert built["target"] == "game"
    assert built["functions"] == {"10000000": "Second"}
    assert built["globals"] == {"10009000": "g_table"}
    assert built["parameters"] == {"10000000": {"2": "count"}}
    assert not re.search(
        r"scratch|rung|approved|a-list|evidence|0{64}", json.dumps(built)
    )
    with pytest.raises(ValueError, match="two targets"):
        build(
            [
                row("1", "global", "DAT_1", "a"),
                {**row("2", "global", "DAT_2", "b"), "target": "other"},
            ]
        )


def listed(target: str, address: str, kind: str, name: str, spelling: str) -> dict:
    step = {"spelling": spelling, "rung": 1, "approved": "2000-01-01", "list": "a-list"}
    return {
        "target": target,
        "address": address,
        "kind": kind,
        "name": name,
        "steps": [step],
    }


def exported(
    folder: Path, address: str, convention: str, *blocks: tuple[str, ...]
) -> None:
    """One function's export: each block its instructions' addresses, first to last."""
    folder.mkdir(exist_ok=True)
    export = {
        "address": address,
        "calling_convention": convention,
        "cfg": [{"start": block[0], "stop": block[-1]} for block in blocks],
        "assembly": [f"{at}  NOP" for block in blocks for at in block],
    }
    (folder / f"{address}.json").write_text(json.dumps(export), encoding="utf-8")


def test_two_dlls_at_one_base_each_name_their_own_rows(tmp_path: Path) -> None:
    shell, game = tmp_path / "shell", tmp_path / "game"
    # The shell's named thiscall: two blocks with no instruction between, then a
    # third past an unnamed function's code.
    exported(
        shell,
        "10001000",
        "__thiscall",
        ("10001000", "10001010"),
        ("10001015", "10001020"),
        ("10001030", "10001038"),
    )
    exported(shell, "10001024", "__cdecl", ("10001024", "1000102a"))
    (shell / "_index.json").write_text("{}", encoding="utf-8")
    exported(game, "10000f00", "__cdecl", ("10000f00", "10001018", "10001030"))
    names = gather(
        [
            (
                "SHELL.DLL",
                build(
                    [
                        listed(
                            "shell", "10001000", "function", "FUN_10001000", "ShellDraw"
                        ),
                        listed("shell", "10001000", "parameter", "this", "screen"),
                        listed("shell", "10001000", "parameter", "param_1", "count"),
                    ],
                    Exports.read(shell),
                ),
            ),
            (
                "GAME.DLL",
                build(
                    [
                        listed(
                            "game", "10000f00", "function", "FUN_10000f00", "GameTick"
                        ),
                        listed("game", "10000f00", "parameter", "param_1", "delta"),
                    ],
                    Exports.read(game),
                ),
            ),
        ]
    )
    assert names["targets"]["shell"]["extents"] == {
        "10001000": ["10001000-10001020", "10001030-10001038"]
    }
    assert names["targets"]["shell"]["parameters"] == {
        "10001000": {"1": "screen", "2": "count"}
    }
    log = [
        "00:00:00.000 D imp.through c=KERNEL32!Sleep from=10001018",
        "00:00:00.001 D imp.call c=KERNEL32!LoadLibraryA m=SHELL.DLL v=10000000",
        "00:00:00.002 D fn.in f=10001000 a=5,7",
        "00:00:00.003 D imp.through c=KERNEL32!Sleep from=10001018",
        "00:00:00.004 D imp.through c=KERNEL32!Sleep from=10001026",
        "00:00:00.005 D imp.call c=KERNEL32!LoadLibraryA m=game.dll v=10000000",
        "00:00:00.006 D imp.through c=KERNEL32!Sleep from=10001018",
        "00:00:00.007 D fn.in f=10000f00 a=9",
        "00:00:00.008 D fn.in f=10001000 a=5,7",
    ]
    reader = Reader(names)
    shown = []
    for text in log:
        line = parse(text)
        assert line is not None, text
        shown.append(reader.render(line))
        reader.see(line)
    assert shown == [
        "imp.through c=KERNEL32!Sleep from=10001018",
        "imp.call c=KERNEL32!LoadLibraryA m=SHELL.DLL v=10000000",
        "fn.in f=10001000(ShellDraw) a=5(screen),7(count)",
        "imp.through c=KERNEL32!Sleep from=10001018(ShellDraw)",
        "imp.through c=KERNEL32!Sleep from=10001026",
        "imp.call c=KERNEL32!LoadLibraryA m=game.dll v=10000000",
        "imp.through c=KERNEL32!Sleep from=10001018(GameTick)",
        "fn.in f=10000f00(GameTick) a=9(delta)",
        "fn.in f=10001000 a=5,7",
    ]


def test_build_refuses_a_map_that_would_hide_a_name(tmp_path: Path) -> None:
    with pytest.raises(ValueError, match="no place"):
        build([listed("game", "10000f00", "parameter", "this", "self")])
    exported(tmp_path, "10000f00", "__cdecl", ("10000f00", "10000f10"))
    exported(tmp_path, "10000f08", "__cdecl", ("10000f08", "10000f0c"))
    with pytest.raises(ValueError, match="overlap"):
        build(
            [
                listed("game", "10000f00", "function", "FUN_10000f00", "Outer"),
                listed("game", "10000f08", "function", "FUN_10000f08", "Inner"),
            ],
            Exports.read(tmp_path),
        )
    tables = build([listed("game", "10000f00", "function", "FUN_10000f00", "Outer")])
    with pytest.raises(ValueError, match="twice"):
        gather([("GAME.DLL", tables), ("Game.dll", {**tables, "target": "other"})])
