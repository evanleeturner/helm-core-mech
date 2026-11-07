// SPDX-License-Identifier: MIT
//! The listener's Windows side: the mapped file, the stubs' blocks, the ring routine and the
//! fault's words, behind [`crate::listen`]'s judging half.
//!
//! **`LISTEN_DIR` turns it on, naming the folder of tables and of each process's mapped file,
//! in the process's own path form.** Unset, or a file with no table there, and this does
//! nothing. The bind's own switch must be set too, since this runs from the bind's load hook.
//!
//! **Everything a stub touches stands at a fixed address,** chosen clear of the game's images:
//! a stub carries absolute addresses, the ring routine needs no register to find the ring, and
//! the game's own allocations land where they land without a listener. If a place is taken the
//! bind refuses by name.
//!
//! **The file is the sink, not the log.** A stub or a fault writes only to mapped memory, with
//! no lock, no call and no stand-in, so it cannot re-enter a stub or wait on a lock a stopped
//! thread holds, and what it wrote is in the file's pages whatever becomes of the process. A
//! program outside the process reads it; the log gets one line a load, after the threads run.

use core::arch::{asm, global_asm};
use core::ffi::c_void;
use core::ptr::{null_mut, with_exposed_provenance, with_exposed_provenance_mut};
use core::slice;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use mech_rows::sites::FILES;

use crate::bind::{HeaderKey, Hex, Memory};
use crate::listen::{
    self, FAULT_BYTES, FAULT_HEAD_WORDS, FAULT_SLOTS, FAULT_STACK_WORDS, FAULTS_AT, FAULTS_SEEN_AT,
    HEAD_AT, MAGIC, MAP_BYTES, MOST_ROWS, MOST_STOLEN, MOST_TABLES, Placed, Protect, RECORD_BYTES,
    RING_AT, RING_RECORDS, ROW_BYTES, ROW_COUNT, ROWS_AT, ROWS_USED_AT, STUB_ROOM, Stub, TABLES_AT,
    TableWord, table_word,
};
use crate::live::{MOST_THREADS, PATH_BYTES, path_in, read_table_in, refused, with_others_stopped};

/// The mapped file's place: above the game's images, below the swap at `0x6000_0000`.
const VIEW_AT: u32 = 0x3000_0000;
/// One stub a row, across all files, so a row's number alone says where its stub is.
const STUBS_AT: u32 = 0x3B00_0000;
const STUBS_BYTES: u32 = MOST_ROWS * STUB_ROOM as u32;
/// One [`Placed`] a row, by number, as the stubs.
const PLACED_AT: u32 = 0x3B40_0000;
const PLACED_BYTES: u32 = MOST_ROWS * size_of::<Placed>() as u32;
/// The pages a bind opened: 2 a row at worst, a head being able to cross a page.
const OPENED_AT: u32 = PLACED_AT + PLACED_BYTES;
const OPENED_BYTES: u32 = 2 * MOST_ROWS * size_of::<(u32, u32)>() as u32;

const _: () = assert!(VIEW_AT + MAP_BYTES <= STUBS_AT);
const _: () = assert!(STUBS_AT + STUBS_BYTES <= PLACED_AT);
// The ring routine multiplies a slot by exactly this.
const _: () = assert!(RECORD_BYTES == 40);
const _: () = assert!(FAULT_BYTES as usize == 4 * (FAULT_HEAD_WORDS + FAULT_STACK_WORDS));

/// The header's second word: the mapped size, so a reader can refuse another layout.
const SIZE_AT: u32 = 4;

const GENERIC_READ_WRITE: u32 = 0xC000_0000;
const FILE_SHARE_READ_WRITE: u32 = 3;
const CREATE_ALWAYS: u32 = 2;
const FILE_ATTRIBUTE_NORMAL: u32 = 0x80;
const INVALID_HANDLE_VALUE: usize = usize::MAX;
const MEM_COMMIT_RESERVE: u32 = 0x3000;
const PAGE_READWRITE: u32 = 4;
const PAGE_EXECUTE_READWRITE: u32 = 0x40;
const FILE_MAP_WRITE: u32 = 2;
/// `GetCurrentProcess`'s fixed answer, so no call for it.
const CURRENT_PROCESS: usize = usize::MAX;
/// `AddVectoredExceptionHandler`'s "ahead of the others".
const CALL_FIRST: u32 = 1;
const EXCEPTION_CONTINUE_SEARCH: i32 = 0;
const PAGE_BYTES: u32 = 0x1000;

