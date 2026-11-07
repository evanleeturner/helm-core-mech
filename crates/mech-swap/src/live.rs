// SPDX-License-Identifier: MIT
//! The bind's Windows side: the process's own memory behind
//! [`crate::bind`]'s seams, the stopped threads the script had from gdb, and where the bind
//! fires. The launcher's table binds in `DllMain`, under the starter's remote `LoadLibraryA`;
//! every other file's after the real `LoadLibraryA` a file calls through the layer, before
//! that caller sees the module, so its slots are bound before its first call to anything.
//!
//! **`HC5_BIND` turns it on, naming the folder of tables, in the process's own path form.
//! Not `HC5_LINK`:** the rig's flights set that for their gdb bind and inherit it, and the
//! swap binding beside gdb would be 2 binds. Unset, the swap is what it was.
//!
//! A refusal is a named stop, an ERROR line and [`BIND_REFUSED`] as the exit code. A process
//! half bound would run on, answer from the wrong place and look alive.
//!
//! No allocator: each table is read into its own `VirtualAlloc` and kept to the process's
//! end, one small block a load. No thread is stopped under a heap call or a log line: see
//! [`bind::apply_sites`].
use core::ffi::c_void;
use core::fmt;
use core::ptr::{null, null_mut, with_exposed_provenance, with_exposed_provenance_mut};

use mech_rows::sites::{BoundFile, FILES, Patched};

use crate::bind::{self, File, Hex, Loaded, Memory, Refusal};

/// The `/fixed` base the swap is linked at: every table's targets assume it.
const SWAP_BASE: u32 = 0x6000_0000;
/// The exit code of a refused bind, which the starter passes on.
const BIND_REFUSED: u32 = 0xB1D0;
/// A PE's headers fit its first page, and a mapped image always holds that page.
const HEADER_BYTES: usize = 0x1000;
pub(crate) const PATH_BYTES: usize = 260;
const NAME_BYTES: usize = 64;
/// A flight measured 2 threads at this moment; the room is for what it did not.
pub(crate) const MOST_THREADS: usize = 64;

const GENERIC_READ: u32 = 0x8000_0000;
const FILE_SHARE_READ: u32 = 1;
const OPEN_EXISTING: u32 = 3;
const FILE_ATTRIBUTE_NORMAL: u32 = 0x80;
const INVALID_HANDLE_VALUE: usize = usize::MAX;
const MEM_COMMIT_RESERVE: u32 = 0x3000;
const PAGE_READWRITE: u32 = 4;
const PAGE_EXECUTE_READWRITE: u32 = 0x40;
/// `GetCurrentProcess`'s fixed answer, so no call for it.
const CURRENT_PROCESS: usize = usize::MAX;
const TH32CS_SNAPTHREAD: u32 = 4;
/// `THREAD_SUSPEND_RESUME | THREAD_GET_CONTEXT | THREAD_QUERY_INFORMATION`.
const THREAD_ACCESS: u32 = 0x0002 | 0x0008 | 0x0040;
const SUSPEND_FAILED: u32 = u32::MAX;
/// `CONTEXT_i386 | CONTEXT_CONTROL`, which holds `Eip`.
const CONTEXT_CONTROL: u32 = 0x0001_0001;
/// A 32-bit `CONTEXT`'s 716 bytes, as words, and `Eip`'s place at byte `0xb8`.
const CONTEXT_WORDS: usize = 179;
const CONTEXT_EIP: usize = 46;

/// `THREADENTRY32`; `size` is its own 28 bytes, which the API requires set.
#[repr(C)]
struct ThreadEntry {
    size: u32,
    usage: u32,
    thread: u32,
    owner: u32,
    base_priority: i32,
    delta_priority: i32,
    flags: u32,
}

unsafe extern "system" {
    fn GetEnvironmentVariableA(name: *const u8, buffer: *mut u8, size: u32) -> u32;
    fn GetModuleHandleA(name: *const u8) -> u32;
    fn IsBadReadPtr(at: *const c_void, size: u32) -> i32;
    fn VirtualProtect(at: *mut c_void, size: u32, new: u32, old: *mut u32) -> i32;
    fn FlushInstructionCache(process: usize, at: *const c_void, size: u32) -> i32;
    fn VirtualAlloc(at: *mut c_void, size: u32, kind: u32, protect: u32) -> *mut u8;
    fn CreateFileA(
        path: *const u8,
        access: u32,
        share: u32,
        security: *mut c_void,
        disposition: u32,
        flags: u32,
        template: *mut c_void,
    ) -> usize;
    fn GetFileSize(file: usize, high: *mut u32) -> u32;
    fn ReadFile(
        file: usize,
        buffer: *mut u8,
        bytes: u32,
        read: *mut u32,
        overlapped: *mut c_void,
    ) -> i32;
    fn CloseHandle(handle: usize) -> i32;
    fn ExitProcess(code: u32) -> !;
    fn GetCurrentProcessId() -> u32;
    fn GetCurrentThreadId() -> u32;
    fn CreateToolhelp32Snapshot(flags: u32, process: u32) -> usize;
    fn Thread32First(snapshot: usize, entry: *mut ThreadEntry) -> i32;
    fn Thread32Next(snapshot: usize, entry: *mut ThreadEntry) -> i32;
    fn OpenThread(access: u32, inherit: i32, thread: u32) -> usize;
    fn SuspendThread(thread: usize) -> u32;
    fn ResumeThread(thread: usize) -> u32;
    fn GetThreadContext(thread: usize, context: *mut u32) -> i32;
}

