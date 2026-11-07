// SPDX-License-Identifier: MIT
//! The 157 import slots of the 3 files that had no row:
//! launcher 20, shell 50, game 87. Each row calls the original and writes one
//! `imp.call` line, `by=orig`: a pass-through, so a reader can say which slots a run reached. Nothing
//! here answers for the original. Written once by a script from the measured counts.
//!
//! Arguments are positional and a row never reads them, so their names say nothing. A void import's
//! `v` is the `eax` it left, as [`crate::smack`] says. Only the counts are measured. A row returning
//! `u32` hands back `eax` alone: no import here is known to answer in `edx` too, from its signature,
//! not from a flight.
//!
//! **The game's 42 `WAIL32` and 2 `DPLAY` rows never link their imports.** The rig routes a name by
//! the shell's imports, so `_AIL_serve@0` would reach the shell's `MSS32`, the wrong library for the
//! game, and nothing refuses that; a load-time import would also load and pin `WAIL32` at the swap's
//! start. These rows ask `GetProcAddress` once and hold the answer until [`forget`]. Whether the game
//! ever frees `WAIL32` is not measured.
//!
//! The export is the entry's symbol, stdcall-decorated, `_GAME_AIL_serve@0`: `entry!` names a symbol
//! plainly, which `_AIL_serve@0` does not match at the rig's `layer()`.
#![allow(clashing_extern_declarations)]

use core::ffi::CStr;
use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};

use crate::left;
use crate::window::Numbers;

unsafe extern "system" {
    fn GetModuleHandleA(name: u32) -> u32;
    fn GetProcAddress(module: u32, name: u32) -> u32;
}

/// Raised by [`forget`]; an answer held under an older one is asked again.
static GENERATION: AtomicU32 = AtomicU32::new(1);

/// A library may have gone, so every held address is stale. `process.rs`'s `FreeLibrary` row calls
/// this, so a second load of the game resolves again.
pub fn forget() {
    GENERATION.fetch_add(1, Ordering::Relaxed);
}

/// The address `file` gives for `what`, a name's address or a by-number import's number; 0 if it gives
/// none. A 0 is not held, so the next call asks again. Two threads asking at once both get the same
/// answer, so the `store` needs no order beyond its own 64 bits.
fn find(held: &AtomicU64, file: &CStr, what: u32) -> u32 {
    let now = GENERATION.load(Ordering::Relaxed);
    let word = held.load(Ordering::Relaxed);
    if (word >> 32) as u32 == now {
        return word as u32;
    }
    // SAFETY: `file` is a C string that outlives the call.
    let module = unsafe { GetModuleHandleA(file.as_ptr().expose_provenance() as u32) };
    if module == 0 {
        return 0;
    }
    // SAFETY: `module` was just answered, `what` a C string's address or a number below 0x10000.
    let found = unsafe { GetProcAddress(module, what) };
    if found != 0 {
        held.store(u64::from(now) << 32 | u64::from(found), Ordering::Relaxed);
    }
    found
}

/// One row's line, after the call. `args` empty leaves `a` out, as the catalog says.
fn line(c: &'static str, args: &[u32], value: u32, error: u32, from: u32) {
    if args.is_empty() {
        kept!(log::debug!(c = c, v:% = mech_trace::__shown!(value),
            e:% = mech_trace::__shown!(error), from:% = mech_trace::__shown!(from),
            by = "orig"; "imp.call"));
    } else {
        kept!(
            log::debug!(c = c, a:% = Numbers(args), v:% = mech_trace::__shown!(value),
            e:% = mech_trace::__shown!(error), from:% = mech_trace::__shown!(from),
            by = "orig"; "imp.call")
        );
    }
}

/// A resolved row's library gave no address: the row answers 0 and says so, level W.
fn unresolved(c: &'static str, from: u32) {
    kept!(
        log::warn!(c = c, from:% = mech_trace::__shown!(from), by = "mech-layer"; "slot.unresolved")
    );
}