/// A 32-bit `CONTEXT`'s registers, as word places.
const CONTEXT_EDI: usize = 39;
const CONTEXT_ESI: usize = 40;
const CONTEXT_EBX: usize = 41;
const CONTEXT_EDX: usize = 42;
const CONTEXT_ECX: usize = 43;
const CONTEXT_EAX: usize = 44;
const CONTEXT_EBP: usize = 45;
const CONTEXT_EIP: usize = 46;
const CONTEXT_EFLAGS: usize = 48;
const CONTEXT_ESP: usize = 49;
/// An `EXCEPTION_RECORD`'s words.
const RECORD_CODE: usize = 0;
const RECORD_ADDRESS: usize = 3;
const RECORD_PARAMETERS: usize = 4;
const RECORD_SECOND_PARAMETER: usize = 6;

/// `EXCEPTION_POINTERS`, each as words.
#[repr(C)]
struct Pointers {
    record: *const u32,
    context: *const u32,
}

unsafe extern "system" {
    fn GetCurrentProcessId() -> u32;
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
    fn CloseHandle(handle: usize) -> i32;
    fn CreateFileMappingA(
        file: usize,
        security: *mut c_void,
        protect: u32,
        high: u32,
        low: u32,
        name: *const u8,
    ) -> usize;
    fn MapViewOfFileEx(
        mapping: usize,
        access: u32,
        high: u32,
        low: u32,
        bytes: u32,
        at: *mut c_void,
    ) -> *mut u8;
    fn AddVectoredExceptionHandler(
        first: u32,
        handler: unsafe extern "system" fn(*const Pointers) -> i32,
    ) -> *mut c_void;
}

unsafe extern "C" {
    /// The ring routine below. Takes the row above its return address and pops it.
    fn listen_record();
}

// The contract, from `crate::listen`'s stub: entered by `call` with the stack holding, from
// the top, the way back into the stub, the row's number, the saved flags, the caller, then the
// caller's arguments. Every register and the flags were already free or saved; it keeps `eax`,
// `ecx` and `edx` as they came and uses nothing else. No float, no call, no lock but the ticket.
//
// The caller's 3 registers go in the record because a value in an MSVC 4.x function's first
// instructions may be on no stack at all. The time stamp comes last, so a slot a reader finds
// half written can be told by it. The `_` is this target's C name for `listen_record`.
global_asm!(
    ".globl _listen_record",
    "_listen_record:",
    "push eax",
    "push ecx",
    "push edx",
    "mov ecx, 1",
    "lock xadd dword ptr [{head}], ecx",
    "and ecx, {mask}",
    "lea ecx, [ecx + 4 * ecx]",
    "shl ecx, 3",
    "add ecx, {ring}",
    "mov eax, dword ptr fs:[0x24]",
    "shl eax, 16",
    "mov ax, word ptr [esp + 16]",
    "mov dword ptr [ecx], eax",
    "mov eax, dword ptr [esp + 24]",
    "mov dword ptr [ecx + 4], eax",
    "mov eax, dword ptr [esp + 8]",
    "mov dword ptr [ecx + 8], eax",
    "mov eax, dword ptr [esp + 4]",
    "mov dword ptr [ecx + 12], eax",
    "mov eax, dword ptr [esp]",
    "mov dword ptr [ecx + 16], eax",
    "mov eax, dword ptr [esp + 28]",
    "mov dword ptr [ecx + 20], eax",
    "mov eax, dword ptr [esp + 32]",
    "mov dword ptr [ecx + 24], eax",
    "mov eax, dword ptr [esp + 36]",
    "mov dword ptr [ecx + 28], eax",
    "mov eax, dword ptr [esp + 40]",
    "mov dword ptr [ecx + 32], eax",
    "rdtsc",
    "mov dword ptr [ecx + 36], eax",
    "pop edx",
    "pop ecx",
    "pop eax",
    "ret 4",
    head = const VIEW_AT + HEAD_AT,
    mask = const RING_RECORDS - 1,
    ring = const VIEW_AT + RING_AT,
);

