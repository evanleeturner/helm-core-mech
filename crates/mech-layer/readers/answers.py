# SPDX-License-Identifier: MIT
"""What each call answered, from the swap's log (``imp.through``'s ``v`` and ``e``).

Per call label (``c``): its rows, its rows whose last error (``e``) is not 0, and the
first of them, with its caller (``from``) and time. Then ``GetAsyncKeyState``'s answers
by key (``a``): ``v`` is all of ``eax``, the answer its low 16 bits, so a key down is
bit 15 set. A row with no ``e`` (``imp.call``'s) counts as a row, never a failure.
Prints a summary, never the log.

    python3 answers.py <log> [--start TIME|EVENT] [--end TIME|EVENT]
"""

from __future__ import annotations

import argparse
import sys
from collections.abc import Iterable
from dataclasses import dataclass
from pathlib import Path

from lines import Line
from lines import read
from lines import window

KEY_STATE = "USER32!GetAsyncKeyState"
"""The label of the one call whose answers are read by key."""

DOWN = 1 << 15
"""``GetAsyncKeyState``'s answer bit for a key held down."""


@dataclass
class Call:
    """One label's rows: how many, how many left a last error, and the first that did."""

    rows: int = 0
    failed: int = 0
    first: Line | None = None


@dataclass
class Key:
    """One key's ``GetAsyncKeyState`` rows, by whether bit 15 was set."""

    down: int = 0
    up: int = 0


def answers(lines: Iterable[Line]) -> tuple[dict[str, Call], dict[int, Key]]:
    """Each label's :class:`Call` in first-seen order, and each key's :class:`Key`."""
    calls: dict[str, Call] = {}
    keys: dict[int, Key] = {}
    for line in lines:
        label = line.fields.get("c")
        if label is None:
            continue
        call = calls.setdefault(label, Call())
        call.rows += 1
        if line.number("e"):
            call.failed += 1
            if call.first is None:
                call.first = line
        value = line.number("v")
        if label == KEY_STATE and value is not None and line.numbers("a"):
            key = keys.setdefault(line.numbers("a")[0], Key())
            if value & 0xFFFF & DOWN:
                key.down += 1
            else:
                key.up += 1
    return calls, keys


def summary(calls: dict[str, Call], keys: dict[int, Key], skipped: int) -> list[str]:
    """The lines :func:`main` prints."""
    rows = sum(call.rows for call in calls.values())
    out = [f"rows with c: {rows}; lines not rows: {skipped}"]
    for label, call in calls.items():
        line = f"{label}: rows {call.rows}, last error not 0: {call.failed}"
        if call.first is not None:
            first = call.first.fields
            line += f", first e={first['e']} from={first.get('from', '')} at {call.first.time}"
        out.append(line)
    for key, seen in keys.items():
        out.append(f"{KEY_STATE} key {key:x}: down {seen.down}, up {seen.up}")
    return out


def main(argv: list[str] | None = None) -> int:
    """Read the log named on the command line and print its summary."""
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("log", type=Path)
    parser.add_argument("--start", help="a time HH:MM:SS[.mmm] or an event")
    parser.add_argument("--end", help="a time HH:MM:SS[.mmm] or an event")
    arguments = parser.parse_args(argv)
    rows, skipped = read(arguments.log)
    calls, keys = answers(window(rows, arguments.start, arguments.end))
    print("\n".join(summary(calls, keys, skipped)))
    return 0


if __name__ == "__main__":
    sys.exit(main())
