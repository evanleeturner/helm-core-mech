// SPDX-License-Identifier: MIT
//! The registry calls (KR1, `JOB.md` step 4): the 5 ADVAPI32 names, each
//! the original's, logged as `kernel_registry`'s front logs them, its
//! fields in the front's order, then `from`, the caller.
//!
//! 8 exports, one a name a file imports: the shell's 5 `HC6_`, the game
//! file's 3 `GAME_` (it opens, reads and closes, never writes); the
//! launcher imports none. Each is a naked entry that reads the address the
//! call returns to first, as [`crate::process`]' entries do.
//!
//! A registry call answers its error as its value, Windows' `LONG` read
//! unsigned as the front's codes are, and leaves no last error, so no line
//! carries `e`. The line's `o` is the numbers the call wrote at its
//! pointers, up to the first it had no pointer for: a query handed no
//! type or size pointer shows none, a shape the run did not ask (wall 3).
use core::ffi::CStr;
use core::fmt;
use core::ptr::with_exposed_provenance;
use kernel_registry::Bytes;
use kernel_registry::table::Row;
use mech_rows::registry::{GAME, SHELL};
use mech_trace::Args;

unsafe extern "system" {
    fn RegOpenKeyExA(root: u32, path: u32, options: u32, access: u32, handle: u32) -> u32;
    fn RegQueryValueExA(
        handle: u32,
        name: u32,
        reserved: u32,
        kind: u32,
        data: u32,
        size: u32,
    ) -> u32;
    fn RegCloseKey(handle: u32) -> u32;
    fn RegCreateKeyExA(
        root: u32,
        path: u32,
        reserved: u32,
        class: u32,
        options: u32,
        access: u32,
        security: u32,
        handle: u32,
        disposition: u32,
    ) -> u32;
    fn RegSetValueExA(
        handle: u32,
        name: u32,
        reserved: u32,
        kind: u32,
        data: u32,
        size: u32,
    ) -> u32;
}

/// A file the rows serve, by its row in `mech_rows::registry`.
trait File {
    const ROW: &'static Row;
}

/// `mw2shell`, the shell.
enum Shell {}
/// `mercsw`, the game file.
enum Game {}

impl File for Shell {
    const ROW: &'static Row = &SHELL;
}

impl File for Game {
    const ROW: &'static Row = &GAME;
}

/// The NUL-ended name at `address`, the NUL left out; empty for a null one.
///
/// # Safety
/// `address` is 0 or a NUL-ended text, as Win32 takes it.
unsafe fn named<'a>(address: u32) -> &'a [u8] {
    if address == 0 {
        return &[];
    }
    // SAFETY: the caller's NUL-ended text.
    unsafe { CStr::from_ptr(with_exposed_provenance(address as usize)) }.to_bytes()
}

/// The number at `address`, `None` for a null pointer.
///
/// # Safety
/// `address` is 0 or 4 bytes the caller handed in.
unsafe fn at(address: u32) -> Option<u32> {
    // SAFETY: the caller's 4 bytes, read where they lie.
    (address != 0)
        .then(|| unsafe { with_exposed_provenance::<u32>(address as usize).read_unaligned() })
}

/// What a call wrote, the line's `o`: its numbers comma-joined, up to the
/// first `None`; empty for a refusal.
struct Wrote([Option<u32>; 3]);

impl fmt::Display for Wrote {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0
            .iter()
            .map_while(|number| *number)
            .enumerate()
            .try_for_each(|(index, number)| {
                if index > 0 {
                    f.write_str(",")?;
                }
                mech_trace::__shown!(number).fmt(f)
            })
    }
}

extern "system" fn open_key<F: File>(
    from: u32,
    root: u32,
    path: u32,
    options: u32,
    access: u32,
    handle: u32,
) -> u32 {
    // SAFETY: the caller's arguments, passed on whole.
    let value = unsafe { RegOpenKeyExA(root, path, options, access, handle) };
    // SAFETY: the caller's key path, or null.
    let text = unsafe { named(path) };
    let opened = if value == 0 {
        // SAFETY: the handle the call wrote, read on a 0 answer alone.
        unsafe { at(handle) }
    } else {
        None
    };
    kept!(log::debug!(c = F::ROW.calls.open_key, a:% = Args([
            mech_trace::__shown!(root),
            mech_trace::__shown!(options),
            mech_trace::__shown!(access),
        ]), s:% = Bytes(text), o:% = Wrote([opened, None, None]),
        v:% = mech_trace::__shown!(value), from:% = mech_trace::__shown!(from); "imp.call"));
    value
}

