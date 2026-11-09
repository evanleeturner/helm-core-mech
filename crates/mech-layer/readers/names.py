# SPDX-License-Identifier: MIT
"""The game's names beside the addresses in its log rows, read in the DLL holding each.

The map, ``names.json`` at the workspace root beside ``logbook.json``, holds the
names as the logbook holds the sentences: a rename is one edit to the map, and no
log site changes. It holds one target a DLL, under the DLL's file name: two DLLs
can load at one base, so one address can be code in each, and only the log says
which is loaded. A trace row keys a function by its address in ``f`` (and in
``c``, a callee's; an import's bare name is no address), a global in ``g``, a
function's arguments by position in ``fn.in``'s ``a``, and a call's return address
in ``from``, which falls inside the calling function, not at its start. So a
target holds its functions, globals and parameters, and each named function's
extent: the code its blocks cover, each range from a block's first instruction to
the address of its last. No row's field carries a local, so locals stay out.

``build`` makes one target's tables from its names list, one JSON row a name
(``target``, ``address``, ``kind``, ``name``, ``steps``), each name its last step's
spelling; the list's evidence, dates and rungs stay behind. Given the target's
exports (``Exports``), it places a thiscall's arguments as its entry row logs them,
``this`` first, and takes each named function's extent; an unnamed function's
never enters. ``gather`` puts the targets in one map.

``Reader`` reads a log's rows in order. A ``LoadLibraryA`` row (``c`` the import,
``m`` the DLL, ``v`` the base it answered) opens that DLL's window at that base,
and the next load at the same base closes it. Each address is read in the window
at the highest base at or below it, in that window's target; outside any window,
or in the window of a DLL the map lacks, it shows bare. ``annotate`` shows one
body in one target's tables. Either way an address the target names shows as
``address(name)``, a named argument as ``value(name)``, and a return address a
named function's extent holds as ``address(name)``: the value stays one word, so
the row still splits by ``lines.fields``.

    python3 names.py build --list NAMES DLL EXPORTS [--list ...] [--out names.json]
    python3 names.py show <log> [--map names.json] [--f ADDRESS] [--start S] [--end E]
"""

from __future__ import annotations

import argparse
import bisect
import json
import re
import sys
from collections.abc import Callable
from collections.abc import Iterable
from dataclasses import dataclass
from pathlib import Path

from lines import fields
from lines import Line
from lines import parse
from lines import window

MAP = Path(__file__).resolve().parents[3] / "names.json"

_ABOUT = (
    "The game's names by address, for the log readers, one target a DLL under the"
    " DLL's file name: functions and globals map an address to its name; parameters"
    " map a function's address to each argument's position in its entry row (param_N"
    " at N; in a thiscall, this at 1 and param_N at N + 1) and its name; extents map"
    " a named function's address to the code it covers, each range from a block's"
    " first instruction to the address of its last, for the return addresses rows"
    " carry. A rename is one edit here."
)
_TABLES = {"function": "functions", "global": "globals"}
_LOAD = "KERNEL32!LoadLibraryA"
_HEX = re.compile(r"[0-9a-f]+\Z")


