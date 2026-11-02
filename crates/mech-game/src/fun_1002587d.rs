// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf74: *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapFree(h: *mut core::ffi::c_void, flags: u32, mem: *mut core::ffi::c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002587d(mem: *mut core::ffi::c_void) {
    if !mem.is_null() {
        // SAFETY: mem points at the game's heap block; its fields are read at the
        // original offsets and the block is freed through the game's heap handle.
        unsafe {
            if mem.byte_add(0x18).cast::<i32>().read_unaligned() == 0 {
                let mut local_8: i32 = 0;
                while local_8 < 4 {
                    FUN_1002587d(
                        mem.byte_add((local_8 as usize).wrapping_mul(4).wrapping_add(0x1c))
                            .cast::<*mut core::ffi::c_void>()
                            .read_unaligned(),
                    );
                    local_8 = local_8.wrapping_add(1);
                }
            }
            HeapFree(DAT_100adf74, 1, mem);
        }
    }
}
