// SPDX-License-Identifier: MIT
//! The three files' process calls under Wine.
//!
//! 15 of the 29 names are answered by `kernel_process`' front in the file's
//! row, over [`Win32`]: each answer equals what the census's run read, or
//! differs where nothing reads it, and no side effect of Wine's is lost. The
//! front writes that call's one `imp.call` line, `by=kernel-process`, so these
//! lines have no `from`. The row keeps its naked entry and takes `_from`.
//! `GetEnvironmentStrings` and its wide twin differ on purpose: the row's block
//! is empty, so the game reads none of the box's variables.
//!
//! The other 14 stay the original's: the row calls the real import through the
//! swap DLL's own import table, then writes the front's `imp.call` line, `c`
//! the name in `mech_rows::process`, `from` the caller inside the file (wall
//! 2), so the host tests replay `kernel_process`' front from these lines. The
//! front cannot answer them: its `LoadLibraryA` loads nothing, and the
//! launcher's call to `SimMain` would miss.
//!
//! 69 exports, one a name a file imports: the launcher's 19 `LAUNCHER_`,
//! the shell's 26 `HC6_`, the game file's 24 `GAME_`. Each is a naked entry
//! that reads the address the call returns to first, as [`crate::threads`]'
//! are.
//!
//! The line goes first where the call may not return (`ExitProcess`,
//! `TerminateProcess`, `RaiseException`, `DebugBreak`, `RtlUnwind`), so it
//! has no `v`. An entry asked by number shows as `#<number>`. The thread's last
//! error is kept around each line, as `imp_call!` keeps it.
use core::ffi::CStr;
use core::fmt;
use core::ptr::{null_mut, with_exposed_provenance, with_exposed_provenance_mut};
use core::sync::atomic::{AtomicPtr, Ordering};
use kernel_process::table::{Module, Row};
use kernel_process::{Access, Front, Process, Text};
use mech_rows::process::{GAME, LAUNCHER, SHELL};
use mech_trace::Args;

use crate::left;

unsafe extern "system" {
    fn CreateProcessA(
        application: u32,
        command: u32,
        process_attributes: u32,
        thread_attributes: u32,
        inherit: i32,
        flags: u32,
        environment: u32,
        directory: u32,
        startup: u32,
        information: u32,
    ) -> i32;
    fn DebugBreak();
    fn ExitProcess(code: u32) -> !;
    fn FreeLibrary(module: u32) -> i32;
    fn GetModuleFileNameA(module: u32, into: u32, size: u32) -> u32;
    fn GetModuleHandleA(name: u32) -> u32;
    fn GetProcAddress(module: u32, name: u32) -> u32;
    fn IsBadCodePtr(address: u32) -> i32;
    fn IsBadReadPtr(address: u32, size: u32) -> i32;
    fn IsBadWritePtr(address: u32, size: u32) -> i32;
    fn LoadLibraryA(name: u32) -> u32;
    fn LoadModule(name: u32, parameters: u32) -> u32;
    fn RaiseException(code: u32, flags: u32, count: u32, arguments: u32);
    fn RtlUnwind(frame: u32, target: u32, record: u32, value: u32);
    fn SetEnvironmentVariableA(name: u32, value: u32) -> i32;
    fn SetUnhandledExceptionFilter(filter: u32) -> u32;
    fn TerminateProcess(process: u32, code: u32) -> i32;
    fn UnhandledExceptionFilter(pointers: u32) -> i32;
}

/// A file the rows serve, by its row in `mech_rows::process`.
trait File {
    const ROW: &'static Row;
}

/// `mercswin`, the launcher.
enum Launcher {}
/// `mw2shell`, the shell.
enum Shell {}
/// `mercsw`, the game file.
enum Game {}

impl File for Launcher {
    const ROW: &'static Row = &LAUNCHER;
}

impl File for Shell {
    const ROW: &'static Row = &SHELL;
}

impl File for Game {
    const ROW: &'static Row = &GAME;
}

/// The back the 15 flipped rows answer over: the thread's own last error, the
/// real end and Wine's memory map. A row makes a [`Front`] for each call, since
/// none of them touches the state a front keeps, its filter.
struct Win32;

impl Process for Win32 {
    fn entry(&self, _module: &Module, _name: &str) -> u32 {
        // No flipped row asks: `GetProcAddress` stays the original's.
        0
    }

