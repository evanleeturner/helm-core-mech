// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1007cb42(param_1: *mut i32, param_2: *mut u8) -> u8;
    fn FUN_10050f5b(param_1: i32) -> u8;
    fn FUN_1002f37a(param_1: i32, param_2: u32, param_3: u32, param_4: u32) -> u8;
    fn FUN_1002f668(param_1: i32, param_2: i32, param_3: i32, param_4: i32, param_5: u32) -> u8;
    fn FUN_10050ecc(param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100715a5(param_1: *mut u32) -> *mut i32 {
    mech_trace::enter!("100715a5", param_1);
    if param_1.is_null() {
        return mech_trace::rt!("100715a5", core::ptr::null_mut());
    }
    // SAFETY: param_1 points at the caller's node; its fields, the child
    // nodes and the callees are the game's own memory and functions.
    unsafe {
        let pi_var1 = mech_trace::rd!("100715a5", param_1.add(2), read_unaligned) as *mut i32;
        if !pi_var1.is_null() {
            mech_trace::cr!(
                "100715a5",
                "1007cb42",
                FUN_1007cb42(
                    pi_var1,
                    FUN_10050ecc as unsafe extern "C" fn(i32) -> u8 as *mut u8,
                )
            );
            mech_trace::cr!("100715a5", "10050f5b", FUN_10050f5b(pi_var1 as i32));
            mech_trace::cr!(
                "100715a5",
                "1002f37a",
                FUN_1002f37a(
                    pi_var1 as i32,
                    mech_trace::rd!("100715a5", param_1.add(4), read_unaligned),
                    mech_trace::rd!("100715a5", param_1.add(5), read_unaligned),
                    mech_trace::rd!("100715a5", param_1.add(6), read_unaligned),
                )
            );
            mech_trace::cr!(
                "100715a5",
                "1002f668",
                FUN_1002f668(
                    pi_var1 as i32,
                    mech_trace::rd!("100715a5", param_1.add(7), read_unaligned) as i32,
                    mech_trace::rd!("100715a5", param_1.add(8), read_unaligned) as i32,
                    mech_trace::rd!("100715a5", param_1.add(9), read_unaligned) as i32,
                    0,
                )
            );
            let pi_var2 =
                FUN_100715a5(mech_trace::rd!("100715a5", param_1, read_unaligned) as *mut u32);
            mech_trace::wr!("100715a5", pi_var1.add(1), write_unaligned, pi_var2 as i32);
            let pi_var2 = FUN_100715a5(
                mech_trace::rd!("100715a5", param_1.add(1), read_unaligned) as *mut u32
            );
            mech_trace::wr!("100715a5", pi_var1.add(2), write_unaligned, pi_var2 as i32);
            mech_trace::wr!(
                "100715a5",
                pi_var1,
                write_unaligned,
                mech_trace::rd!("100715a5", param_1.add(3), read_unaligned) as i32
            );
        }
        mech_trace::rt!("100715a5", pi_var1)
    }
}
