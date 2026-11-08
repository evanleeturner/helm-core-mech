# SPDX-License-Identifier: MIT
"""The files and samples a stretch of the swap's log touched (``imp.through``'s ``a``).

Each name ``CreateFileA`` opened (``p``) and the handles it answered (``v``); each
handle's writes and reads (``a[2]`` the bytes asked, ``v`` the answer, ``e`` the last
error), and its seeks (``a[1]`` the offset, ``a[3]`` the method, ``v`` the place); each
sample ``_AIL_end_sample`` ended (``a[0]``). A handle is named by the last open that
answered it: ``CloseHandle`` writes no ``a`` here, so a reused handle takes the new name.
The bytes a call moved lie behind ``a[3]``, a pointer, and are not in the row.

The stretch is ``--start`` and ``--end``, each a time or an event (the burst's
``mission.end``); the whole log without them. Prints a summary, never the log.

    python3 exit_files.py <log> [--start TIME|EVENT] [--end TIME|EVENT]
"""

from __future__ import annotations

import argparse
import sys
from collections.abc import Iterable
from dataclasses import dataclass
from dataclasses import field
from pathlib import Path

from lines import Line
from lines import read
from lines import window

OPEN = "KERNEL32!CreateFileA"
WRITE = "KERNEL32!WriteFile"
READ = "KERNEL32!ReadFile"
SEEK = "KERNEL32!SetFilePointer"
END_SAMPLE = "WAIL32!_AIL_end_sample@4"
"""The labels this reader reads, as the layer's rows spell them."""


@dataclass
class Name:
    """One file name: how often it was opened, and the handles the opens answered."""

    opened: int = 0
    handles: list[int] = field(default_factory=list)


@dataclass
class Handle:
    """One handle's calls: ``(bytes asked, v, e)`` per write and read, ``(offset, method,
    v)`` per seek."""

    name: str = ""
    writes: list[tuple[int, int, int]] = field(default_factory=list)
    reads: list[tuple[int, int, int]] = field(default_factory=list)
    seeks: list[tuple[int, int, int]] = field(default_factory=list)


@dataclass
class Files:
    """What the stretch touched: names, handles and samples, each in first-seen order."""

    rows: int = 0
    names: dict[str, Name] = field(default_factory=dict)
    handles: dict[int, Handle] = field(default_factory=dict)
    samples: dict[int, int] = field(default_factory=dict)


def _answer(line: Line) -> int:
    return line.number("v") or 0


def _error(line: Line) -> int:
    return line.number("e") or 0


def exit_files(lines: Iterable[Line]) -> Files:
    """The :class:`Files` the rows with ``a`` name."""
    files = Files()
    for line in lines:
        a = line.numbers("a")
        if not a:
            continue
        files.rows += 1
        label = line.fields.get("c")
        if label == OPEN:
            name = line.fields.get("p", "")
            opened = files.names.setdefault(name, Name())
            opened.opened += 1
            handle = _answer(line)
            if handle not in opened.handles:
                opened.handles.append(handle)
            files.handles.setdefault(handle, Handle()).name = name
        elif label in (WRITE, READ) and len(a) >= 3:
            handle = files.handles.setdefault(a[0], Handle())
            moves = handle.writes if label == WRITE else handle.reads
            moves.append((a[2], _answer(line), _error(line)))
        elif label == SEEK and len(a) >= 4:
            handle = files.handles.setdefault(a[0], Handle())
            handle.seeks.append((a[1], a[3], _answer(line)))
        elif label == END_SAMPLE:
            files.samples[a[0]] = files.samples.get(a[0], 0) + 1
    return files


def _moves(kind: str, moves: list[tuple[int, int, int]]) -> str:
    failed = sum(1 for _, answer, _ in moves if answer == 0)
    asked = sum(size for size, _, _ in moves)
    return f"{kind} {len(moves)} ({asked:x} bytes asked, {failed} failed)"


def summary(files: Files, skipped: int) -> list[str]:
    """The lines :func:`main` prints."""
    out = [f"rows with a: {files.rows}; lines not rows: {skipped}"]
    for name, opened in files.names.items():
        handles = ",".join(f"{handle:x}" for handle in opened.handles)
        out.append(f"opened {name}: {opened.opened} times, handles {handles}")
    for number, handle in files.handles.items():
        if not (handle.writes or handle.reads or handle.seeks):
            continue
        seeks = ",".join(
            f"{offset:x}/{method}->{at:x}" for offset, method, at in handle.seeks
        )
        out.append(
            f"handle {number:x} ({handle.name}): {_moves('writes', handle.writes)}, "
            f"{_moves('reads', handle.reads)}, seeks {len(handle.seeks)} {seeks}".rstrip()
        )
    ended = ", ".join(f"{sample:x} x{count}" for sample, count in files.samples.items())
    out.append(f"samples ended: {sum(files.samples.values())} {ended}".rstrip())
    return out


def main(argv: list[str] | None = None) -> int:
    """Read the log named on the command line and print its summary."""
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("log", type=Path)
    parser.add_argument("--start", help="a time HH:MM:SS[.mmm] or an event")
    parser.add_argument("--end", help="a time HH:MM:SS[.mmm] or an event")
    arguments = parser.parse_args(argv)
    rows, skipped = read(arguments.log)
    files = exit_files(window(rows, arguments.start, arguments.end))
    print("\n".join(summary(files, skipped)))
    return 0


if __name__ == "__main__":
    sys.exit(main())
