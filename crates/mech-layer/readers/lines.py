# SPDX-License-Identifier: MIT
"""The swap's log lines, split for the layer's readers.

A line is ``HH:MM:SS.mmm L event k=v ... tid=<hex>``: the time, the level's first
letter, the event, then fields. A value that is empty or holds a space, ``"`` or
``=`` is double-quoted, ``\\`` and ``"`` escaped by ``\\`` (``oxide-logline``'s
rule), so the fields are split by that rule, never by spaces alone: a game path
may hold a space. Numbers are lowercase hex, no prefix.

A line not of that form (a header, Wine's own words) is no row: the readers
count it and go on.
"""

from __future__ import annotations

import re
from collections.abc import Iterable
from collections.abc import Iterator
from dataclasses import dataclass
from pathlib import Path

_HEAD = re.compile(r"(\d\d:\d\d:\d\d\.\d{3}) ([A-Z]) (\S+)(?: (.*))?\Z")
_TIME = re.compile(r"\d\d:\d\d:\d\d(\.\d{3})?\Z")


@dataclass(frozen=True)
class Line:
    """One row: its time, level letter, event, and fields in the order written."""

    time: str
    level: str
    event: str
    fields: dict[str, str]

    def number(self, key: str) -> int | None:
        """Field ``key`` as a hex number; ``None`` when the row has none or it is empty."""
        value = self.fields.get(key)
        return int(value, 16) if value else None

    def numbers(self, key: str) -> list[int]:
        """Field ``key`` as comma-joined hex numbers, ``a``'s form; empty when absent."""
        value = self.fields.get(key)
        return [int(word, 16) for word in value.split(",")] if value else []


def fields(text: str) -> dict[str, str]:
    """The ``k=v`` words after the event, a quoted value unescaped.

    Raises ``ValueError`` on a word with no ``=`` or a quote left open.
    """
    found: dict[str, str] = {}
    index = 0
    while index < len(text):
        if text[index] == " ":
            index += 1
            continue
        equals = text.find("=", index)
        space = text.find(" ", index)
        if equals < 0 or 0 <= space < equals:
            raise ValueError(f"not key=value at {index}")
        key = text[index:equals]
        index = equals + 1
        if text.startswith('"', index):
            index += 1
            value = []
            while True:
                if index >= len(text):
                    raise ValueError(f"a quote left open in {key}")
                char = text[index]
                if char == '"':
                    index += 1
                    break
                if char == "\\":
                    index += 1
                    if index >= len(text):
                        raise ValueError(f"a quote left open in {key}")
                    char = text[index]
                value.append(char)
                index += 1
            found[key] = "".join(value)
        else:
            end = text.find(" ", index)
            end = len(text) if end < 0 else end
            found[key] = text[index:end]
            index = end
    return found


def parse(text: str) -> Line | None:
    """``text`` as a row, or ``None`` when it is not one."""
    head = _HEAD.match(text.rstrip("\r\n"))
    if head is None:
        return None
    time, level, event, rest = head.groups()
    try:
        return Line(time, level, event, fields(rest or ""))
    except ValueError:
        return None


def read(path: Path) -> tuple[list[Line], int]:
    """Every row of the log at ``path``, and the count of its lines that are not rows."""
    rows: list[Line] = []
    skipped = 0
    with path.open(encoding="utf-8", errors="replace") as log:
        for text in log:
            line = parse(text)
            if line is None:
                skipped += 1
            else:
                rows.append(line)
    return rows, skipped


def _reached(line: Line, bound: str, *, past: bool) -> bool:
    """``line`` meets ``bound``: at or past a time (``past``: beyond it), or that event."""
    if _TIME.match(bound):
        stamp = line.time[: len(bound)]
        return stamp > bound if past else stamp >= bound
    return line.event == bound


def window(lines: Iterable[Line], start: str | None, end: str | None) -> Iterator[Line]:
    """The rows from the first ``start`` names through the last before ``end`` is passed.

    Each bound is a time (``HH:MM:SS``, or with ``.mmm``) or an event name; ``None``
    is the log's own edge. An ``end`` event's row is kept; a row past an ``end``
    time is not. Times compare as written, so a stretch across midnight reads wrong.
    """
    started = start is None
    for line in lines:
        if not started:
            started = _reached(line, start, past=False)
            if not started:
                continue
        if end is not None and _TIME.match(end) and _reached(line, end, past=True):
            return
        yield line
        if end is not None and not _TIME.match(end) and line.event == end:
            return
