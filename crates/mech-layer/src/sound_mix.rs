// SPDX-License-Identifier: MIT
//! The game file's Miles calls through the front: `sound_mix`'s answers,
//! the mix going to [`NullBack`], no device and no sound. Exported `GAME_<name>`,
//! a name the loader's `HC6_` match cannot take, and bound to no slot here. Whether
//! the shell's calls take a prefix of their own and share this front is the first
//! flight's to pin. Each export's arguments are the size the name's `@N` wears in
//! the game's and the shell's import tables, so a stop's stack is right as well.
//!
//! No thread is made here: a sample's end fires its callback at the next call, so
//! the first flight also binds a thread to [`Front::pump`]. The door has no export:
//! nothing calls it while the original `SMACKW32` stays.
use ::sound_mix::{Front, Host, NullBack};
use core::ffi::{CStr, c_char, c_void};
use core::mem::{size_of, transmute};
use core::ptr::{null_mut, with_exposed_provenance};
use core::slice;
use kernel_time::Clocks;
use mech_rows::sound_mix::GAME;

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
    fn GetFileSize(file: *mut c_void, high: *mut u32) -> u32;
    fn ReadFile(
        file: *mut c_void,
        buffer: *mut c_void,
        bytes: u32,
        read: *mut u32,
        overlapped: *mut c_void,
    ) -> i32;
    fn CloseHandle(object: *mut c_void) -> i32;
    fn GetProcessHeap() -> *mut c_void;
    fn HeapAlloc(heap: *mut c_void, flags: u32, bytes: usize) -> *mut c_void;
    fn HeapFree(heap: *mut c_void, flags: u32, block: *mut c_void) -> i32;
    fn VirtualQuery(address: *const c_void, info: *mut Region, bytes: usize) -> usize;
}

const GENERIC_READ: u32 = 0x8000_0000;
const FILE_SHARE_READ: u32 = 1;
const OPEN_EXISTING: u32 = 3;
const FILE_ATTRIBUTE_NORMAL: u32 = 0x80;
const INVALID_FILE_SIZE: u32 = u32::MAX;
const MAX_PATH: usize = 260;
const MEM_COMMIT: u32 = 0x1000;
const PAGE_NOACCESS: u32 = 1;
const PAGE_GUARD: u32 = 0x100;

/// `MEMORY_BASIC_INFORMATION`, as a 32-bit Windows lays it out.
#[repr(C)]
struct Region {
    base: *mut c_void,
    allocation_base: *mut c_void,
    allocation_protect: u32,
    size: usize,
    state: u32,
    protect: u32,
    kind: u32,
}

/// A pointer as the number the front holds it by. This module builds only for
/// 32-bit x86, so nothing is cut.
#[allow(clippy::cast_possible_truncation)]
fn number(pointer: *mut c_void) -> u32 {
    pointer.expose_provenance() as u32
}

/// The bytes readable from `at` before the region holding it ends.
fn reach_from(at: *const u8) -> usize {
    let mut region = Region {
        base: null_mut(),
        allocation_base: null_mut(),
        allocation_protect: 0,
        size: 0,
        state: 0,
        protect: 0,
        kind: 0,
    };
    // SAFETY: Windows writes the one record it is handed, and reads no memory at `at`.
    let wrote = unsafe { VirtualQuery(at.cast(), &raw mut region, size_of::<Region>()) };
    if wrote == 0
        || region.state != MEM_COMMIT
        || region.protect & (PAGE_NOACCESS | PAGE_GUARD) != 0
    {
        return 0;
    }
    (region.base.expose_provenance() + region.size).saturating_sub(at.expose_provenance())
}

/// The file image at `address`: RIFF size at +4, plus 8, cut where the memory ends.
///
/// The size is not to be trusted past the memory: 7 of the 132 WAVEs in the game's
/// archives declare a size running over the next WAVE, by 1,545 to 44,029 bytes, and
/// this reads the whole file at once where Miles walks it as it plays. An `address`
/// nothing is mapped at answers an empty file, which the front stops on by name.
///
/// # Safety
/// `address` is what the program handed as a file image, and nothing writes it
/// while the front copies from it.
unsafe fn file_at<'a>(address: u32) -> &'a [u8] {
    let head = with_exposed_provenance::<u8>(address as usize);
    let reach = reach_from(head);
    if reach < 8 {
        return &[];
    }
    // SAFETY: the first 8 bytes are in reach, by the check above.
    let declared = unsafe { head.add(4).cast::<u32>().read_unaligned() } as usize + 8;
    if declared > reach {
        Target.quiet(|| log::debug!(at = address, declared = declared, reach = reach; "sound.file.cut"));
    }
    // SAFETY: the bytes are committed and readable, by `reach_from`.
    unsafe { slice::from_raw_parts(head, declared.min(reach)) }
}

