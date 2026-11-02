// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1009463c: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003b570(index_Likely: i32) {
    let i_var1 = (index_Likely as u32).wrapping_add(1) as i32;
    // SAFETY: DAT_1009463c is the game's own global, read and written by value.
    unsafe {
        if DAT_1009463c < i_var1 {
            DAT_1009463c = i_var1;
        }
    }
}
