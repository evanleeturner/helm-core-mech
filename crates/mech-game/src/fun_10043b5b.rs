// SPDX-License-Identifier: MIT

unsafe extern "C" {
    fn FUN_1002950f(param_1: i32, param_2: i16);
    fn FUN_10044054(param_1: *mut i32) -> i32;
    fn FUN_100297b4(param_1: i32) -> i32;
    fn FUN_1002a132(param_1: i32) -> i32;
    fn FUN_10022b70(param_1: i32, param_2: i32);
    fn FUN_10044a83(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10043b5b(param_1: *mut i32, param_2: i16) {
    mech_trace::enter!("10043b5b", param_1, param_2);
    // SAFETY: the callees are the game's own; param_1 points at the game's
    // object, whose field 0x13 holds a pointer to a record read and written
    // where the original does.
    unsafe {
        FUN_1002950f(param_1 as i32, param_2);
        let rec = mech_trace::rd!("10043b5b", param_1.add(0x13), read_unaligned) as *mut u8;
        let local_8: i32;
        if (mech_trace::rd!("10043b5b", rec.byte_add(0x30), read) as i8) == 0 {
            if mech_trace::cr!("10043b5b", "10044054", FUN_10044054(param_1)) == 0 {
                local_8 = mech_trace::cr!("10043b5b", "100297b4", FUN_100297b4(param_1 as i32));
            } else {
                local_8 = mech_trace::cr!("10043b5b", "1002a132", FUN_1002a132(param_1 as i32));
            }
        } else {
            local_8 = mech_trace::cr!("10043b5b", "100297b4", FUN_100297b4(param_1 as i32));
        }
        FUN_10022b70(param_1 as i32, local_8);
        let rec2 = mech_trace::rd!("10043b5b", param_1.add(0x13), read_unaligned) as *mut u8;
        mech_trace::wr!(
            "10043b5b",
            rec2.byte_add(8).cast::<i32>(),
            write_unaligned,
            0x400
        );
        FUN_10044a83(param_1 as i32);
    }
}
