# SPDX-License-Identifier: MIT
"""``names.py`` on the heap ports' harvested rows and the workspace's map: each name
shown is the map's, an address it lacks shows bare, a row still splits after, and a
rename is one edit to the map."""

from __future__ import annotations

import json
import re
from pathlib import Path

import pytest
from lines import fields
from names import annotate
from names import build
from names import load
from names import MAP

HARVEST = Path(__file__).resolve().parents[2] / "mech-game" / "harvest"
FIVE = (
    "heap_1001c303",
    "heap_1001c5d1",
    "heap_1001c8a3",
    "heap_1001c989",
    "heap_10086951",
)
SHOWN = re.compile(r"\A([0-9a-f]+)\(([^()]+)\)\Z")


def bodies() -> list[str]:
    return [
        body
        for name in FIVE
        for body in (HARVEST / f"{name}.log").read_text(encoding="utf-8").splitlines()
    ]


def split(body: str) -> dict[str, str]:
    return fields(body.partition(" ")[2])


def test_each_shown_name_is_the_maps_and_the_row_still_splits() -> None:
    names = load()
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
    names = load()
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
    body = next(
        body
        for body in bodies()
        if body.startswith("fn.in ") and split(body)["f"] in names["functions"]
    )
    address = split(body)["f"]
    names["functions"][address] = "RenamedInTheCopy"
    copy = tmp_path / "names.json"
    copy.write_text(json.dumps(names), encoding="utf-8")
    assert split(annotate(body, load(copy)))["f"] == f"{address}(RenamedInTheCopy)"
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
