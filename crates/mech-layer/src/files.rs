// SPDX-License-Identifier: MIT
//! The shell's file calls through the seam (HC9, ruling 1387): `mech_files`'
//! front over the Windows folder, the shell's 11 file imports exported
//! `HC6_<name>` as the layer's other rows are, so the swap's loader points
//! the shell's own slots at them unchanged.
//!
//! The Windows folder answers from the real imports, from the swap DLL's
//! own import table, asked in the shell's own numbers through the row's way
//! back; a failure carries the import's last error whole, so the shell
//! reads the code the import left. The thread's last error is kept around
//! each line, as `imp_call!` keeps it.
use core::ffi::{CStr, c_char, c_void};
use core::ptr::{null_mut, without_provenance_mut};
use core::slice;
use mech_files::table::{FIND_BYTES, Row, SHELL};
use mech_files::{Error, Files, Folder, Kind, Mode, Origin};

unsafe extern "system" {
    fn CreateFileA(
        name: *const c_char,
        access: u32,
        share: u32,
        security: *mut c_void,
        disposition: u32,
        flags: u32,
        template: *mut c_void,
    ) -> *mut c_void;
    fn ReadFile(
        file: *mut c_void,
        buffer: *mut c_void,
        bytes: u32,
        read: *mut u32,
        overlapped: *mut c_void,
    ) -> i32;
    fn WriteFile(
        file: *mut c_void,
        buffer: *const c_void,
        bytes: u32,
        written: *mut u32,
        overlapped: *mut c_void,
    ) -> i32;
    fn SetFilePointer(file: *mut c_void, distance: i32, high: *mut i32, method: u32) -> u32;
    fn GetFileType(file: *mut c_void) -> u32;
    fn CloseHandle(object: *mut c_void) -> i32;
    fn DeleteFileA(name: *const c_char) -> i32;
    fn MoveFileA(from: *const c_char, to: *const c_char) -> i32;
    fn FindFirstFileA(name: *const c_char, data: *mut c_void) -> *mut c_void;
    fn FindNextFileA(find: *mut c_void, data: *mut c_void) -> i32;
    fn FindClose(find: *mut c_void) -> i32;
}

/// The find data a Windows find fills: the folder's own, its name copied
/// out for the front.
#[repr(C, align(4))]
struct FindData([u8; FIND_BYTES]);

