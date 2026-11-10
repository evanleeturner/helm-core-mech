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
//! **`HC5_SWAP` jumps a file's own functions to their rebuilt copies in the swap:** unset or
//! `none`, nothing; `all`, or a comma list of the addresses its swap table names. Only a file
//! whose row names a swap table is swapped, at its load, after its sites, so the file's own
//! start-up already runs the copies.
//!
//! A refusal is a named stop, an ERROR line and [`BIND_REFUSED`] as the exit code. A process
//! half bound would run on, answer from the wrong place and look alive.
//!
//! No allocator: each table is read into its own `VirtualAlloc` and kept to the process's
//! end, one small block a load. No thread is stopped under a heap call or a log line: see
//! [`bind::apply_sites`]. A swap's plan, its pages and the file it reads are freed after
//! its load.
use core::ffi::c_void;
use core::fmt;
use core::mem::ManuallyDrop;
use core::ops::{Deref, DerefMut};
use core::ptr::{null, null_mut, with_exposed_provenance, with_exposed_provenance_mut};
use core::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};

use mech_rows::pieces::{HEIGHT, KEPT};
use mech_rows::sites::{BoundFile, FILES, Patched};

use crate::bind::{self, File, Hex, Loaded, Memory, Refusal, Swap};
use crate::listen::{self, JUMP_BYTES, MOST_STOLEN, Placed};
use crate::listen_live::{Pages, Plain};

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
const MEM_RELEASE: u32 = 0x8000;
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
    fn GetModuleFileNameA(module: u32, path: *mut u8, size: u32) -> u32;
    fn IsBadReadPtr(at: *const c_void, size: u32) -> i32;
    fn VirtualProtect(at: *mut c_void, size: u32, new: u32, old: *mut u32) -> i32;
    fn FlushInstructionCache(process: usize, at: *const c_void, size: u32) -> i32;
    fn VirtualAlloc(at: *mut c_void, size: u32, kind: u32, protect: u32) -> *mut u8;
    fn VirtualFree(at: *mut c_void, size: u32, kind: u32) -> i32;
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
    core::str::from_utf8(read_whole(&path)?.keep()).ok()
}

/// Fresh pages holding `count` values, freed when dropped: a load's working room, the bind
/// having no allocator. [`Block::keep`] keeps them to the process's end instead.
struct Block<T> {
    at: *mut T,
    count: usize,
}

impl<T: Copy> Block<T> {
    /// Place `i` holding `fill(i)`; `None` if the pages would not come.
    fn new(count: usize, fill: impl Fn(usize) -> T) -> Option<Self> {
        let bytes = u32::try_from(count.max(1).checked_mul(size_of::<T>())?).ok()?;
        // SAFETY: asks for fresh pages; a null answer is checked before use.
        let at = unsafe { VirtualAlloc(null_mut(), bytes, MEM_COMMIT_RESERVE, PAGE_READWRITE) }
            .cast::<T>();
        if at.is_null() {
            return None;
        }
        for i in 0..count {
            // SAFETY: the pages hold `count` values and start on a page, so each place is in
            // bounds and aligned.
            unsafe { at.add(i).write(fill(i)) };
        }
        Some(Self { at, count })
    }

    /// The values kept to the process's end, as a table's bytes are.
    fn keep(self) -> &'static mut [T] {
        let kept = ManuallyDrop::new(self);
        // SAFETY: `new` wrote every place, and with the drop gone nothing frees the pages.
        unsafe { core::slice::from_raw_parts_mut(kept.at, kept.count) }
    }
}

impl<T> Deref for Block<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        // SAFETY: `new` wrote every place, and the pages stand until the drop.
        unsafe { core::slice::from_raw_parts(self.at, self.count) }
    }
}

impl<T> DerefMut for Block<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        // SAFETY: as `deref`, and `&mut self` makes this the only borrow.
        unsafe { core::slice::from_raw_parts_mut(self.at, self.count) }
    }
}

impl<T> Drop for Block<T> {
    fn drop(&mut self) {
        // SAFETY: frees the pages `new` asked for, once; no borrow of them outlives the drop.
        unsafe { VirtualFree(self.at.cast(), 0, MEM_RELEASE) };
    }
}

/// The whole of the file at the NUL-terminated `path`, in fresh pages.
fn read_whole(path: &[u8; PATH_BYTES]) -> Option<Block<u8>> {
    // SAFETY: the caller leaves a NUL inside the buffer after the path.
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
    // SAFETY: the handle is open; the high half is not asked for, no file the bind reads
    // nearing 4 GB.
    let size = unsafe { GetFileSize(file, null_mut()) };
    let block = Block::new(size as usize, |_| 0);
    let mut read = 0;
    let whole = block.as_ref().is_some_and(|block| {
        // SAFETY: the block is `size` bytes, made by `new`, and nothing borrows it yet.
        let done = unsafe { ReadFile(file, block.at, size, &raw mut read, null_mut()) };
        done != 0
    }) && read == size;
    // SAFETY: closes the handle opened above, once.
    unsafe { CloseHandle(file) };
    block.filter(|_| whole)
}

