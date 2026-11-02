// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1001ca37(param_1: i32, param_2: u16) -> u8;
    fn FUN_1002fcce(param_1: *mut i32) -> u8;
    fn FUN_1002f727(param_1: i32) -> u8;
    fn FUN_1002f7f7(param_1: i32) -> u8;
    fn FUN_1002fbc0(param_1: *mut i32) -> u8;
    fn FUN_10050ecc(param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100301b0(param_1: *mut i32) {
    if !param_1.is_null() {
        // SAFETY: param_1 is non-null and points at the game's object.
        unsafe {
            let v = param_1.add(0x1b).read_unaligned();
            if v != 0 {
                FUN_1001ca37(v, 0x50);
                FUN_1002fcce(param_1);
                FUN_1002f727(param_1 as i32);
                FUN_1002f7f7(param_1 as i32);
                FUN_1002fbc0(param_1);
                FUN_10050ecc(param_1 as i32);
            }
        }
    }
}