/// One file's files on the Windows file system, through its row.
pub struct Win32(&'static Row);

/// A handle as the import takes it: a number, never a pointer read.
fn handle(number: u32) -> *mut c_void {
    without_provenance_mut(number as usize)
}

/// The thread's last error, as the import left it.
fn last_error() -> Error {
    // SAFETY: reads the thread's own last error.
    Error::Other(unsafe { crate::GetLastError() })
}

/// A yes-or-no import's answer.
fn done(answer: i32) -> Result<(), Error> {
    if answer == 0 {
        Err(last_error())
    } else {
        Ok(())
    }
}

impl Win32 {
    /// A handle the import answered, or its failure.
    fn opened(&self, answer: *mut c_void) -> Result<u32, Error> {
        let number = u32::try_from(answer.addr()).unwrap_or(self.0.invalid);
        if number == self.0.invalid {
            Err(last_error())
        } else {
            Ok(number)
        }
    }

    /// The found name, from the folder's find data into the front's.
    fn found(&self, data: &FindData, name: &mut [u8]) {
        let at = self.0.name_at;
        name.copy_from_slice(&data.0[at..at + name.len()]);
    }
}

impl Folder for Win32 {
    fn open(&self, name: &CStr, mode: Mode) -> Result<u32, Error> {
        let row = self.0;
        // SAFETY: a C string, and the arguments the shell's opens pass.
        let answer = unsafe {
            CreateFileA(
                name.as_ptr(),
                row.access(mode),
                row.share,
                null_mut(),
                row.disposition(mode.create),
                row.plain,
                null_mut(),
            )
        };
        self.opened(answer)
    }

    fn read(&self, file: u32, into: &mut [u8]) -> Result<u32, Error> {
        let bytes = u32::try_from(into.len()).map_err(|_| Error::BadArgument)?;
        let mut read = 0;
        // SAFETY: `into` holds `bytes`; the count is this frame's.
        let answer = unsafe {
            ReadFile(
                handle(file),
                into.as_mut_ptr().cast(),
                bytes,
                &raw mut read,
                null_mut(),
            )
        };
        done(answer).map(|()| read)
    }

    fn write(&self, file: u32, from: &[u8]) -> Result<u32, Error> {
        let bytes = u32::try_from(from.len()).map_err(|_| Error::BadArgument)?;
        let mut written = 0;
        // SAFETY: `from` holds `bytes`; the count is this frame's.
        let answer = unsafe {
            WriteFile(
                handle(file),
                from.as_ptr().cast(),
                bytes,
                &raw mut written,
                null_mut(),
            )
        };
        done(answer).map(|()| written)
    }

    fn seek(&self, file: u32, distance: i32, origin: Origin) -> Result<u32, Error> {
        // SAFETY: no high half, as the shell's one seek passes none.
        let at =
            unsafe { SetFilePointer(handle(file), distance, null_mut(), self.0.method(origin)) };
        if at == self.0.invalid {
            Err(last_error())
        } else {
            Ok(at)
        }
    }

    fn kind(&self, file: u32) -> Result<Kind, Error> {
        // SAFETY: a handle number; the import checks it.
        let number = unsafe { GetFileType(handle(file)) };
        self.0.kind(number).ok_or_else(last_error)
    }

    fn close(&self, file: u32) -> Result<(), Error> {
        // SAFETY: a handle number; the import checks it.
        done(unsafe { CloseHandle(handle(file)) })
    }

    fn delete(&self, name: &CStr) -> Result<(), Error> {
        // SAFETY: a C string.
        done(unsafe { DeleteFileA(name.as_ptr()) })
    }

    fn rename(&self, from: &CStr, to: &CStr) -> Result<(), Error> {
        // SAFETY: two C strings.
        done(unsafe { MoveFileA(from.as_ptr(), to.as_ptr()) })
    }

    fn find_first(&self, pattern: &CStr, name: &mut [u8]) -> Result<u32, Error> {
        let mut data = FindData([0; FIND_BYTES]);
        // SAFETY: a C string, and find data of the import's size.
        let answer = unsafe { FindFirstFileA(pattern.as_ptr(), (&raw mut data).cast()) };
        let find = self.opened(answer)?;
        self.found(&data, name);
        Ok(find)
    }

    fn find_next(&self, find: u32, name: &mut [u8]) -> Result<(), Error> {
        let mut data = FindData([0; FIND_BYTES]);
        // SAFETY: a find number, and find data of the import's size.
        done(unsafe { FindNextFileA(handle(find), (&raw mut data).cast()) })?;
        self.found(&data, name);
        Ok(())
    }

    fn find_close(&self, find: u32) -> Result<(), Error> {
        // SAFETY: a find number; the import checks it.
        done(unsafe { FindClose(handle(find)) })
    }

    fn fail(&self, code: u32) {
        // SAFETY: sets the thread's own last error.
        unsafe { crate::SetLastError(code) };
    }

    fn quiet(&self, line: impl FnOnce()) {
        // SAFETY: reads the thread's own last error.
        let error = unsafe { crate::GetLastError() };
        line();
        // SAFETY: sets back the value read above.
        unsafe { crate::SetLastError(error) };
    }
}

/// The shell's file calls.
static SHELL_FILES: Files<Win32> = Files::new(&SHELL, Win32(&SHELL));

/// A name the shell passed. A null one reads as the empty name, which every
/// file import refuses (a1 passes none).
///
/// # Safety
///
/// `name` is null or a C string that outlives the call.
unsafe fn text<'a>(name: *const c_char) -> &'a CStr {
    if name.is_null() {
        c""
    } else {
        // SAFETY: the caller's promise.
        unsafe { CStr::from_ptr(name) }
    }
}

/// The bytes at `buffer`, or none when there are none to move.
///
/// # Safety
///
/// `buffer` holds `bytes` bytes, or `bytes` is 0.
unsafe fn bytes<'a>(buffer: *mut c_void, bytes: u32) -> &'a mut [u8] {
    if bytes == 0 {
        &mut []
    } else {
        // SAFETY: the caller's promise.
        unsafe { slice::from_raw_parts_mut(buffer.cast(), bytes as usize) }
    }
}

#[unsafe(export_name = "HC6_CreateFileA")]
unsafe extern "system" fn create_file(
    name: *const c_char,
    access: u32,
    share: u32,
    _security: *mut c_void,
    disposition: u32,
    flags: u32,
    _template: *mut c_void,
) -> *mut c_void {
    // SAFETY: the shell's own call: its name is a C string.
    let name = unsafe { text(name) };
    handle(SHELL_FILES.open(name, access, share, disposition, flags))
}

#[unsafe(export_name = "HC6_ReadFile")]
unsafe extern "system" fn read_file(
    file: *mut c_void,
    buffer: *mut c_void,
    count: u32,
    read: *mut u32,
    _overlapped: *mut c_void,
) -> i32 {
    // SAFETY: the shell's own call: its buffer holds `count` bytes, its
    // count pointer is null or its own.
    let (into, read) = unsafe { (bytes(buffer, count), read.as_mut()) };
    SHELL_FILES.read(number(file), into, read)
}

#[unsafe(export_name = "HC6_WriteFile")]
unsafe extern "system" fn write_file(
    file: *mut c_void,
    buffer: *const c_void,
    count: u32,
    written: *mut u32,
    _overlapped: *mut c_void,
) -> i32 {
    // SAFETY: the shell's own call: its buffer holds `count` bytes, which
    // the write only reads; its count pointer is null or its own.
    let (from, written) = unsafe { (bytes(buffer.cast_mut(), count), written.as_mut()) };
    SHELL_FILES.write(number(file), from, written)
}

#[unsafe(export_name = "HC6_SetFilePointer")]
unsafe extern "system" fn set_file_pointer(
    file: *mut c_void,
    distance: i32,
    _high: *mut i32,
    method: u32,
) -> u32 {
    SHELL_FILES.seek(number(file), distance, method)
}

#[unsafe(export_name = "HC6_GetFileType")]
unsafe extern "system" fn get_file_type(file: *mut c_void) -> u32 {
    SHELL_FILES.kind(number(file))
}

#[unsafe(export_name = "HC6_CloseHandle")]
unsafe extern "system" fn close_handle(object: *mut c_void) -> i32 {
    SHELL_FILES.close(number(object))
}

#[unsafe(export_name = "HC6_DeleteFileA")]
unsafe extern "system" fn delete_file(name: *const c_char) -> i32 {
    // SAFETY: the shell's own call: its name is a C string.
    SHELL_FILES.delete(unsafe { text(name) })
}

#[unsafe(export_name = "HC6_MoveFileA")]
unsafe extern "system" fn move_file(from: *const c_char, to: *const c_char) -> i32 {
    // SAFETY: the shell's own call: its names are C strings.
    let (from, to) = unsafe { (text(from), text(to)) };
    SHELL_FILES.rename(from, to)
}

#[unsafe(export_name = "HC6_FindFirstFileA")]
unsafe extern "system" fn find_first_file(name: *const c_char, data: *mut c_void) -> *mut c_void {
    // SAFETY: the shell's own call: a C string, and its find data.
    let (pattern, data) = unsafe { (text(name), find_data(data)) };
    handle(SHELL_FILES.find_first(pattern, data))
}

#[unsafe(export_name = "HC6_FindNextFileA")]
unsafe extern "system" fn find_next_file(find: *mut c_void, data: *mut c_void) -> i32 {
    // SAFETY: the shell's own call: its find data.
    SHELL_FILES.find_next(number(find), unsafe { find_data(data) })
}

#[unsafe(export_name = "HC6_FindClose")]
unsafe extern "system" fn find_close(find: *mut c_void) -> i32 {
    SHELL_FILES.find_close(number(find))
}

/// A handle as the front takes it: its number, the shell's 32 bits.
fn number(handle: *mut c_void) -> u32 {
    u32::try_from(handle.addr()).unwrap_or(SHELL.invalid)
}

/// The shell's find data.
///
/// # Safety
///
/// `data` holds the find data a find import fills.
unsafe fn find_data<'a>(data: *mut c_void) -> &'a mut [u8] {
    // SAFETY: the caller's promise.
    unsafe { slice::from_raw_parts_mut(data.cast(), FIND_BYTES) }
}
