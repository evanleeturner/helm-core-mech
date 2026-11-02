// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ab23c: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10003d60(param_1: i32, param_2: i32) -> i32;
    fn FUN_1001d916(param_1: u32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005ddde(param_1: u32, param_2: i32) {
    let _ = param_1;
    // SAFETY: the callees and the global are the game's own; the call order
    // and the conditional write match the original.
    unsafe {
        let i_var1 = FUN_10003d60(param_2, 0xf);
        if i_var1 != DAT_100ab23c {
            DAT_100ab23c = i_var1;
            FUN_1001d916(i_var1 as u32);
        }
    }
}
