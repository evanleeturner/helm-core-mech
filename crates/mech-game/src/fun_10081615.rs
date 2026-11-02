// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf74: *mut c_void;
    static mut DAT_100b98a0: i32;
    static mut DAT_100b98a4: u32;
    static mut DAT_100c8240: [u32; 1];
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapFree(h_heap: *mut c_void, dw_flags: u32, lp_mem: *mut c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10081615() {
    // SAFETY: the globals, the table at DAT_100c8240 and HeapFree are the
    // game's own; entries are read and cleared in the original's order.
    unsafe {
        let mut local_8: i32 = 0;
        while local_8 < DAT_100b98a0 {
            let entry = (&raw mut DAT_100c8240)
                .cast::<u32>()
                .offset(local_8.wrapping_mul(3) as isize);
            HeapFree(DAT_100adf74, 1, entry.read_unaligned() as *mut c_void);
            entry.write_unaligned(0);
            local_8 = local_8.wrapping_add(1);
        }
        DAT_100b98a0 = 0;
        DAT_100b98a4 = 0;
    }
}
