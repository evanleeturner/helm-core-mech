// SPDX-License-Identifier: MIT
//! The three files' process calls under Wine (KP1, ruling 1407): the
//! original's calls, logged in the swap (the plan's "Win32 under Wine").
//! Each row calls the real import through the swap DLL's own import table,
//! then writes the front's `imp.call` line, `c` the name in
//! `mech_rows::process`, `from` the caller inside the file (wall 2), so the
//! host tests replay `kernel_process`' front from these lines. The front
//! itself never runs here: its `LoadLibraryA` loads nothing, and the
//! launcher's call to `SimMain` would miss.
//!
//! 67 exports, one a name a file imports: the launcher's 19 `LAUNCHER_`,
//! the shell's 24 `HC6_` (its `GetLastError` and `SetLastError` stay
//! [`crate::imports`]'), the game file's 24 `GAME_`. Each is a naked entry
//! that reads the address the call returns to first, as [`crate::threads`]'
//! are.
//!
//! The line goes first where the call may not return (`ExitProcess`,
//! `TerminateProcess`, `RaiseException`, `DebugBreak`, `RtlUnwind`), so it
//! has no `v`. A wide character past one byte shows as `\uNNNN`; an entry
//! asked by number as `#<number>`. The thread's last error is kept around
//! each line, as `imp_call!` keeps it.
use core::ffi::CStr;
use core::fmt;
use core::ptr::with_exposed_provenance;
use kernel_process::Text;
use kernel_process::table::Row;
use mech_rows::process::{GAME, LAUNCHER, SHELL};
use mech_trace::Args;

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
    fn FreeEnvironmentStringsA(block: u32) -> i32;
    fn FreeEnvironmentStringsW(block: u32) -> i32;
    fn FreeLibrary(module: u32) -> i32;
    fn GetCommandLineA() -> u32;
    fn GetCurrentProcess() -> u32;
    fn GetEnvironmentStrings() -> u32;
    fn GetEnvironmentStringsW() -> u32;
    fn GetModuleFileNameA(module: u32, into: u32, size: u32) -> u32;
    fn GetModuleHandleA(name: u32) -> u32;
    fn GetProcAddress(module: u32, name: u32) -> u32;
    fn GetStartupInfoA(record: u32);
    fn GetVersion() -> u32;
    fn IsBadCodePtr(address: u32) -> i32;
    fn IsBadReadPtr(address: u32, size: u32) -> i32;
    fn IsBadWritePtr(address: u32, size: u32) -> i32;
    fn LoadLibraryA(name: u32) -> u32;
    fn LoadModule(name: u32, parameters: u32) -> u32;
    fn OutputDebugStringA(text: u32);
    fn RaiseException(code: u32, flags: u32, count: u32, arguments: u32);
    fn RtlUnwind(frame: u32, target: u32, record: u32, value: u32);
    fn SetEnvironmentVariableA(name: u32, value: u32) -> i32;
    fn SetUnhandledExceptionFilter(filter: u32) -> u32;
    fn TerminateProcess(process: u32, code: u32) -> i32;
    fn UnhandledExceptionFilter(pointers: u32) -> i32;
}

/// One `imp.call` line, the thread's last error kept around it. Each row
/// writes its `log::debug!` whole, so the logline fence reads its fields.
macro_rules! kept {
    ($line:expr) => {{
        // SAFETY: reads the thread's own last error.
        let kept = unsafe { crate::GetLastError() };
        $line;
        // SAFETY: sets back the value read above.
        unsafe { crate::SetLastError(kept) };
    }};
}

