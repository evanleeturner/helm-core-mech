#!/usr/bin/env python3
# SPDX-License-Identifier: MIT
"""Each press of a run beside the `input.key` rows it drew in the game's log.

Purpose: a run's press list holds one line a press, as the flight's press tool prints it:
`press keysym=<name> keycode=<n> down=<epoch s> up=<epoch s> ...`; any other line is counted
and passed over. The game's log is the logline grammar, fmt 1: a header line
`= <program> run <id> fmt 1 date <YYYY-MM-DD> tz utc`, a second `= job ...` line, then one row
a line, `HH:MM:SS.mmm L event key=value ...`, the time of day in UTC; a row earlier than the
one before it is taken as the next day. For each press this counts the `input.key` rows from
the key's down to 3 s past its up by their `act=` codes. `fun_10070381` is handed key-ups
alone, so a press's row comes at its up.

Which `act=` codes are steering is unread. A caller who knows names them with `--steering`;
each other code in a press's window is then flagged. `none` (nothing queued) is no act and is
never flagged. With no `--steering`, nothing is flagged and the summary says so: a code is
written as the log wrote it, never as a guessed word. It prints a summary, never a log line.

Call: key_story.py <press list> <game log> [--steering CODE,CODE...]
Exit: 0 read; 1 a file unreadable, or a log not in grammar fmt 1; 2 usage.
"""

import collections
import datetime
import pathlib
import sys

# Rows from a key's down to this many seconds past its up are the press's.
AFTER_UP_S = 3.0

Press = collections.namedtuple("Press", "number keysym down up")
Row = collections.namedtuple("Row", "at event fields")


class Refused(Exception):
    """A file this reader will not read, with the reason as its text."""


def fields(text):
    """The `key=value` pairs of a row's text past its event, a quoted value unescaped."""
    pairs = {}
    rest = text.strip()
    while rest:
        key, sep, rest = rest.partition("=")
        if not sep:
            break
        if rest.startswith('"'):
            value, at = [], 1
            while at < len(rest) and rest[at] != '"':
                if rest[at] == "\\" and at + 1 < len(rest):
                    at += 1
                value.append(rest[at])
                at += 1
            pairs[key] = "".join(value)
            rest = rest[at + 1 :].lstrip()
        else:
            value, _, rest = rest.partition(" ")
            pairs[key] = value
            rest = rest.lstrip()
    return pairs


def read_presses(lines):
    """The presses in a press list's lines, numbered from 1, and the count of other lines."""
    presses, others = [], 0
    for line in lines:
        words = line.split()
        if not words or words[0] != "press":
            others += 1
            continue
        pairs = fields(" ".join(words[1:]))
        try:
            down, up = float(pairs["down"]), float(pairs["up"])
        except (KeyError, ValueError):
            others += 1
            continue
        presses.append(Press(len(presses) + 1, pairs.get("keysym", "?"), down, up))
    return presses, others


def read_log(lines):
    """A log's rows, each with its epoch time, and the header's date; refuses a log not fmt 1."""
    lines = iter(lines)
    head = next(lines, "").split()
    if head[:1] != ["="] or "fmt" not in head or "date" not in head:
        raise Refused("no logline header on the first line")
    grammar = head[head.index("fmt") + 1] if head.index("fmt") + 1 < len(head) else ""
    if grammar != "1":
        raise Refused(f"grammar fmt {grammar or '?'}, not fmt 1")
    try:
        date = datetime.date.fromisoformat(head[head.index("date") + 1])
    except (IndexError, ValueError):
        raise Refused("the header's date is not YYYY-MM-DD") from None
    midnight = datetime.datetime(date.year, date.month, date.day, tzinfo=datetime.UTC).timestamp()
    rows, day, last = [], 0, -1.0
    for line in lines:
        if line.startswith("="):
            continue
        words = line.split(" ", 3)
        if len(words) < 3:
            continue
        try:
            hours, minutes, seconds = words[0].split(":")
            clock = int(hours) * 3600 + int(minutes) * 60 + float(seconds)
        except ValueError:
            continue
        if clock < last:
            day += 1
        last = clock
        text = words[3] if len(words) > 3 else ""
        rows.append(Row(midnight + day * 86_400 + clock, words[2], fields(text)))
    return rows, date


def story(presses, rows, steering=None):
    """Each press's `input.key` act counts and flagged codes, then the rows outside every press."""
    keys = [row for row in rows if row.event == "input.key"]
    told, claimed = [], set()
    for press in presses:
        inside = [
            at for at, row in enumerate(keys) if press.down <= row.at <= press.up + AFTER_UP_S
        ]
        claimed.update(inside)
        acts = collections.Counter(keys[at].fields.get("act", "?") for at in inside)
        flagged = None
        if steering is not None:
            flagged = collections.Counter(
                {act: n for act, n in acts.items() if act != "none" and act not in steering}
            )
        told.append((press, len(inside), acts, flagged))
    outside = collections.Counter(
        row.fields.get("act", "?") for at, row in enumerate(keys) if at not in claimed
    )
    return told, outside, len(keys)


def counted(counter):
    """A counter as `code xN` words, in the order first seen, or `none seen`."""
    return ", ".join(f"{act} x{n}" for act, n in counter.items()) or "none seen"


def main(argv):
    """Route the call, read both files and print the summary."""
    args, steering = list(argv[1:]), None
    if "--steering" in args:
        at = args.index("--steering")
        if at + 1 >= len(args):
            print(__doc__, file=sys.stderr)
            return 2
        steering = {code for code in args[at + 1].split(",") if code}
        del args[at : at + 2]
    if len(args) != 2:
        print(__doc__, file=sys.stderr)
        return 2
    try:
        press_lines = pathlib.Path(args[0]).read_text(errors="replace").splitlines()
        log_lines = pathlib.Path(args[1]).read_text(errors="replace").splitlines()
        presses, others = read_presses(press_lines)
        rows, date = read_log(log_lines)
    except (OSError, Refused) as error:
        print(f"refused: {error}", file=sys.stderr)
        return 1
    told, outside, keys = story(presses, rows, steering)
    print(
        f"key_story: {len(presses)} presses, {others} other lines in the press list; "
        f"log of {date.isoformat()}, {len(rows)} rows, {keys} input.key"
    )
    for press, n, acts, flagged in told:
        line = f"press {press.number} {press.keysym} held {press.up - press.down:.2f} s: "
        line += f"{n} input.key, acts {counted(acts)}"
        if flagged is not None:
            line += f"; flagged {counted(flagged)}"
        print(line)
    print(f"outside every press: {sum(outside.values())} input.key, acts {counted(outside)}")
    if steering is None:
        print("steering codes unread: no --steering given, so no act is flagged")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
