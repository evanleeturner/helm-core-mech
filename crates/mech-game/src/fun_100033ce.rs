// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a40f8: u32;
    static mut DAT_100adf74: *mut c_void;
    static mut DAT_101e8db4: [u8; 0];
    static mut PTR_DAT_100af0a8: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn _AIL_sample_user_data_8(param_1: u32, param_2: u32) -> u32;
    fn HeapFree(h: *mut c_void, flags: u32, p: *mut c_void) -> i32;
    fn _AIL_set_sample_user_data_12(param_1: u32, param_2: u32, param_3: u32) -> u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1008664f(sample_handle: u32, param_2: *mut i32);
    fn FUN_10002d8f(
        sample_handle: u32,
        param_2: *mut c_void,
        param_3: u16,
        param_4: i16,
        param_5: i32,
    ) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100033ce(sample_handle: u32) {
    // SAFETY: all globals, callees and computed addresses belong to the game's
    // own memory; offsets are applied exactly as the original does.
    unsafe {
        let u_var2 = _AIL_sample_user_data_8(sample_handle, 1);
        let slot_index = _AIL_sample_user_data_8(sample_handle, 2) as i32;
        let mut next = _AIL_sample_user_data_8(sample_handle, 3);

        let slot4 = (slot_index as u32).wrapping_mul(4);
        let slot2 = (slot_index as u32).wrapping_mul(2);

        ((DAT_100a40f8.wrapping_add(0x48).wrapping_add(slot4)) as *mut u32).write_unaligned(0);

        let counter = (&raw mut DAT_101e8db4 as u32)
            .wrapping_add(u_var2.wrapping_mul(8))
            .wrapping_add(2) as *mut i16;
        let c = counter.read_unaligned();
        if 0 < c {
            counter.write_unaligned(c.wrapping_sub(1));
        }

        if u_var2 != 0 && 0 < (u_var2 as i32) && u_var2 != next {
            FUN_1008664f(u_var2, PTR_DAT_100af0a8 as *mut i32);
        }

        let heap_block = _AIL_sample_user_data_8(sample_handle, 4) as *mut c_void;
        if !heap_block.is_null() {
            HeapFree(DAT_100adf74, 1, heap_block);
        }

        if next != 0 {
            if next == 0xffffffff {
                next = 0;
            } else {
                let flags = (DAT_100a40f8.wrapping_add(0x4e8).wrapping_add(slot2) as *const u16)
                    .read_unaligned();
                let r = FUN_10002d8f(next, core::ptr::null_mut(), flags, slot_index as i16, 0);
                if (r as i32) < 0 {
                    FUN_1008664f(next, PTR_DAT_100af0a8 as *mut i32);
                    next = 0;
                } else {
                    _AIL_set_sample_user_data_12(sample_handle, 1, next);
                    let flags2 = (DAT_100a40f8.wrapping_add(0x4e8).wrapping_add(slot2)
                        as *const u16)
                        .read_unaligned();
                    if (flags2 & 0x1000) == 0 {
                        _AIL_set_sample_user_data_12(sample_handle, 3, 0xffffffff);
                    } else {
                        _AIL_set_sample_user_data_12(sample_handle, 3, next);
                    }
                    ((DAT_100a40f8.wrapping_add(0x48).wrapping_add(slot4)) as *mut u32)
                        .write_unaligned(1);
                }
            }
        }

        if next == 0 {
            ((DAT_100a40f8.wrapping_add(0x4a8).wrapping_add(slot4)) as *mut u32).write_unaligned(0);
            ((DAT_100a40f8.wrapping_add(0x4e8).wrapping_add(slot2)) as *mut u16).write_unaligned(0);
            let user_data_ptr = _AIL_sample_user_data_8(sample_handle, 0) as *mut u32;
            user_data_ptr.write_unaligned(0);
        }
    }
}
