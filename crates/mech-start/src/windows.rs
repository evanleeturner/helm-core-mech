// SPDX-License-Identifier: MIT
//! The starter's steps. No debug call, so no debugger for the program to meet,
//! and nothing written on disk. The parked bytes live only in the program's memory, for as
//! long as the library takes to load, and the thread is held while they go back so it never
//! reads half of them.
//!
//! **A program that ends inside a wait has its exit code passed on, never read as a failure of
//! the wait:** a refused bind ends the process from the library's `DllMain`, under the remote
//! load, and that code is the refusal's name. **A stop of the starter's own ends the program
//! too,** which otherwise spins on its entry for ever.
//!
//! `LoadLibraryA` is taken from this process's own `kernel32`, the same address in each process
//! of a boot, as measured on a parked entry.
use core::ffi::c_void;
use core::panic::PanicInfo;
use core::ptr::{null, null_mut};

use mech_start::{entry_address, two_arguments};

const CREATE_SUSPENDED: u32 = 4;
const GENERIC_READ: u32 = 0x8000_0000;
const FILE_SHARE_READ: u32 = 1;
const OPEN_EXISTING: u32 = 3;
const INVALID_HANDLE_VALUE: usize = usize::MAX;
const MEM_COMMIT_RESERVE: u32 = 0x3000;
const PAGE_READWRITE: u32 = 4;
const PAGE_EXECUTE_READWRITE: u32 = 0x40;
const STD_OUTPUT_HANDLE: u32 = 0xffff_fff5;
const WAIT_OBJECT_0: u32 = 0;
const FOREVER: u32 = u32::MAX;
/// `CONTEXT_i386 | CONTEXT_CONTROL`, which holds `Eip`.
const CONTEXT_CONTROL: u32 = 0x0001_0001;
/// A 32-bit `CONTEXT`'s 716 bytes, as words, and `Eip`'s place at byte `0xb8`.
const CONTEXT_WORDS: usize = 179;
const CONTEXT_EIP: usize = 46;
const HEADER_BYTES: usize = 0x1000;
const PARK: [u8; 2] = [0xEB, 0xFE];
/// Polls at 1 ms each, before the park is called failed: well over Wine's start.
const PARK_TRIES: u32 = 30_000;
/// The remote load's whole time. A library hung in `DllMain` is a named stop, not a live process.
const LOAD_MS: u32 = 120_000;
/// A thread's end can be seen just before its process's: the program is parked, so a live one
/// costs only this wait.
const PROCESS_END_MS: u32 = 200;

const EXIT_PANIC: u32 = 70;
const EXIT_USAGE: u32 = 2;
const EXIT_NOT_MADE: u32 = 3;
const EXIT_NOT_PARKED: u32 = 4;
const EXIT_NOT_LOADED: u32 = 5;
const EXIT_NOT_GIVEN_BACK: u32 = 6;

#[repr(C)]
struct StartupInfo {
    size: u32,
    rest: [u32; 16],
}

#[repr(C)]
#[derive(Default)]
struct ProcessInfo {
    process: usize,
    thread: usize,
    process_id: u32,
    thread_id: u32,
}

