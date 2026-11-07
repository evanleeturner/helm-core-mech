// SPDX-License-Identifier: MIT
//! The registry calls: the 5 ADVAPI32 names,
//! each answered by `kernel_registry`'s front in the file's row, which writes
//! the call's one `imp.call` line, `by=kernel-registry`. A row turns Win32's
//! addresses into the front's slices and numbers and writes nothing of its
//! own, so `from`, the caller, left the line with the original.
//!
//! The settings stay where they were: [`Win32`] is the front's back over
//! Wine's own registry, as the file rows' back is over its folder, so a flight
//! reads the prefix's 3 values and no new file appears on disk.
//!
//! 8 exports, one a name a file imports: the shell's 5 `HC6_`, the game
//! file's 3 `GAME_` (it opens, reads and closes, never writes); the
//! launcher imports none. Each is a naked entry that hands the row the
//! address the call returns to first, as [`crate::process`]' entries do; no
//! row here reads it.
use core::ffi::CStr;
use core::ptr::{from_mut, from_ref, with_exposed_provenance, with_exposed_provenance_mut};
use core::slice;
use kernel_registry::table::Row;
use kernel_registry::{Front, Settings};
use mech_rows::registry::{GAME, SHELL};

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

/// The access `r0` measured the shell asking: `KEY_QUERY_VALUE` to open.
const QUERY_VALUE: u32 = 1;
/// And `KEY_ALL_ACCESS` to create.
const ALL_ACCESS: u32 = 0xf_003f;
/// `REG_DWORD`, the one type the front's store holds.
const DWORD: u32 = 4;
/// `REG_CREATED_NEW_KEY`.
const CREATED_NEW_KEY: u32 = 1;
/// The longest path or name the back passes on, its NUL among them; `MAX_PATH`.
const LONGEST: usize = 260;

/// `place`'s address as Win32's arguments take it.
fn address<T>(place: &mut T) -> u32 {
    from_mut(place).expose_provenance() as u32
}

/// `call` with `text` NUL-ended at the address it is given; `None` for a text
/// past [`LONGEST`], which the back cannot hold.
fn ended<T>(text: &[u8], call: impl FnOnce(u32) -> T) -> Option<T> {
    let mut held = [0_u8; LONGEST];
    if text.len() >= LONGEST {
        return None;
    }
    held[..text.len()].copy_from_slice(text);
    Some(call(address(&mut held)))
}

