// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1005ade8: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1005adf0(param_1: *mut i16, param_2: i32) -> *mut i16;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005aff9(param_1: i32, param_2: *mut u8) {
    // SAFETY: the callee and the global are the game's own; the callee returns
    // a buffer of at least 0x300 bytes and param_2 has room for 0x300 bytes.
    unsafe {
        let mut pb_var1 = FUN_1005adf0((&raw mut DAT_1005ade8).cast::<i16>(), param_1) as *const u8;
        let mut p2 = param_2;
        let mut i_var2: i32 = 0x300;
        loop {
            *p2 = *pb_var1 >> 2;
            i_var2 = i_var2.wrapping_sub(1);
            pb_var1 = pb_var1.add(1);
            p2 = p2.add(1);
            if i_var2 == 0 {
                break;
            }
        }
    }
}
