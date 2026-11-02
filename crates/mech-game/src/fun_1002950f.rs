// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100832d4(param_1: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002950f(param_1: i32, param_2: i16) -> bool {
    mech_trace::enter!("1002950f", param_1, param_2);
    // SAFETY: param_1 is the address of a game object; the field at offset
    // 0xdc is written where the original does, and the callee is the game's own.
    unsafe {
        mech_trace::wr!(
            "1002950f",
            ((param_1.wrapping_add(0xdc)) as *mut i32),
            write_unaligned,
            param_2 as i32
        );
        let i_var1 = mech_trace::cr!("1002950f", "100832d4", FUN_100832d4(param_1)) as i32;
        mech_trace::rt!("1002950f", 0 < i_var1)
    }
}
