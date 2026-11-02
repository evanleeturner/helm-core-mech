// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002950f(param_1: i32, param_2: i16);
    fn FUN_1002a132(param_1: i32) -> i32;
    fn FUN_10022b70(param_1: i32, param_2: i32);
    fn FUN_10044054(param_1: *mut i32) -> u32;
    fn FUN_100295f4(param_3: *mut i32, param_4: i32) -> i32;
    fn FUN_100297b4(param_1: i32);
    fn FUN_10044a83(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10043677(param_1: *mut i32) -> u8 {
    mech_trace::enter!("10043677", param_1);
    // SAFETY: the callees are the game's own; param_1 points at the caller's
    // object, read and written at the same offsets as the original.
    unsafe {
        FUN_1002950f(
            param_1 as i32,
            mech_trace::rd!(
                "10043677",
                param_1.byte_add(0x17a).cast::<i16>(),
                read_unaligned
            ),
        );
        let i_var2 = mech_trace::cr!("10043677", "1002a132", FUN_1002a132(param_1 as i32));
        FUN_10022b70(param_1 as i32, i_var2);
        FUN_1002950f(
            param_1 as i32,
            mech_trace::rd!("10043677", param_1.add(0x5e), read_unaligned) as i16,
        );
        let u_var1 = mech_trace::cr!("10043677", "10044054", FUN_10044054(param_1));
        if u_var1 == 0 {
            let u_var3 = mech_trace::cr!("10043677", "100295f4", FUN_100295f4(param_1, 3000));
            let base = mech_trace::rd!("10043677", param_1.add(0x13), read_unaligned);
            mech_trace::wr!(
                "10043677",
                (base.wrapping_add(8) as *mut i32),
                write_unaligned,
                u_var3
            );
            FUN_100297b4(param_1 as i32);
        }
        FUN_10044a83(param_1 as i32);
        mech_trace::rt!(
            "10043677",
            (mech_trace::rd!("10043677", param_1.add(0x30), read_unaligned) < 0xbb9) as u8
        )
    }
}
