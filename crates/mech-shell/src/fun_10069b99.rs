// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10069988: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10069990(param_1: *mut i16, param_2: i32) -> *mut i16;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10069b99(param_1: i32, param_2: *mut u8) {
    mech_trace::enter!("10069b99", param_1, param_2);
    // SAFETY: the callee and the global are the game's own; the source
    // returned by the callee holds 0x300 bytes and param_2 has room for 0x300.
    unsafe {
        let mut pb_var1 = mech_trace::cr!(
            "10069b99",
            "10069990",
            FUN_10069990((&raw mut DAT_10069988).cast::<i16>(), param_1)
        ) as *const u8;
        let mut out = param_2;
        let mut i_var2: i32 = 0x300;
        loop {
            mech_trace::dw!("10069b99", out, mech_trace::dr!("10069b99", pb_var1) >> 2);
            i_var2 = i_var2.wrapping_sub(1);
            pb_var1 = pb_var1.add(1);
            out = out.add(1);
            if i_var2 == 0 {
                break;
            }
        }
    }
}
