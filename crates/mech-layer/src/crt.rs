// SPDX-License-Identifier: MIT
//! The C library's strings and numbers (HC6, `JOB.md` I4 as ruling 1381
//! amends it; session 24's design, its third family): a stand-in for each
//! of the eleven functions inside the DLL the shell's screens call,
//! exported `HC5_<address>` and swapped as a port is. Each does the job
//! with Rust's own code, never by calling the original, and writes one
//! `imp.call` line, `c` the name `ORDER.tsv` gives it.
//!
//! The family swaps together: no member shares state with the rest of the
//! C library but `rand` with `srand`. The locale is the "C" one a program
//! that never calls `setlocale` sees: case is ASCII's, whitespace the six
//! bytes of `isspace`. A comparison returns the difference of the first
//! bytes that differ, as MSVC's C source does; only its sign is promised.
//!
//! `rand` keeps one seed for the process where MSVC keeps one per thread:
//! the same sequence for a program that draws from one thread, unmeasured
//! for one that draws from two.
use core::ffi::CStr;
use core::ptr::null_mut;
use core::sync::atomic::{AtomicU32, Ordering};

unsafe extern "system" {
    fn GetSystemTimeAsFileTime(time: *mut u64);
}

/// Windows' clock at 1970-01-01 00:00 UTC, in its 100 ns ticks.
const UNIX_EPOCH_TICKS: u64 = 116_444_736_000_000_000;

/// `rand`'s seed; MSVC's starts at 1 until `srand` is called.
static SEED: AtomicU32 = AtomicU32::new(1);

/// The difference of the first bytes that differ once folded, reading at
/// most `count`, stopped by `first`'s NUL.
///
/// # Safety
/// Both strings end in a NUL or hold `count` bytes.
unsafe fn compare(first: *const u8, last: *const u8, count: usize, fold: fn(u8) -> i32) -> i32 {
    for i in 0..count {
        // SAFETY: the caller's contract; reading stops at `first`'s NUL,
        // and at `last`'s, which differs from any byte but a NUL.
        let (f, l) = unsafe { (fold(*first.add(i)), fold(*last.add(i))) };
        if f != l || f == 0 {
            return f - l;
        }
    }
    0
}

fn exact(byte: u8) -> i32 {
    i32::from(byte)
}

fn lowered(byte: u8) -> i32 {
    i32::from(byte.to_ascii_lowercase())
}

/// strncpy: `count` bytes written, `source` up to its NUL, then NULs.
#[unsafe(export_name = "HC5_1006f650")]
pub unsafe extern "C" fn strncpy(dest: *mut u8, source: *const u8, count: u32) -> *mut u8 {
    let mut ended = false;
    for i in 0..count as usize {
        // SAFETY: the caller's `dest` holds `count` bytes; `source` is read
        // only up to its NUL.
        unsafe {
            let byte = if ended { 0 } else { *source.add(i) };
            ended |= byte == 0;
            *dest.add(i) = byte;
        }
    }
    imp_call!("_strncpy", [dest, source, count], dest)
}

/// strncmp: the first `count` bytes compared.
#[unsafe(export_name = "HC5_10070040")]
pub unsafe extern "C" fn strncmp(first: *const u8, last: *const u8, count: u32) -> i32 {
    // SAFETY: the caller's two strings, as the C library's contract.
    let value = unsafe { compare(first, last, count as usize, exact) };
    imp_call!("_strncmp", [first, last, count], value)
}

/// _strcmpi: compared with case folded.
#[unsafe(export_name = "HC5_1007d0e0")]
pub unsafe extern "C" fn strcmpi(first: *const u8, last: *const u8) -> i32 {
    // SAFETY: the caller's two NUL-ended strings.
    let value = unsafe { compare(first, last, usize::MAX, lowered) };
    imp_call!("__strcmpi", [first, last], value)
}

/// _strnicmp: the first `count` bytes compared with case folded.
#[unsafe(export_name = "HC5_1007e1c0")]
pub unsafe extern "C" fn strnicmp(first: *const u8, last: *const u8, count: u32) -> i32 {
    // SAFETY: the caller's two strings, as the C library's contract.
    let value = unsafe { compare(first, last, count as usize, lowered) };
    imp_call!("__strnicmp", [first, last, count], value)
}

