// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1003e369(param_1: i32) -> u8;
    fn FUN_1003e3e3(param_1: i32, param_2: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003dfde(param_1: *mut u16) {
    mech_trace::enter!("1003dfde", param_1);
    if param_1.is_null() {
        return;
    }
    // SAFETY: param_1 is non-null and points at the caller's structure; the
    // callees are the game's own functions.
    unsafe {
        if (mech_trace::rd!("1003dfde", param_1, read_unaligned) & 0x1000) != 0 {
            return;
        }
        mech_trace::wr!(
            "1003dfde",
            param_1,
            write_unaligned,
            mech_trace::rd!("1003dfde", param_1, read_unaligned) | 0x1000
        );
        if mech_trace::rd!("1003dfde", param_1.add(2).cast::<i32>(), read_unaligned) != 0 {
            mech_trace::cr!("1003dfde", "1003e369", FUN_1003e369(param_1 as i32));
            mech_trace::cr!(
                "1003dfde",
                "1003e3e3",
                FUN_1003e3e3(param_1 as i32, 0x100c6d30)
            );
        }
    }
}