    fn end(&self, code: u32) {
        // SAFETY: the caller's code, passed on.
        unsafe { ExitProcess(code) }
    }

    fn fail(&self, code: u32) {
        // SAFETY: sets the thread's own last error.
        unsafe { crate::SetLastError(code) };
    }

    fn last_error(&self) -> u32 {
        left()
    }

    fn reaches(&self, address: u32, size: u32, access: Access) -> bool {
        // SAFETY: Wine's 3 answer from its map and fault on nothing, whatever
        // the caller's address; 0 means good.
        let bad = unsafe {
            match access {
                Access::Read => IsBadReadPtr(address, size),
                Access::Write => IsBadWritePtr(address, size),
                Access::Run => IsBadCodePtr(address),
            }
        };
        bad == 0
    }

    fn quiet(&self, line: impl FnOnce()) {
        kept!(line());
    }
}

/// The rows' command line with the NUL its reader wants; a row's `&str` has
/// none. One for all 3 files, since `mech_rows`' test holds their text the same.
static COMMAND_LINE: [u8; LAUNCHER.command_line.len() + 1] = {
    let mut line = [0; LAUNCHER.command_line.len() + 1];
    let text = LAUNCHER.command_line.as_bytes();
    let mut at = 0;
    while at < text.len() {
        line[at] = text[at];
        at += 1;
    }
    line
};

/// The rows' environment block, each byte one UTF-16 unit as the front's iterator
/// gives it, at an address the caller can read. One for all 3, as [`COMMAND_LINE`].
static ENVIRONMENT_W: [u16; LAUNCHER.environment.len()] = {
    let mut wide = [0; LAUNCHER.environment.len()];
    let mut at = 0;
    while at < wide.len() {
        wide[at] = LAUNCHER.environment[at] as u16;
        at += 1;
    }
    wide
};

/// The NUL-ended text at `address`, empty for a null one.
///
/// # Safety
/// `address` is 0 or a NUL-ended text the caller gave or Win32 answered.
unsafe fn text<'a>(address: u32) -> &'a [u8] {
    if address == 0 {
        return &[];
    }
    // SAFETY: the caller's NUL-ended text, as Win32 takes it.
    unsafe { CStr::from_ptr(with_exposed_provenance(address as usize)) }.to_bytes()
}

/// An environment variable as `SetEnvironmentVariableA` was given it:
/// `NAME=VALUE`, or the name alone for a variable taken away.
struct Variable<'a>(&'a [u8], Option<&'a [u8]>);

impl fmt::Display for Variable<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Text(self.0).fmt(f)?;
        self.1.map_or(Ok(()), |value| write!(f, "={}", Text(value)))
    }
}

/// The entry `GetProcAddress` was asked: its name, or `#<number>` for an
/// address below `0x1_0000`, which Win32 reads as a number.
struct Asked(u32);

impl fmt::Display for Asked {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 < 0x1_0000 {
            return write!(f, "#{}", self.0);
        }
        // SAFETY: a name, by Win32's rule above.
        Text(unsafe { text(self.0) }).fmt(f)
    }
}

extern "system" fn command_line<F: File>(_from: u32) -> u32 {
    let _ = Front::new(F::ROW, Win32).command_line();
    COMMAND_LINE.as_ptr().expose_provenance() as u32
}

extern "system" fn version<F: File>(_from: u32) -> u32 {
    Front::new(F::ROW, Win32).version()
}

extern "system" fn current_process<F: File>(_from: u32) -> u32 {
    Front::new(F::ROW, Win32).current_process()
}

extern "system" fn module_handle<F: File>(from: u32, name: u32) -> u32 {
    // SAFETY: the caller's arguments, passed on whole.
    let value = unsafe { GetModuleHandleA(name) };
    let error = left();
    // SAFETY: the caller's name or null.
    let asked = unsafe { text(name) };
    kept!(
        log::debug!(c = F::ROW.calls.module_handle, m:% = Text(asked),
        v:% = mech_trace::__shown!(value), e:% = mech_trace::__shown!(error),
        from:% = mech_trace::__shown!(from), by = "orig"; "imp.call")
    );
    value
}

/// What runs after each real `LoadLibraryA` a file makes, before its caller sees the module:
/// the swap's bind. The swap sets it from its `DllMain`, because the swap depends
/// on the layer and so the layer cannot name it. Null runs nothing, as under the rig's gdb.
static AFTER_LOAD: AtomicPtr<()> = AtomicPtr::new(null_mut());

