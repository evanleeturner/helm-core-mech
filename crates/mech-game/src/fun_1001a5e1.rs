// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ab318: i32;
    static mut DAT_100ab098: i32;
    static mut DAT_100ab088: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001a5e1(param_1: i32) {
    let mut param_1 = param_1;
    // SAFETY: the globals are the game's own memory, read and written
    // where the original does.
    unsafe {
        if DAT_100ab318 != 0 {
            if DAT_100ab098 == -1 {
                DAT_100ab098 = 1;
            }
            param_1 = DAT_100ab098;
        }
        DAT_100ab088 = param_1;
    }
}
