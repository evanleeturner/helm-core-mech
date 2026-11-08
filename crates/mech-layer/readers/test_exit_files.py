# SPDX-License-Identifier: MIT
"""``exit_files.py`` on the Wine test's own rows, ``harvest/through.log``.

Each expected value is that file's, as the layer wrote it under Wine
(``tests/through.rs``), never worked out by hand.
"""

from pathlib import Path

from exit_files import exit_files
from exit_files import main
from lines import read
from lines import window

FIXTURE = Path(__file__).resolve().parent.parent / "harvest" / "through.log"
MISSING = "C:\\through-test\\missing.bin"
MADE = "C:\\through-test.bin"


def test_names_and_handles():
    files = exit_files(read(FIXTURE)[0])
    assert files.rows == 11
    assert {
        name: (seen.opened, seen.handles) for name, seen in files.names.items()
    } == {
        MISSING: (5, [0xFFFFFFFF]),
        MADE: (1, [0x3C]),
    }


def test_a_handles_writes_reads_and_seeks():
    handle = exit_files(read(FIXTURE)[0]).handles[0x3C]
    assert handle.name == MADE
    assert handle.writes == [(0xB, 1, 0)]
    assert handle.reads == [(5, 1, 0)]
    assert handle.seeks == [(2, 0, 2)]


def test_samples_ended():
    assert exit_files(read(FIXTURE)[0]).samples == {0x1234560: 1}


def test_window_by_time():
    files = exit_files(window(read(FIXTURE)[0], None, "23:15:39.232"))
    assert files.rows == 5
    assert files.samples == {}
    files = exit_files(window(read(FIXTURE)[0], "23:15:39.307", None))
    assert files.names[MISSING].opened == 4
    assert MADE not in files.names


def test_prints_a_summary_never_the_log(capsys):
    assert main([str(FIXTURE)]) == 0
    printed = capsys.readouterr().out.splitlines()
    assert not any("imp.through" in line for line in printed)
    assert printed[0] == "rows with a: 11; lines not rows: 0"
    assert (
        f"handle 3c ({MADE}): writes 1 (b bytes asked, 0 failed), "
        "reads 1 (5 bytes asked, 0 failed), seeks 1 2/0->2"
    ) in printed
    assert printed[-1] == "samples ended: 1 1234560 x1"
