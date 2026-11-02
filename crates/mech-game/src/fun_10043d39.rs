// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002950f(param_1: i32, param_2: i16);
    fn FUN_1002a132(param_1: i32) -> i32;
    fn FUN_10022b70(param_1: i32, param_2: i32);
    fn FUN_10044054(param_1: *mut i32) -> i32;
    fn FUN_100295f4(param_1: *mut i32, param_2: i32) -> i64;
    fn FUN_100297b4(param_1: i32);
    fn FUN_1002a5d4(param_1: i32);
    fn FUN_10042f22(param_1: i32, param_2: i32, param_3: i32, param_4: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10043d39(param_1: *mut i32) -> i32 {
    mech_trace::enter!("10043d39", param_1);
    // SAFETY: param_1 points at the game's object; offsets are read and
    // written where the original does, and the callees are the game's own.
    unsafe {
        let obj = param_1 as i32;
        FUN_1002950f(
            obj,
            mech_trace::rd!(
                "10043d39",
                param_1.byte_add(0x17a).cast::<i16>(),
                read_unaligned
            ),
        );
        let i_var2 = mech_trace::cr!("10043d39", "1002a132", FUN_1002a132(obj));
        FUN_10022b70(obj, i_var2);
        FUN_1002950f(
            obj,
            mech_trace::rd!(
                "10043d39",
                param_1.byte_add(0x178).cast::<i16>(),
                read_unaligned
            ),
        );
        if mech_trace::cr!("10043d39", "10044054", FUN_10044054(param_1)) == 0 {
            let u_var3 = mech_trace::cr!("10043d39", "100295f4", FUN_100295f4(param_1, 3000));
            let p = mech_trace::rd!("10043d39", param_1.add(0x13), read_unaligned) as *mut i32;
            mech_trace::wr!("10043d39", p.byte_add(8), write_unaligned, u_var3 as i32);
            FUN_100297b4(obj);
        }
        if mech_trace::rd!("10043d39", param_1.add(0x30), read_unaligned) < 0xbb9 {
            FUN_1002a5d4(obj);
            if mech_trace::rd!(
                "10043d39",
                param_1.byte_add(0x17a).cast::<i16>(),
                read_unaligned
            ) == mech_trace::rd!(
                "10043d39",
                param_1.byte_add(0x178).cast::<i16>(),
                read_unaligned
            ) {
                FUN_10042f22(
                    obj,
                    mech_trace::rd!("10043d39", param_1.add(1), read_unaligned) | 0x200,
                    1,
                    0x57e4,
                );
            } else {
                FUN_1002950f(
                    obj,
                    mech_trace::rd!(
                        "10043d39",
                        param_1.byte_add(0x17a).cast::<i16>(),
                        read_unaligned
                    ),
                );
                mech_trace::wr!(
                    "10043d39",
                    param_1.byte_add(0x178).cast::<i16>(),
                    write_unaligned,
                    mech_trace::rd!(
                        "10043d39",
                        param_1.byte_add(0x17a).cast::<i16>(),
                        read_unaligned
                    )
                );
            }
        }
        mech_trace::rt!("10043d39", 0)
    }
}