/// This process's own memory. Reads by `IsBadReadPtr` and a copy, not `ReadProcessMemory`,
/// which is a server call. It logs nothing: [`bind::apply_sites`] runs on it with every other
/// thread stopped.
struct Live;

impl Memory for Live {
    fn read(&self, at: u32, out: &mut [u8]) -> bool {
        let from = with_exposed_provenance::<u8>(at as usize);
        // SAFETY: only probes the range; a bad one answers true and touches nothing.
        if unsafe { IsBadReadPtr(from.cast(), out.len() as u32) } != 0 {
            return false;
        }
        // SAFETY: the range was just probed readable, and `out` is this call's own.
        unsafe { core::ptr::copy_nonoverlapping(from, out.as_mut_ptr(), out.len()) };
        true
    }

    /// One protection for the whole run: no row or slot crosses a page, a row being at most
    /// 62 bytes at an address its image's 64K alignment keeps clear of a page's end.
    fn write(&mut self, at: u32, bytes: &[u8]) -> bool {
        let to = with_exposed_provenance_mut::<u8>(at as usize);
        let size = bytes.len() as u32;
        let mut was = 0;
        // SAFETY: changes the range's protection; a bad range answers 0 and changes nothing.
        if unsafe { VirtualProtect(to.cast(), size, PAGE_EXECUTE_READWRITE, &raw mut was) } == 0 {
            return false;
        }
        // SAFETY: the range is mapped and writable, just made so, and `bytes` is the caller's.
        unsafe { core::ptr::copy_nonoverlapping(bytes.as_ptr(), to, bytes.len()) };
        let mut back = 0;
        // SAFETY: as the first call, the protection the range had. Code is flushed by range.
        unsafe {
            VirtualProtect(to.cast(), size, was, &raw mut back);
            FlushInstructionCache(CURRENT_PROCESS, to.cast(), size);
        }
        true
    }
}

/// A mapped module's header page, the whole of what the bind reads of it.
fn headers(base: u32) -> Option<&'static [u8]> {
    let from = with_exposed_provenance::<u8>(base as usize);
    // SAFETY: only probes; base 0, a failed load's answer, reads as bad.
    if base == 0 || unsafe { IsBadReadPtr(from.cast(), HEADER_BYTES as u32) } != 0 {
        return None;
    }
    // SAFETY: readable as just probed, and a module's header page stays while it is mapped;
    // the caller keeps it no longer than this load's judgment, with the module held.
    Some(unsafe { core::slice::from_raw_parts(from, HEADER_BYTES) })
}

/// A DLL by its name, as the sites' plan asks.
fn loaded(dll: &str) -> Option<Loaded<'static>> {
    let mut name = [0; NAME_BYTES];
    name.get_mut(..dll.len())?.copy_from_slice(dll.as_bytes());
    let base = if dll.len() < NAME_BYTES {
        // SAFETY: the check above keeps `dll` shorter than the zeroed buffer, so a NUL ends it.
        unsafe { GetModuleHandleA(name.as_ptr()) }
    } else {
        0
    };
    let head = headers(base)?;
    let key = bind::header_key(&File(head))?;
    Some(Loaded {
        base,
        size: key.size,
        headers: head,
    })
}

/// `name` in the folder the variable `env` names, NUL-terminated `env` asked of the process,
/// into the zeroed `path`; false if the variable is unset or the whole would not fit.
pub(crate) fn path_in(env: &[u8], name: &str, path: &mut [u8; PATH_BYTES]) -> bool {
    // SAFETY: the name is NUL-terminated; the buffer's size is passed, so it writes inside.
    let folder = unsafe {
        GetEnvironmentVariableA(env.as_ptr(), path.as_mut_ptr(), PATH_BYTES as u32)
    } as usize;
    if folder == 0 || folder + 1 + name.len() >= PATH_BYTES {
        return false;
    }
    path[folder] = b'\\';
    path[folder + 1..folder + 1 + name.len()].copy_from_slice(name.as_bytes());
    true
}