@dataclass(frozen=True)
class Exports:
    """A target's function exports: each function's calling convention and blocks,
    and the instruction that follows each instruction any export lists."""

    conventions: dict[str, str]
    blocks: dict[str, list[tuple[int, int]]]
    following: dict[int, int]

    @classmethod
    def read(cls, folder: Path) -> Exports:
        """The exports in ``folder``, one ``<address>.json`` a function.

        A file whose stem is no hex address is the folder's index, no function's.
        An export gives ``calling_convention``, ``cfg`` blocks of ``start`` and
        ``stop`` (a block's first instruction and the address of its last), and
        ``assembly`` lines that open with their instruction's address.

        Raises ``ValueError`` when a file's ``address`` is not its stem.
        """
        conventions: dict[str, str] = {}
        blocks: dict[str, list[tuple[int, int]]] = {}
        listed: set[int] = set()
        for path in sorted(folder.glob("*.json")):
            if not _HEX.match(path.stem):
                continue
            export = json.loads(path.read_text(encoding="utf-8"))
            if export["address"] != path.stem:
                raise ValueError(f"{path.name} holds {export['address']}")
            conventions[path.stem] = export["calling_convention"]
            blocks[path.stem] = sorted(
                (int(block["start"], 16), int(block["stop"], 16))
                for block in export["cfg"]
            )
            listed.update(int(line.split()[0], 16) for line in export["assembly"])
        order = sorted(listed)
        return cls(conventions, blocks, dict(zip(order, order[1:])))

    def extent(self, function: str) -> list[tuple[int, int]]:
        """The function's blocks merged where they overlap or touch, as (first, last).

        Two blocks touch when no instruction an export lists lies between the one's
        last instruction and the other's first. Empty for a function with no
        export or no block.
        """
        merged: list[list[int]] = []
        for start, stop in self.blocks.get(function, []):
            if merged and (
                start <= merged[-1][1] or start == self.following.get(merged[-1][1])
            ):
                merged[-1][1] = max(merged[-1][1], stop)
            else:
                merged.append([start, stop])
        return [(start, stop) for start, stop in merged]


def place(name: str, convention: str) -> int | None:
    """Where a parameter named ``name`` stands in its function's ``fn.in`` ``a``.

    ``param_N`` stands at N. A thiscall's port takes ``this`` first and its entry
    row logs the arguments in that order, so in a ``__thiscall`` ``this`` stands at
    1 and ``param_N`` at N + 1. ``None`` when the name gives no place.
    """
    thiscall = convention == "__thiscall"
    if thiscall and name == "this":
        return 1
    number = name.removeprefix("param_")
    if number == name or not number.isdigit():
        return None
    return int(number) + (1 if thiscall else 0)


def build(rows: Iterable[dict], exports: Exports | None = None) -> dict:
    """One target's tables from its names list's rows, its extents from its exports.

    Without exports no function is a thiscall and none has an extent. A named
    function its exports give no block keeps its name and no extent.

    Raises ``ValueError`` on rows of two targets, a kind the map does not know, a
    parameter with no place (``place``), one address and kind (or one parameter
    position) named twice, or two named functions' extents overlapping: the map
    would hide which name is meant.
    """
    target: str | None = None
    tables: dict[str, dict[str, str]] = {"functions": {}, "globals": {}}
    parameters: dict[str, dict[int, str]] = {}
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
            convention = exports.conventions.get(address, "") if exports else ""
            position = place(row["name"], convention)
            if position is None:
                raise ValueError(
                    f"parameter {row['name']} at {address} has no place in a"
                )
            slot = parameters.setdefault(address, {})
            if position in slot:
                raise ValueError(f"position {position} at {address} named twice")
            slot[position] = spelling
            continue
        if kind not in _TABLES:
            raise ValueError(f"kind {kind} at {address}")
        table = tables[_TABLES[kind]]
        if address in table:
            raise ValueError(f"{kind} {address} named twice")
        table[address] = spelling
    extents: dict[str, list[tuple[int, int]]] = {}
    if exports is not None:
        for function in sorted(tables["functions"]):
            if ranges := exports.extent(function):
                extents[function] = ranges
    spans = sorted(
        (start, stop, f) for f, ranges in extents.items() for start, stop in ranges
    )
    for (_, stop, one), (start, _, other) in zip(spans, spans[1:]):
        if start <= stop:
            raise ValueError(f"extents of {one} and {other} overlap at {start:08x}")
    return {
        "target": target,
        "functions": dict(sorted(tables["functions"].items())),
        "globals": dict(sorted(tables["globals"].items())),
        "parameters": {
            address: {str(at): name for at, name in sorted(slot.items())}
            for address, slot in sorted(parameters.items())
        },
        "extents": {
            function: [f"{start:08x}-{stop:08x}" for start, stop in ranges]
            for function, ranges in extents.items()
        },
    }