/// Whether the mapping and the blocks stand.
static BORN: AtomicBool = AtomicBool::new(false);
/// A bind is between opening pages and closing them: their list has one home.
static BUSY: AtomicBool = AtomicBool::new(false);

/// The mapped file. One exists only once the whole mapping stands at [`VIEW_AT`], which
/// nothing unmaps, so its words can be read and written from then on.
#[derive(Clone, Copy)]
struct View;

impl View {
    fn word(at: u32) -> usize {
        (VIEW_AT + at) as usize
    }

    fn get(self, at: u32) -> u32 {
        // SAFETY: a `View` means the mapping stands, and every caller's `at` is inside it.
        unsafe { with_exposed_provenance::<u32>(Self::word(at)).read_volatile() }
    }

    fn set(self, at: u32, value: u32) {
        // SAFETY: as `get`; the mapping is writable.
        unsafe { with_exposed_provenance_mut::<u32>(Self::word(at)).write_volatile(value) }
    }

    fn add(self, at: u32, value: u32) {
        self.set(at, self.get(at).wrapping_add(value));
    }
}

fn born() -> Option<View> {
    BORN.load(Ordering::Acquire).then_some(View)
}

/// The image the game is yet to map at a place this holds would move and refuse, so each
/// block's place is checked against every file the rows name.
fn clear_of_images(low: u32, high: u32) -> bool {
    FILES
        .iter()
        .all(|file| high <= file.base || file.base + file.size <= low)
}

/// A block at its fixed place, or the refusal naming it.
fn block(what: &str, at: u32, bytes: u32, protect: u32) {
    if !clear_of_images(at, at + bytes) {
        refused(what, &"its fixed place is where an image must map");
    }
    let wanted = with_exposed_provenance_mut::<c_void>(at as usize);
    // SAFETY: asks for fresh pages at a place; an answer anywhere else is checked below.
    let got = unsafe { VirtualAlloc(wanted, bytes, MEM_COMMIT_RESERVE, protect) };
    if got.addr() != at as usize {
        refused(what, &"its fixed place is taken");
    }
}

/// The mapped file made, its handles closed since the view holds what it needs, the blocks
/// made and the fault routine on. Refuses by name: a listener half made would look like a
/// flight with nothing to log.
fn birth(pid: u32) -> View {
    const WHAT: &str = "the listener's file";
    if !clear_of_images(VIEW_AT, VIEW_AT + MAP_BYTES) {
        refused(WHAT, &"its fixed place is where an image must map");
    }
    let mut name = [0; 32];
    let length = listen::map_name(pid, &mut name);
    let Ok(name) = core::str::from_utf8(&name[..length]) else {
        refused(WHAT, &"its name is not text");
    };
    let mut path = [0; PATH_BYTES];
    if !path_in(b"LISTEN_DIR\0", name, &mut path) {
        refused(WHAT, &"no folder to put it in");
    }
    // SAFETY: the path is NUL-terminated, in the zeroed buffer; a bad answer is checked.
    let file = unsafe {
        CreateFileA(
            path.as_ptr(),
            GENERIC_READ_WRITE,
            FILE_SHARE_READ_WRITE,
            null_mut(),
            CREATE_ALWAYS,
            FILE_ATTRIBUTE_NORMAL,
            null_mut(),
        )
    };
    if file == INVALID_HANDLE_VALUE {
        refused(WHAT, &"it would not open");
    }
    // SAFETY: the handle is open for read and write; no name is asked for; 0 is checked.
    let mapping = unsafe {
        CreateFileMappingA(
            file,
            null_mut(),
            PAGE_READWRITE,
            0,
            MAP_BYTES,
            core::ptr::null(),
        )
    };
    if mapping == 0 {
        refused(WHAT, &"it would not map");
    }
    let wanted = with_exposed_provenance_mut::<c_void>(VIEW_AT as usize);
    // SAFETY: the mapping is open and writable, and its size is the size asked for; an answer
    // anywhere else, null among them, is checked below and then not used.
    let view = unsafe { MapViewOfFileEx(mapping, FILE_MAP_WRITE, 0, 0, MAP_BYTES, wanted) };
    // SAFETY: closes the 2 handles opened above, once each; the view keeps its own hold.
    unsafe {
        CloseHandle(mapping);
        CloseHandle(file);
    }
    if view.addr() != VIEW_AT as usize {
        refused(WHAT, &"its fixed place is taken");
    }
    block(
        "the stub block",
        STUBS_AT,
        STUBS_BYTES,
        PAGE_EXECUTE_READWRITE,
    );
    block(
        "the bind's block",
        PLACED_AT,
        PLACED_BYTES + OPENED_BYTES,
        PAGE_READWRITE,
    );
    let view = View;
    view.set(SIZE_AT, MAP_BYTES);
    view.set(0, MAGIC);
    BORN.store(true, Ordering::Release);
    // SAFETY: `on_fault` is `'static` and has the handler's signature.
    if unsafe { AddVectoredExceptionHandler(CALL_FIRST, on_fault) }.is_null() {
        refused(WHAT, &"no fault routine");
    }
    log::info!(at:% = Hex(VIEW_AT), bytes = MAP_BYTES; "listen.map");
    view
}

