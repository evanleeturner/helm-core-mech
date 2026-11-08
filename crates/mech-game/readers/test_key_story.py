# SPDX-License-Identifier: MIT
"""`key_story.py` against the fixture `harvest/input-key.log`.

The fixture is the rows `tests/input_key.rs` wrote under Wine, one body a line, harvested by
`MECH_GAME_KEY_LOG`: the port's own `fn.in`, `mem.wr` and `input.key` rows. Each test gives those
bodies times and a press list of its own; the act codes expected are read from the fixture's
rows, never written here by hand.

Call: python3 -m pytest -p no:cacheprovider crates/mech-game/readers
"""

import pathlib
import sys

sys.dont_write_bytecode = True
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))

import key_story  # noqa: E402  the bytecode line must come first: no cache in the tree

FIXTURE = pathlib.Path(__file__).resolve().parent.parent / "harvest" / "input-key.log"
# 2026-11-08 12:00:00 UTC, the test log's first row; any day serves.
NOON = 1_794_139_200.0
HEADER = [
    "= key_story_test run 0-0 fmt 1 date 2026-11-08 tz utc",
    "= job /job base /base cwd /cwd",
]


def bodies():
    """The fixture's rows, one body a line."""
    return [line for line in FIXTURE.read_text().splitlines() if line]


def acts_of(lines):
    """The `act=` of each `input.key` body, read by a plain split, in order."""
    found = []
    for line in lines:
        words = line.split(" ")
        if words[0] == "input.key":
            found += [word[len("act=") :] for word in words[1:] if word.startswith("act=")]
    return found


def clock(at):
    """An epoch time as the grammar's `HH:MM:SS.mmm`, in UTC."""
    seconds = at % 86_400
    return f"{int(seconds // 3600):02d}:{int(seconds % 3600 // 60):02d}:{seconds % 60:06.3f}"


def log(timed):
    """A log in grammar fmt 1 of (epoch time, body) rows."""
    return HEADER + [f"{clock(at)} D {body}" for at, body in timed]


def press(down, up, keysym="Left"):
    """A press list's line, as the flight's press tool prints it."""
    return f"press keysym={keysym} keycode=113 down={down:.3f} up={up:.3f} local=x utc=y"


def calls():
    """The fixture's rows cut into calls: each ends at its `input.key`, the call's last row."""
    cut, call = [], []
    for body in bodies():
        call.append(body)
        if body.split(" ")[0] == "input.key":
            cut.append(call)
            call = []
    return cut


def test_the_fixture_holds_both_branches():
    acts = acts_of(bodies())
    assert len(acts) == len(calls()) == 4
    assert "none" in acts and any(act != "none" for act in acts)


def test_each_press_counts_the_rows_from_its_down_to_3_s_past_its_up():
    timed, presses = [], []
    for number, call in enumerate(calls()):
        down = NOON + 10 * number
        presses.append(press(down, down + 1))
        timed += [(down + 1.5, body) for body in call]
    rows, _ = key_story.read_log(log(timed))
    told, outside, keys = key_story.story(key_story.read_presses(presses)[0], rows)
    assert keys == 4 and not outside
    for (_, n, acts, flagged), call in zip(told, calls(), strict=True):
        assert n == 1 and list(acts) == acts_of(call) and flagged is None


def test_a_row_past_3_s_after_the_up_is_outside_every_press():
    call = calls()[0]
    timed = [(NOON + 1 + key_story.AFTER_UP_S + 0.5, body) for body in call]
    rows, _ = key_story.read_log(log(timed))
    told, outside, _ = key_story.story(key_story.read_presses([press(NOON, NOON + 1)])[0], rows)
    assert told[0][1] == 0
    assert list(outside.elements()) == acts_of(call)


def test_steering_flags_each_other_code_and_never_none():
    acts = acts_of(bodies())
    steer = next(act for act in acts if act != "none")
    timed = [(NOON + 0.5, body) for body in bodies()]
    rows, _ = key_story.read_log(log(timed))
    told, _, _ = key_story.story(key_story.read_presses([press(NOON, NOON + 1)])[0], rows, {steer})
    flagged = told[0][3]
    assert set(flagged) == {act for act in acts if act not in ("none", steer)}
    assert "none" not in flagged and steer not in flagged


def test_a_row_earlier_than_the_one_before_is_the_next_day():
    call = calls()[0]
    late = NOON + 12 * 3600 - 1  # 23:59:59 UTC
    timed = [(late, call[0])] + [(late + 2, body) for body in call[1:]]
    rows, _ = key_story.read_log(log(timed))
    assert [row.at for row in rows] == [at for at, _ in timed]


def test_other_lines_in_the_press_list_are_counted_and_passed_over():
    lines = [press(NOON, NOON + 1), "no display :9", "press keysym=Left down=x up=y"]
    presses, others = key_story.read_presses(lines)
    assert [p.number for p in presses] == [1] and others == 2


def test_a_quoted_value_is_unescaped():
    assert key_story.fields('a=1 b="x \\"y\\" z" c=3') == {"a": "1", "b": 'x "y" z', "c": "3"}


def test_a_log_without_the_header_is_refused(tmp_path):
    (tmp_path / "presses.txt").write_text(press(NOON, NOON + 1) + "\n")
    (tmp_path / "game.log").write_text("\n".join(f"12:00:00.000 D {b}" for b in bodies()))
    argv = ["key_story.py", str(tmp_path / "presses.txt"), str(tmp_path / "game.log")]
    assert key_story.main(argv) == 1


def test_the_summary_prints_no_log_line(tmp_path, capsys):
    timed = [(NOON + 0.5, body) for body in bodies()]
    (tmp_path / "presses.txt").write_text(press(NOON, NOON + 1) + "\n")
    (tmp_path / "game.log").write_text("\n".join(log(timed)) + "\n")
    argv = ["key_story.py", str(tmp_path / "presses.txt"), str(tmp_path / "game.log")]
    assert key_story.main(argv) == 0
    printed = capsys.readouterr().out
    assert not any(body in printed for body in bodies())
    assert "steering codes unread" in printed
