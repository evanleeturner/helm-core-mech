# SPDX-License-Identifier: MIT
"""The game's names beside the addresses in its log rows.

The map, ``names.json`` at the workspace root beside ``logbook.json``, holds the
names as the logbook holds the sentences: a rename is one edit to the map, and no
log site changes. A trace row keys a function by its address in ``f`` (and in
``c``, a callee's; an import's bare name is no address), a global in ``g``, and a
function's arguments by position in ``fn.in``'s ``a``; the map holds those three
kinds alone. No row's field carries a local, so locals stay out.

``build`` makes the map from a names list, one JSON row a name (``target``,
``address``, ``kind``, ``name``, ``steps``), each name its last step's spelling;
the list's evidence, dates and rungs stay behind. ``annotate`` shows a row's body,
``event k=v ...``, with each address the map holds as ``address(name)`` and each
named argument as ``value(name)``: the value stays one word, so the row still
splits by ``lines.fields``. An address the map lacks shows bare.

    python3 names.py build <names.jsonl> [--out names.json]
    python3 names.py show <log> [--map names.json] [--f ADDRESS] [--start S] [--end E]
"""

from __future__ import annotations

import argparse
import json
import sys
from collections.abc import Iterable
from pathlib import Path

from lines import fields
from lines import Line
from lines import parse
from lines import window

MAP = Path(__file__).resolve().parents[3] / "names.json"

_ABOUT = (
    "The game's names by address, for the log readers: functions and globals map an"
    " address to its name; parameters map a function's address to each argument's"
    " position (param_N at N) and its name. A rename is one edit here."
)
_TABLES = {"function": "functions", "global": "globals"}


def build(rows: Iterable[dict], program: str = "helm-core-mech") -> dict:
    """The map from a names list's rows.

    Raises ``ValueError`` on rows of two targets, a kind the map does not know, a
    parameter not named ``param_N``, or one address and kind (or one parameter
    position) named twice: the map would hide which name is meant.
    """
    target: str | None = None
    tables: dict[str, dict[str, str]] = {"functions": {}, "globals": {}}
    parameters: dict[str, dict[str, str]] = {}
    for row in rows:
        if target is None:
            target = row["target"]
        elif row["target"] != target:
            raise ValueError(f"two targets: {target} and {row['target']}")
        kind, address = row["kind"], row["address"]
        spelling = row["steps"][-1]["spelling"]
        if kind == "local":
            continue
        if kind == "parameter":
            position = row["name"].removeprefix("param_")
            if position == row["name"] or not position.isdigit():
                raise ValueError(f"parameter {row['name']} at {address} is not param_N")
            slot = parameters.setdefault(address, {})
            if position in slot:
                raise ValueError(f"param_{position} at {address} named twice")
            slot[position] = spelling
            continue
        if kind not in _TABLES:
            raise ValueError(f"kind {kind} at {address}")
        table = tables[_TABLES[kind]]
        if address in table:
            raise ValueError(f"{kind} {address} named twice")
        table[address] = spelling
    return {
        "program": program,
        "about": _ABOUT,
        "target": target,
        "functions": dict(sorted(tables["functions"].items())),
        "globals": dict(sorted(tables["globals"].items())),
        "parameters": {
            address: dict(sorted(slot.items(), key=lambda item: int(item[0])))
            for address, slot in sorted(parameters.items())
        },
    }


def load(path: Path = MAP) -> dict:
    """The map at ``path``."""
    return json.loads(path.read_text(encoding="utf-8"))


def _quoted(value: str) -> str:
    """``value`` as ``oxide-logline`` writes it: quoted when empty or holding a space, ``"`` or ``=``."""
    if value and not any(char in value for char in ' "='):
        return value
    return '"' + value.replace("\\", "\\\\").replace('"', '\\"') + '"'


def _named(value: str, table: dict[str, str]) -> str:
    return f"{value}({table[value]})" if value in table else value


def _render(event: str, found: dict[str, str], names: dict) -> str:
    """A body from its event and fields, each address and argument the map names shown."""
    words = [event]
    for key, value in found.items():
        if key in ("f", "c"):
            value = _named(value, names["functions"])
        elif key == "g":
            value = _named(value, names["globals"])
        elif key == "a" and event == "fn.in":
            slot = names["parameters"].get(found.get("f", ""), {})
            value = ",".join(
                f"{word}({slot[str(at)]})" if str(at) in slot else word
                for at, word in enumerate(value.split(","), start=1)
            )
        words.append(f"{key}={_quoted(value)}")
    return " ".join(words)


def annotate(body: str, names: dict) -> str:
    """The body ``event k=v ...`` with the map's names shown beside its addresses.

    Raises ``ValueError`` when the fields do not split by ``lines.fields``.
    """
    event, _, rest = body.partition(" ")
    return _render(event, fields(rest), names)


def _rows(path: Path) -> tuple[list[Line], int]:
    """The log's rows, a timed line or a bare body (a harvested fixture's form) each one."""
    rows: list[Line] = []
    skipped = 0
    with path.open(encoding="utf-8", errors="replace") as log:
        for text in log:
            line = parse(text)
            if line is None:
                event, _, rest = text.rstrip("\r\n").partition(" ")
                try:
                    line = Line("", "", event, fields(rest))
                except ValueError:
                    skipped += 1
                    continue
            rows.append(line)
    return rows, skipped


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    commands = parser.add_subparsers(dest="command", required=True)
    made = commands.add_parser("build", help="make the map from a names list")
    made.add_argument("names", type=Path, help="the names list, one JSON row a name")
    made.add_argument("--out", type=Path, default=MAP)
    show = commands.add_parser("show", help="print a log's rows with the names shown")
    show.add_argument("log", type=Path)
    show.add_argument("--map", type=Path, default=MAP)
    show.add_argument("--f", help="only the rows whose f is this address")
    show.add_argument("--start", help="a time or an event: the first row shown")
    show.add_argument("--end", help="a time or an event: the last row shown")
    args = parser.parse_args(argv)
    if args.command == "build":
        with args.names.open(encoding="utf-8") as listed:
            built = build(json.loads(text) for text in listed if text.strip())
        args.out.write_text(json.dumps(built, indent=2) + "\n", encoding="utf-8")
        return 0
    names = load(args.map)
    rows, skipped = _rows(args.log)
    for line in window(rows, args.start, args.end):
        if args.f is not None and line.fields.get("f") != args.f:
            continue
        body = _render(line.event, line.fields, names)
        print(f"{line.time} {line.level} {body}" if line.time else body)
    if skipped:
        print(f"{skipped} lines not rows", file=sys.stderr)
    return 0


if __name__ == "__main__":
    sys.exit(main())