/// A fault's words into the next slot, then always on to whoever handles it: this watches, and
/// the game's own handling of a fault is the game's. `IsBadReadPtr` faults arrive here too, so
/// slots are a ring and [`FAULTS_SEEN_AT`] tells what was overwritten: the fatal one comes last.
///
/// Stack words are bounded by the thread's own stack, not probed: a probe raises, and would
/// come back here.
unsafe extern "system" fn on_fault(pointers: *const Pointers) -> i32 {
    // SAFETY: this runs only after birth, so the word is mapped; the handler's argument is the
    // system's and valid for this call, and a record and a context are whole objects there.
    unsafe {
        let seen = AtomicU32::from_ptr(with_exposed_provenance_mut(View::word(FAULTS_SEEN_AT)))
            .fetch_add(1, Ordering::Relaxed);
        let slot = FAULTS_AT + (seen & (FAULT_SLOTS - 1)) * FAULT_BYTES;
        let record = (*pointers).record;
        let context = (*pointers).context;
        let esp = *context.add(CONTEXT_ESP);
        let (tid, top, bottom): (u32, u32, u32);
        asm!(
            "mov {tid}, dword ptr fs:[0x24]",
            "mov {top}, dword ptr fs:[4]",
            "mov {bottom}, dword ptr fs:[8]",
            tid = out(reg) tid,
            top = out(reg) top,
            bottom = out(reg) bottom,
            options(nostack, readonly, preserves_flags),
        );
        let view = View;
        let second = if *record.add(RECORD_PARAMETERS) >= 2 {
            *record.add(RECORD_SECOND_PARAMETER)
        } else {
            0
        };
        let head = [
            *record.add(RECORD_CODE),
            *record.add(RECORD_ADDRESS),
            *context.add(CONTEXT_EIP),
            esp,
            *context.add(CONTEXT_EBP),
            *context.add(CONTEXT_EAX),
            *context.add(CONTEXT_ECX),
            *context.add(CONTEXT_EDX),
            *context.add(CONTEXT_EBX),
            *context.add(CONTEXT_ESI),
            *context.add(CONTEXT_EDI),
            *context.add(CONTEXT_EFLAGS),
            tid,
            view.get(HEAD_AT),
            second,
        ];
        for (place, word) in (1u32..).zip(head) {
            view.set(slot + 4 * place, word);
        }
        let room = if (bottom..top).contains(&esp) {
            ((top - esp) / 4).min(FAULT_STACK_WORDS as u32)
        } else {
            0
        };
        for place in 0..FAULT_STACK_WORDS as u32 {
            let word = if place < room {
                with_exposed_provenance::<u32>((esp + 4 * place) as usize).read_volatile()
            } else {
                0
            };
            view.set(slot + 4 * (FAULT_HEAD_WORDS as u32 + place), word);
        }
        // Last, as a record's time stamp: a slot with no ticket was never finished.
        view.set(slot, seen.wrapping_add(1));
    }
    EXCEPTION_CONTINUE_SEARCH
}

