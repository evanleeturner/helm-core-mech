// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10022bc1(param_1: i32, param_2: i32) -> i32;
    fn FUN_10022e3c(param_1: i32, param_2: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10022b70(param_1: i32, param_2: i32) -> i32 {
    mech_trace::enter!("10022b70", param_1, param_2);
    // SAFETY: param_1 is an address of the caller's object; the field at
    // offset 0x1c0 is read, and the callees are the game's own.
    unsafe {
        mech_trace::rt!(
            "10022b70",
            if mech_trace::rd!(
                "10022b70",
                ((param_1.wrapping_add(0x1c0)) as *const i32),
                read_unaligned
            ) == 0
            {
                mech_trace::cr!("10022b70", "10022e3c", FUN_10022e3c(param_1, param_2))
            } else {
                mech_trace::cr!("10022b70", "10022bc1", FUN_10022bc1(param_1, param_2))
            }
        )
    }
}
