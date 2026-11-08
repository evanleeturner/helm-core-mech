# SPDX-License-Identifier: MIT
"""``answers.py`` on the Wine test's own rows, ``harvest/through.log``.

Each expected value is that file's, as the layer wrote it under Wine
(``tests/through.rs``), never worked out by hand.
"""

from pathlib import Path

from answers import answers
from answers import Key
from answers import KEY_STATE
from answers import main
from lines import read
from lines import window

FIXTURE = Path(__file__).resolve().parent.parent / "harvest" / "through.log"


def test_each_label_counted():
    rows, skipped = read(FIXTURE)
    calls, _ = answers(rows)
    assert skipped == 0
    assert {label: call.rows for label, call in calls.items()} == {
        "KERNEL32!CreateFileA": 6,
        "KERNEL32!WriteFile": 1,
        "KERNEL32!SetFilePointer": 1,
        "KERNEL32!ReadFile": 1,
        "USER32!GetAsyncKeyState": 1,
        "TEST!outer@12": 3,
        "TEST!mixed": 1,
        "TEST!wide@4": 1,
        "WAIL32!_AIL_end_sample@4": 1,
    }


def test_first_failure_and_its_caller():
    calls, _ = answers(read(FIXTURE)[0])
    create = calls["KERNEL32!CreateFileA"]
    assert create.failed == 5
    assert create.first is not None
    assert (
        create.first.time,
        create.first.fields["e"],
        create.first.fields["from"],
    ) == (
        "23:15:39.226",
        "3",
        "40a12b",
    )
    assert calls["TEST!mixed"].failed == 1
    assert calls["TEST!mixed"].first.fields["e"] == "2a"
    assert calls["KERNEL32!ReadFile"].failed == 0
    assert calls["KERNEL32!ReadFile"].first is None


def test_key_state_by_key():
    _, keys = answers(read(FIXTURE)[0])
    assert keys == {0x10: Key(down=0, up=1)}
    assert KEY_STATE == "USER32!GetAsyncKeyState"


def test_window_by_time():
    calls, _ = answers(window(read(FIXTURE)[0], "23:15:39.307", None))
    assert calls["KERNEL32!CreateFileA"].rows == 4
    assert "KERNEL32!WriteFile" not in calls


def test_prints_a_summary_never_the_log(capsys):
    assert main([str(FIXTURE)]) == 0
    printed = capsys.readouterr().out
    assert "imp.through" not in printed
    assert printed.splitlines()[0] == "rows with c: 16; lines not rows: 0"
    assert (
        "KERNEL32!CreateFileA: rows 6, last error not 0: 5, first e=3 from=40a12b at 23:15:39.226"
        in printed.splitlines()
    )