/// Reads an open file whole into a block of the process heap.
fn read_whole(file: *mut c_void) -> Option<u32> {
    // SAFETY: the file is open, and no high word is asked for: no file here nears 4 GB.
    let bytes = unsafe { GetFileSize(file, null_mut()) };
    if bytes == INVALID_FILE_SIZE {
        return None;
    }
    // SAFETY: the process's own heap, and a size the call may refuse.
    let block = unsafe { HeapAlloc(GetProcessHeap(), 0, bytes as usize) };
    if block.is_null() {
        return None;
    }
    let mut read = 0;
    // SAFETY: the block holds `bytes`; the count is this frame's.
    let answer = unsafe { ReadFile(file, block, bytes, &raw mut read, null_mut()) };
    if answer == 0 || read != bytes {
        // SAFETY: the block allocated above, freed once.
        unsafe { HeapFree(GetProcessHeap(), 0, block) };
        return None;
    }
    Some(number(block))
}

/// Windows under the front, and the thread's last error kept across a line.
struct Target;

impl Host for Target {
    fn now_us(&self) -> u64 {
        crate::time::Win32.micros()
    }

    fn call_back(&self, callback: u32, handle: u32) {
        // SAFETY: the number the program registered as its end-of-sample callback,
        // and the game's own, `100033ce`, ends `ret 4`: stdcall, one word.
        unsafe {
            let call =
                transmute::<usize, unsafe extern "system" fn(u32)>(callback as usize);
            call(handle);
        }
    }

    fn read_file(&self, name: &str) -> Option<u32> {
        if name.len() >= MAX_PATH {
            return None;
        }
        let mut path = [0u8; MAX_PATH];
        path[..name.len()].copy_from_slice(name.as_bytes());
        // SAFETY: `path` ends in a NUL, by the check above; a plain open for reading.
        let file = unsafe {
            CreateFileA(
                path.as_ptr().cast(),
                GENERIC_READ,
                FILE_SHARE_READ,
                null_mut(),
                OPEN_EXISTING,
                FILE_ATTRIBUTE_NORMAL,
                null_mut(),
            )
        };
        if file.addr() == usize::MAX {
            return None;
        }
        let block = read_whole(file);
        // SAFETY: opened above, closed once.
        unsafe { CloseHandle(file) };
        block
    }

    fn free_block(&self, block: u32) {
        // SAFETY: the front frees only what `read_file` gave, and each once.
        unsafe { HeapFree(GetProcessHeap(), 0, with_exposed_provenance::<c_void>(block as usize).cast_mut()) };
    }

    fn quiet(&self, line: impl FnOnce()) {
        // SAFETY: reads the thread's own last error.
        let error = unsafe { crate::GetLastError() };
        line();
        // SAFETY: sets back the value read above.
        unsafe { crate::SetLastError(error) };
    }
}

static FRONT: Front<Target, NullBack> = Front::new(&GAME, Target, NullBack);

/// The fronts whose numbers pass straight through.
macro_rules! fronts {
    ($($export:literal $rust:ident($($argument:ident: $type:ty),*) $(-> $answer:ty)? = $call:ident;)+) => {
        $(
            #[unsafe(export_name = concat!("GAME_", $export))]
            unsafe extern "system" fn $rust($($argument: $type),*) $(-> $answer)? {
                FRONT.$call($($argument),*)
            }
        )+
    };
}

fronts! {
    "AIL_startup" startup() = startup;
    "AIL_shutdown" shutdown() = shutdown;
    "AIL_serve" serve() = serve;
    "AIL_waveOutClose" wave_out_close(driver: u32) = wave_out_close;
    "AIL_set_preference" set_preference(number: u32, value: u32) -> u32 = set_preference;
    "AIL_allocate_sample_handle" allocate_sample_handle(driver: u32) -> u32 = allocate_sample_handle;
    "AIL_init_sample" init_sample(handle: u32) = init_sample;
    "AIL_release_sample_handle" release_sample_handle(handle: u32) = release_sample_handle;
    "AIL_set_sample_volume" set_sample_volume(handle: u32, volume: i32) = set_sample_volume;
    "AIL_set_sample_loop_count" set_sample_loop_count(handle: u32, loops: u32) = set_sample_loop_count;
    "AIL_start_sample" start_sample(handle: u32) = start_sample;
    "AIL_stop_sample" stop_sample(handle: u32) = stop_sample;
    "AIL_resume_sample" resume_sample(handle: u32) = resume_sample;
    "AIL_end_sample" end_sample(handle: u32) = end_sample;
    "AIL_sample_status" sample_status(handle: u32) -> u32 = sample_status;
    "AIL_set_sample_type" set_sample_type(handle: u32, format: u32, flags: u32) = set_sample_type;
    "AIL_set_sample_playback_rate" set_sample_playback_rate(handle: u32, rate_hz: u32) = set_sample_playback_rate;
    "AIL_set_sample_pan" set_sample_pan(handle: u32, pan: i32) = set_sample_pan;
    "AIL_set_sample_user_data" set_sample_user_data(handle: u32, index: u32, value: u32) = set_sample_user_data;
    "AIL_sample_user_data" sample_user_data(handle: u32, index: u32) -> u32 = sample_user_data;
    "AIL_register_EOS_callback" register_eos_callback(handle: u32, callback: u32) -> u32 = register_eos_callback;
    "AIL_sample_buffer_ready" sample_buffer_ready(handle: u32) -> i32 = sample_buffer_ready;
    "MEM_free_lock" mem_free_lock(block: u32) = mem_free_lock;
}

