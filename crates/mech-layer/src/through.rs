// SPDX-License-Identifier: MIT
//! The pass-through: with the pieces off, a slot the bind does not keep points at an entry of
//! [`pool`], and its call goes on to the original the slot held, nothing answered here. Each
//! entry is one `call` into [`common`], 5 bytes, so the address it pushes names the entry;
//! [`note`] hands back where `common` returns to, with every register and flag as the caller
//! left them and the caller's return address on top.
//!
//! With `imp.through` off, `note` hands back the entry's original, and the call runs as if
//! the caller had called it itself, as before the frames. With it on, `note` takes a frame of
//! this thread's and hands back its stub in [`fronts`]: that keeps the call's declared
//! arguments, puts the frame's stub in [`backs`] where the caller's return address stood, and
//! goes on to the original. The original returns into that stub, which reads `eax` and the
//! last error, writes the row, sets the error back and returns to the caller, `esp` as the
//! original left it, so a stdcall's pops hold. A stub a frame: its own return address names
//! the frame, so a call an unwind left cannot send a later return to the wrong caller. With
//! every frame of this thread's taken, the row comes before the call, with no `v` or `e`.
//!
//! One entry serves each original and label, however many loads bind it.
use core::ptr::{null_mut, with_exposed_provenance, with_exposed_provenance_mut};
use core::slice;
use core::sync::atomic::{AtomicPtr, AtomicU8, AtomicU32, AtomicUsize, Ordering};

use kernel_process::Text;

use crate::window::Numbers;

/// The entries: the 3 tables the rig links today hold 267 rows, so room to spare.
const COUNT: usize = 512;
/// One `call rel32`, the bytes an entry or a stub takes.
const STRIDE: usize = 5;
/// The threads that may hold frames at once.
const THREADS: usize = 32;
/// The frames one thread may hold, its calls nested.
const DEEP: usize = 32;
/// Every frame, and so the stubs in [`fronts`] and in [`backs`]: row `t`'s frame `j` is
/// `DEEP * t + j`.
const FRAMES: usize = THREADS * DEEP;
/// The most arguments a frame keeps: `CreateFileA`'s 7, with 1 to spare.
const MOST: usize = 8;
/// The most bytes of `CreateFileA`'s name a row shows: `MAX_PATH`.
const NAME_MOST: u32 = 260;