/// Wine's own registry as the front's back. **`holds` says yes to any number
/// but 0:** Windows keeps the handles and no import here asks whether one is
/// open, so a wrong one is refused at its read, write or close, where Windows
/// answers 6 at each. No measured call hands a wrong one.
struct Win32(&'static Row);

impl Settings for Win32 {
    fn open(&self, within: Option<u32>, path: &[u8], create: bool) -> Option<(u32, bool)> {
        let parent = within.unwrap_or(self.0.root);
        let (mut key, mut disposition) = (0_u32, 0_u32);
        let (out, told) = (address(&mut key), address(&mut disposition));
        let code = ended(path, |text| {
            // SAFETY: a NUL-ended path and this frame's 2 numbers.
            unsafe {
                if create {
                    RegCreateKeyExA(parent, text, 0, 0, 0, ALL_ACCESS, 0, out, told)
                } else {
                    RegOpenKeyExA(parent, text, 0, QUERY_VALUE, out)
                }
            }
        })?;
        (code == 0).then_some((key, disposition == CREATED_NEW_KEY))
    }

    fn holds(&self, handle: u32) -> bool {
        handle != 0
    }

    fn read(&self, handle: u32, name: &[u8]) -> Option<u32> {
        let (mut kind, mut value, mut size) = (0_u32, 0_u32, 4_u32);
        let (kind_at, value_at, size_at) =
            (address(&mut kind), address(&mut value), address(&mut size));
        let code = ended(name, |text| {
            // SAFETY: a NUL-ended name and this frame's 3 numbers, 4 bytes of room.
            unsafe { RegQueryValueExA(handle, text, 0, kind_at, value_at, size_at) }
        })?;
        (code == 0 && kind == DWORD && size == 4).then_some(value)
    }

    fn write(&self, handle: u32, name: &[u8], value: u32) -> bool {
        let data = from_ref(&value).expose_provenance() as u32;
        // SAFETY: a NUL-ended name and this frame's 4 bytes.
        ended(name, |text| unsafe {
            RegSetValueExA(handle, text, 0, DWORD, data, 4)
        }) == Some(0)
    }

    fn close(&self, handle: u32) -> bool {
        // SAFETY: a key number; the import checks it.
        unsafe { RegCloseKey(handle) == 0 }
    }

    fn quiet(&self, line: impl FnOnce()) {
        kept!(line());
    }
}

/// A file the rows serve, by its row in `mech_rows::registry`.
trait File {
    const FRONT: Front<Win32>;
}

/// `mw2shell`, the shell.
enum Shell {}
/// `mercsw`, the game file.
enum Game {}

impl File for Shell {
    const FRONT: Front<Win32> = Front::new(&SHELL, Win32(&SHELL));
}

impl File for Game {
    const FRONT: Front<Win32> = Front::new(&GAME, Win32(&GAME));
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
    // SAFETY: the caller's 4 bytes, not null here.
    (address != 0)
        .then(|| unsafe { with_exposed_provenance::<u32>(address as usize).read_unaligned() })
}

/// Writes `number` at `address`, nothing for a null pointer.
///
/// # Safety
/// `address` is 0 or 4 bytes the caller gave to be written.
unsafe fn put(address: u32, number: u32) {
    if address != 0 {
        // SAFETY: the caller's 4 bytes, not null here.
        unsafe { with_exposed_provenance_mut::<u32>(address as usize).write_unaligned(number) };
    }
}

extern "system" fn open_key<F: File>(
    _from: u32,
    root: u32,
    path: u32,
    options: u32,
    access: u32,
    handle: u32,
) -> u32 {
    // SAFETY: the caller's NUL-ended path.
    let text = unsafe { named(path) };
    match F::FRONT.open_key(root, text, options, access) {
        Ok(key) => {
            // SAFETY: the caller's pointer, given to be written.
            unsafe { put(handle, key) };
            0
        }
        Err(code) => code,
    }
}

extern "system" fn query_value<F: File>(
    _from: u32,
    handle: u32,
    name: u32,
    reserved: u32,
    kind: u32,
    data: u32,
    size: u32,
) -> u32 {
    // SAFETY: the caller's NUL-ended name and its size pointer, or null.
    let (text, room) = unsafe { (named(name), at(size)) };
    // SAFETY: the caller's buffer of the size it gave, as Win32 takes them.
    let out = (data != 0).then(|| unsafe {
        slice::from_raw_parts_mut(
            with_exposed_provenance_mut::<u8>(data as usize),
            room.unwrap_or(0) as usize,
        )
    });
    let queried = F::FRONT.query_value(handle, text, reserved, room, out);
    if let Some((written_kind, written_size)) = queried.wrote {
        // SAFETY: the caller's 2 pointers, given to be written, or null.
        unsafe {
            put(kind, written_kind);
            put(size, written_size);
        }
    }
    queried.code
}

extern "system" fn close_key<F: File>(_from: u32, handle: u32) -> u32 {
    F::FRONT.close_key(handle)
}

#[allow(
    clippy::too_many_arguments,
    reason = "RegCreateKeyExA's own 9, and the caller first"
)]
extern "system" fn create_key<F: File>(
    _from: u32,
    root: u32,
    path: u32,
    _reserved: u32,
    _class: u32,
    options: u32,
    access: u32,
    _security: u32,
    handle: u32,
    disposition: u32,
) -> u32 {
    // SAFETY: the caller's NUL-ended path.
    let text = unsafe { named(path) };
    match F::FRONT.create_key(root, text, options, access) {
        Ok((key, told)) => {
            // SAFETY: the caller's 2 pointers, given to be written, or null.
            unsafe {
                put(handle, key);
                put(disposition, told);
            }
            0
        }
        Err(code) => code,
    }
}

extern "system" fn set_value<F: File>(
    _from: u32,
    handle: u32,
    name: u32,
    _reserved: u32,
    kind: u32,
    data: u32,
    size: u32,
) -> u32 {
    // SAFETY: the caller's NUL-ended name.
    let text = unsafe { named(name) };
    let bytes: &[u8] = if data == 0 {
        &[]
    } else {
        // SAFETY: the caller's `size` bytes at `data`.
        unsafe { slice::from_raw_parts(with_exposed_provenance(data as usize), size as usize) }
    };
    F::FRONT.set_value(handle, text, kind, bytes)
}

entry!(HC6_RegOpenKeyExA, open_key::<Shell>, 5);
entry!(HC6_RegQueryValueExA, query_value::<Shell>, 6);
entry!(HC6_RegCloseKey, close_key::<Shell>, 1);
entry!(HC6_RegCreateKeyExA, create_key::<Shell>, 9);
entry!(HC6_RegSetValueExA, set_value::<Shell>, 6);
entry!(GAME_RegOpenKeyExA, open_key::<Game>, 5);
entry!(GAME_RegQueryValueExA, query_value::<Game>, 6);
entry!(GAME_RegCloseKey, close_key::<Game>, 1);