/// The naked export: pushes the caller's arguments again, then the address the call returns to, so
/// `row` takes the caller first and pops what it was pushed, as `entry!` does. `row`'s answer stays
/// in `eax`, and the entry returns past the caller's arguments.
macro_rules! export {
    ($file:ident, $name:ident, $($a:ident)*) => {
        #[unsafe(naked)]
        #[unsafe(export_name = concat!(stringify!($file), "_", stringify!($name)))]
        unsafe extern "system" fn entry($($a: u32),*) -> u32 {
            core::arch::naked_asm!(
                ".rept {pushes}",
                "push dword ptr [esp + {bytes}]",
                ".endr",
                "call {row}",
                "ret {bytes}",
                pushes = const <[&str]>::len(&[$(stringify!($a)),*]) + 1,
                bytes = const 4 * <[&str]>::len(&[$(stringify!($a)),*]),
                row = sym row,
            )
        }
    };
}

/// A row whose import links: `$name` decorates to the import's own symbol, so a wrong count is
/// refused at the link.
macro_rules! linked {
    ($file:ident $c:literal $name:ident $($a:ident)*) => {
        const _: () = {
            unsafe extern "system" {
                fn $name($($a: u32),*) -> u32;
            }

            extern "system" fn row(from: u32, $($a: u32),*) -> u32 {
                // SAFETY: the file's own call, its arguments passed on unchanged to the import its
                // slot named, through the swap DLL's own table.
                let value = unsafe { $name($($a),*) };
                let error = left();
                line($c, &[$($a),*], value, error, from);
                value
            }

            export!($file, $name, $($a)*);
        };
    };
}

/// A game row asking its library at run time; see the head.
macro_rules! resolved {
    ($name:ident, $library:expr, $what:expr, $c:expr, $($a:ident)*) => {
        const _: () = {
            static HELD: AtomicU64 = AtomicU64::new(0);

            extern "system" fn row(from: u32, $($a: u32),*) -> u32 {
                let address = find(&HELD, $library, $what);
                if address == 0 {
                    unresolved($c, from);
                    return 0;
                }
                // SAFETY: the address the game's own library gave for this import, its count the
                // one measured; the game's arguments passed on unchanged.
                let value = unsafe {
                    let real: unsafe extern "system" fn($($a: u32),*) -> u32 =
                        core::mem::transmute(address);
                    real($($a),*)
                };
                let error = left();
                line($c, &[$($a),*], value, error, from);
                value
            }

            export!(GAME, $name, $($a)*);
        };
    };
}

/// A game row of `WAIL32`, by the import's decorated name.
macro_rules! wail {
    ($import:literal $name:ident $($a:ident)*) => {
        resolved!(
            $name,
            c"WAIL32.DLL",
            concat!($import, "\0").as_ptr().expose_provenance() as u32,
            concat!("WAIL32!", $import),
            $($a)*
        );
    };
}