/// Each import whose signature helm declares, and its count of arguments: `slots.rs`'s rows
/// and `input.rs`'s `GetAsyncKeyState`, by the same label and count. The host test
/// `tests/signatures.rs` fails a count that differs and an `_AIL_` row left out. An import
/// missing here writes no `a`.
const SIGNATURES: &[(&str, u8)] = &[
    ("KERNEL32!CreateFileA", 7),
    ("KERNEL32!ReadFile", 5),
    ("KERNEL32!SetFilePointer", 4),
    ("KERNEL32!WriteFile", 5),
    ("MSS32!_AIL_active_sample_count@4", 1),
    ("MSS32!_AIL_active_sequence_count@4", 1),
    ("MSS32!_AIL_allocate_sequence_handle@4", 1),
    ("MSS32!_AIL_end_sample@4", 1),
    ("MSS32!_AIL_init_sequence@12", 3),
    ("MSS32!_AIL_load_sample_buffer@16", 4),
    ("MSS32!_AIL_midiOutOpen@12", 3),
    ("MSS32!_AIL_minimum_sample_buffer_size@12", 3),
    ("MSS32!_AIL_release_sequence_handle@4", 1),
    ("MSS32!_AIL_sample_buffer_ready@4", 1),
    ("MSS32!_AIL_serve@0", 0),
    ("MSS32!_AIL_set_sequence_loop_count@8", 2),
    ("MSS32!_AIL_set_sequence_volume@12", 3),
    ("MSS32!_AIL_shutdown@0", 0),
    ("MSS32!_AIL_start_sequence@4", 1),
    ("MSS32!_AIL_startup@0", 0),
    ("MSS32!_AIL_stop_sequence@4", 1),
    ("MSS32!_AIL_waveOutClose@4", 1),
    ("MSS32!_AIL_waveOutOpen@16", 4),
    ("USER32!GetAsyncKeyState", 1),
    ("WAIL32!_AIL_allocate_file_sample@12", 3),
    ("WAIL32!_AIL_allocate_sample_handle@4", 1),
    ("WAIL32!_AIL_allocate_sequence_handle@4", 1),
    ("WAIL32!_AIL_end_sample@4", 1),
    ("WAIL32!_AIL_init_sample@4", 1),
    ("WAIL32!_AIL_init_sequence@12", 3),
    ("WAIL32!_AIL_load_sample_buffer@16", 4),
    ("WAIL32!_AIL_lock_channel@4", 1),
    ("WAIL32!_AIL_midiOutOpen@12", 3),
    ("WAIL32!_AIL_register_EOS_callback@8", 2),
    ("WAIL32!_AIL_register_timer@4", 1),
    ("WAIL32!_AIL_release_channel@8", 2),
    ("WAIL32!_AIL_release_sample_handle@4", 1),
    ("WAIL32!_AIL_release_sequence_handle@4", 1),
    ("WAIL32!_AIL_release_timer_handle@4", 1),
    ("WAIL32!_AIL_resume_sample@4", 1),
    ("WAIL32!_AIL_resume_sequence@4", 1),
    ("WAIL32!_AIL_sample_buffer_ready@4", 1),
    ("WAIL32!_AIL_sample_user_data@8", 2),
    ("WAIL32!_AIL_send_channel_voice_message@20", 5),
    ("WAIL32!_AIL_sequence_status@4", 1),
    ("WAIL32!_AIL_serve@0", 0),
    ("WAIL32!_AIL_set_preference@8", 2),
    ("WAIL32!_AIL_set_sample_loop_count@8", 2),
    ("WAIL32!_AIL_set_sample_pan@8", 2),
    ("WAIL32!_AIL_set_sample_playback_rate@8", 2),
    ("WAIL32!_AIL_set_sample_type@12", 3),
    ("WAIL32!_AIL_set_sample_user_data@12", 3),
    ("WAIL32!_AIL_set_sample_volume@8", 2),
    ("WAIL32!_AIL_set_sequence_volume@12", 3),
    ("WAIL32!_AIL_set_timer_divisor@8", 2),
    ("WAIL32!_AIL_set_timer_period@8", 2),
    ("WAIL32!_AIL_shutdown@0", 0),
    ("WAIL32!_AIL_start_sample@4", 1),
    ("WAIL32!_AIL_start_sequence@4", 1),
    ("WAIL32!_AIL_start_timer@4", 1),
    ("WAIL32!_AIL_startup@0", 0),
    ("WAIL32!_AIL_stop_sample@4", 1),
    ("WAIL32!_AIL_stop_sequence@4", 1),
    ("WAIL32!_AIL_waveOutOpen@16", 4),
];

/// Each entry's original, stored last, so a reader that sees it sees its label too.
static ORIGINAL: [AtomicU32; COUNT] = [const { AtomicU32::new(0) }; COUNT];
static LABEL: [AtomicPtr<u8>; COUNT] = [const { AtomicPtr::new(null_mut()) }; COUNT];
static LENGTH: [AtomicUsize; COUNT] = [const { AtomicUsize::new(0) }; COUNT];
/// Each entry's declared count of arguments, 0 for none declared.
static ARGUMENTS: [AtomicU8; COUNT] = [const { AtomicU8::new(0) }; COUNT];
/// Entries handed out; past `COUNT`, the pool is full.
static TAKEN: AtomicUsize = AtomicUsize::new(0);

/// The thread holding each row of frames, 0 for a free row.
static OWNER: [AtomicU32; THREADS] = [const { AtomicU32::new(0) }; THREADS];
/// The frames open on each row; only the row's owner reads or writes it.
static DEPTH: [AtomicUsize; THREADS] = [const { AtomicUsize::new(0) }; THREADS];
/// Each frame's entry.
static FRAME_ENTRY: [AtomicUsize; FRAMES] = [const { AtomicUsize::new(0) }; FRAMES];
/// Each frame's caller's return address.
static FRAME_FROM: [AtomicU32; FRAMES] = [const { AtomicU32::new(0) }; FRAMES];
/// Each frame's arguments, as the call was handed them.
static FRAME_ARGS: [[AtomicU32; MOST]; FRAMES] =
    [const { [const { AtomicU32::new(0) }; MOST] }; FRAMES];