#[link(name = "kernel32")]
unsafe extern "system" {
    fn CreateFileA(
        path: *const u8,
        access: u32,
        share: u32,
        security: *mut c_void,
        disposition: u32,
        flags: u32,
        template: *mut c_void,
    ) -> usize;
    fn ReadFile(
        file: usize,
        buffer: *mut u8,
        bytes: u32,
        read: *mut u32,
        overlapped: *mut c_void,
    ) -> i32;
    fn CloseHandle(handle: usize) -> i32;
    fn CreateProcessA(
        program: *const u8,
        command: *mut u8,
        process_security: *mut c_void,
        thread_security: *mut c_void,
        inherit: i32,
        flags: u32,
        environment: *mut c_void,
        folder: *const u8,
        startup: *mut StartupInfo,
        made: *mut ProcessInfo,
    ) -> i32;
    fn ReadProcessMemory(p: usize, at: usize, out: *mut u8, n: usize, done: *mut usize) -> i32;
    fn WriteProcessMemory(p: usize, at: usize, from: *const u8, n: usize, done: *mut usize) -> i32;
    fn VirtualProtectEx(p: usize, at: usize, n: usize, new: u32, old: *mut u32) -> i32;
    fn VirtualAllocEx(p: usize, at: usize, n: usize, kind: u32, protect: u32) -> usize;
    fn FlushInstructionCache(p: usize, at: usize, n: usize) -> i32;
    fn ResumeThread(thread: usize) -> u32;
    fn SuspendThread(thread: usize) -> u32;
    fn GetThreadContext(thread: usize, context: *mut u32) -> i32;
    fn CreateRemoteThread(
        p: usize,
        security: *mut c_void,
        stack: usize,
        start: usize,
        argument: usize,
        flags: u32,
        id: *mut u32,
    ) -> usize;
    fn WaitForSingleObject(handle: usize, ms: u32) -> u32;
    fn GetExitCodeThread(thread: usize, code: *mut u32) -> i32;
    fn GetExitCodeProcess(p: usize, code: *mut u32) -> i32;
    fn TerminateProcess(p: usize, code: u32) -> i32;
    fn GetModuleHandleA(name: *const u8) -> usize;
    fn GetProcAddress(module: usize, name: *const u8) -> usize;
    fn GetStdHandle(which: u32) -> usize;
    fn WriteFile(file: usize, from: *const u8, n: u32, done: *mut u32, over: *mut c_void) -> i32;
    fn GetCommandLineA() -> *mut u8;
    fn GetLastError() -> u32;
    fn Sleep(ms: u32);
    fn ExitProcess(code: u32) -> !;
}

/// The precompiled `core` names this in its unwind tables; `panic = "abort"` never calls it.
#[unsafe(no_mangle)]
pub extern "C" fn rust_eh_personality() {}

#[panic_handler]
fn panic(_: &PanicInfo<'_>) -> ! {
    // SAFETY: ends this process.
    unsafe { ExitProcess(EXIT_PANIC) }
}

/// One line to stdout: `mech-start`, words, a number in hex.
fn say(words: &str, number: u32) {
    const PREFIX: &[u8] = b"mech-start: ";
    const DIGITS: usize = 8;
    let mut line = [0u8; 120];
    let words = words.as_bytes();
    let end = PREFIX.len() + words.len();
    line[..PREFIX.len()].copy_from_slice(PREFIX);
    line[PREFIX.len()..end].copy_from_slice(words);
    line[end] = b' ';
    let digits = &mut line[end + 1..end + 1 + DIGITS];
    for (place, digit) in digits.iter_mut().enumerate() {
        let shift = (DIGITS - 1 - place) * 4;
        *digit = b"0123456789abcdef"[((number >> shift) & 0xf) as usize];
    }
    line[end + 1 + DIGITS] = b'\n';
    let mut done = 0;
    // SAFETY: the buffer outlives the call, and the length stays inside it.
    unsafe {
        WriteFile(
            GetStdHandle(STD_OUTPUT_HANDLE),
            line.as_ptr(),
            (end + 2 + DIGITS) as u32,
            &raw mut done,
            null_mut(),
        );
    }
}

/// The named stop of a run with no program yet: the words, the system's reason, `code`.
fn stop(words: &str, code: u32) -> ! {
    // SAFETY: reads a number, then ends this process.
    unsafe {
        say(words, GetLastError());
        ExitProcess(code)
    }
}

/// The program's first instruction, from its file; a stop if the file is not a 32-bit PE.
fn entry_in_file(path: &[u8]) -> u32 {
    // SAFETY: the path ends in its NUL, by `two_arguments`; a bad answer is checked.
    let file = unsafe {
        CreateFileA(path.as_ptr(), GENERIC_READ, FILE_SHARE_READ, null_mut(), OPEN_EXISTING, 0, null_mut())
    };
    if file == INVALID_HANDLE_VALUE {
        stop("the program's file would not open, reason", EXIT_NOT_MADE);
    }
    let mut head = [0; HEADER_BYTES];
    let mut read = 0;
    // SAFETY: the buffer's size is passed, and the handle is open.
    let ok = unsafe { ReadFile(file, head.as_mut_ptr(), HEADER_BYTES as u32, &raw mut read, null_mut()) };
    // SAFETY: closes the handle opened above, once.
    unsafe { CloseHandle(file) };
    if ok == 0 {
        stop("the program's file would not read, reason", EXIT_NOT_MADE);
    }
    entry_address(&head[..read as usize]).unwrap_or_else(|| {
        say("the program's file is no 32-bit PE, bytes read", read);
        // SAFETY: ends this process.
        unsafe { ExitProcess(EXIT_NOT_MADE) }
    })
}