// The shell, 50 rows.
linked!(HC6 "KERNEL32!CreateDirectoryA" CreateDirectoryA a b);
linked!(HC6 "KERNEL32!FlushFileBuffers" FlushFileBuffers a);
linked!(HC6 "KERNEL32!GetDriveTypeA" GetDriveTypeA a);
linked!(HC6 "KERNEL32!GetFileAttributesA" GetFileAttributesA a);
linked!(HC6 "KERNEL32!GetLogicalDriveStringsA" GetLogicalDriveStringsA a b);
linked!(HC6 "KERNEL32!GetStdHandle" GetStdHandle a);
linked!(HC6 "KERNEL32!GetWindowsDirectoryA" GetWindowsDirectoryA a b);
linked!(HC6 "KERNEL32!HeapCreate" HeapCreate a b c);
linked!(HC6 "KERNEL32!HeapDestroy" HeapDestroy a);
linked!(HC6 "KERNEL32!HeapSize" HeapSize a b c);
linked!(HC6 "KERNEL32!QueryPerformanceCounter" QueryPerformanceCounter a);
linked!(HC6 "KERNEL32!SetEndOfFile" SetEndOfFile a);
linked!(HC6 "KERNEL32!SetHandleCount" SetHandleCount a);
linked!(HC6 "KERNEL32!SetStdHandle" SetStdHandle a b);
linked!(HC6 "KERNEL32!Sleep" Sleep a);
linked!(HC6 "MSS32!_AIL_active_sample_count@4" AIL_active_sample_count a);
linked!(HC6 "MSS32!_AIL_active_sequence_count@4" AIL_active_sequence_count a);
linked!(HC6 "MSS32!_AIL_allocate_sequence_handle@4" AIL_allocate_sequence_handle a);
linked!(HC6 "MSS32!_AIL_end_sample@4" AIL_end_sample a);
linked!(HC6 "MSS32!_AIL_init_sequence@12" AIL_init_sequence a b c);
linked!(HC6 "MSS32!_AIL_load_sample_buffer@16" AIL_load_sample_buffer a b c d);
linked!(HC6 "MSS32!_AIL_midiOutOpen@12" AIL_midiOutOpen a b c);
linked!(HC6 "MSS32!_AIL_minimum_sample_buffer_size@12" AIL_minimum_sample_buffer_size a b c);
linked!(HC6 "MSS32!_AIL_release_sequence_handle@4" AIL_release_sequence_handle a);
linked!(HC6 "MSS32!_AIL_sample_buffer_ready@4" AIL_sample_buffer_ready a);
linked!(HC6 "MSS32!_AIL_serve@0" AIL_serve);
linked!(HC6 "MSS32!_AIL_set_sequence_loop_count@8" AIL_set_sequence_loop_count a b);
linked!(HC6 "MSS32!_AIL_set_sequence_volume@12" AIL_set_sequence_volume a b c);
linked!(HC6 "MSS32!_AIL_shutdown@0" AIL_shutdown);
linked!(HC6 "MSS32!_AIL_start_sequence@4" AIL_start_sequence a);
linked!(HC6 "MSS32!_AIL_startup@0" AIL_startup);
linked!(HC6 "MSS32!_AIL_stop_sequence@4" AIL_stop_sequence a);
linked!(HC6 "MSS32!_AIL_waveOutClose@4" AIL_waveOutClose a);
linked!(HC6 "MSS32!_AIL_waveOutOpen@16" AIL_waveOutOpen a b c d);
linked!(HC6 "USER32!CheckDlgButton" CheckDlgButton a b c);
linked!(HC6 "USER32!CheckMenuItem" CheckMenuItem a b c);
linked!(HC6 "USER32!DialogBoxParamA" DialogBoxParamA a b c d e);
linked!(HC6 "USER32!DrawMenuBar" DrawMenuBar a);
linked!(HC6 "USER32!EnableMenuItem" EnableMenuItem a b c);
linked!(HC6 "USER32!EndDialog" EndDialog a b);
linked!(HC6 "USER32!GetDlgItem" GetDlgItem a b);
linked!(HC6 "USER32!IsDlgButtonChecked" IsDlgButtonChecked a b);
linked!(HC6 "USER32!LoadMenuA" LoadMenuA a b);
linked!(HC6 "USER32!MessageBoxA" MessageBoxA a b c d);
linked!(HC6 "USER32!SetDlgItemTextA" SetDlgItemTextA a b c);
linked!(HC6 "USER32!SetMenu" SetMenu a b);
linked!(HC6 "USER32!WinHelpA" WinHelpA a b c d);
linked!(HC6 "WINMM!joyGetDevCapsA" joyGetDevCapsA a b c);
linked!(HC6 "WINMM!joyGetNumDevs" joyGetNumDevs);
linked!(HC6 "WINMM!joyGetPosEx" joyGetPosEx a b);