/// `HC5_BIND`'s `name`, as [`read_table_in`].
fn read_table(name: &str) -> Option<&'static str> {
    read_table_in(b"HC5_BIND\0", name)
}

/// `name` in the folder `env` names, its bytes kept to the process's end: nothing frees it,
/// because the refusals' words borrow it and each load's table is a small block.
pub(crate) fn read_table_in(env: &[u8], name: &str) -> Option<&'static str> {
    let mut path = [0; PATH_BYTES];
    if !path_in(env, name, &mut path) {
        return None;
    }
    // SAFETY: `path_in` leaves the byte after the name inside the zeroed buffer.
    let file = unsafe {
        CreateFileA(
            path.as_ptr(),
            GENERIC_READ,
            FILE_SHARE_READ,
            null_mut(),
            OPEN_EXISTING,
            FILE_ATTRIBUTE_NORMAL,
            null_mut(),
        )
    };
    if file == INVALID_HANDLE_VALUE {
        return None;
    }
    // SAFETY: the handle is open; the high half is not asked for, a table being small.
    let size = unsafe { GetFileSize(file, null_mut()) };
    // SAFETY: asks for fresh pages; a null answer is checked before use.
    let block =
        unsafe { VirtualAlloc(null_mut(), size.max(1), MEM_COMMIT_RESERVE, PAGE_READWRITE) };
    let mut read = 0;
    // SAFETY: the block is `size` bytes, or null and then not read into.
    let whole = !block.is_null()
        && unsafe { ReadFile(file, block, size, &raw mut read, null_mut()) } != 0
        && read == size;
    // SAFETY: closes the handle opened above, once.
    unsafe { CloseHandle(file) };
    if !whole {
        return None;
    }
    // SAFETY: `read` bytes were just written there, and the block is never freed or written
    // again, so it lives as `'static` says.
    let bytes = unsafe { core::slice::from_raw_parts(block.cast_const(), size as usize) };
    core::str::from_utf8(bytes).ok()
}

/// The named stop.
pub(crate) fn refused(file: &str, why: &dyn fmt::Display) -> ! {
    log::error!(file = file, why:% = why; "bind.refused");
    // SAFETY: ends this process, which is the point: see the module's words on half a bind.
    unsafe { ExitProcess(BIND_REFUSED) }
}

/// Every other thread of this process stopped, `work` handed their addresses, then all of
/// them run again; how many, with `work`'s answer, or why they could not all be stopped, in
/// which case `work` never ran.
///
/// This is the script's "every thread stopped", so `Eip` can be judged: `GetThreadContext`
/// waits for a suspend to land. **A thread born after the snapshot goes unjudged, and runs
/// through the write:** whether one can be born at this moment is unmeasured.
pub(crate) fn with_others_stopped<R>(
    pcs: &mut [u32; MOST_THREADS],
    work: impl FnOnce(&[u32]) -> R,
) -> Result<(usize, R), Refusal<'static>> {
    // SAFETY: asks for a copy of the thread list; a bad answer is checked.
    let snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0) };
    if snapshot == INVALID_HANDLE_VALUE {
        return Err(Refusal::ThreadsNotStopped("no snapshot"));
    }
    // SAFETY: read this process's and this thread's own ids; touch no memory.
    let (process, me) = unsafe { (GetCurrentProcessId(), GetCurrentThreadId()) };
    let mut handles = [0; MOST_THREADS];
    let mut count = 0;
    let mut failed = None;
    let mut entry = ThreadEntry {
        size: size_of::<ThreadEntry>() as u32,
        usage: 0,
        thread: 0,
        owner: 0,
        base_priority: 0,
        delta_priority: 0,
        flags: 0,
    };
    // SAFETY: the snapshot is open and `entry` is the size it says.
    let mut more = unsafe { Thread32First(snapshot, &raw mut entry) } != 0;
    while more {
        if entry.owner == process && entry.thread != me {
            match stop_thread(entry.thread, count) {
                Ok((handle, pc)) => {
                    handles[count] = handle;
                    pcs[count] = pc;
                    count += 1;
                }
                Err((handle, why)) => {
                    if handle != 0 {
                        handles[count] = handle;
                        count += 1;
                    }
                    failed = Some(why);
                    break;
                }
            }
        }
        // SAFETY: as `Thread32First`.
        more = unsafe { Thread32Next(snapshot, &raw mut entry) } != 0;
    }
    let answer = failed.is_none().then(|| work(&pcs[..count]));
    for handle in &handles[..count] {
        // SAFETY: each handle is open and its thread suspended once, by `stop_thread`.
        unsafe {
            ResumeThread(*handle);
            CloseHandle(*handle);
        }
    }
    // SAFETY: closes the snapshot opened above, once.
    unsafe { CloseHandle(snapshot) };
    match (answer, failed) {
        (Some(answer), _) => Ok((count, answer)),
        (None, why) => Err(Refusal::ThreadsNotStopped(why.unwrap_or("no reason kept"))),
    }
}