unsafe extern "system" {
    fn GetCurrentThreadId() -> u32;
    fn IsBadStringPtrA(text: *const u8, most: u32) -> i32;
}

/// The entries, `COUNT` calls into [`common`] in a row; entry `k` starts `STRIDE * k` in.
#[unsafe(naked)]
unsafe extern "C" fn pool() {
    core::arch::naked_asm!(
        ".rept {count}",
        "call {common}",
        ".endr",
        count = const COUNT,
        common = sym common,
    )
}

/// Keeps every register and the flags, hands [`note`] the entry's return address and the
/// caller's, writes the address it answers where the entry's return address stood, and
/// returns there with all restored. After `pushad` and `pushfd`, 36 bytes, the entry's return
/// address is at `esp + 36` and the caller's at `esp + 40`; each push moves the next word down
/// to `esp + 40`, so the 2 pushes hand on the caller's, then the entry's.
#[unsafe(naked)]
unsafe extern "C" fn common() {
    core::arch::naked_asm!(
        "pushad",
        "pushfd",
        "push dword ptr [esp + 40]",
        "push dword ptr [esp + 40]",
        "call {note}",
        "mov dword ptr [esp + 36], eax",
        "popfd",
        "popad",
        "ret",
        note = sym note,
    )
}

/// The entry whose call returns to `at`, and where [`common`] goes on to: its original with
/// `imp.through` off; else a fresh frame's stub in [`fronts`], or, every frame of this
/// thread's taken, the original after a row written before the call. `from` is where the
/// caller returns to. The thread's last error is set back as it was.
extern "stdcall" fn note(at: u32, from: u32) -> u32 {
    let entry = (at as usize).wrapping_sub(pool as unsafe extern "C" fn() as usize) / STRIDE - 1;
    let original = ORIGINAL[entry].load(Ordering::Acquire);
    // SAFETY: reads the thread's own last error.
    let kept = unsafe { crate::GetLastError() };
    let next = if !log::log_enabled!(log::Level::Debug) {
        original
    } else if let Some((frame, front)) = claim() {
        FRAME_ENTRY[frame].store(entry, Ordering::Relaxed);
        FRAME_FROM[frame].store(from, Ordering::Relaxed);
        front
    } else {
        log::debug!(
            c = label(entry), from:% = mech_trace::__shown!(from), by = "orig";
            "imp.through"
        );
        original
    };
    // SAFETY: sets back the value read above.
    unsafe { crate::SetLastError(kept) };
    next
}

/// A fresh frame of this thread's and its stub in [`fronts`]; `None` when every row is held
/// by another thread, or this thread's row holds `DEEP` frames.
fn claim() -> Option<(usize, u32)> {
    // SAFETY: asks the thread's own id.
    let thread = unsafe { GetCurrentThreadId() };
    let row = (0..THREADS)
        .find(|&t| OWNER[t].load(Ordering::Acquire) == thread)
        .or_else(|| {
            (0..THREADS).find(|&t| {
                OWNER[t]
                    .compare_exchange(0, thread, Ordering::AcqRel, Ordering::Relaxed)
                    .is_ok()
            })
        })?;
    let depth = DEPTH[row].load(Ordering::Relaxed);
    if depth >= DEEP {
        return None;
    }
    let frame = DEEP * row + depth;
    let front = stub(fronts as unsafe extern "C" fn() as usize, frame)?;
    DEPTH[row].store(depth + 1, Ordering::Relaxed);
    Some((frame, front))
}

/// The address of frame `frame`'s stub in the run of stubs that starts at `run`.
fn stub(run: usize, frame: usize) -> Option<u32> {
    u32::try_from(run + STRIDE * frame).ok()
}

/// The frame whose stub, in the run of stubs that starts at `run`, returns to `at`.
fn frame_of(run: usize, at: u32) -> usize {
    (at as usize).wrapping_sub(run) / STRIDE - 1
}