/// The thread's last error as the call left it, read before any line.
fn left() -> u32 {
    // SAFETY: reads the thread's own last error.
    unsafe { crate::GetLastError() }
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

/// The environment block at `address` through the two NULs that end it,
/// both kept, as the front shows its row's; empty for a null one.
///
/// # Safety
/// `address` is 0 or a block Win32 answered.
unsafe fn block<'a, T: Copy + Default + PartialEq>(address: u32) -> &'a [T] {
    if address == 0 {
        return &[];
    }
    let start: *const T = with_exposed_provenance(address as usize);
    let nul = T::default();
    let mut count = 0;
    // SAFETY: Win32's block ends at its first two NULs in a row.
    while unsafe { (*start.add(count), *start.add(count + 1)) } != (nul, nul) {
        count += 1;
    }
    // SAFETY: the `count + 2` units read above.
    unsafe { core::slice::from_raw_parts(start, count + 2) }
}

/// Wide text as the line shows it: printable ASCII as its character, a
/// unit to `0xFF` as `\xNN`, any other `\uNNNN`.
struct Wide<'a>(&'a [u16]);

impl fmt::Display for Wide<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.iter().try_for_each(|unit| match unit {
            0x20..=0x7E => Text(&[unit.to_le_bytes()[0]]).fmt(f),
            0..=0xFF => write!(f, "\\x{unit:02x}"),
            _ => write!(f, "\\u{unit:04x}"),
        })
    }
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

extern "system" fn command_line<F: File>(from: u32) -> u32 {
    // SAFETY: takes no argument.
    let value = unsafe { GetCommandLineA() };
    // SAFETY: Win32's NUL-ended command line.
    let line = unsafe { text(value) };
    kept!(log::debug!(c = F::ROW.calls.command_line, v:% = Text(line),
        from:% = mech_trace::__shown!(from); "imp.call"));
    value
}

extern "system" fn version<F: File>(from: u32) -> u32 {
    // SAFETY: takes no argument.
    let value = unsafe { GetVersion() };
    kept!(
        log::debug!(c = F::ROW.calls.version, v:% = mech_trace::__shown!(value),
        from:% = mech_trace::__shown!(from); "imp.call")
    );
    value
}

extern "system" fn current_process<F: File>(from: u32) -> u32 {
    // SAFETY: takes no argument.
    let value = unsafe { GetCurrentProcess() };
    kept!(
        log::debug!(c = F::ROW.calls.current_process, v:% = mech_trace::__shown!(value),
        from:% = mech_trace::__shown!(from); "imp.call")
    );
    value
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
        from:% = mech_trace::__shown!(from); "imp.call")
    );
    value
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
        from:% = mech_trace::__shown!(from); "imp.call")
    );
    value
}

extern "system" fn proc_address<F: File>(from: u32, module: u32, name: u32) -> u32 {
    // SAFETY: as `module_handle`.
    let value = unsafe { GetProcAddress(module, name) };
    let error = left();
    kept!(
        log::debug!(c = F::ROW.calls.proc_address, a:% = Args([mech_trace::__shown!(module)]),
        q:% = Asked(name), v:% = mech_trace::__shown!(value), e:% = mech_trace::__shown!(error),
        from:% = mech_trace::__shown!(from); "imp.call")
    );
    value
}

extern "system" fn free_library<F: File>(from: u32, module: u32) -> i32 {
    // SAFETY: as `module_handle`.
    let value = unsafe { FreeLibrary(module) };
    let error = left();
    kept!(
        log::debug!(c = F::ROW.calls.free_library, a:% = Args([mech_trace::__shown!(module)]),
        v:% = mech_trace::__shown!(value), e:% = mech_trace::__shown!(error),
        from:% = mech_trace::__shown!(from); "imp.call")
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
        from:% = mech_trace::__shown!(from); "imp.call"));
    value
}

extern "system" fn last_error<F: File>(from: u32) -> u32 {
    let value = left();
    kept!(
        log::debug!(c = F::ROW.calls.last_error, v:% = mech_trace::__shown!(value),
        from:% = mech_trace::__shown!(from); "imp.call")
    );
    value
}

extern "system" fn set_last_error<F: File>(from: u32, code: u32) {
    // SAFETY: sets the thread's own last error.
    unsafe { crate::SetLastError(code) };
    kept!(
        log::debug!(c = F::ROW.calls.set_last_error, a:% = Args([mech_trace::__shown!(code)]),
        from:% = mech_trace::__shown!(from); "imp.call")
    );
}

