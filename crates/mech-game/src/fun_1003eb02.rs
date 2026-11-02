// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100af17c: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1008668b(param_1: i32, param_2: u32, param_3: *mut i32) -> *mut u16;
    fn FUN_1008664f(param_1: u32, param_2: *mut i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003eb02(param_1: u32, param_2: *mut i32) -> bool {
    // SAFETY: the callees and the global are the game's own; param_2 is passed
    // through unchanged as the caller supplied it.
    unsafe {
        let p_var1 = FUN_1008668b(DAT_100af17c as i32, param_1, param_2);
        if !p_var1.is_null() {
            FUN_1008664f(param_1, param_2);
        }
        !p_var1.is_null()
    }
}
