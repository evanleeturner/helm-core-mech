// SPDX-License-Identifier: MIT
//! The layer (HC6, `JOB.md` I4): a Rust function for each Windows and
//! Miles import the shell's screens call, and one for each Smacker thunk.
//! Each calls the real function, then writes one DEBUG line, `imp.call`:
//! the import as the counter spells it (`KERNEL32!HeapAlloc`), its
//! arguments in order, the value it returned. Nothing is reimplemented:
//! the real function answers every call, through Wine.
//!
//! The 45 imports ([`imports`]) are exported `HC6_<name>`: with
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
    // The file rows (HC9, wall 1): the same line, and what the call moved
    // as text or a count, read after the call.
    ($c:literal, [$($argument:expr),+], $value:expr, p = $p:expr) => {{
        let value = $value;
        // SAFETY: reads the thread's own last error.
        let error = unsafe { $crate::GetLastError() };
        log::debug!(
            c = $c, a:% = mech_trace::Args([$(mech_trace::__shown!($argument)),+]),
            v:% = mech_trace::__shown!(value), p:% = $p; "imp.call"
        );
        // SAFETY: sets back the value read above.
        unsafe { $crate::SetLastError(error) };
        value
    }};
    ($c:literal, [$($argument:expr),+], $value:expr, p = $p:expr, to = $to:expr) => {{
        let value = $value;
        // SAFETY: reads the thread's own last error.
        let error = unsafe { $crate::GetLastError() };
        log::debug!(
            c = $c, a:% = mech_trace::Args([$(mech_trace::__shown!($argument)),+]),
            v:% = mech_trace::__shown!(value), p:% = $p, to:% = $to; "imp.call"
        );
        // SAFETY: sets back the value read above.
        unsafe { $crate::SetLastError(error) };
        value
    }};
    ($c:literal, [$($argument:expr),+], $value:expr, p = $p:expr, found = $found:expr) => {{
        let value = $value;
        // SAFETY: reads the thread's own last error.
        let error = unsafe { $crate::GetLastError() };
        log::debug!(
            c = $c, a:% = mech_trace::Args([$(mech_trace::__shown!($argument)),+]),
            v:% = mech_trace::__shown!(value), p:% = $p, found:% = $found; "imp.call"
        );
        // SAFETY: sets back the value read above.
        unsafe { $crate::SetLastError(error) };
        value
    }};
    ($c:literal, [$($argument:expr),+], $value:expr, found = $found:expr) => {{
        let value = $value;
        // SAFETY: reads the thread's own last error.
        let error = unsafe { $crate::GetLastError() };
        log::debug!(
            c = $c, a:% = mech_trace::Args([$(mech_trace::__shown!($argument)),+]),
            v:% = mech_trace::__shown!(value), found:% = $found; "imp.call"
        );
        // SAFETY: sets back the value read above.
        unsafe { $crate::SetLastError(error) };
        value
    }};
    ($c:literal, [$($argument:expr),+], $value:expr, n = $n:expr) => {{
        let value = $value;
        // SAFETY: reads the thread's own last error.
        let error = unsafe { $crate::GetLastError() };
        log::debug!(
            c = $c, a:% = mech_trace::Args([$(mech_trace::__shown!($argument)),+]),
            v:% = mech_trace::__shown!(value), n:% = $n; "imp.call"
        );
        // SAFETY: sets back the value read above.
        unsafe { $crate::SetLastError(error) };
        value
    }};
}

/// A file name the shell passed, shown as its text: at most 260 bytes (the
/// Windows path limit), a byte outside printable ASCII as `\xNN`, a null
/// pointer as `?`. The import read the same bytes to its NUL first.
#[cfg(all(target_arch = "x86", target_os = "windows"))]
pub struct Name(pub *const u8);

#[cfg(all(target_arch = "x86", target_os = "windows"))]
impl core::fmt::Display for Name {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.0.is_null() {
            return f.write_str("?");
        }
        for at in 0..260 {
            // SAFETY: a C string the import itself read up to its NUL;
            // the loop stops at that NUL.
            let byte = unsafe { self.0.add(at).read() };
            match byte {
                0 => break,
                b' '..=b'~' => core::fmt::Write::write_char(f, char::from(byte))?,
                _ => write!(f, "\\x{byte:02x}")?,
            }
        }
        Ok(())
    }
}

/// The count a read or write moved, read from the call's own out-pointer
/// after it returns, in hex; a null pointer as `?`.
#[cfg(all(target_arch = "x86", target_os = "windows"))]
pub struct Count(pub *mut u32);

#[cfg(all(target_arch = "x86", target_os = "windows"))]
impl core::fmt::Display for Count {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.0.is_null() {
            return f.write_str("?");
        }
        // SAFETY: the out-pointer the shell gave the call, just written.
        write!(f, "{:x}", unsafe { self.0.read() })
    }
}

/// The name a find call found: `cFileName`, 44 bytes into the
/// `WIN32_FIND_DATAA` the shell passed; empty when the call found none.
/// The only field of the find data the shell's own code reads (HC9 wall 3:
/// `10008520`, `1001baa0`, `10037c90`, `10037ce0`, through `__findfirst`).
#[cfg(all(target_arch = "x86", target_os = "windows"))]
pub struct Found(pub *mut core::ffi::c_void, pub bool);

#[cfg(all(target_arch = "x86", target_os = "windows"))]
impl core::fmt::Display for Found {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.1 {
            core::fmt::Display::fmt(&Name(self.0.cast::<u8>().wrapping_add(44)), f)
        } else {
            Ok(())
        }
    }
}

#[cfg(all(target_arch = "x86", target_os = "windows"))]
pub mod crt;
#[cfg(all(target_arch = "x86", target_os = "windows"))]
pub mod heap;
#[cfg(all(target_arch = "x86", target_os = "windows"))]
pub mod imports;
#[cfg(all(target_arch = "x86", target_os = "windows"))]
pub mod smack;