/// Names the swap's bind to [`load_library`].
pub fn set_after_load(hook: fn(u32)) {
    AFTER_LOAD.store(hook as *mut (), Ordering::Release);
}

/// This is the one bind point. The page asked for 2, `LoadLibraryA` and `GetProcAddress`;
/// this return already sees all a later `GetProcAddress` would, the module mapped and its
/// `DllMain` run, and no call of the caller's comes between.
fn after_load(module: u32) {
    let hook = AFTER_LOAD.load(Ordering::Acquire);
    if !hook.is_null() {
        // SAFETY: only `set_after_load` stores here, and what it stores is a `fn(u32)`.
        let hook = unsafe { core::mem::transmute::<*mut (), fn(u32)>(hook) };
        hook(module);
    }
}

extern "system" fn load_library<F: File>(from: u32, name: u32) -> u32 {
    // SAFETY: as `module_handle`.
    let value = unsafe { LoadLibraryA(name) };
    let error = left();
    // SAFETY: the caller's name.
    let asked = unsafe { text(name) };
    kept!(
        log::debug!(c = F::ROW.calls.load_library, m:% = Text(asked),
        v:% = mech_trace::__shown!(value), e:% = mech_trace::__shown!(error),
        from:% = mech_trace::__shown!(from), by = "orig"; "imp.call")
    );
    kept!(after_load(value));
    value
}

extern "system" fn proc_address<F: File>(from: u32, module: u32, name: u32) -> u32 {
    // SAFETY: as `module_handle`.
    let value = unsafe { GetProcAddress(module, name) };
    let error = left();
    kept!(
        log::debug!(c = F::ROW.calls.proc_address, a:% = Args([mech_trace::__shown!(module)]),
        q:% = Asked(name), v:% = mech_trace::__shown!(value), e:% = mech_trace::__shown!(error),
        from:% = mech_trace::__shown!(from), by = "orig"; "imp.call")
    );
    value
}

extern "system" fn free_library<F: File>(from: u32, module: u32) -> i32 {
    // SAFETY: as `module_handle`.
    let value = unsafe { FreeLibrary(module) };
    let error = left();
    crate::slots::forget();
    kept!(
        log::debug!(c = F::ROW.calls.free_library, a:% = Args([mech_trace::__shown!(module)]),
        v:% = mech_trace::__shown!(value), e:% = mech_trace::__shown!(error),
        from:% = mech_trace::__shown!(from), by = "orig"; "imp.call")
    );
    value
}

extern "system" fn module_file_name<F: File>(from: u32, module: u32, into: u32, size: u32) -> u32 {
    // SAFETY: as `module_handle`.
    let value = unsafe { GetModuleFileNameA(module, into, size) };
    let error = left();
    let count = value.min(size) as usize;
    let written: &[u8] = if into == 0 {
        &[]
    } else {
        // SAFETY: the `count` bytes Win32 wrote into the caller's buffer.
        unsafe { core::slice::from_raw_parts(with_exposed_provenance(into as usize), count) }
    };
    kept!(log::debug!(c = F::ROW.calls.module_file_name,
        a:% = Args([mech_trace::__shown!(module), mech_trace::__shown!(size)]),
        m:% = Text(written), v:% = mech_trace::__shown!(value), e:% = mech_trace::__shown!(error),
        from:% = mech_trace::__shown!(from), by = "orig"; "imp.call"));
    value
}

extern "system" fn last_error<F: File>(_from: u32) -> u32 {
    Front::new(F::ROW, Win32).last_error()
}

extern "system" fn set_last_error<F: File>(_from: u32, code: u32) {
    Front::new(F::ROW, Win32).set_last_error(code);
}

extern "system" fn exit<F: File>(_from: u32, code: u32) {
    Front::new(F::ROW, Win32).exit(code);
}

extern "system" fn terminate<F: File>(from: u32, process: u32, code: u32) -> i32 {
    kept!(log::debug!(c = F::ROW.calls.terminate,
        a:% = Args([mech_trace::__shown!(process), mech_trace::__shown!(code)]),
        from:% = mech_trace::__shown!(from), by = "orig"; "imp.call"));
    // SAFETY: as `module_handle`.
    unsafe { TerminateProcess(process, code) }
}

