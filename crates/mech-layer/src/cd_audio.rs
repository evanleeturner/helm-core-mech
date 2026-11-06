// SPDX-License-Identifier: MIT
//! The game file's CD-audio calls through the front: `cd_audio`'s
//! answers over the game's own records, no disc read and no sound. Exported
//! `GAME_<name>`, a name the loader's `HC6_` match cannot take: `mciSendCommandA`,
//! the game's slot `1021a5dc`; `auxGetNumDevs`, `1021a5ec`; and the 4 names that
//! stop by name: `mciGetErrorStringA`, `1021a5d8`; `auxGetDevCapsA`, `1021a5e4`;
//! `auxSetVolume`, `1021a5d4`; `auxGetVolume`, its slot unread here.
//!
//! A stop ends the program: the game treats any failure of its CD test as fatal,
//! so a guessed answer would only move the failure somewhere with no name on it.
use ::cd_audio::{Back, Front, Record};
use core::ptr::copy_nonoverlapping;
use mech_rows::cd_audio::GAME;

unsafe extern "system" {
    fn ExitProcess(code: u32) -> !;
}

/// What a stop leaves as the program's exit code.
const STOPPED: u32 = 1;

/// Ends the program under Windows, and keeps the thread's last error across a line.
struct Exit;

impl Back for Exit {
    fn stop(&self, _name: &str) -> ! {
        log::logger().flush();
        // SAFETY: ends the program; touches no memory.
        unsafe { ExitProcess(STOPPED) }
    }

    fn quiet(&self, line: impl FnOnce()) {
        // SAFETY: reads the thread's own last error.
        let error = unsafe { crate::GetLastError() };
        line();
        // SAFETY: sets back the value read above.
        unsafe { crate::SetLastError(error) };
    }
}

/// The record the game handed: its 32-bit words, where the game keeps them.
struct Words(*mut u32);

impl Record for Words {
    fn word(&self, index: usize) -> u32 {
        // SAFETY: the game built this record for this message, and the front
        // asks only for words the message's record holds.
        unsafe { self.0.add(index).read_unaligned() }
    }

    fn set_word(&mut self, index: usize, value: u32) {
        // SAFETY: as `word`; the word is the game's to have written.
        unsafe { self.0.add(index).write_unaligned(value) };
    }

    fn write_through(&mut self, index: usize, chunks: &[&[u8]]) {
        let mut to = self.word(index) as usize as *mut u8;
        for chunk in chunks {
            // SAFETY: the word is the game's return pointer, and the front
            // checked the record's size against these bytes before it wrote.
            unsafe {
                copy_nonoverlapping(chunk.as_ptr(), to, chunk.len());
                to = to.add(chunk.len());
            }
        }
    }
}

static FRONT: Front<Exit> = Front::new(&GAME, Exit);

#[unsafe(export_name = "GAME_mciSendCommandA")]
unsafe extern "system" fn mci_send_command(
    device: u32,
    message: u32,
    flags: u32,
    record: *mut u32,
) -> u32 {
    FRONT.send(device, message, flags, &mut Words(record))
}

#[unsafe(export_name = "GAME_auxGetNumDevs")]
unsafe extern "system" fn aux_get_num_devs() -> u32 {
    FRONT.aux_devices()
}

#[unsafe(export_name = "GAME_mciGetErrorStringA")]
unsafe extern "system" fn mci_get_error_string(code: u32, _text: *mut u8, size: u32) -> i32 {
    FRONT.error_string(code, size)
}

#[unsafe(export_name = "GAME_auxGetDevCapsA")]
unsafe extern "system" fn aux_get_dev_caps(device: u32, _caps: *mut u8, size: u32) -> u32 {
    FRONT.aux_caps(device, size)
}

#[unsafe(export_name = "GAME_auxSetVolume")]
unsafe extern "system" fn aux_set_volume(device: u32, volume: u32) -> u32 {
    FRONT.aux_set_volume(device, volume)
}

#[unsafe(export_name = "GAME_auxGetVolume")]
unsafe extern "system" fn aux_get_volume(device: u32, _volume: *mut u32) -> u32 {
    FRONT.aux_get_volume(device)
}
