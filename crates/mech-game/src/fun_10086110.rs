// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ba7bc: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10086336(param_1: *mut u16) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10086110() {
    mech_trace::enter!("10086110");
    // SAFETY: the global table and the nodes it links are the game's own
    // memory; reads follow the original's order and the callee is the game's.
    unsafe {
        if mech_trace::gr!("10086110", "100ba7bc", DAT_100ba7bc) != 0 {
            let mut local_c: i32 = 0;
            while local_c < 0x3f1 {
                let mut p_var1: *mut u16 = mech_trace::rd!(
                    "10086110",
                    ((mech_trace::gr!("10086110", "100ba7bc", DAT_100ba7bc) as usize)
                        .wrapping_add((local_c as usize).wrapping_mul(4))
                        as *const u32),
                    read_unaligned
                ) as usize as *mut u16;
                while !p_var1.is_null() {
                    let local_8 = p_var1;
                    p_var1 =
                        mech_trace::rd!("10086110", (local_8.add(4) as *const u32), read_unaligned)
                            as usize as *mut u16;
                    if 0xefff < mech_trace::rd!("10086110", local_8, read_unaligned) {
                        mech_trace::cr!("10086110", "10086336", FUN_10086336(local_8));
                    }
                }
                local_c = local_c.wrapping_add(1);
            }
        }
    }
}