extern "system" fn environment<F: File>(_from: u32) -> u32 {
    // The row's own bytes: its block ends in 2 NULs, as `mech_rows`' test holds.
    Front::new(F::ROW, Win32)
        .environment()
        .as_ptr()
        .expose_provenance() as u32
}

extern "system" fn environment_w<F: File>(_from: u32) -> u32 {
    // The front's iterator has no address to hand back; called for its line.
    let _ = Front::new(F::ROW, Win32).environment_w();
    ENVIRONMENT_W.as_ptr().expose_provenance() as u32
}

extern "system" fn free_environment<F: File>(_from: u32, block: u32) -> i32 {
    Front::new(F::ROW, Win32).free_environment(block)
}

extern "system" fn free_environment_w<F: File>(_from: u32, block: u32) -> i32 {
    Front::new(F::ROW, Win32).free_environment_w(block)
}

extern "system" fn startup_info<F: File>(_from: u32, record: u32) {
    let answer = Front::new(F::ROW, Win32).startup_info();
    let at = |offset: usize| -> *mut u8 { with_exposed_provenance_mut(record as usize + offset) };
    // SAFETY: the caller's 68-byte record, as Win32 takes it: `cb` at 0,
    // `dwFlags` at 44, `wShowWindow` at 48, `cbReserved2` at 50, `lpReserved2` at 52.
    unsafe {
        at(0).write_bytes(0, 68);
        at(0).cast::<u32>().write_unaligned(68);
        at(44).cast::<u32>().write_unaligned(answer.flags);
        at(48).cast::<u16>().write_unaligned(answer.show_window);
        at(50).cast::<u16>().write_unaligned(answer.reserved_count);
        at(52).cast::<u32>().write_unaligned(answer.reserved);
    }
}

extern "system" fn set_environment<F: File>(from: u32, name: u32, value: u32) -> i32 {
    // SAFETY: as `module_handle`.
    let answer = unsafe { SetEnvironmentVariableA(name, value) };
    // SAFETY: the caller's name, and its value or null.
    let (name, value) = unsafe { (text(name), (value != 0).then(|| text(value))) };
    kept!(
        log::debug!(c = F::ROW.calls.set_environment, s:% = Variable(name, value),
        v:% = mech_trace::__shown!(answer),
        from:% = mech_trace::__shown!(from), by = "orig"; "imp.call")
    );
    answer
}

extern "system" fn load_module<F: File>(from: u32, name: u32, parameters: u32) -> u32 {
    // SAFETY: as `module_handle`.
    let value = unsafe { LoadModule(name, parameters) };
    let error = left();
    // SAFETY: the caller's name.
    let asked = unsafe { text(name) };
    kept!(
        log::debug!(c = F::ROW.calls.load_module, a:% = Args([mech_trace::__shown!(parameters)]),
        m:% = Text(asked), v:% = mech_trace::__shown!(value), e:% = mech_trace::__shown!(error),
        from:% = mech_trace::__shown!(from), by = "orig"; "imp.call")
    );
    value
}

#[allow(
    clippy::too_many_arguments,
    reason = "CreateProcessA's own 10, and the caller first"
)]
extern "system" fn create_process<F: File>(
    from: u32,
    application: u32,
    command: u32,
    process_attributes: u32,
    thread_attributes: u32,
    inherit: i32,
    flags: u32,
    environment: u32,
    directory: u32,
    startup: u32,
    information: u32,
) -> i32 {
    // SAFETY: as `module_handle`.
    let value = unsafe {
        CreateProcessA(
            application,
            command,
            process_attributes,
            thread_attributes,
            inherit,
            flags,
            environment,
            directory,
            startup,
            information,
        )
    };
    let error = left();
    // SAFETY: the caller's two texts, each or null.
    let (application, command) = unsafe { (text(application), text(command)) };
    kept!(
        log::debug!(c = F::ROW.calls.create_process, m:% = Text(application), s:% = Text(command),
        v:% = mech_trace::__shown!(value), e:% = mech_trace::__shown!(error),
        from:% = mech_trace::__shown!(from), by = "orig"; "imp.call")
    );
    value
}

extern "system" fn debug_string<F: File>(_from: u32, text_at: u32) {
    // SAFETY: the caller's text.
    Front::new(F::ROW, Win32).debug_string(unsafe { text(text_at) });
}