/// The file `module` was loaded from, read whole: its bytes as the disk holds them.
fn own_file(module: u32) -> Option<Block<u8>> {
    let mut path = [0; PATH_BYTES];
    // SAFETY: the buffer's size is passed, so it writes inside; a cut path answers that size.
    let length =
        unsafe { GetModuleFileNameA(module, path.as_mut_ptr(), PATH_BYTES as u32) } as usize;
    if length == 0 || length >= PATH_BYTES {
        return None;
    }
    read_whole(&path)
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
    let pieces = bind::pieces(pieces_value()).unwrap_or_else(|refusal| refused(row.file, &refusal));
    let setting = bind::Routes { pieces, kept: KEPT, height: HEIGHT };
    let routed = bind::bind_routed(table, &mut Live, row.base, high, &setting, |route, label, live| {
        match route {
            bind::Route::Through => mech_layer::through::entry(live, label),
            bind::Route::Height => mech_layer::draw::height_row(live),
            bind::Route::Export => None,
        }
    })
    .unwrap_or_else(|refusal| refused(row.file, &refusal));
    let pointed = routed.pointed;
    log::info!(
        file = row.file, table = row.table, written = pointed.written, failed = pointed.failed;
        "bind.table"
    );
    if pieces != bind::Pieces::All {
        log::info!(
            file = row.file, export = routed.export, through = routed.through,
            height = routed.height;
            "bind.routed"
        );
    }
    if let Some(slot) = pointed.first_failed {
        refused(row.file, &Hex(slot));
    }
    if pointed.failed > 0 {
        refused(row.file, &"a bad line in its table");
    }
    if !row.sites.is_empty() {
        write_sites(row.file, row.sites);
    }
    let setting = swap_value();
    if let Some(name) = row.swaps.filter(|_| setting != "none") {
        write_swaps(row, name, setting);
    }
}

/// The swap table's chosen rows jumped to their rebuilt copies, at the sites' moment: the
/// plan judged while the threads run, against the file's own bytes on disk read by RVA as
/// the rig read them; the pages opened, the jumps written with the others stopped, the pages
/// closed; then the lines, and a refusal last.
fn write_swaps(row: &'static BoundFile, name: &'static str, setting: &'static str) {
    let Some(table) = read_table(name) else {
        refused(row.file, &name);
    };
    let Some(copies) = headers(SWAP_BASE).and_then(|head| bind::header_key(&File(head))) else {
        refused(row.file, &"the swap's own headers would not read");
    };
    let bounds = bind::SwapBounds {
        file: row.file,
        low: row.base,
        high: row.base + row.size,
        copies_low: SWAP_BASE,
        copies_high: SWAP_BASE + copies.size,
        sites: row.sites,
    };
    let lines = table.lines().filter(|line| !line.trim().is_empty()).count();
    let Some(mut room) = Block::new(lines, |_| Swap::EMPTY) else {
        refused(row.file, &"no room for its swap plan");
    };
    let planned = {
        let Some(own) = own_file(row.base) else {
            refused(row.file, &"its own file would not read");
        };
        bind::plan_swaps(table, setting, &bounds, &File(&own), &Live, &mut room)
    }
    .unwrap_or_else(|refusal| refused(row.file, &refusal));
    let plan = &room[..planned];
    let pages =
        Block::new(plan.len(), |i| as_placed(&plan[i])).zip(Block::new(2 * plan.len(), |_| (0, 0)));
    let Some((placed, mut opened)) = pages else {
        refused(row.file, &"no room for its swap's pages");
    };
    let open = listen::open_pages(&placed, PAGE_EXECUTE_READWRITE, &mut Pages, &mut opened)
        .unwrap_or_else(|page| refused(row.file, &Hex(page)));
    let mut pcs = [0; MOST_THREADS];
    let stopped = with_others_stopped(&mut pcs, |pcs| bind::apply_swaps(plan, pcs, &mut Plain));
    listen::close_pages(&opened[..open], &mut Pages);
    if let (Some(first), Some(last)) = (plan.first(), plan.last()) {
        // SAFETY: flushes the range the rows span, inside the file's image.
        unsafe {
            FlushInstructionCache(
                CURRENT_PROCESS,
                with_exposed_provenance::<c_void>(first.address as usize),
                last.address + JUMP_BYTES as u32 - first.address,
            );
        }
    }
    bind::report_swaps(row.file, setting, plan, &Live);
    match stopped {
        Ok((_, Ok(()))) => {}
        Ok((_, Err(refusal))) | Err(refusal) => refused(row.file, &refusal),
    }
}