/// The program, once made: both handles, and the stop that takes it down too.
struct Program {
    process: usize,
    thread: usize,
}

impl Program {
    fn make(path: &[u8]) -> Self {
        let mut startup = StartupInfo { size: size_of::<StartupInfo>() as u32, rest: [0; 16] };
        let mut made = ProcessInfo::default();
        // SAFETY: each pointer outlives the call and the path ends in its NUL. A null command
        // line makes it the path, and a null folder keeps this process's own, as `start /d` set.
        let ok = unsafe {
            CreateProcessA(
                path.as_ptr(),
                null_mut(),
                null_mut(),
                null_mut(),
                1,
                CREATE_SUSPENDED,
                null_mut(),
                null(),
                &raw mut startup,
                &raw mut made,
            )
        };
        if ok == 0 {
            stop("CreateProcessA refused, reason", EXIT_NOT_MADE);
        }
        say("made suspended, pid", made.process_id);
        Self { process: made.process, thread: made.thread }
    }

    fn stop(&self, words: &str, code: u32) -> ! {
        // SAFETY: reads a number; the handle is this program's, and it is meant to end.
        unsafe {
            say(words, GetLastError());
            TerminateProcess(self.process, code);
            ExitProcess(code)
        }
    }

    /// True when the program has ended within `ms`.
    fn ended(&self, ms: u32) -> bool {
        // SAFETY: the handle stays open to this process's end.
        unsafe { WaitForSingleObject(self.process, ms) == WAIT_OBJECT_0 }
    }

    /// This run's end, the program's own: its exit code, passed on.
    fn pass_on(&self) -> ! {
        let mut code = 0;
        // SAFETY: the handle is open and the buffer outlives the call; then ends this process.
        unsafe {
            GetExitCodeProcess(self.process, &raw mut code);
            say("the program's exit code", code);
            ExitProcess(code)
        }
    }

    /// The entry's 2 bytes. The program's image answers only some time after it is made, so
    /// this polls: what the park measured.
    fn read_entry(&self, entry: usize) -> [u8; 2] {
        let mut old = [0; 2];
        let mut done = 0;
        let mut tries = 0;
        // SAFETY: the buffer's size is passed; a read not yet answered changes nothing.
        while unsafe { ReadProcessMemory(self.process, entry, old.as_mut_ptr(), 2, &raw mut done) } == 0 {
            tries += 1;
            if tries == PARK_TRIES {
                self.stop("the entry never answered, reason", EXIT_NOT_PARKED);
            }
            // SAFETY: no pointer.
            unsafe { Sleep(1) };
        }
        say("the entry read after polls", tries);
        old
    }

    /// The park written, the program let go: the protection the entry had, to give back.
    fn park(&self, entry: usize) -> u32 {
        let mut protection = 0;
        let mut done = 0;
        // SAFETY: the process is suspended; each buffer outlives its call.
        let ok = unsafe {
            VirtualProtectEx(self.process, entry, PARK.len(), PAGE_EXECUTE_READWRITE, &raw mut protection) != 0
                && WriteProcessMemory(self.process, entry, PARK.as_ptr(), PARK.len(), &raw mut done) != 0
        };
        if !ok {
            self.stop("the park would not write, reason", EXIT_NOT_PARKED);
        }
        // SAFETY: the range was just written, and the thread is this program's.
        unsafe {
            FlushInstructionCache(self.process, entry, PARK.len());
            ResumeThread(self.thread);
        }
        protection
    }

    /// The thread's `Eip`, read with it held; 0 when the read fails, as it can this early.
    fn eip(&self) -> u32 {
        let mut context = [0u32; CONTEXT_WORDS];
        context[0] = CONTEXT_CONTROL;
        // SAFETY: the buffer is a whole 32-bit `CONTEXT`, and the thread is held around the read.
        unsafe {
            SuspendThread(self.thread);
            let ok = GetThreadContext(self.thread, context.as_mut_ptr());
            ResumeThread(self.thread);
            if ok == 0 { 0 } else { context[CONTEXT_EIP] }
        }
    }