/// This process's pages copied plainly, for under the stop: `VirtualProtect`, which the bind's
/// other memory calls per write, may wait on a lock a stopped thread holds. Good only over
/// pages already open, and only for addresses a checked table named.
struct Plain;

impl Memory for Plain {
    fn read(&self, at: u32, out: &mut [u8]) -> bool {
        // SAFETY: a checked table's rows lie inside a loaded image, read whole; `out` is ours.
        unsafe {
            core::ptr::copy_nonoverlapping(
                with_exposed_provenance::<u8>(at as usize),
                out.as_mut_ptr(),
                out.len(),
            );
        }
        true
    }

    fn write(&mut self, at: u32, bytes: &[u8]) -> bool {
        // SAFETY: as `read`, and `open_pages` made the row's pages writable before the stop.
        unsafe {
            core::ptr::copy_nonoverlapping(
                bytes.as_ptr(),
                with_exposed_provenance_mut::<u8>(at as usize),
                bytes.len(),
            );
        }
        true
    }
}

/// `VirtualProtect` by the page, for [`listen::open_pages`].
struct Pages;

impl Protect for Pages {
    fn set(&mut self, page: u32, protection: u32) -> Option<u32> {
        let mut was = 0;
        let at = with_exposed_provenance_mut::<c_void>(page as usize);
        // SAFETY: changes a page's protection; a bad page answers 0 and changes nothing.
        let done = unsafe { VirtualProtect(at, PAGE_BYTES, protection, &raw mut was) } != 0;
        done.then_some(was)
    }
}

/// A file's rows made once: a slot in the header, a stub and a [`Placed`] per row, each stub
/// holding the head as this first load has it. A later load reuses them, its head judged
/// against the same bytes.
fn build(module: u32, key: &HeaderKey, name: &str, table: &'static str) -> u32 {
    let count =
        listen::check_table(table, module, key.size).unwrap_or_else(|why| refused(name, &why));
    let view = born().unwrap_or_else(|| {
        // SAFETY: reads this process's id; touches no memory.
        birth(unsafe { GetCurrentProcessId() })
    });
    let index = view.get(TABLES_AT);
    if index == MOST_TABLES {
        refused(name, &"more files than the header has room for");
    }
    let first = view.get(ROWS_USED_AT);
    if first + count.rows > MOST_ROWS {
        refused(name, &"more rows than the header has room for");
    }
    for (number, row) in (first..).zip(listen::rows(table)) {
        let row = row.unwrap_or_else(|why| refused(name, &why));
        let mut saved = [0; MOST_STOLEN];
        Plain.read(row.address, &mut saved[..row.length]);
        let at = STUBS_AT + number * STUB_ROOM as u32;
        // SAFETY: the block stands after birth, and a checked number's room is its own.
        let out = unsafe {
            slice::from_raw_parts_mut(with_exposed_provenance_mut::<u8>(at as usize), STUB_ROOM)
        };
        let _written = listen::write_stub(
            &Stub {
                at,
                row: &row,
                stolen: &saved[..row.length],
                count: VIEW_AT + ROWS_AT + number * ROW_BYTES + ROW_COUNT,
                number,
                record: listen_record as *const () as u32,
            },
            out,
        );
        view.set(ROWS_AT + number * ROW_BYTES, row.address);
        view.set(ROWS_AT + number * ROW_BYTES + 4, row.length as u32);
        let placed = Placed {
            address: row.address,
            stub: at,
            length: row.length as u32,
            saved,
        };
        // SAFETY: as the stub's room; this block holds a `Placed` per number, aligned.
        unsafe {
            with_exposed_provenance_mut::<Placed>(PLACED_AT as usize)
                .add(number as usize)
                .write(placed);
        }
    }
    // SAFETY: the block stands; only the range just written is named.
    unsafe {
        FlushInstructionCache(
            CURRENT_PROCESS,
            with_exposed_provenance::<c_void>((STUBS_AT + first * STUB_ROOM as u32) as usize),
            count.rows * STUB_ROOM as u32,
        );
    }
    for (word, value) in [
        (TableWord::Stamp, key.stamp),
        (TableWord::Size, key.size),
        (TableWord::Base, module),
        (TableWord::FirstRow, first),
        (TableWord::Rows, count.rows),
        (TableWord::Listed, count.listed),
        (TableWord::Found, count.found),
        (
            TableWord::Placed,
            PLACED_AT + first * size_of::<Placed>() as u32,
        ),
        (TableWord::Stubs, STUBS_AT + first * STUB_ROOM as u32),
    ] {
        view.set(table_word(index, word), value);
    }
    view.set(ROWS_USED_AT, first + count.rows);
    view.set(TABLES_AT, index + 1);
    index
}

