// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7578: i32;
    static mut DAT_100a757c: i32;
    static mut DAT_100a7580: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000c674(param_1: u32, param_2: u32) -> bool {
    // SAFETY: the globals are the game's own memory; the buffer address held
    // in DAT_100a7580 is written at the offsets the original uses.
    unsafe {
        let b_var1 = DAT_100a7578 < DAT_100a757c;
        if b_var1 {
            let base = DAT_100a7580 as u32;
            let idx = DAT_100a7578 as u32;
            let p0 = base.wrapping_add(idx.wrapping_mul(8)) as *mut u32;
            p0.write_unaligned(param_1);
            let p1 = base
                .wrapping_add(4)
                .wrapping_add((DAT_100a7578 as u32).wrapping_mul(8))
                as *mut u32;
            p1.write_unaligned(param_2);
            DAT_100a7578 = (DAT_100a7578 as u32).wrapping_add(1) as i32;
        }
        b_var1
    }
}
