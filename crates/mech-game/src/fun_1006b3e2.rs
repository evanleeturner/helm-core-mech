// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002f783(param_1: i32) -> u8;
    fn FUN_1007be21(param_1: *mut i32, param_2: u32) -> i32;
    fn FUN_1007bf28(param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1006b3e2(param_1: *mut i32) {
    // SAFETY: param_1 points at the game's object; the callees are the
    // game's own and the fields are read in the original's order.
    unsafe {
        let base = param_1.read_unaligned();
        let arg = (base.wrapping_add(0x40) as *const i32).read_unaligned();
        FUN_1002f783(arg);
        let base = param_1.read_unaligned();
        let pi_var1 = (base.wrapping_add(0x40) as *const i32).read_unaligned() as *mut i32;
        let pi_var2 = param_1.add(0xe4).read_unaligned() as *mut i32;
        let p5 = param_1.add(0xe5).read_unaligned() as *mut i32;
        let mut i_var3 = FUN_1007be21(p5, 1);
        FUN_1007bf28(i_var3);
        i_var3 = FUN_1007be21(pi_var2, 1);
        FUN_1007bf28(i_var3);
        i_var3 = FUN_1007be21(pi_var1, 1);
        FUN_1007bf28(i_var3);
    }
}
