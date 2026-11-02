// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1007cbc4(param_1: *mut i32) -> u8;
    fn FUN_1007c9fd(param_1: i32) -> i32;
    fn FUN_1007c97d(param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1007c28e(param_1: *mut i32, param_2: *mut u8) {
    mech_trace::enter!("1007c28e", param_1, param_2);
    if !param_1.is_null() {
        // SAFETY: the callees are the game's own; param_2, when non-null, is
        // a callback taking one pointer argument, called as the original does.
        unsafe {
            mech_trace::cr!("1007c28e", "1007cbc4", FUN_1007cbc4(param_1));
            if !param_2.is_null() {
                let f: unsafe extern "C" fn(*mut i32) =
                    core::mem::transmute::<*mut u8, unsafe extern "C" fn(*mut i32)>(param_2);
                mech_trace::cr!("1007c28e", "ind", f(param_1));
            }
            let i_var1 = mech_trace::cr!("1007c28e", "1007c9fd", FUN_1007c9fd(param_1 as i32));
            if i_var1 != -1 {
                mech_trace::cr!("1007c28e", "1007c97d", FUN_1007c97d(i_var1));
            }
        }
    }
}