/// One thread stopped and read: its handle and `Eip`. On a failure, the handle if the thread
/// is stopped and so is owed a resume, else 0, and why.
fn stop_thread(thread: u32, count: usize) -> Result<(usize, u32), (usize, &'static str)> {
    if count == MOST_THREADS {
        return Err((0, "more threads than the room"));
    }
    // SAFETY: opens a handle by id; 0 is checked.
    let handle = unsafe { OpenThread(THREAD_ACCESS, 0, thread) };
    if handle == 0 {
        return Err((0, "a thread would not open"));
    }
    // SAFETY: the handle is open with the access to suspend.
    if unsafe { SuspendThread(handle) } == SUSPEND_FAILED {
        // SAFETY: closes the handle opened above; its thread was not stopped.
        unsafe { CloseHandle(handle) };
        return Err((0, "a thread would not stop"));
    }
    let mut context = [0u32; CONTEXT_WORDS];
    context[0] = CONTEXT_CONTROL;
    // SAFETY: the buffer is a whole aligned 32-bit CONTEXT, and the thread is suspended.
    if unsafe { GetThreadContext(handle, context.as_mut_ptr()) } == 0 {
        return Err((handle, "a thread's context would not read"));
    }
    Ok((handle, context[CONTEXT_EIP]))
}

/// The sites, the moment the script made with gdb.
fn write_sites(file: &str, rows: &'static [Patched]) {
    let plan = bind::plan_sites(rows, loaded).unwrap_or_else(|refusal| refused(file, &refusal));
    let mut live = Live;
    let mut pcs = [0; MOST_THREADS];
    let (count, applied) =
        with_others_stopped(&mut pcs, |pcs| bind::apply_sites(&plan, pcs, &mut live))
            .unwrap_or_else(|refusal| refused(file, &refusal));
    bind::report_sites(&plan, &pcs[..count], &live);
    if let Err(refusal) = applied {
        refused(file, &refusal);
    }
    log::info!(file = file, threads = count; "bind.sites");
}

/// One load's bind: a file the rows name gets its whole table, and its sites if it has any;
/// any other module is noted and left alone, one line a load.
fn bind_module(module: u32) {
    let Some(key) = headers(module).and_then(|head| bind::header_key(&File(head))) else {
        log::debug!(module:% = Hex(module); "bind.unread");
        return;
    };
    let row = FILES
        .iter()
        .find(|row| (row.pe, row.stamp, row.size) == (key.pe, key.stamp, key.size));
    if let Some(row) = row {
        bind_file(row, module);
    } else {
        log::debug!(module:% = Hex(module), stamp:% = Hex(key.stamp); "bind.other");
    }
    crate::listen_live::after_bind(module, &key);
}

fn bind_file(row: &'static BoundFile, module: u32) {
    if module != row.base {
        refused(
            row.file,
            &"its table's slots would miss: it is not at its base",
        );
    }
    let Some(table) = read_table(row.table) else {
        refused(row.file, &row.table);
    };
    let high = row.base + row.size;
    let pointed = bind::bind_table(table, &mut Live, row.base, high)
        .unwrap_or_else(|refusal| refused(row.file, &refusal));
    log::info!(
        file = row.file, table = row.table, written = pointed.written, failed = pointed.failed;
        "bind.table"
    );
    if let Some(slot) = pointed.first_failed {
        refused(row.file, &Hex(slot));
    }
    if pointed.failed > 0 {
        refused(row.file, &"a bad line in its table");
    }
    if !row.sites.is_empty() {
        write_sites(row.file, row.sites);
    }
}

/// `DllMain`'s part: with `HC5_BIND` set, the layer told where to call after each load and
/// the launcher, already parked and mapped, bound now.
pub fn attach(module: u32) {
    // SAFETY: the name is NUL-terminated and a size of 0 asks only for the length.
    let set = unsafe { GetEnvironmentVariableA(b"HC5_BIND\0".as_ptr(), null_mut(), 0) } != 0;
    if !set {
        return;
    }
    if module != SWAP_BASE {
        refused("the swap", &Hex(module));
    }
    mech_layer::process::set_after_load(bind_module);
    // SAFETY: a null name asks for the process's own module, which nothing can unmap.
    bind_module(unsafe { GetModuleHandleA(null()) });
}
