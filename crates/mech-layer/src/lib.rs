// SPDX-License-Identifier: MIT
//! The layer (HC6, `JOB.md` I4): a Rust function for each Windows and
//! Miles import the shell's screens call, and one for each Smacker thunk.
//! Each calls the real function, then writes one DEBUG line, `imp.call`:
//! the import as the counter spells it (`KERNEL32!HeapAlloc`), its
//! arguments in order, the value it returned. Nothing is reimplemented:
//! the real function answers every call, through Wine.
//!
//! The 18 imports ([`imports`]), the 11 file calls ([`files`], HC9:
//! `kernel_files`' seam over the Windows folder) and the 5 time calls
//! ([`time`], KT1: `kernel_time`'s seam over Windows' clocks) and the 9
//! thread calls ([`threads`], KH1: `kernel_threads`' seam over Windows'
//! threads) and the 24 process calls ([`process`], KP1: the original's,
//! logged as `kernel_process`' front logs them) and the 13 text calls ([`text`], KX1:
//! the original's, logged as `kernel_text`'s front logs them) and the 5 registry calls
//! ([`registry`], KR1: the original's, logged as `kernel_registry`'s front
//! logs them) and the 19 window calls ([`window`], WN1: the original's,
//! logged with their callers for `window`'s fronts) and the 7 input calls
//! ([`input`], WI1: the same, for `window-input`'s fronts) and the 19
//! drawing calls ([`draw`], DB1: the same, for `draw`'s fronts), 130 in all,
//! are exported `HC6_<name>`: with
//! `HC6_LAYER=all` the swap's loader points the shell's own import slot at
//! each, and each calls the real import through the swap DLL's own import
//! table, so no call loops back through the shell's slot. The six Smacker
//! thunks ([`smack`]) are exported `HC5_<address>` and swapped as the
//! ports are: the shell imports smackw32 by ordinal, through thunks the
//! Ghidra index names, and each calls the real function through the
//! shell's slot, which the loader never writes.
//!
//! The C library inside the DLL is not an import: its functions the
//! screens call get stand-ins ([`crt`], ruling 1381), exported
//! `HC5_<address>` and swapped as the ports are, which do the job with
//! Rust's own code and log the same `imp.call` line.
//!
//! The thread's last error is read after the call and set back after the
//! line is written, so the sink's `WriteFile` cannot change what the
//! caller reads (`TlsGetValue` and `CreateFileA` set it on success).
//!
//! 32-bit x86 Windows only; on any other target the crate is empty.
#![no_std]

#[cfg(all(target_arch = "x86", target_os = "windows"))]
unsafe extern "system" {
    fn GetLastError() -> u32;
    fn SetLastError(code: u32);
}

/// `imp.call`: an import's call, logged after it returns; the value comes
/// back as the call gave it, the thread's last error as the call left it.
#[cfg(all(target_arch = "x86", target_os = "windows"))]
macro_rules! imp_call {
    ($c:literal, [], $value:expr) => {{
        let value = $value;
        // SAFETY: reads the thread's own last error.
        let error = unsafe { $crate::GetLastError() };
        log::debug!(c = $c, v:% = mech_trace::__shown!(value); "imp.call");
        // SAFETY: sets back the value read above.
        unsafe { $crate::SetLastError(error) };
        value
    }};
    ($c:literal, [$($argument:expr),+]) => {{
        // SAFETY: reads the thread's own last error.
        let error = unsafe { $crate::GetLastError() };
        log::debug!(
            c = $c, a:% = mech_trace::Args([$(mech_trace::__shown!($argument)),+]); "imp.call"
        );
        // SAFETY: sets back the value read above.
        unsafe { $crate::SetLastError(error) };
    }};
    ($c:literal, [$($argument:expr),+], $value:expr) => {{
        let value = $value;
        // SAFETY: reads the thread's own last error.
        let error = unsafe { $crate::GetLastError() };
        log::debug!(
            c = $c, a:% = mech_trace::Args([$(mech_trace::__shown!($argument)),+]),
            v:% = mech_trace::__shown!(value); "imp.call"
        );
        // SAFETY: sets back the value read above.
        unsafe { $crate::SetLastError(error) };
        value
    }};
}