/// One stub a frame, `FRAMES` calls into [`front_common`] in a row.
#[unsafe(naked)]
unsafe extern "C" fn fronts() {
    core::arch::naked_asm!(
        ".rept {frames}",
        "call {front_common}",
        ".endr",
        frames = const FRAMES,
        front_common = sym front_common,
    )
}

/// Keeps every register and the flags, hands [`front`] the stub's return address and the
/// address of the caller's, writes the original it answers where the stub's return address
/// stood, and returns into the original with all restored. After `pushad` and `pushfd` the
/// stub's return address is at `esp + 36` and the caller's at `esp + 40`; the first push moves
/// the stub's down to `esp + 40`.
#[unsafe(naked)]
unsafe extern "C" fn front_common() {
    core::arch::naked_asm!(
        "pushad",
        "pushfd",
        "lea eax, [esp + 40]",
        "push eax",
        "push dword ptr [esp + 40]",
        "call {front}",
        "mov dword ptr [esp + 36], eax",
        "popfd",
        "popad",
        "ret",
        front = sym front,
    )
}

/// The frame whose stub returns to `at` keeps the call's declared arguments, the words above
/// the caller's return address at `slot`, and puts its stub in [`backs`] there; the answer is
/// the original.
extern "stdcall" fn front(at: u32, slot: u32) -> u32 {
    let frame = frame_of(fronts as unsafe extern "C" fn() as usize, at);
    let entry = FRAME_ENTRY[frame].load(Ordering::Relaxed);
    let count = usize::from(ARGUMENTS[entry].load(Ordering::Acquire)).min(MOST);
    for (k, kept) in FRAME_ARGS[frame].iter().take(count).enumerate() {
        let at = slot as usize + 4 * (k + 1);
        // SAFETY: the caller's own stack: a call's arguments lie above its return address.
        let word = unsafe { with_exposed_provenance::<u32>(at).read_unaligned() };
        kept.store(word, Ordering::Relaxed);
    }
    if let Some(back) = stub(backs as unsafe extern "C" fn() as usize, frame) {
        // SAFETY: `slot` is where the caller's return address lies, on the caller's stack.
        unsafe { with_exposed_provenance_mut::<u32>(slot as usize).write_unaligned(back) };
    }
    ORIGINAL[entry].load(Ordering::Acquire)
}

/// One stub a frame, `FRAMES` calls into [`back_common`] in a row.
#[unsafe(naked)]
unsafe extern "C" fn backs() {
    core::arch::naked_asm!(
        ".rept {frames}",
        "call {back_common}",
        ".endr",
        frames = const FRAMES,
        back_common = sym back_common,
    )
}

/// Keeps every register and the flags, hands [`back`] the stub's return address and `eax`,
/// writes the caller's return address it answers where the stub's stood, and returns home with
/// all restored: `esp` as the original left it, and `eax`, `edx` and the flags as it answered.
/// After `pushad` and `pushfd` the stub's return address is at `esp + 36`; pushing `eax` moves
/// it down to `esp + 40`.
#[unsafe(naked)]
unsafe extern "C" fn back_common() {
    core::arch::naked_asm!(
        "pushad",
        "pushfd",
        "push eax",
        "push dword ptr [esp + 40]",
        "call {back}",
        "mov dword ptr [esp + 36], eax",
        "popfd",
        "popad",
        "ret",
        back = sym back,
    )
}

/// The frame whose stub returns to `at` writes its row, the original returned: `value` is its
/// `eax`, the last error as it left it read first. The frame is freed, with any above it an
/// unwind left; the last error is set back; the answer is where the caller returns to.
extern "stdcall" fn back(at: u32, value: u32) -> u32 {
    let error = crate::left();
    let frame = frame_of(backs as unsafe extern "C" fn() as usize, at);
    let entry = FRAME_ENTRY[frame].load(Ordering::Relaxed);
    let from = FRAME_FROM[frame].load(Ordering::Relaxed);
    let count = usize::from(ARGUMENTS[entry].load(Ordering::Acquire)).min(MOST);
    let mut args = [0; MOST];
    for (word, kept) in args.iter_mut().zip(&FRAME_ARGS[frame]).take(count) {
        *word = kept.load(Ordering::Relaxed);
    }
    let (row, depth) = (frame / DEEP, frame % DEEP);
    DEPTH[row].store(depth, Ordering::Relaxed);
    if depth == 0 {
        OWNER[row].store(0, Ordering::Release);
    }
    after(label(entry), &args[..count], value, error, from);
    // SAFETY: sets back the last error the original left.
    unsafe { crate::SetLastError(error) };
    from
}

