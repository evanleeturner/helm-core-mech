// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "fastcall" {
    fn FUN_100501c0(param_1: *mut i32) -> u8;
    fn FUN_1005f890(param_1: *mut u8) -> u8;
    fn FUN_10050100(param_1: *mut i32) -> u8;
    fn FUN_1005f810(param_1: *mut u8) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10040af0(param_1: i32, param_2: i32, param_3: i32) -> u32 {
    mech_trace::enter!("10040af0", param_1, param_2, param_3);
    let mut param_3 = param_3;
    // SAFETY: param_1 points at the caller's array of 0x2c-byte records; the
    // callees are the game's own and are called in the original's order.
    unsafe {
        if 0 < param_3 {
            let mut p = (param_1 as u32).wrapping_add(4) as *const u32;
            loop {
                let a = mech_trace::rd!("10040af0", p.byte_sub(4), read_unaligned);
                mech_trace::cr!("10040af0", "100501c0", FUN_100501c0(a as *mut i32));
                let b = mech_trace::rd!("10040af0", p, read_unaligned);
                if b != 0 {
                    mech_trace::cr!("10040af0", "1005f890", FUN_1005f890(b as *mut u8));
                }
                p = p.byte_add(0x2c);
                param_3 = param_3.wrapping_sub(1);
                if param_3 == 0 {
                    break;
                }
            }
        }
        let q = (param_2.wrapping_mul(0x2c).wrapping_add(param_1) as u32) as *const u32;
        if !q.is_null() {
            mech_trace::cr!(
                "10040af0",
                "10050100",
                FUN_10050100(mech_trace::rd!("10040af0", q, read_unaligned) as *mut i32)
            );
            let b = mech_trace::rd!("10040af0", q.byte_add(4), read_unaligned);
            if b != 0 {
                mech_trace::cr!("10040af0", "1005f810", FUN_1005f810(b as *mut u8));
            }
        }
        mech_trace::rt!("10040af0", mech_trace::rd!("10040af0", q, read_unaligned))
    }
}