extern "system" fn exit<F: File>(from: u32, code: u32) -> ! {
    kept!(
        log::debug!(c = F::ROW.calls.exit, a:% = Args([mech_trace::__shown!(code)]),
        from:% = mech_trace::__shown!(from); "imp.call")
    );
    // SAFETY: the caller's code, passed on.
    unsafe { ExitProcess(code) }
}

extern "system" fn terminate<F: File>(from: u32, process: u32, code: u32) -> i32 {
    kept!(log::debug!(c = F::ROW.calls.terminate,
        a:% = Args([mech_trace::__shown!(process), mech_trace::__shown!(code)]),
        from:% = mech_trace::__shown!(from); "imp.call"));
    // SAFETY: as `module_handle`.
    unsafe { TerminateProcess(process, code) }
}

extern "system" fn environment<F: File>(from: u32) -> u32 {
    // SAFETY: takes no argument.
    let value = unsafe { GetEnvironmentStrings() };
    // SAFETY: Win32's block.
    let bytes = unsafe { block::<u8>(value) };
    kept!(log::debug!(c = F::ROW.calls.environment, s:% = Text(bytes),
        from:% = mech_trace::__shown!(from); "imp.call"));
    value
}

extern "system" fn environment_w<F: File>(from: u32) -> u32 {
    // SAFETY: takes no argument.
    let value = unsafe { GetEnvironmentStringsW() };
    // SAFETY: Win32's wide block.
    let units = unsafe { block::<u16>(value) };
    kept!(
        log::debug!(c = F::ROW.calls.environment_w, s:% = Wide(units),
        from:% = mech_trace::__shown!(from); "imp.call")
    );
    value
}

extern "system" fn free_environment<F: File>(from: u32, block: u32) -> i32 {
    // SAFETY: as `module_handle`.
    let value = unsafe { FreeEnvironmentStringsA(block) };
    kept!(
        log::debug!(c = F::ROW.calls.free_environment, a:% = Args([mech_trace::__shown!(block)]),
        v:% = mech_trace::__shown!(value),
        from:% = mech_trace::__shown!(from); "imp.call")
    );
    value
}

extern "system" fn free_environment_w<F: File>(from: u32, block: u32) -> i32 {
    // SAFETY: as `module_handle`.
    let value = unsafe { FreeEnvironmentStringsW(block) };
    kept!(log::debug!(c = F::ROW.calls.free_environment_w,
        a:% = Args([mech_trace::__shown!(block)]),
        v:% = mech_trace::__shown!(value),
        from:% = mech_trace::__shown!(from); "imp.call"));
    value
}

extern "system" fn startup_info<F: File>(from: u32, record: u32) {
    // SAFETY: as `module_handle`.
    unsafe { GetStartupInfoA(record) };
    let at = |offset: usize| -> *const u8 { with_exposed_provenance(record as usize + offset) };
    // SAFETY: the 68-byte record Win32 wrote: `dwFlags` at 44,
    // `wShowWindow` at 48, `cbReserved2` at 50, `lpReserved2` at 52.
    let (flags, show_window, reserved_count, reserved) = unsafe {
        (
            at(44).cast::<u32>().read_unaligned(),
            at(48).cast::<u16>().read_unaligned(),
            at(50).cast::<u16>().read_unaligned(),
            at(52).cast::<u32>().read_unaligned(),
        )
    };
    kept!(log::debug!(c = F::ROW.calls.startup_info, r:% = Args([
        mech_trace::__shown!(flags),
        mech_trace::__shown!(show_window),
        mech_trace::__shown!(reserved_count),
        mech_trace::__shown!(reserved),
    ]),
        from:% = mech_trace::__shown!(from); "imp.call"));
}