// GAME, 87 rows.
resolved!(DirectPlayCreate, c"DPLAY.DLL", 1, "DPLAY!DirectPlayCreate", a b c);
resolved!(DirectPlayEnumerate, c"DPLAY.DLL", 2, "DPLAY!DirectPlayEnumerate", a b);
linked!(GAME "KERNEL32!CloseHandle" CloseHandle a);
linked!(GAME "KERNEL32!CreateFileA" CreateFileA a b c d e f g);
linked!(GAME "KERNEL32!DeleteFileA" DeleteFileA a);
linked!(GAME "KERNEL32!FindClose" FindClose a);
linked!(GAME "KERNEL32!FindFirstFileA" FindFirstFileA a b);
linked!(GAME "KERNEL32!FindNextFileA" FindNextFileA a b);
linked!(GAME "KERNEL32!FlushFileBuffers" FlushFileBuffers a);
linked!(GAME "KERNEL32!GetCurrentDirectoryA" GetCurrentDirectoryA a b);
linked!(GAME "KERNEL32!GetDriveTypeA" GetDriveTypeA a);
linked!(GAME "KERNEL32!GetFileType" GetFileType a);
linked!(GAME "KERNEL32!GetLocalTime" GetLocalTime a);
linked!(GAME "KERNEL32!GetLogicalDriveStringsA" GetLogicalDriveStringsA a b);
linked!(GAME "KERNEL32!GetStdHandle" GetStdHandle a);
linked!(GAME "KERNEL32!GetSystemTime" GetSystemTime a);
linked!(GAME "KERNEL32!GetTimeZoneInformation" GetTimeZoneInformation a);
linked!(GAME "KERNEL32!HeapAlloc" HeapAlloc a b c);
linked!(GAME "KERNEL32!HeapCreate" HeapCreate a b c);
linked!(GAME "KERNEL32!HeapDestroy" HeapDestroy a);
linked!(GAME "KERNEL32!HeapFree" HeapFree a b c);
linked!(GAME "KERNEL32!HeapReAlloc" HeapReAlloc a b c d);
linked!(GAME "KERNEL32!HeapSize" HeapSize a b c);
linked!(GAME "KERNEL32!HeapValidate" HeapValidate a b c);
linked!(GAME "KERNEL32!QueryPerformanceCounter" QueryPerformanceCounter a);
linked!(GAME "KERNEL32!ReadFile" ReadFile a b c d e);
linked!(GAME "KERNEL32!SetEndOfFile" SetEndOfFile a);
linked!(GAME "KERNEL32!SetFilePointer" SetFilePointer a b c d);
linked!(GAME "KERNEL32!SetHandleCount" SetHandleCount a);
linked!(GAME "KERNEL32!SetStdHandle" SetStdHandle a b);
linked!(GAME "KERNEL32!Sleep" Sleep a);
linked!(GAME "KERNEL32!VirtualProtect" VirtualProtect a b c d);
linked!(GAME "KERNEL32!WriteConsoleA" WriteConsoleA a b c d e);
linked!(GAME "KERNEL32!WriteFile" WriteFile a b c d e);
linked!(GAME "USER32!MessageBoxA" MessageBoxA a b c d);
wail!("_AIL_allocate_file_sample@12" AIL_allocate_file_sample a b c);
wail!("_AIL_allocate_sample_handle@4" AIL_allocate_sample_handle a);
wail!("_AIL_allocate_sequence_handle@4" AIL_allocate_sequence_handle a);
wail!("_AIL_end_sample@4" AIL_end_sample a);
wail!("_AIL_init_sample@4" AIL_init_sample a);
wail!("_AIL_init_sequence@12" AIL_init_sequence a b c);
wail!("_AIL_load_sample_buffer@16" AIL_load_sample_buffer a b c d);
wail!("_AIL_lock_channel@4" AIL_lock_channel a);
wail!("_AIL_midiOutOpen@12" AIL_midiOutOpen a b c);
wail!("_AIL_register_EOS_callback@8" AIL_register_EOS_callback a b);
wail!("_AIL_register_timer@4" AIL_register_timer a);
wail!("_AIL_release_channel@8" AIL_release_channel a b);
wail!("_AIL_release_sample_handle@4" AIL_release_sample_handle a);
wail!("_AIL_release_sequence_handle@4" AIL_release_sequence_handle a);
wail!("_AIL_release_timer_handle@4" AIL_release_timer_handle a);
wail!("_AIL_resume_sample@4" AIL_resume_sample a);
wail!("_AIL_resume_sequence@4" AIL_resume_sequence a);
wail!("_AIL_sample_buffer_ready@4" AIL_sample_buffer_ready a);
wail!("_AIL_sample_user_data@8" AIL_sample_user_data a b);
wail!("_AIL_send_channel_voice_message@20" AIL_send_channel_voice_message a b c d e);
wail!("_AIL_sequence_status@4" AIL_sequence_status a);
wail!("_AIL_serve@0" AIL_serve);
wail!("_AIL_set_preference@8" AIL_set_preference a b);
wail!("_AIL_set_sample_loop_count@8" AIL_set_sample_loop_count a b);
wail!("_AIL_set_sample_pan@8" AIL_set_sample_pan a b);
wail!("_AIL_set_sample_playback_rate@8" AIL_set_sample_playback_rate a b);
wail!("_AIL_set_sample_type@12" AIL_set_sample_type a b c);
wail!("_AIL_set_sample_user_data@12" AIL_set_sample_user_data a b c);
wail!("_AIL_set_sample_volume@8" AIL_set_sample_volume a b);
wail!("_AIL_set_sequence_volume@12" AIL_set_sequence_volume a b c);
wail!("_AIL_set_timer_divisor@8" AIL_set_timer_divisor a b);
wail!("_AIL_set_timer_period@8" AIL_set_timer_period a b);
wail!("_AIL_shutdown@0" AIL_shutdown);
wail!("_AIL_start_sample@4" AIL_start_sample a);
wail!("_AIL_start_sequence@4" AIL_start_sequence a);
wail!("_AIL_start_timer@4" AIL_start_timer a);
wail!("_AIL_startup@0" AIL_startup);
wail!("_AIL_stop_sample@4" AIL_stop_sample a);
wail!("_AIL_stop_sequence@4" AIL_stop_sequence a);
wail!("_AIL_waveOutOpen@16" AIL_waveOutOpen a b c d);
wail!("_FILE_read@8" FILE_read a b);
wail!("_MEM_free_lock@4" MEM_free_lock a);
linked!(GAME "WINMM!auxGetDevCapsA" auxGetDevCapsA a b c);
linked!(GAME "WINMM!auxGetNumDevs" auxGetNumDevs);
linked!(GAME "WINMM!auxGetVolume" auxGetVolume a b);
linked!(GAME "WINMM!auxSetVolume" auxSetVolume a b);
linked!(GAME "WINMM!joyGetDevCapsA" joyGetDevCapsA a b c);
linked!(GAME "WINMM!joyGetNumDevs" joyGetNumDevs);
linked!(GAME "WINMM!joyGetPosEx" joyGetPosEx a b);
linked!(GAME "WINMM!mciGetErrorStringA" mciGetErrorStringA a b c);
linked!(GAME "WINMM!mciSendCommandA" mciSendCommandA a b c d);
linked!(GAME "WINMM!timeGetTime" timeGetTime);

