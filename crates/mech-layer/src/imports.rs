// SPDX-License-Identifier: MIT
//! The 16 imports the shell's screens call past its file, time, thread, text, window, drawing and process calls
//! (HC6 session 21's 42, less the 11 file calls the seam now answers,
//! [`crate::files`], and the 2 time calls KT1's answers, [`crate::time`], and `CompareStringA`, KX1's, [`crate::text`],
//! and the 5 window calls WN1 logs with their callers, [`crate::window`], and the 4 drawing calls
//! logged with theirs, [`crate::draw`], and `GetLastError` and `SetLastError`, which `kernel_process`'
//! front answers, [`crate::process`]), one row each: the name the counter labels it by, the import, its
//! arguments, its return. Each row makes a module of the import's name
//! holding the real import, from the swap DLL's own import table, and its
//! layer function, exported `HC6_<name>`; stdcall, as the shell calls
//! them. The 12 MSS32 rows' `@N` match the shell's own decorated names, so
//! a row with the wrong argument count is refused at the link.
//!
//! The MSS32 return types are mss.h's, unmeasured: a row called void that
//! truly returns leaves its eax unseen by the layer, which a caller would
//! miss.
use core::ffi::c_void;

type Handle = *mut c_void;
type Pointer = *mut c_void;
type Text = *const u8;
type Sample = *mut c_void;

macro_rules! layer {
    ($c:literal, $name:ident($($argument:ident: $type:ty),*) -> $ret:ty) => {
        #[allow(non_snake_case)]
        pub mod $name {
            #[allow(unused_imports)]
            use super::{Handle, Pointer, Sample, Text};

            unsafe extern "system" {
                fn $name($($argument: $type),*) -> $ret;
            }

            #[unsafe(export_name = concat!("HC6_", stringify!($name)))]
            unsafe extern "system" fn layer($($argument: $type),*) -> $ret {
                // SAFETY: the shell's own call, its arguments passed on
                // unchanged to the import its slot named.
                let value = unsafe { $name($($argument),*) };
                imp_call!("orig", $c, [$($argument),*], value)
            }
        }
    };
    ($c:literal, $name:ident($($argument:ident: $type:ty),+)) => {
        #[allow(non_snake_case)]
        pub mod $name {
            #[allow(unused_imports)]
            use super::{Handle, Pointer, Sample, Text};

            unsafe extern "system" {
                fn $name($($argument: $type),+);
            }

            #[unsafe(export_name = concat!("HC6_", stringify!($name)))]
            unsafe extern "system" fn layer($($argument: $type),+) {
                // SAFETY: the shell's own call, its arguments passed on
                // unchanged to the import its slot named.
                unsafe { $name($($argument),+) };
                imp_call!("orig", $c, [$($argument),+]);
            }
        }
    };
}

layer!("KERNEL32!HeapAlloc", HeapAlloc(heap: Handle, flags: u32, bytes: usize) -> Pointer);
layer!("KERNEL32!HeapReAlloc", HeapReAlloc(heap: Handle, flags: u32, block: Pointer, bytes: usize) -> Pointer);
layer!("KERNEL32!HeapFree", HeapFree(heap: Handle, flags: u32, block: Pointer) -> i32);

layer!("WINMM!timeGetTime", timeGetTime() -> u32);

layer!("MSS32!_AIL_set_sample_loop_count@8", AIL_set_sample_loop_count(sample: Sample, count: i32));
layer!("MSS32!_AIL_set_sample_volume@8", AIL_set_sample_volume(sample: Sample, volume: i32));
layer!("MSS32!_AIL_start_sample@4", AIL_start_sample(sample: Sample));
layer!("MSS32!_AIL_sample_status@4", AIL_sample_status(sample: Sample) -> u32);
layer!("MSS32!_MEM_free_lock@4", MEM_free_lock(block: Pointer));
layer!("MSS32!_AIL_stop_sample@4", AIL_stop_sample(sample: Sample));
layer!("MSS32!_AIL_release_sample_handle@4", AIL_release_sample_handle(sample: Sample));
layer!("MSS32!_FILE_read@8", FILE_read(name: Text, destination: Pointer) -> Pointer);
layer!("MSS32!_AIL_allocate_sample_handle@4", AIL_allocate_sample_handle(driver: u32) -> Sample);
layer!("MSS32!_AIL_init_sample@4", AIL_init_sample(sample: Sample));
layer!("MSS32!_AIL_set_sample_file@12", AIL_set_sample_file(sample: Sample, file: Pointer, block: i32) -> i32);
layer!("MSS32!_AIL_set_sample_type@12", AIL_set_sample_type(sample: Sample, format: i32, flags: u32));
