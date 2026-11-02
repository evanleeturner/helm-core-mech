// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa55c: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100145a1(param_1: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004448b(param_1: i32) -> bool {
    mech_trace::enter!("1004448b", param_1);
    // SAFETY: the callee and the global are the game's own; param_1 is an
    // address of the caller's object, read at byte offset 2 as the original does.
    unsafe {
        let i_var1 = mech_trace::cr!("1004448b", "100145a1", FUN_100145a1(param_1));
        mech_trace::rt!(
            "1004448b",
            if i_var1 == 0 || mech_trace::gr!("1004448b", "100aa55c", DAT_100aa55c) < 0xc41b {
                (mech_trace::rd!("1004448b", (param_1.wrapping_add(2) as *const u8), read) & 0xf0)
                    == 0x50
            } else {
                true
            }
        )
    }
}