/// Writes through a pointer the program may have left null, as mss.h's own
/// examples leave the second of `AIL_waveOutOpen` and `AIL_midiOutOpen`.
///
/// # Safety
/// `to` is null, or the program's own place for an answer.
unsafe fn answer_through(to: *mut u32, value: u32) {
    if !to.is_null() {
        // SAFETY: not null, and the program's own place for this answer.
        unsafe { to.write_unaligned(value) };
    }
}

/// `format` is a `PCMWAVEFORMAT`: channels at +2, rate at +4, bits at +14.
#[unsafe(export_name = "GAME_AIL_waveOutOpen")]
unsafe extern "system" fn wave_out_open(
    driver_at: *mut u32,
    out: *mut u32,
    device: u32,
    format: *const u8,
) -> u32 {
    // SAFETY: the record the program built for this call.
    let (channels, rate_hz, bits) = unsafe {
        (
            format.add(2).cast::<u16>().read_unaligned(),
            format.add(4).cast::<u32>().read_unaligned(),
            format.add(14).cast::<u16>().read_unaligned(),
        )
    };
    let (answer, driver) = FRONT.wave_out_open(device, rate_hz, channels.into(), bits.into());
    // SAFETY: the two places the program handed for the answers.
    unsafe {
        answer_through(driver_at, driver);
        answer_through(out, 0);
    }
    answer
}

#[unsafe(export_name = "GAME_AIL_midiOutOpen")]
unsafe extern "system" fn midi_out_open(driver_at: *mut u32, out: *mut u32, device: u32) -> u32 {
    let answer = FRONT.midi_out_open(device);
    // SAFETY: the two places the program handed for the answers.
    unsafe {
        answer_through(driver_at, 0);
        answer_through(out, 0);
    }
    answer
}

#[unsafe(export_name = "GAME_AIL_set_sample_file")]
unsafe extern "system" fn set_sample_file(handle: u32, file: u32, block: u32) -> u32 {
    // SAFETY: the program's file image; the front takes its copy before answering.
    FRONT.set_sample_file(handle, file, unsafe { file_at(file) }, block)
}

#[unsafe(export_name = "GAME_AIL_allocate_file_sample")]
unsafe extern "system" fn allocate_file_sample(driver: u32, file: u32, block: u32) -> u32 {
    // SAFETY: the program's file image; the front takes its copy before answering.
    FRONT.allocate_file_sample(driver, file, unsafe { file_at(file) }, block)
}

#[unsafe(export_name = "GAME_AIL_load_sample_buffer")]
unsafe extern "system" fn load_sample_buffer(handle: u32, slot: u32, address: u32, bytes: u32) {
    let data = if bytes == 0 {
        &[]
    } else {
        // SAFETY: the buffer and length the program handed; the front takes a copy.
        unsafe { slice::from_raw_parts(with_exposed_provenance::<u8>(address as usize), bytes as usize) }
    };
    FRONT.load_sample_buffer(handle, slot, address, data);
}

/// A name past ASCII stops by name: no file the program was read asking for has one.
#[unsafe(export_name = "GAME_FILE_read")]
unsafe extern "system" fn file_read(name: u32, destination: u32) -> u32 {
    // SAFETY: the C string the program handed as a file's name.
    let text = unsafe { CStr::from_ptr(with_exposed_provenance::<c_char>(name as usize)) };
    match text.to_str() {
        Ok(text) => FRONT.file_read(name, text, destination),
        Err(_) => FRONT.named_stop("FILE_read"),
    }
}

/// The names read as unreached. Each ends the program, but keeps its `@N`'s size.
macro_rules! stops {
    ($($name:ident($($argument:ident),*);)+) => {
        $(
            #[allow(non_snake_case)]
            #[unsafe(export_name = concat!("GAME_", stringify!($name)))]
            unsafe extern "system" fn $name($($argument: u32),*) -> ! {
                FRONT.named_stop(stringify!($name))
            }
        )+
    };
}

stops! {
    AIL_active_sample_count(_a);
    AIL_minimum_sample_buffer_size(_a, _b, _c);
    AIL_lock_channel(_a);
    AIL_release_channel(_a, _b);
    AIL_send_channel_voice_message(_a, _b, _c, _d, _e);
    AIL_allocate_sequence_handle(_a);
    AIL_init_sequence(_a, _b, _c);
    AIL_start_sequence(_a);
    AIL_stop_sequence(_a);
    AIL_resume_sequence(_a);
    AIL_sequence_status(_a);
    AIL_set_sequence_volume(_a, _b, _c);
    AIL_set_sequence_loop_count(_a, _b);
    AIL_release_sequence_handle(_a);
    AIL_active_sequence_count(_a);
}
