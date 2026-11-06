// SPDX-License-Identifier: MIT
//! mercs's CD-audio row, as `cd_audio`'s front serves it: what the game
//! file's CD test reads back. One letter: the test fails if `10081975`'s number
//! is past the count `100461be` writes. 22 tracks: the count `10046304` compares.
use cd_audio::table::{Calls, Row};

/// The game file. The device number is what the game keeps at `100b04b0` and
/// hands back on every later call; it marks a closed device there with -1, so
/// the row's is neither that nor 0, the number of the calls before the open.
pub const GAME: Row = Row {
    file: "mercsw",
    calls: Calls {
        send_command: "WINMM!mciSendCommandA",
        aux_devices: "WINMM!auxGetNumDevs",
        error_string: "WINMM!mciGetErrorStringA",
        aux_caps: "WINMM!auxGetDevCapsA",
        aux_set_volume: "WINMM!auxSetVolume",
        aux_get_volume: "WINMM!auxGetVolume",
    },
    letters: 1,
    tracks: 22,
    device_name: "cdaudio",
    device_id: 1,
};