// LAUNCHER, 20 rows.
linked!(LAUNCHER "KERNEL32!CloseHandle" CloseHandle a);
linked!(LAUNCHER "KERNEL32!CreateFileA" CreateFileA a b c d e f g);
linked!(LAUNCHER "KERNEL32!FindClose" FindClose a);
linked!(LAUNCHER "KERNEL32!FindFirstFileA" FindFirstFileA a b);
linked!(LAUNCHER "KERNEL32!FlushFileBuffers" FlushFileBuffers a);
linked!(LAUNCHER "KERNEL32!GetDriveTypeA" GetDriveTypeA a);
linked!(LAUNCHER "KERNEL32!GetFileType" GetFileType a);
linked!(LAUNCHER "KERNEL32!GetLogicalDriveStringsA" GetLogicalDriveStringsA a b);
linked!(LAUNCHER "KERNEL32!GetStdHandle" GetStdHandle a);
linked!(LAUNCHER "KERNEL32!HeapAlloc" HeapAlloc a b c);
linked!(LAUNCHER "KERNEL32!HeapCreate" HeapCreate a b c);
linked!(LAUNCHER "KERNEL32!HeapFree" HeapFree a b c);
linked!(LAUNCHER "KERNEL32!ReadFile" ReadFile a b c d e);
linked!(LAUNCHER "KERNEL32!SetEndOfFile" SetEndOfFile a);
linked!(LAUNCHER "KERNEL32!SetFilePointer" SetFilePointer a b c d);
linked!(LAUNCHER "KERNEL32!SetHandleCount" SetHandleCount a);
linked!(LAUNCHER "KERNEL32!SetStdHandle" SetStdHandle a b);
linked!(LAUNCHER "KERNEL32!WriteFile" WriteFile a b c d e);
linked!(LAUNCHER "USER32!MessageBoxA" MessageBoxA a b c d);
linked!(LAUNCHER "USER32!SetMenu" SetMenu a b);
