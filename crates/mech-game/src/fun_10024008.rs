// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10023b0d() -> *mut u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10024008(param_1: u32, param_2: u32, param_3: u32) -> *mut u32 {
    mech_trace::enter!("10024008", param_1, param_2, param_3);
    // SAFETY: the callee is the game's own; the returned pointer, when
    // non-null, points at a structure large enough for the writes below.
    unsafe {
        let p_var1 = mech_trace::cr!("10024008", "10023b0d", FUN_10023b0d());
        if !p_var1.is_null() {
            mech_trace::wr!("10024008", p_var1.byte_add(0x1d6), write_unaligned, param_1);
            mech_trace::wr!("10024008", p_var1.byte_add(0x1da), write_unaligned, param_2);
            mech_trace::wr!("10024008", p_var1.byte_add(0x1f6), write_unaligned, param_3);
        }
        mech_trace::rt!("10024008", p_var1)
    }
}
