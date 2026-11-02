// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38fc: i32;
    static mut DAT_100a1208: f64;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002950f(param_1: i32, param_2: i16);
    fn FUN_1002a5d4(param_1: i32);
    fn FUN_1002a6ac(param_1: i32);
    fn FUN_1002a132(param_1: i32) -> i32;
    fn FUN_10022b70(param_1: i32, param_2: i32);
    fn FUN_10044054(param_1: *mut i32) -> i32;
    fn FUN_100295f4(param_1: i32, param_2: i32) -> u64;
    fn FUN_100297b4(param_1: i32);
    fn FUN_10044a83(param_1: i32);
    fn FUN_1002a89b(param_1: i32, param_2: i16);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10043bff(param_1: *mut i32) -> i32 {
    mech_trace::enter!("10043bff", param_1);
    // SAFETY: param_1 points at the game's object; the globals and callees
    // are the game's own, accessed in the original order.
    unsafe {
        let p = param_1 as i32;
        FUN_1002950f(
            p,
            mech_trace::rd!(
                "10043bff",
                param_1.byte_add(0x17a).cast::<i16>(),
                read_unaligned
            ),
        );
        if mech_trace::rd!("10043bff", param_1.add(0x74), read_unaligned)
            < mech_trace::gr!("10043bff", "100b38fc", DAT_100b38fc)
        {
            if mech_trace::gr!("10043bff", "100a1208", DAT_100a1208)
                < (mech_trace::rd!("10043bff", param_1.add(0x30), read_unaligned) as f64)
            {
                FUN_1002a5d4(p);
                FUN_1002a6ac(p);
            }
            mech_trace::wr!(
                "10043bff",
                param_1.add(0x74),
                write_unaligned,
                mech_trace::gr!("10043bff", "100b38fc", DAT_100b38fc).wrapping_add(0x5a8)
            );
        }
        let i_var2 = mech_trace::cr!("10043bff", "1002a132", FUN_1002a132(p));
        FUN_10022b70(p, i_var2);
        FUN_1002950f(
            p,
            mech_trace::rd!("10043bff", param_1.add(0x5e), read_unaligned) as i16,
        );
        if mech_trace::cr!("10043bff", "10044054", FUN_10044054(param_1)) == 0 {
            let u_var3 = mech_trace::cr!("10043bff", "100295f4", FUN_100295f4(p, 3000));
            let base = mech_trace::rd!("10043bff", param_1.add(0x13), read_unaligned);
            mech_trace::wr!(
                "10043bff",
                (base.wrapping_add(8) as *mut i32),
                write_unaligned,
                u_var3 as i32
            );
            FUN_100297b4(p);
        }
        FUN_10044a83(p);
        if mech_trace::rd!("10043bff", param_1.add(0x30), read_unaligned) < 0xbb9 {
            FUN_1002a89b(
                p,
                mech_trace::rd!("10043bff", param_1.add(0x5e), read_unaligned) as i16,
            );
        }
        mech_trace::rt!("10043bff", 0)
    }
}