extern "system" fn debug_break<F: File>(from: u32) {
    kept!(log::debug!(c = F::ROW.calls.debug_break,
        from:% = mech_trace::__shown!(from), by = "orig"; "imp.call"));
    // SAFETY: takes no argument.
    unsafe { DebugBreak() };
}

extern "system" fn raise<F: File>(from: u32, code: u32, flags: u32, count: u32, arguments: u32) {
    kept!(log::debug!(c = F::ROW.calls.raise, a:% = Args([
        mech_trace::__shown!(code),
        mech_trace::__shown!(flags),
        mech_trace::__shown!(count),
        mech_trace::__shown!(arguments),
    ]),
        from:% = mech_trace::__shown!(from), by = "orig"; "imp.call"));
    // SAFETY: as `module_handle`.
    unsafe { RaiseException(code, flags, count, arguments) };
}

extern "system" fn set_filter<F: File>(from: u32, filter: u32) -> u32 {
    // SAFETY: as `module_handle`.
    let value = unsafe { SetUnhandledExceptionFilter(filter) };
    kept!(
        log::debug!(c = F::ROW.calls.set_filter, a:% = Args([mech_trace::__shown!(filter)]),
        v:% = mech_trace::__shown!(value),
        from:% = mech_trace::__shown!(from), by = "orig"; "imp.call")
    );
    value
}

extern "system" fn unhandled_filter<F: File>(from: u32, pointers: u32) -> i32 {
    // SAFETY: as `module_handle`.
    let value = unsafe { UnhandledExceptionFilter(pointers) };
    kept!(log::debug!(c = F::ROW.calls.unhandled_filter,
        a:% = Args([mech_trace::__shown!(pointers)]),
        v:% = mech_trace::__shown!(value),
        from:% = mech_trace::__shown!(from), by = "orig"; "imp.call"));
    value
}

extern "system" fn bad_read<F: File>(_from: u32, address: u32, size: u32) -> i32 {
    Front::new(F::ROW, Win32).bad_read(address, size)
}

extern "system" fn bad_write<F: File>(_from: u32, address: u32, size: u32) -> i32 {
    Front::new(F::ROW, Win32).bad_write(address, size)
}

extern "system" fn bad_code<F: File>(_from: u32, address: u32) -> i32 {
    Front::new(F::ROW, Win32).bad_code(address)
}

extern "system" fn unwind<F: File>(from: u32, frame: u32, target: u32, record: u32, value: u32) {
    kept!(log::debug!(c = F::ROW.calls.unwind, a:% = Args([
        mech_trace::__shown!(frame),
        mech_trace::__shown!(target),
        mech_trace::__shown!(record),
        mech_trace::__shown!(value),
    ]),
        from:% = mech_trace::__shown!(from), by = "orig"; "imp.call"));
    // SAFETY: as `module_handle`.
    unsafe { RtlUnwind(frame, target, record, value) };
}

entry!(LAUNCHER_ExitProcess, exit::<Launcher>, 1);
entry!(
    LAUNCHER_FreeEnvironmentStringsA,
    free_environment::<Launcher>,
    1
);
entry!(
    LAUNCHER_FreeEnvironmentStringsW,
    free_environment_w::<Launcher>,
    1
);
entry!(LAUNCHER_FreeLibrary, free_library::<Launcher>, 1);
entry!(LAUNCHER_GetCommandLineA, command_line::<Launcher>, 0);
entry!(LAUNCHER_GetCurrentProcess, current_process::<Launcher>, 0);
entry!(LAUNCHER_GetEnvironmentStrings, environment::<Launcher>, 0);
entry!(
    LAUNCHER_GetEnvironmentStringsW,
    environment_w::<Launcher>,
    0
);
entry!(LAUNCHER_GetLastError, last_error::<Launcher>, 0);
entry!(LAUNCHER_GetModuleFileNameA, module_file_name::<Launcher>, 3);
entry!(LAUNCHER_GetModuleHandleA, module_handle::<Launcher>, 1);
entry!(LAUNCHER_GetProcAddress, proc_address::<Launcher>, 2);
entry!(LAUNCHER_GetStartupInfoA, startup_info::<Launcher>, 1);
entry!(LAUNCHER_GetVersion, version::<Launcher>, 0);
entry!(LAUNCHER_LoadLibraryA, load_library::<Launcher>, 1);
entry!(LAUNCHER_OutputDebugStringA, debug_string::<Launcher>, 1);
entry!(LAUNCHER_RtlUnwind, unwind::<Launcher>, 4);
entry!(LAUNCHER_TerminateProcess, terminate::<Launcher>, 2);
entry!(
    LAUNCHER_UnhandledExceptionFilter,
    unhandled_filter::<Launcher>,
    1
);