/// The rows' jumps into a file just loaded. Pages open before the stop and close after it, by
/// the page, since the stopped threads may hold what `VirtualProtect` waits on.
fn place(view: View, module: u32, index: u32, name: &str) {
    if view.get(table_word(index, TableWord::Base)) != module {
        refused(name, &"a later load is not at the first load's base");
    }
    if BUSY.swap(true, Ordering::Acquire) {
        refused(name, &"2 threads bound at once");
    }
    let first = view.get(table_word(index, TableWord::FirstRow));
    let rows = view.get(table_word(index, TableWord::Rows)) as usize;
    // SAFETY: the blocks stand after birth; `build` wrote these rows, and nothing writes them
    // again; `BUSY` makes this the list's only holder.
    let (placed, opened) = unsafe {
        (
            slice::from_raw_parts(
                with_exposed_provenance::<Placed>(PLACED_AT as usize).add(first as usize),
                rows,
            ),
            slice::from_raw_parts_mut(
                with_exposed_provenance_mut::<(u32, u32)>(OPENED_AT as usize),
                2 * rows,
            ),
        )
    };
    let mut pages = Pages;
    let open = listen::open_pages(placed, PAGE_EXECUTE_READWRITE, &mut pages, opened)
        .unwrap_or_else(|page| refused(name, &Hex(page)));
    let mut pcs = [0; MOST_THREADS];
    let (threads, placement) =
        with_others_stopped(&mut pcs, |pcs| listen::place_rows(placed, pcs, &mut Plain))
            .unwrap_or_else(|why| refused(name, &why));
    let stuck = listen::close_pages(&opened[..open], &mut pages);
    BUSY.store(false, Ordering::Release);
    let (low, high) = placed.iter().fold((u32::MAX, 0), |(low, high), row| {
        (low.min(row.address), high.max(row.address + row.length))
    });
    // SAFETY: flushes the image's range the rows span; a range with no rows flushes nothing.
    unsafe {
        FlushInstructionCache(
            CURRENT_PROCESS,
            with_exposed_provenance::<c_void>(low as usize),
            high.saturating_sub(low),
        );
    }
    view.add(table_word(index, TableWord::Loads), 1);
    view.add(table_word(index, TableWord::Written), placement.written);
    view.add(table_word(index, TableWord::Held), placement.held);
    view.add(table_word(index, TableWord::Moved), placement.moved);
    view.add(table_word(index, TableWord::Parked), placement.parked);
    log::info!(
        file = name, module:% = Hex(module), first = first, rows = rows, written = placement.written,
        held = placement.held, moved = placement.moved, parked = placement.parked,
        threads = threads, pages = open, stuck = stuck;
        "listen.rows"
    );
}

/// One load's part, after the bind's own: sites first, then rows, so a site is written over
/// the file's own bytes and a row's head is read with the sites in. A file seen before reuses
/// its stubs, and a file with no table is left alone in silence: `bind.other` already says so.
pub(crate) fn after_bind(module: u32, key: &HeaderKey) {
    let mut name = [0; 32];
    let length = listen::table_name(key.stamp, key.size, &mut name);
    let Ok(name) = core::str::from_utf8(&name[..length]) else {
        return;
    };
    let seen = born().and_then(|view| {
        (0..view.get(TABLES_AT)).find(|index| {
            view.get(table_word(*index, TableWord::Stamp)) == key.stamp
                && view.get(table_word(*index, TableWord::Size)) == key.size
        })
    });
    let index = match seen {
        Some(index) => index,
        None => {
            let Some(table) = read_table_in(b"LISTEN_DIR\0", name) else {
                return;
            };
            build(module, key, name, table)
        }
    };
    place(View, module, index, name);
}