def gather(
    targets: Iterable[tuple[str, dict]], program: str = "helm-core-mech"
) -> dict:
    """The map from each DLL's file name and its target's tables, ``build``'s.

    Raises ``ValueError`` on one target or one DLL given twice.
    """
    held: dict[str, dict] = {}
    modules: set[str] = set()
    for dll, tables in targets:
        target = tables["target"]
        if target in held or _module(dll) in modules:
            raise ValueError(f"{target} or {dll} given twice")
        modules.add(_module(dll))
        held[target] = {"dll": dll} | {k: v for k, v in tables.items() if k != "target"}
    return {"program": program, "about": _ABOUT, "targets": dict(sorted(held.items()))}


def load(path: Path = MAP) -> dict:
    """The map at ``path``."""
    return json.loads(path.read_text(encoding="utf-8"))


def _module(name: str) -> str:
    """A DLL's file name as Windows matches it: without its folder, in any case."""
    return name.replace("\\", "/").rsplit("/", 1)[-1].upper()


def _address(value: str) -> int | None:
    """``value`` as an address, a row's lowercase hex word; ``None`` for others."""
    return int(value, 16) if _HEX.match(value) else None


class _Target:
    """One target's tables, its extents sorted for a lookup by address."""

    def __init__(self, tables: dict) -> None:
        self.functions: dict[str, str] = tables["functions"]
        self.globals: dict[str, str] = tables["globals"]
        self.parameters: dict[str, dict[str, str]] = tables["parameters"]
        self._spans = sorted(
            (int(first, 16), int(last, 16), function)
            for function, ranges in tables["extents"].items()
            for first, _, last in (span.partition("-") for span in ranges)
        )
        self._starts = [start for start, _, _ in self._spans]

    def holder(self, address: int) -> str | None:
        """The named function whose extent holds ``address``."""
        at = bisect.bisect_right(self._starts, address) - 1
        if at >= 0 and address <= self._spans[at][1]:
            return self._spans[at][2]
        return None


def _quoted(value: str) -> str:
    """``value`` as ``oxide-logline`` writes it: quoted when empty or holding a space, ``"`` or ``=``."""
    if value and not any(char in value for char in ' "='):
        return value
    return '"' + value.replace("\\", "\\\\").replace('"', '\\"') + '"'


def _named(value: str, table: dict[str, str]) -> str:
    return f"{value}({table[value]})" if value in table else value


def _render(
    event: str, found: dict[str, str], at: Callable[[str], _Target | None]
) -> str:
    """A body from its event and fields, each address in the target ``at`` says."""
    words = [event]
    for key, value in found.items():
        if key in ("f", "c") and (target := at(value)) is not None:
            value = _named(value, target.functions)
        elif key == "g" and (target := at(value)) is not None:
            value = _named(value, target.globals)
        elif key == "from" and (target := at(value)) is not None:
            address = _address(value)
            holder = target.holder(address) if address is not None else None
            if holder is not None:
                value = f"{value}({target.functions[holder]})"
        elif key == "a" and event == "fn.in":
            function = found.get("f", "")
            owner = at(function)
            slot = owner.parameters.get(function, {}) if owner is not None else {}
            value = ",".join(
                f"{word}({slot[str(position)]})" if str(position) in slot else word
                for position, word in enumerate(value.split(","), start=1)
            )
        words.append(f"{key}={_quoted(value)}")
    return " ".join(words)


def annotate(body: str, names: dict) -> str:
    """The body ``event k=v ...`` with one target's names shown beside its addresses.

    ``names`` is one target's tables, as the map holds each under ``targets``.
    Raises ``ValueError`` when the fields do not split by ``lines.fields``.
    """
    event, _, rest = body.partition(" ")
    target = _Target(names)
    return _render(event, fields(rest), lambda _value: target)