/// A naked entry `export` for a stdcall of `count` arguments: pushes the
/// caller's arguments again, then the address the call returns to, so
/// `inner` takes the caller first and pops what it was pushed; the entry
/// returns past the caller's arguments, `inner`'s answer still in `eax`.
/// Each push reads `[esp + 4 * count]`: every push moves the next one
/// down there. `extern "C"` names the symbol plainly, with no `@` count;
/// the code keeps stdcall's contract. Past 2 arguments, `.rept` writes the
/// `count + 1` pushes.
#[cfg(all(target_arch = "x86", target_os = "windows"))]
macro_rules! entry {
    ($export:ident, $inner:path, 0) => {
        #[unsafe(naked)]
        #[unsafe(no_mangle)]
        #[allow(non_snake_case)]
        unsafe extern "C" fn $export() {
            core::arch::naked_asm!("push dword ptr [esp]", "call {inner}", "ret", inner = sym $inner)
        }
    };
    ($export:ident, $inner:path, 1) => {
        #[unsafe(naked)]
        #[unsafe(no_mangle)]
        #[allow(non_snake_case)]
        unsafe extern "C" fn $export() {
            core::arch::naked_asm!(
                "push dword ptr [esp + 4]",
                "push dword ptr [esp + 4]",
                "call {inner}",
                "ret 4",
                inner = sym $inner
            )
        }
    };
    ($export:ident, $inner:path, 2) => {
        #[unsafe(naked)]
        #[unsafe(no_mangle)]
        #[allow(non_snake_case)]
        unsafe extern "C" fn $export() {
            core::arch::naked_asm!(
                "push dword ptr [esp + 8]",
                "push dword ptr [esp + 8]",
                "push dword ptr [esp + 8]",
                "call {inner}",
                "ret 8",
                inner = sym $inner
            )
        }
    };
    ($export:ident, $inner:path, $count:literal) => {
        #[unsafe(naked)]
        #[unsafe(no_mangle)]
        #[allow(non_snake_case)]
        unsafe extern "C" fn $export() {
            core::arch::naked_asm!(
                ".rept {pushes}",
                "push dword ptr [esp + {bytes}]",
                ".endr",
                "call {inner}",
                "ret {bytes}",
                pushes = const $count + 1,
                bytes = const 4 * $count,
                inner = sym $inner
            )
        }
    };
}

/// One `imp.call` line, the thread's last error kept around it. Each row
/// writes its `log::debug!` whole, so the logline fence reads its fields.
#[cfg(all(target_arch = "x86", target_os = "windows"))]
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
#[cfg(all(target_arch = "x86", target_os = "windows"))]
fn left() -> u32 {
    // SAFETY: reads the thread's own last error.
    unsafe { crate::GetLastError() }
}

#[cfg(all(target_arch = "x86", target_os = "windows"))]
pub mod crt;
#[cfg(all(target_arch = "x86", target_os = "windows"))]
pub mod draw;
#[cfg(all(target_arch = "x86", target_os = "windows"))]
pub mod files;
#[cfg(all(target_arch = "x86", target_os = "windows"))]
pub mod heap;
#[cfg(all(target_arch = "x86", target_os = "windows"))]
pub mod imports;
#[cfg(all(target_arch = "x86", target_os = "windows"))]
pub mod input;
#[cfg(all(target_arch = "x86", target_os = "windows"))]
pub mod process;
#[cfg(all(target_arch = "x86", target_os = "windows"))]
pub mod registry;
#[cfg(all(target_arch = "x86", target_os = "windows"))]
pub mod smack;
#[cfg(all(target_arch = "x86", target_os = "windows"))]
pub mod text;
#[cfg(all(target_arch = "x86", target_os = "windows"))]
pub mod threads;
#[cfg(all(target_arch = "x86", target_os = "windows"))]
pub mod time;
#[cfg(all(target_arch = "x86", target_os = "windows"))]
pub mod window;
