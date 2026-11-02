// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1006346b();
    fn FUN_1005c2d0();
    fn FUN_1002aa71();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1003eae8() {
    // SAFETY: the callees are the game's own functions, called in the original order.
    unsafe {
        FUN_1006346b();
        FUN_1005c2d0();
        FUN_1002aa71();
    }
}