class Reader:
    """A log's rows in order, each address read in the DLL window that holds it."""

    def __init__(self, names: dict) -> None:
        self._targets = {
            _module(tables["dll"]): _Target(tables)
            for tables in names["targets"].values()
        }
        self._bases: list[int] = []
        self._windows: dict[int, _Target | None] = {}

    def render(self, line: Line) -> str:
        """The row's body, each address read in the window holding it then."""
        return _render(line.event, line.fields, self._at)

    def see(self, line: Line) -> None:
        """Open the window a load row names, closing the one at its base.

        A load that answered 0 failed and opens none.
        """
        if line.fields.get("c") != _LOAD:
            return
        base = _address(line.fields.get("v", ""))
        if not base:
            return
        if base not in self._windows:
            bisect.insort(self._bases, base)
        self._windows[base] = self._targets.get(_module(line.fields.get("m", "")))

    def _at(self, value: str) -> _Target | None:
        address = _address(value)
        if address is None:
            return None
        at = bisect.bisect_right(self._bases, address) - 1
        return self._windows[self._bases[at]] if at >= 0 else None


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


def _built(names: Path, dll: str, folder: Path) -> dict:
    """One target's tables from its list and exports, each parameter with no place
    left out and each named function with no extent said."""
    exports = Exports.read(folder)
    with names.open(encoding="utf-8") as listed:
        rows = [json.loads(text) for text in listed if text.strip()]
    kept = []
    for row in rows:
        convention = exports.conventions.get(row["address"], "")
        if row["kind"] == "parameter" and place(row["name"], convention) is None:
            print(
                f"left out: {row['target']} parameter {row['name']} at {row['address']}"
            )
            continue
        kept.append(row)
    tables = build(kept, exports)
    for function in tables["functions"]:
        if function not in tables["extents"]:
            print(f"no extent: {tables['target']} function {function}")
    entries = sum(len(slot) for slot in tables["parameters"].values())
    ranges = sum(len(spans) for spans in tables["extents"].values())
    print(
        f"{tables['target']} {dll}: functions {len(tables['functions'])},"
        f" globals {len(tables['globals'])}, parameter entries {entries},"
        f" extents {ranges} over {len(tables['extents'])} functions"
    )
    return tables


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    commands = parser.add_subparsers(dest="command", required=True)
    made = commands.add_parser(
        "build", help="make the map from the targets' lists and exports"
    )
    made.add_argument(
        "--list",
        action="append",
        nargs=3,
        required=True,
        metavar=("NAMES", "DLL", "EXPORTS"),
        help="a names list (one JSON row a name), its DLL, the exports' folder",
    )
    made.add_argument("--out", type=Path, default=MAP)
    show = commands.add_parser("show", help="print a log's rows with the names shown")
    show.add_argument("log", type=Path)
    show.add_argument("--map", type=Path, default=MAP)
    show.add_argument("--f", help="only the rows whose f is this address")
    show.add_argument("--start", help="a time or an event: the first row shown")
    show.add_argument("--end", help="a time or an event: the last row shown")
    args = parser.parse_args(argv)
    if args.command == "build":
        targets = [
            (dll, _built(Path(names), dll, Path(folder)))
            for names, dll, folder in args.list
        ]
        args.out.write_text(
            json.dumps(gather(targets), indent=2) + "\n", encoding="utf-8"
        )
        return 0
    reader = Reader(load(args.map))
    rows, skipped = _rows(args.log)
    shown = {id(line) for line in window(rows, args.start, args.end)}
    for line in rows:
        if id(line) in shown and (args.f is None or line.fields.get("f") == args.f):
            body = reader.render(line)
            print(f"{line.time} {line.level} {body}" if line.time else body)
        reader.see(line)
    if skipped:
        print(f"{skipped} lines not rows", file=sys.stderr)
    return 0


if __name__ == "__main__":
    sys.exit(main())
