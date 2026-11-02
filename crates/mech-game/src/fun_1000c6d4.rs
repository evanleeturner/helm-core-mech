// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7578: i32;
    static mut DAT_100a7580: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000c6d4(param_1: i32) -> u32 {
    // SAFETY: the globals are the game's own; the table at DAT_100a7580 holds
    // DAT_100a7578 entries of two 32-bit words, read where the original does.
    unsafe {
        let mut local_c: i32 = DAT_100a7578;
        loop {
            local_c = (local_c as u32).wrapping_sub(1) as i32;
            if local_c < 0 {
                return 0;
            }
            let base = DAT_100a7580 as u32;
            let addr = base.wrapping_add((local_c as u32).wrapping_mul(8));
            if (addr as *const i32).read_unaligned() == param_1 {
                break;
            }
        }
        let base = DAT_100a7580 as u32;
        let addr = base
            .wrapping_add(4)
            .wrapping_add((local_c as u32).wrapping_mul(8));
        (addr as *const u32).read_unaligned()
    }
}