extern "system" fn set_environment<F: File>(from: u32, name: u32, value: u32) -> i32 {
    // SAFETY: as `module_handle`.
    let answer = unsafe { SetEnvironmentVariableA(name, value) };
    // SAFETY: the caller's name, and its value or null.
    let (name, value) = unsafe { (text(name), (value != 0).then(|| text(value))) };
    kept!(
        log::debug!(c = F::ROW.calls.set_environment, s:% = Variable(name, value),
        v:% = mech_trace::__shown!(answer),
        from:% = mech_trace::__shown!(from); "imp.call")
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
        from:% = mech_trace::__shown!(from); "imp.call")
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
        from:% = mech_trace::__shown!(from); "imp.call")
    );
    value
}

extern "system" fn debug_string<F: File>(from: u32, text_at: u32) {
    // SAFETY: as `module_handle`.
    unsafe { OutputDebugStringA(text_at) };
    // SAFETY: the caller's text.
    let shown = unsafe { text(text_at) };
    kept!(
        log::debug!(c = F::ROW.calls.debug_string, s:% = Text(shown),
        from:% = mech_trace::__shown!(from); "imp.call")
    );
}

extern "system" fn debug_break<F: File>(from: u32) {
    kept!(log::debug!(c = F::ROW.calls.debug_break,
        from:% = mech_trace::__shown!(from); "imp.call"));
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
        from:% = mech_trace::__shown!(from); "imp.call"));
    // SAFETY: as `module_handle`.
    unsafe { RaiseException(code, flags, count, arguments) };
}

extern "system" fn set_filter<F: File>(from: u32, filter: u32) -> u32 {
    // SAFETY: as `module_handle`.
    let value = unsafe { SetUnhandledExceptionFilter(filter) };
    kept!(
        log::debug!(c = F::ROW.calls.set_filter, a:% = Args([mech_trace::__shown!(filter)]),
        v:% = mech_trace::__shown!(value),
        from:% = mech_trace::__shown!(from); "imp.call")
    );
    value
}

extern "system" fn unhandled_filter<F: File>(from: u32, pointers: u32) -> i32 {
    // SAFETY: as `module_handle`.
    let value = unsafe { UnhandledExceptionFilter(pointers) };
    kept!(log::debug!(c = F::ROW.calls.unhandled_filter,
        a:% = Args([mech_trace::__shown!(pointers)]),
        v:% = mech_trace::__shown!(value),
        from:% = mech_trace::__shown!(from); "imp.call"));
    value
}

extern "system" fn bad_read<F: File>(from: u32, address: u32, size: u32) -> i32 {
    // SAFETY: as `module_handle`.
    let value = unsafe { IsBadReadPtr(address, size) };
    kept!(log::debug!(c = F::ROW.calls.bad_read,
        a:% = Args([mech_trace::__shown!(address), mech_trace::__shown!(size)]),
        v:% = mech_trace::__shown!(value),
        from:% = mech_trace::__shown!(from); "imp.call"));
    value
}

extern "system" fn bad_write<F: File>(from: u32, address: u32, size: u32) -> i32 {
    // SAFETY: as `module_handle`.
    let value = unsafe { IsBadWritePtr(address, size) };
    kept!(log::debug!(c = F::ROW.calls.bad_write,
        a:% = Args([mech_trace::__shown!(address), mech_trace::__shown!(size)]),
        v:% = mech_trace::__shown!(value),
        from:% = mech_trace::__shown!(from); "imp.call"));
    value
}

extern "system" fn bad_code<F: File>(from: u32, address: u32) -> i32 {
    // SAFETY: as `module_handle`.
    let value = unsafe { IsBadCodePtr(address) };
    kept!(
        log::debug!(c = F::ROW.calls.bad_code, a:% = Args([mech_trace::__shown!(address)]),
        v:% = mech_trace::__shown!(value),
        from:% = mech_trace::__shown!(from); "imp.call")
    );
    value
}

extern "system" fn unwind<F: File>(from: u32, frame: u32, target: u32, record: u32, value: u32) {
    kept!(log::debug!(c = F::ROW.calls.unwind, a:% = Args([
        mech_trace::__shown!(frame),
        mech_trace::__shown!(target),
        mech_trace::__shown!(record),
        mech_trace::__shown!(value),
    ]),
        from:% = mech_trace::__shown!(from); "imp.call"));
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
