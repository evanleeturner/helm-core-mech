// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a40f8: *mut core::ffi::c_void;
    static mut DAT_100adf74: *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn _AIL_end_sample_4(a: u32);
    fn HeapFree(hHeap: *mut core::ffi::c_void, dwFlags: u32, lpMem: *mut core::ffi::c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1000278e() {
    // SAFETY: the globals and callees are the game's own; the table pointed to
    // by DAT_100a40f8 is read and written at the same offsets as the original.
    unsafe {
        if !DAT_100a40f8.is_null() {
            let mut sample_index: u32 = 0;
            while sample_index < DAT_100a40f8.byte_add(0x44).cast::<u32>().read_unaligned() {
                let idx4 = (sample_index.wrapping_mul(4)) as usize;
                let idx8 = (sample_index.wrapping_mul(8)) as usize;
                if DAT_100a40f8
                    .byte_add(idx4)
                    .byte_add(0x48)
                    .cast::<i32>()
                    .read_unaligned()
                    != 0
                {
                    _AIL_end_sample_4(
                        DAT_100a40f8
                            .byte_add(idx4)
                            .byte_add(4)
                            .cast::<u32>()
                            .read_unaligned(),
                    );
                    DAT_100a40f8
                        .byte_add(idx4)
                        .byte_add(0x48)
                        .cast::<u32>()
                        .write_unaligned(0);
                }
                HeapFree(
                    DAT_100adf74,
                    1,
                    DAT_100a40f8
                        .byte_add(idx8)
                        .byte_add(0x2a8)
                        .cast::<*mut core::ffi::c_void>()
                        .read_unaligned(),
                );
                sample_index = sample_index.wrapping_add(1);
            }
            HeapFree(DAT_100adf74, 1, DAT_100a40f8);
            DAT_100a40f8 = core::ptr::null_mut();
        }
    }
}
