// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf74: *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002587d(param_1: *mut core::ffi::c_void);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapFree(h: *mut core::ffi::c_void, flags: u32, mem: *mut core::ffi::c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001c110(param_1: i32) {
    if param_1 == 0 {
        return;
    }
    // SAFETY: param_1 points at the game's own object; the fields are read and
    // written where the original does, and the callees and global are the game's.
    unsafe {
        let base = param_1 as *mut u8;
        let lp_mem = base
            .byte_add(0x4c)
            .cast::<*mut core::ffi::c_void>()
            .read_unaligned();
        if !lp_mem.is_null() {
            let kind = base.byte_add(0x24).cast::<i32>().read_unaligned();
            if kind == 0 {
                HeapFree(DAT_100adf74, 1, lp_mem);
            } else if base.byte_add(0x24).cast::<i32>().read_unaligned() == 5 {
                FUN_1002587d(lp_mem);
            }
            base.byte_add(0x4c).cast::<u32>().write_unaligned(0);
        }
    }
}
