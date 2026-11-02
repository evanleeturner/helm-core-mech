// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000a5d0(record_likely: i32);
    fn FUN_1000a690(record_likely: i32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1000a6e0();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000a890(record_likely: i32) {
    // SAFETY: the callees are the game's own functions; they are called
    // in the original's order with the original's argument.
    unsafe {
        FUN_1000a5d0(record_likely);
        FUN_1000a690(record_likely);
        FUN_1000a6e0();
    }
}