entry!(HC6_CreateProcessA, create_process::<Shell>, 10);
entry!(HC6_ExitProcess, exit::<Shell>, 1);
entry!(HC6_FreeEnvironmentStringsA, free_environment::<Shell>, 1);
entry!(HC6_FreeEnvironmentStringsW, free_environment_w::<Shell>, 1);
entry!(HC6_GetCommandLineA, command_line::<Shell>, 0);
entry!(HC6_GetCurrentProcess, current_process::<Shell>, 0);
entry!(HC6_GetEnvironmentStrings, environment::<Shell>, 0);
entry!(HC6_GetEnvironmentStringsW, environment_w::<Shell>, 0);
entry!(HC6_GetLastError, last_error::<Shell>, 0);
entry!(HC6_GetModuleFileNameA, module_file_name::<Shell>, 3);
entry!(HC6_GetModuleHandleA, module_handle::<Shell>, 1);
entry!(HC6_GetProcAddress, proc_address::<Shell>, 2);
entry!(HC6_GetStartupInfoA, startup_info::<Shell>, 1);
entry!(HC6_GetVersion, version::<Shell>, 0);
entry!(HC6_IsBadCodePtr, bad_code::<Shell>, 1);
entry!(HC6_IsBadReadPtr, bad_read::<Shell>, 2);
entry!(HC6_IsBadWritePtr, bad_write::<Shell>, 2);
entry!(HC6_LoadLibraryA, load_library::<Shell>, 1);
entry!(HC6_LoadModule, load_module::<Shell>, 2);
entry!(HC6_OutputDebugStringA, debug_string::<Shell>, 1);
entry!(HC6_RaiseException, raise::<Shell>, 4);
entry!(HC6_RtlUnwind, unwind::<Shell>, 4);
entry!(HC6_SetEnvironmentVariableA, set_environment::<Shell>, 2);
entry!(HC6_SetLastError, set_last_error::<Shell>, 1);
entry!(HC6_SetUnhandledExceptionFilter, set_filter::<Shell>, 1);
entry!(HC6_TerminateProcess, terminate::<Shell>, 2);

entry!(GAME_DebugBreak, debug_break::<Game>, 0);
entry!(GAME_ExitProcess, exit::<Game>, 1);
entry!(GAME_FreeEnvironmentStringsA, free_environment::<Game>, 1);
entry!(GAME_FreeEnvironmentStringsW, free_environment_w::<Game>, 1);
entry!(GAME_GetCommandLineA, command_line::<Game>, 0);
entry!(GAME_GetCurrentProcess, current_process::<Game>, 0);
entry!(GAME_GetEnvironmentStrings, environment::<Game>, 0);
entry!(GAME_GetEnvironmentStringsW, environment_w::<Game>, 0);
entry!(GAME_GetLastError, last_error::<Game>, 0);
entry!(GAME_GetModuleFileNameA, module_file_name::<Game>, 3);
entry!(GAME_GetModuleHandleA, module_handle::<Game>, 1);
entry!(GAME_GetProcAddress, proc_address::<Game>, 2);
entry!(GAME_GetStartupInfoA, startup_info::<Game>, 1);
entry!(GAME_GetVersion, version::<Game>, 0);
entry!(GAME_IsBadReadPtr, bad_read::<Game>, 2);
entry!(GAME_IsBadWritePtr, bad_write::<Game>, 2);
entry!(GAME_LoadLibraryA, load_library::<Game>, 1);
entry!(GAME_LoadModule, load_module::<Game>, 2);
entry!(GAME_OutputDebugStringA, debug_string::<Game>, 1);
entry!(GAME_RaiseException, raise::<Game>, 4);
entry!(GAME_RtlUnwind, unwind::<Game>, 4);
entry!(GAME_SetEnvironmentVariableA, set_environment::<Game>, 2);
entry!(GAME_SetLastError, set_last_error::<Game>, 1);
entry!(GAME_TerminateProcess, terminate::<Game>, 2);