extern "system" fn query_value<F: File>(
    from: u32,
    handle: u32,
    name: u32,
    reserved: u32,
    kind: u32,
    data: u32,
    size: u32,
) -> u32 {
    // SAFETY: the size the caller handed in, read before the call writes it.
    let given = unsafe { at(size) }.unwrap_or(0);
    // SAFETY: the caller's arguments, passed on whole.
    let value = unsafe { RegQueryValueExA(handle, name, reserved, kind, data, size) };
    // SAFETY: the caller's value name, or null.
    let text = unsafe { named(name) };
    let wrote = if value == 0 || value == F::ROW.errors.more_data {
        // SAFETY: the type and size the call wrote, at the caller's
        // pointers, or null.
        let (written_kind, written_size) = unsafe { (at(kind), at(size)) };
        // SAFETY: the value the call wrote, read only as the 4 bytes it
        // said it wrote.
        let read = (value == 0 && written_size == Some(4))
            .then(|| unsafe { at(data) })
            .flatten();
        [written_kind, written_size, read]
    } else {
        [None; 3]
    };
    kept!(log::debug!(c = F::ROW.calls.query_value, a:% = Args([
            mech_trace::__shown!(handle),
            mech_trace::__shown!(reserved),
            mech_trace::__shown!(given),
        ]), s:% = Bytes(text), o:% = Wrote(wrote),
        v:% = mech_trace::__shown!(value), from:% = mech_trace::__shown!(from); "imp.call"));
    value
}

extern "system" fn close_key<F: File>(from: u32, handle: u32) -> u32 {
    // SAFETY: the caller's argument, passed on whole.
    let value = unsafe { RegCloseKey(handle) };
    kept!(
        log::debug!(c = F::ROW.calls.close_key, a:% = Args([mech_trace::__shown!(handle)]),
        v:% = mech_trace::__shown!(value), from:% = mech_trace::__shown!(from); "imp.call")
    );
    value
}

#[allow(
    clippy::too_many_arguments,
    reason = "RegCreateKeyExA's own 9, and the caller first"
)]
extern "system" fn create_key<F: File>(
    from: u32,
    root: u32,
    path: u32,
    reserved: u32,
    class: u32,
    options: u32,
    access: u32,
    security: u32,
    handle: u32,
    disposition: u32,
) -> u32 {
    // SAFETY: the caller's arguments, passed on whole.
    let value = unsafe {
        RegCreateKeyExA(
            root,
            path,
            reserved,
            class,
            options,
            access,
            security,
            handle,
            disposition,
        )
    };
    // SAFETY: the caller's key path, or null.
    let text = unsafe { named(path) };
    let made = if value == 0 {
        // SAFETY: the handle and disposition the call wrote, or null.
        unsafe { [at(handle), at(disposition), None] }
    } else {
        [None; 3]
    };
    kept!(log::debug!(c = F::ROW.calls.create_key, a:% = Args([
            mech_trace::__shown!(root),
            mech_trace::__shown!(options),
            mech_trace::__shown!(access),
        ]), s:% = Bytes(text), o:% = Wrote(made),
        v:% = mech_trace::__shown!(value), from:% = mech_trace::__shown!(from); "imp.call"));
    value
}

extern "system" fn set_value<F: File>(
    from: u32,
    handle: u32,
    name: u32,
    reserved: u32,
    kind: u32,
    data: u32,
    size: u32,
) -> u32 {
    // SAFETY: the caller's arguments, passed on whole.
    let value = unsafe { RegSetValueExA(handle, name, reserved, kind, data, size) };
    // SAFETY: the caller's value name, or null.
    let text = unsafe { named(name) };
    // SAFETY: the value kept, read only as the 4 bytes the caller handed.
    let stored = (value == 0 && size == 4)
        .then(|| unsafe { at(data) })
        .flatten();
    kept!(log::debug!(c = F::ROW.calls.set_value, a:% = Args([
            mech_trace::__shown!(handle),
            mech_trace::__shown!(kind),
            mech_trace::__shown!(size),
        ]), s:% = Bytes(text), o:% = Wrote([stored, None, None]),
        v:% = mech_trace::__shown!(value), from:% = mech_trace::__shown!(from); "imp.call"));
    value
}

entry!(HC6_RegOpenKeyExA, open_key::<Shell>, 5);
entry!(HC6_RegQueryValueExA, query_value::<Shell>, 6);
entry!(HC6_RegCloseKey, close_key::<Shell>, 1);
entry!(HC6_RegCreateKeyExA, create_key::<Shell>, 9);
entry!(HC6_RegSetValueExA, set_value::<Shell>, 6);

entry!(GAME_RegOpenKeyExA, open_key::<Game>, 5);
entry!(GAME_RegQueryValueExA, query_value::<Game>, 6);
entry!(GAME_RegCloseKey, close_key::<Game>, 1);
