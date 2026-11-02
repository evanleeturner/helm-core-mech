// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10066609(param_1: *mut i32, param_2: i32);
    fn FUN_1003517a(param_1: i32, param_2: i32, param_3: i32, param_4: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10068a11(param_1: i32, param_2: i32, param_3: i32) {
    // SAFETY: param_1 points at the game's object; the field at 0xa4 is a
    // pointer and the field at 0xb8 is a short, read where the original does.
    unsafe {
        let pi_var1 = (param_1 as usize).wrapping_add(0xa4) as *const *mut i32;
        let pi_var1 = pi_var1.read_unaligned();
        FUN_10066609(pi_var1, param_2);
        let s = ((param_1 as usize).wrapping_add(0xb8) as *const i16).read_unaligned();
        FUN_1003517a((s as i32) >> 1, 2, param_3, pi_var1);
    }
}
