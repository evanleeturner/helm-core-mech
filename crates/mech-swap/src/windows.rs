// SPDX-License-Identifier: MIT
//! The swap's Windows side: `DllMain`, which opens the trace file and
//! installs the sink; the sink, one `WriteFile` a line; and the panic
//! handler, which traps (`ud2`) as the checker's builds do, so a port's
//! panic shows as an illegal instruction in Wine's `seh` trace.
//!
//! Each line ends `tid=<hex>`, the writing thread's id (KH1, wall 4): the
//! Miles timer calls in from a thread of its own, and the line says whose
//! it is. The field comes last, so a reader's third word is still the
//! event; a line cut at [`crate::LINE_BYTES`] loses it. Two threads'
//! lines cannot tear each other: the file is opened for appending and a
//! line is one `WriteFile`, so each lands whole at the end.
//!
//! The panic handler and the personality are the swap profile's alone
//! (`panic = "abort"`, the crate built by itself, no `std`): in the
//! workspace's own builds `mech_logline` turns on `log`'s `std`, which
//! brings `std`'s handler, and a second one is a duplicate lang item.
use core::ffi::c_void;
use core::fmt::Write as _;
use core::sync::atomic::{AtomicUsize, Ordering};

use log::{Level, LevelFilter, Log, Metadata, Record};

use crate::{Line, body};

const DLL_PROCESS_ATTACH: u32 = 1;
const FILE_APPEND_DATA: u32 = 0x0004;
const FILE_SHARE_READ: u32 = 0x0001;
const OPEN_ALWAYS: u32 = 4;
const FILE_ATTRIBUTE_NORMAL: u32 = 0x80;
const INVALID_HANDLE_VALUE: usize = usize::MAX;
const PATH_BYTES: usize = 260;

#[repr(C)]
struct SystemTime {
    year: u16,
    month: u16,
    day_of_week: u16,
    day: u16,
    hour: u16,
    minute: u16,
    second: u16,
    milliseconds: u16,
}

unsafe extern "system" {
    fn GetEnvironmentVariableA(name: *const u8, buffer: *mut u8, size: u32) -> u32;
    fn CreateFileA(
        path: *const u8,
        access: u32,
        share: u32,
        security: *mut c_void,
        disposition: u32,
        flags: u32,
        template: *mut c_void,
    ) -> usize;
    fn WriteFile(
        file: usize,
        buffer: *const u8,
        bytes: u32,
        written: *mut u32,
        overlapped: *mut c_void,
    ) -> i32;
    fn GetSystemTime(time: *mut SystemTime);
    fn GetCurrentThreadId() -> u32;
}

/// The trace file's handle, `INVALID_HANDLE_VALUE` until `DllMain` opens it.
static TRACE: AtomicUsize = AtomicUsize::new(INVALID_HANDLE_VALUE);

struct Sink;

static SINK: Sink = Sink;

impl Log for Sink {
    fn enabled(&self, _: &Metadata<'_>) -> bool {
        TRACE.load(Ordering::Relaxed) != INVALID_HANDLE_VALUE
    }

    fn log(&self, record: &Record<'_>) {
        let file = TRACE.load(Ordering::Relaxed);
        if file == INVALID_HANDLE_VALUE {
            return;
        }
        let mut now = SystemTime {
            year: 0,
            month: 0,
            day_of_week: 0,
            day: 0,
            hour: 0,
            minute: 0,
            second: 0,
            milliseconds: 0,
        };
        // SAFETY: GetSystemTime fills the struct it is handed, which lives here.
        unsafe { GetSystemTime(&raw mut now) };
        let mut line = Line::new();
        let _ = write!(
            line,
            "{:02}:{:02}:{:02}.{:03} {} ",
            now.hour,
            now.minute,
            now.second,
            now.milliseconds,
            letter(record.level())
        );
        body(record, &mut line);
        // SAFETY: reads the calling thread's own id; touches no memory.
        let _ = write!(line, " tid={:x}", unsafe { GetCurrentThreadId() });
        let _ = line.write_char('\n');
        let bytes = line.as_bytes();
        let mut written = 0;
        // SAFETY: the handle is the trace file DllMain opened, never closed;
        // the bytes are the line's own. A failed write drops the line.
        let _ = unsafe {
            WriteFile(
                file,
                bytes.as_ptr(),
                bytes.len() as u32,
                &raw mut written,
                core::ptr::null_mut(),
            )
        };
    }

    fn flush(&self) {}
}

/// The level as the grammar's one letter.
fn letter(level: Level) -> char {
    match level {
        Level::Error => 'E',
        Level::Warn => 'W',
        Level::Info => 'I',
        Level::Debug => 'D',
        Level::Trace => 'T',
    }
}

/// At the process's attach, opens the file `HC5_LOG` names for appending
/// and routes every record at DEBUG or above to it; without `HC5_LOG`, or
/// when the file will not open, the sink stays off. Always returns TRUE:
/// the swap loads whether or not it traces.
#[unsafe(no_mangle)]
pub extern "system" fn DllMain(_module: *mut c_void, reason: u32, _reserved: *mut c_void) -> i32 {
    if reason == DLL_PROCESS_ATTACH {
        let mut path = [0u8; PATH_BYTES];
        // SAFETY: the name is NUL-terminated; the buffer is PATH_BYTES long
        // and its size is passed, so the call writes inside it.
        let len = unsafe {
            GetEnvironmentVariableA(b"HC5_LOG\0".as_ptr(), path.as_mut_ptr(), PATH_BYTES as u32)
        };
        if len > 0 && (len as usize) < PATH_BYTES {
            // SAFETY: the path is NUL-terminated inside its buffer (len < its size).
            let file = unsafe {
                CreateFileA(
                    path.as_ptr(),
                    FILE_APPEND_DATA,
                    FILE_SHARE_READ,
                    core::ptr::null_mut(),
                    OPEN_ALWAYS,
                    FILE_ATTRIBUTE_NORMAL,
                    core::ptr::null_mut(),
                )
            };
            if file != INVALID_HANDLE_VALUE && log::set_logger(&SINK).is_ok() {
                TRACE.store(file, Ordering::Relaxed);
                log::set_max_level(LevelFilter::Debug);
            }
        }
    }
    1
}

#[cfg(panic = "abort")]
#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
    // SAFETY: ud2 raises an illegal instruction and never returns.
    unsafe { core::arch::asm!("ud2", options(noreturn)) }
}

/// Core's code names the unwinding personality; a `panic = "abort"` build
/// never calls it. It traps if called.
#[cfg(panic = "abort")]
#[unsafe(no_mangle)]
pub extern "C" fn rust_eh_personality() {
    // SAFETY: as the panic handler's.
    unsafe { core::arch::asm!("ud2", options(noreturn)) }
}