/// One `imp.through` row after the call: `a` left out for an import with no declared
/// signature, `p` written for `CreateFileA` alone.
fn after(c: &'static str, args: &[u32], value: u32, error: u32, from: u32) {
    match args.first() {
        Some(&name) if c.ends_with("!CreateFileA") => log::debug!(
            c = c, a:% = Numbers(args), p:% = Text(file_name(name)),
            v:% = mech_trace::__shown!(value), e:% = mech_trace::__shown!(error),
            from:% = mech_trace::__shown!(from), by = "orig";
            "imp.through"
        ),
        Some(_) => log::debug!(
            c = c, a:% = Numbers(args), v:% = mech_trace::__shown!(value),
            e:% = mech_trace::__shown!(error), from:% = mech_trace::__shown!(from),
            by = "orig";
            "imp.through"
        ),
        None => log::debug!(
            c = c, v:% = mech_trace::__shown!(value), e:% = mech_trace::__shown!(error),
            from:% = mech_trace::__shown!(from), by = "orig";
            "imp.through"
        ),
    }
}

/// `CreateFileA`'s name at `address`, read at the return: empty for a null or unreadable one,
/// cut at [`NAME_MOST`] bytes.
fn file_name<'a>(address: u32) -> &'a [u8] {
    let start = with_exposed_provenance::<u8>(address as usize);
    // SAFETY: only probes the bytes; the probe catches its own fault.
    if address == 0 || unsafe { IsBadStringPtrA(start, NAME_MOST) } != 0 {
        return &[];
    }
    let mut length = 0;
    // SAFETY: the probe found these bytes readable, up to the NUL or the bound.
    while length < NAME_MOST && unsafe { start.add(length as usize).read() } != 0 {
        length += 1;
    }
    // SAFETY: the `length` bytes just read.
    unsafe { slice::from_raw_parts(start, length as usize) }
}

/// Entry `entry`'s label, empty until it is stored.
fn label(entry: usize) -> &'static str {
    let at = LABEL[entry].load(Ordering::Acquire);
    let length = LENGTH[entry].load(Ordering::Acquire);
    if at.is_null() {
        return "";
    }
    // SAFETY: `entry` stored a `'static` str's pointer and length, never changed after.
    unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(at.cast_const(), length)) }
}

/// `name`'s declared count of arguments, 0 for an import helm declares none for.
fn declared(name: &str) -> u8 {
    SIGNATURES
        .iter()
        .find(|(label, _)| *label == name)
        .map_or(0, |&(_, count)| count)
}

/// The address of the pool's entry for `original` and `name`: the one already holding both,
/// else a fresh one; `None` when the pool is full, which the bind counts as a failed slot. A
/// name is compared by its text, since each load reads its table into a new block.
pub fn entry(original: u32, name: &'static str) -> Option<u32> {
    let taken = TAKEN.load(Ordering::Acquire).min(COUNT);
    let held =
        (0..taken).find(|&k| ORIGINAL[k].load(Ordering::Acquire) == original && label(k) == name);
    let entry = if let Some(k) = held {
        k
    } else {
        let k = TAKEN.fetch_add(1, Ordering::AcqRel);
        if k >= COUNT {
            return None;
        }
        LABEL[k].store(name.as_ptr().cast_mut(), Ordering::Relaxed);
        LENGTH[k].store(name.len(), Ordering::Relaxed);
        ARGUMENTS[k].store(declared(name), Ordering::Relaxed);
        ORIGINAL[k].store(original, Ordering::Release);
        k
    };
    u32::try_from(pool as unsafe extern "C" fn() as usize + STRIDE * entry).ok()
}
