// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1009463c: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003b570(index_Likely: i32) {
    mech_trace::enter!("1003b570", index_Likely);
    let i_var1 = (index_Likely as u32).wrapping_add(1) as i32;
    // SAFETY: DAT_1009463c is the game's own global, read and written by value.
    unsafe {
        if mech_trace::gr!("1003b570", "1009463c", DAT_1009463c) < i_var1 {
            mech_trace::gw!("1003b570", "1009463c", DAT_1009463c, i_var1);
        }
    }
}