/// A planned swap as [`listen::open_pages`] reads a row: its 5 bytes, the jump's target and
/// the file's bytes there.
fn as_placed(swap: &Swap<'_>) -> Placed {
    let mut saved = [0; MOST_STOLEN];
    saved[..JUMP_BYTES].copy_from_slice(&swap.old);
    Placed {
        address: swap.address,
        stub: swap.target,
        length: JUMP_BYTES as u32,
        saved,
    }
}

/// `HC5_PIECES`' value as [`attach`] read it, its bytes and length; null for unset.
static PIECES_AT: AtomicPtr<u8> = AtomicPtr::new(null_mut());
static PIECES_LENGTH: AtomicUsize = AtomicUsize::new(0);
/// `HC5_SWAP`'s value as [`attach`] judged it, as `PIECES_AT` holds the pieces'.
static SWAP_AT: AtomicPtr<u8> = AtomicPtr::new(null_mut());
static SWAP_LENGTH: AtomicUsize = AtomicUsize::new(0);

/// The pieces setting [`attach`] read, for each load's bind.
fn pieces_value() -> Option<&'static str> {
    kept(&PIECES_AT, &PIECES_LENGTH)
}

/// The swap setting [`attach`] judged, for each load's bind; `none` before it has.
fn swap_value() -> &'static str {
    kept(&SWAP_AT, &SWAP_LENGTH).unwrap_or("none")
}

/// A setting `attach` stored; `None` for none stored.
fn kept(at: &AtomicPtr<u8>, length: &AtomicUsize) -> Option<&'static str> {
    let at = at.load(Ordering::Acquire);
    if at.is_null() {
        return None;
    }
    let length = length.load(Ordering::Relaxed);
    // SAFETY: `attach` stored a `'static` str's length, then its pointer, never changed after.
    Some(unsafe {
        core::str::from_utf8_unchecked(core::slice::from_raw_parts(at.cast_const(), length))
    })
}

/// `name`'s value, NUL-terminated `name` asked of the process, its bytes kept to the
/// process's end as a table's are; `None` for unset. Windows answers an empty value as it
/// answers an unset one, so an empty value is unset here too.
fn variable(name: &[u8]) -> Option<&'static [u8]> {
    // SAFETY: the name is NUL-terminated and a size of 0 asks only for the length.
    let size = unsafe { GetEnvironmentVariableA(name.as_ptr(), null_mut(), 0) };
    if size <= 1 {
        return None;
    }
    // SAFETY: asks for fresh pages; a null answer is checked before use.
    let block = unsafe { VirtualAlloc(null_mut(), size, MEM_COMMIT_RESERVE, PAGE_READWRITE) };
    if block.is_null() {
        return None;
    }
    // SAFETY: the block is `size` bytes and the size is passed, so it writes inside.
    let length = unsafe { GetEnvironmentVariableA(name.as_ptr(), block, size) };
    if length == 0 || length >= size {
        return None;
    }
    // SAFETY: `length` bytes were just written there, and the block is never freed or
    // written again, so it lives as `'static` says.
    Some(unsafe { core::slice::from_raw_parts(block.cast_const(), length as usize) })
}

/// `DllMain`'s part: with `HC5_BIND` set, the pieces and swap settings read and judged once,
/// the layer told where to call after each load and the launcher, already parked and mapped,
/// bound now.
pub fn attach(module: u32) {
    // SAFETY: the name is NUL-terminated and a size of 0 asks only for the length.
    let set = unsafe { GetEnvironmentVariableA(b"HC5_BIND\0".as_ptr(), null_mut(), 0) } != 0;
    if !set {
        return;
    }
    if module != SWAP_BASE {
        refused("the swap", &Hex(module));
    }
    let value = variable(b"HC5_PIECES\0").map(|bytes| {
        core::str::from_utf8(bytes).unwrap_or_else(|_| refused("the swap", &"HC5_PIECES: no text"))
    });
    log::info!(pieces = value.unwrap_or("unset"); "bind.pieces");
    if let Err(refusal) = bind::pieces(value) {
        refused("the swap", &refusal);
    }
    if let Some(value) = value {
        PIECES_LENGTH.store(value.len(), Ordering::Relaxed);
        PIECES_AT.store(value.as_ptr().cast_mut(), Ordering::Release);
    }
    let swap = variable(b"HC5_SWAP\0").map(|bytes| {
        core::str::from_utf8(bytes).unwrap_or_else(|_| refused("the swap", &"HC5_SWAP: no text"))
    });
    let swap = bind::swap_set(swap).unwrap_or_else(|refusal| refused("the swap", &refusal));
    SWAP_LENGTH.store(swap.len(), Ordering::Relaxed);
    SWAP_AT.store(swap.as_ptr().cast_mut(), Ordering::Release);
    mech_layer::process::set_after_load(bind_module);
    // SAFETY: a null name asks for the process's own module, which nothing can unmap.
    bind_module(unsafe { GetModuleHandleA(null()) });
}
