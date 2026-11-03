// SPDX-License-Identifier: MIT
//! The layer (HC6, `JOB.md` I4): a Rust function for each Windows and
//! Miles import the shell's screens call, and one for each Smacker thunk.
//! Each calls the real function, then writes one DEBUG line, `imp.call`:
//! the import as the counter spells it (`KERNEL32!HeapAlloc`), its
//! arguments in order, the value it returned. Nothing is reimplemented:
//! the real function answers every call, through Wine.
//!
//! The 42 imports ([`imports`]) are exported `HC6_<name>`: with
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

#[cfg(all(target_arch = "x86", target_os = "windows"))]
pub mod crt;
#[cfg(all(target_arch = "x86", target_os = "windows"))]
pub mod imports;
#[cfg(all(target_arch = "x86", target_os = "windows"))]
pub mod smack;
