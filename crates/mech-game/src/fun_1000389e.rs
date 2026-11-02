// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a40f8: i32;
    static mut DAT_100a5538: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn _AIL_end_sample_4(a: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000389e(param_1: i32) {
    // SAFETY: the globals and the sample table they point to are the game's
    // own memory; reads and writes happen in the original's order.
    unsafe {
        if DAT_100a40f8 != 0 {
            let mut i: i32 = 0;
            while i < 0x10 {
                let base = DAT_100a40f8;
                let off = i.wrapping_mul(4);
                let flag_addr = base.wrapping_add(0x48).wrapping_add(off) as *const i32;
                if flag_addr.read_unaligned() != 0 {
                    let h_addr = DAT_100a40f8.wrapping_add(4).wrapping_add(off) as *const i32;
                    if h_addr.read_unaligned() != DAT_100a5538 || param_1 != 0 {
                        let h_addr2 = DAT_100a40f8.wrapping_add(4).wrapping_add(off) as *const u32;
                        _AIL_end_sample_4(h_addr2.read_unaligned());
                        let flag_addr2 =
                            DAT_100a40f8.wrapping_add(0x48).wrapping_add(off) as *mut u32;
                        flag_addr2.write_unaligned(0);
                    }
                }
                i = i.wrapping_add(1);
            }
        }
    }
}
