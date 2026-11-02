// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1008628d(param_1: u32, param_2: *mut i32) -> *mut u16;
    fn FUN_10086336(param_1: *mut u16);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1008689c(param_1: u32, param_2: *mut i32) {
    mech_trace::enter!("1008689c", param_1, param_2);
    // SAFETY: the callees are the game's own functions; the pointer returned
    // by the lookup is passed on only when non-null.
    unsafe {
        let p_var1 = mech_trace::cr!("1008689c", "1008628d", FUN_1008628d(param_1, param_2));
        if !p_var1.is_null() {
            FUN_10086336(p_var1);
        }
    }
}