/// strstr: where `wanted` first starts in `string`; `string` itself for
/// an empty `wanted`, null for none.
#[unsafe(export_name = "HC5_10070400")]
pub unsafe extern "C" fn strstr(string: *const u8, wanted: *const u8) -> *mut u8 {
    // SAFETY: the caller's two NUL-ended strings.
    let (haystack, needle) = unsafe {
        (
            CStr::from_ptr(string.cast()).to_bytes(),
            CStr::from_ptr(wanted.cast()).to_bytes(),
        )
    };
    let at = if needle.is_empty() {
        Some(0)
    } else {
        haystack
            .windows(needle.len())
            .position(|window| window == needle)
    };
    // SAFETY: `at` lies inside `string`.
    let value = at.map_or(null_mut(), |at| unsafe { string.add(at) }.cast_mut());
    imp_call!("_strstr", [string, wanted], value)
}

/// strchr: where the byte `wanted` first sits in `string`, its NUL
/// included; null for none.
#[unsafe(export_name = "HC5_1006f930")]
pub unsafe extern "C" fn strchr(string: *const u8, wanted: i32) -> *mut u8 {
    let byte = wanted.to_le_bytes()[0];
    let mut at = string;
    let value = loop {
        // SAFETY: the caller's NUL-ended string, read up to its NUL.
        let here = unsafe { *at };
        if here == byte {
            break at.cast_mut();
        }
        if here == 0 {
            break null_mut();
        }
        // SAFETY: `at` is still before the NUL.
        at = unsafe { at.add(1) };
    };
    imp_call!("_strchr", [string, wanted], value)
}

/// toupper: a lower-case ASCII letter raised; any other value unchanged.
#[unsafe(export_name = "HC5_10070560")]
pub extern "C" fn toupper(c: i32) -> i32 {
    let value = if (i32::from(b'a')..=i32::from(b'z')).contains(&c) {
        c - 0x20
    } else {
        c
    };
    imp_call!("_toupper", [c], value)
}

/// atoi: the decimal number after any whitespace and one sign; the digits
/// end it, and a number past `i32` wraps, as MSVC's does.
#[unsafe(export_name = "HC5_1006faa0")]
pub unsafe extern "C" fn atoi(string: *const u8) -> i32 {
    // SAFETY: the caller's NUL-ended string.
    let bytes = unsafe { CStr::from_ptr(string.cast()) }.to_bytes();
    let mut rest = bytes
        .iter()
        .copied()
        .skip_while(|b| matches!(b, b' ' | b'\t'..=b'\r'))
        .peekable();
    let negative = rest.next_if(|b| matches!(b, b'+' | b'-')) == Some(b'-');
    let total = rest
        .take_while(u8::is_ascii_digit)
        .fold(0i32, |total, digit| {
            total.wrapping_mul(10).wrapping_add(i32::from(digit - b'0'))
        });
    let value = if negative {
        total.wrapping_neg()
    } else {
        total
    };
    imp_call!("_atoi", [string], value)
}

/// srand: the seed `rand` draws from next.
#[unsafe(export_name = "HC5_100706c0")]
pub extern "C" fn srand(seed: u32) {
    SEED.store(seed, Ordering::Relaxed);
    imp_call!("FUN_100706c0", [seed]);
}

/// rand: MSVC's sequence, 0 to 32767.
#[unsafe(export_name = "HC5_100706d0")]
pub extern "C" fn rand() -> i32 {
    let seed = SEED
        .load(Ordering::Relaxed)
        .wrapping_mul(214_013)
        .wrapping_add(2_531_011);
    SEED.store(seed, Ordering::Relaxed);
    let value = i32::from(u16::try_from((seed >> 16) & 0x7fff).unwrap_or_default());
    imp_call!("_rand", [], value)
}

/// time: the whole seconds since 1970 UTC by Windows' clock, also written
/// to `timer` when it is not null. MSVC reads the local time and takes its
/// time zone back off, which comes to the same seconds.
#[unsafe(export_name = "HC5_10070850")]
pub unsafe extern "C" fn time(timer: *mut i32) -> i32 {
    let mut ticks = 0u64;
    // SAFETY: Windows writes the one FILETIME it is handed.
    unsafe { GetSystemTimeAsFileTime(&raw mut ticks) };
    let seconds = ticks.saturating_sub(UNIX_EPOCH_TICKS) / 10_000_000;
    let value = i32::try_from(seconds).unwrap_or(i32::MAX);
    if !timer.is_null() {
        // SAFETY: the caller's own `time_t`.
        unsafe { *timer = value };
    }
    imp_call!("_time", [timer], value)
}
