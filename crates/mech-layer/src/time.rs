// SPDX-License-Identifier: MIT
//! The shell's time calls through the seam (KT1, ruling 1398): `kernel_time`'s
//! front over Windows' own clocks, the shell's 5 date imports exported
//! `HC6_<name>` as the layer's other rows are, so the swap's loader points
//! the shell's own slots at them unchanged.
//!
//! The Windows clocks answer from the real imports, from the swap DLL's own
//! import table: the game clock the counter over its own rate, the wall
//! clock `GetSystemTimeAsFileTime`, the zone the real record read back
//! whole, so the front writes the shell the bytes Windows wrote. The zone
//! is now's, whatever the instant asked, as Windows' is; a refused zone
//! call reads as no zone in force. The thread's last error is kept around
//! each line, as `imp_call!` keeps it.
//!
//! Not moved: the game's rebuilt `Sleep` and `QueryPerformanceCounter`
//! call sites (`mech-game`'s own `extern`s) and `mech-swap`'s own
//! `GetSystemTime`.
use core::ffi::c_void;
use core::sync::atomic::{AtomicU64, Ordering};
use kernel_time::table::{Row, ZONE_BYTES};
use kernel_time::{Clocks, InForce, Times, Zone};
use mech_rows::time::SHELL;

unsafe extern "system" {
    fn QueryPerformanceCounter(count: *mut i64) -> i32;
    fn QueryPerformanceFrequency(rate: *mut i64) -> i32;
    fn GetSystemTimeAsFileTime(time: *mut u64);
    fn GetTimeZoneInformation(zone: *mut c_void) -> u32;
    fn Sleep(millis: u32);
}

/// The zone record a Windows zone call fills.
#[repr(C, align(4))]
struct ZoneRecord([u8; ZONE_BYTES]);

/// The counter's rate, its ticks a second, read once; 0 unread.
static RATE: AtomicU64 = AtomicU64::new(0);

/// One file's clocks on Windows, through its row.
pub struct Win32(&'static Row);

impl Win32 {
    fn rate() -> u64 {
        let known = RATE.load(Ordering::Relaxed);
        if known != 0 {
            return known;
        }
        let mut rate = 0i64;
        // SAFETY: Windows writes the one count it is handed.
        unsafe { QueryPerformanceFrequency(&raw mut rate) };
        let rate = u64::try_from(rate).unwrap_or(0).max(1);
        RATE.store(rate, Ordering::Relaxed);
        rate
    }
}

impl Clocks for Win32 {
    /// The counter in microseconds, split by its rate so it never
    /// overflows.
    fn micros(&self) -> u64 {
        let mut count = 0i64;
        // SAFETY: Windows writes the one count it is handed.
        unsafe { QueryPerformanceCounter(&raw mut count) };
        let (count, rate) = (u64::try_from(count).unwrap_or(0), Self::rate());
        count / rate * 1_000_000 + count % rate * 1_000_000 / rate
    }

    fn wall(&self) -> u64 {
        let mut ticks = 0u64;
        // SAFETY: Windows writes the one FILETIME it is handed.
        unsafe { GetSystemTimeAsFileTime(&raw mut ticks) };
        ticks
    }

    fn zone(&self, _at: u64) -> (Zone, InForce) {
        let mut record = ZoneRecord([0; ZONE_BYTES]);
        // SAFETY: Windows fills the one record it is handed.
        let id = unsafe { GetTimeZoneInformation((&raw mut record).cast()) };
        let in_force = match self.0.ids.iter().position(|&answer| answer == id) {
            Some(1) => InForce::Standard,
            Some(2) => InForce::Daylight,
            _ => InForce::Unknown,
        };
        (Zone::read(&record.0), in_force)
    }

    fn sleep(&self, millis: u32) {
        // SAFETY: waits; touches no memory.
        unsafe { Sleep(millis) };
    }

    fn fail(&self, code: u32) {
        // SAFETY: sets the thread's own last error.
        unsafe { crate::SetLastError(code) };
    }

    fn quiet(&self, line: impl FnOnce()) {
        // SAFETY: reads the thread's own last error.
        let error = unsafe { crate::GetLastError() };
        line();
        // SAFETY: sets back the value read above.
        unsafe { crate::SetLastError(error) };
    }
}

/// The shell's time calls.
pub(crate) static SHELL_TIMES: Times<Win32> = Times::new(&SHELL, Win32(&SHELL));

#[unsafe(export_name = "HC6_GetSystemTime")]
unsafe extern "system" fn get_system_time(time: *mut c_void) {
    // SAFETY: the shell's own call: its own `SYSTEMTIME`.
    SHELL_TIMES.system_time(unsafe { &mut *time.cast() });
}

#[unsafe(export_name = "HC6_GetLocalTime")]
unsafe extern "system" fn get_local_time(time: *mut c_void) {
    // SAFETY: the shell's own call: its own `SYSTEMTIME`.
    SHELL_TIMES.local_time(unsafe { &mut *time.cast() });
}

#[unsafe(export_name = "HC6_GetTimeZoneInformation")]
unsafe extern "system" fn get_time_zone_information(zone: *mut c_void) -> u32 {
    // SAFETY: the shell's own call: its own record.
    SHELL_TIMES.zone(unsafe { &mut *zone.cast() })
}

#[unsafe(export_name = "HC6_FileTimeToLocalFileTime")]
unsafe extern "system" fn file_time_to_local_file_time(from: *const u64, to: *mut u64) -> i32 {
    // SAFETY: the shell's own call: two `FILETIME`s of its own, 4-aligned.
    let ticks = unsafe { from.read_unaligned() };
    let mut local = 0;
    let value = SHELL_TIMES.to_local_file_time(ticks, &mut local);
    // SAFETY: as above.
    unsafe { to.write_unaligned(local) };
    value
}

#[unsafe(export_name = "HC6_FileTimeToSystemTime")]
unsafe extern "system" fn file_time_to_system_time(from: *const u64, to: *mut c_void) -> i32 {
    // SAFETY: the shell's own call: its `FILETIME` and its `SYSTEMTIME`.
    let (ticks, into) = unsafe { (from.read_unaligned(), &mut *to.cast()) };
    SHELL_TIMES.to_system_time(ticks, into)
}