    fn wait_for_the_park(&self, entry: u32) {
        let mut polls = 0;
        while self.eip() != entry {
            if self.ended(0) {
                self.pass_on();
            }
            polls += 1;
            if polls == PARK_TRIES {
                self.stop("the thread never stood on the entry, reason", EXIT_NOT_PARKED);
            }
            // SAFETY: no pointer.
            unsafe { Sleep(1) };
        }
        say("parked after polls", polls);
    }

    /// The library loaded by a thread of the program's own, which runs its `DllMain` there.
    fn load(&self, library: &[u8]) {
        let mut done = 0;
        // SAFETY: asks for fresh pages in the program; 0 is checked.
        let path = unsafe {
            VirtualAllocEx(self.process, 0, library.len(), MEM_COMMIT_RESERVE, PAGE_READWRITE)
        };
        // SAFETY: the pages are `library.len()` bytes, and so is the source, NUL included.
        let written = path != 0 && unsafe {
            WriteProcessMemory(self.process, path, library.as_ptr(), library.len(), &raw mut done) != 0
        };
        if !written {
            self.stop("the library's path would not write, reason", EXIT_NOT_LOADED);
        }
        // SAFETY: both names end in their NULs; a missing one answers 0, which is checked.
        let load = unsafe {
            GetProcAddress(GetModuleHandleA(b"kernel32\0".as_ptr()), b"LoadLibraryA\0".as_ptr())
        };
        if load == 0 {
            self.stop("no LoadLibraryA, reason", EXIT_NOT_LOADED);
        }
        // SAFETY: `load` takes one word and answers one, the shape a thread's start has, and its
        // argument is the path just written; a null thread is checked.
        let remote = unsafe { CreateRemoteThread(self.process, null_mut(), 0, load, path, 0, null_mut()) };
        if remote == 0 {
            self.stop("CreateRemoteThread refused, reason", EXIT_NOT_LOADED);
        }
        // SAFETY: the handle was just checked.
        let waited = unsafe { WaitForSingleObject(remote, LOAD_MS) };
        if self.ended(PROCESS_END_MS) {
            self.pass_on();
        }
        if waited != WAIT_OBJECT_0 {
            self.stop("the remote load did not end, reason", EXIT_NOT_LOADED);
        }
        let mut module = 0;
        // SAFETY: the buffer outlives the call, and the handle is open.
        unsafe {
            GetExitCodeThread(remote, &raw mut module);
            CloseHandle(remote);
        }
        if module == 0 {
            self.stop("the remote load answered no module, reason", EXIT_NOT_LOADED);
        }
        say("the library loaded, module", module);
    }

    /// The entry's bytes and protection put back with the thread held, then the program goes on.
    fn give_back(&self, entry: usize, old: [u8; 2], protection: u32) {
        let mut done = 0;
        let mut unused = 0;
        // SAFETY: the thread is this program's, held across the write; the buffers outlive it.
        unsafe {
            SuspendThread(self.thread);
            if WriteProcessMemory(self.process, entry, old.as_ptr(), old.len(), &raw mut done) == 0 {
                self.stop("the entry would not go back, reason", EXIT_NOT_GIVEN_BACK);
            }
            FlushInstructionCache(self.process, entry, old.len());
            VirtualProtectEx(self.process, entry, old.len(), protection, &raw mut unused);
            ResumeThread(self.thread);
        }
        say("given back, old bytes", u32::from(old[0]) | u32::from(old[1]) << 8);
    }
}

/// The steps, in the order they run.
#[unsafe(no_mangle)]
pub extern "system" fn start() -> ! {
    // SAFETY: the command line lives to the process's end with a NUL last, and this process
    // takes it once, here, so the slice is the only way to it.
    let line = unsafe {
        let line = GetCommandLineA();
        let mut size = 0;
        while *line.add(size) != 0 {
            size += 1;
        }
        core::slice::from_raw_parts_mut(line, size + 1)
    };
    let Some((target, library)) = two_arguments(line) else {
        stop("usage: mech-start program library; reason", EXIT_USAGE);
    };
    let entry = entry_in_file(target);
    say("the file's entry", entry);
    let program = Program::make(target);
    let at = entry as usize;
    let old = program.read_entry(at);
    let protection = program.park(at);
    program.wait_for_the_park(entry);
    program.load(library);
    program.give_back(at, old, protection);
    // SAFETY: the handle stays open to this process's end.
    unsafe { WaitForSingleObject(program.process, FOREVER) };
    program.pass_on()
}
