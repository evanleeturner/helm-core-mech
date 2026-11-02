// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10123230: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1004d4b9(param_1: i32) -> u32;
    fn FUN_1002b107(param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10026950() {
    mech_trace::enter!("10026950");
    let mut local_8: u32 = 0;
    // SAFETY: the callees and the global are the game's own; the loop
    // re-reads the global each iteration as the original does.
    unsafe {
        while local_8 < mech_trace::gr!("10026950", "10123230", DAT_10123230) {
            mech_trace::cr!("10026950", "1004d4b9", FUN_1004d4b9(local_8 as i32));
            mech_trace::cr!("10026950", "1002b107", FUN_1002b107(local_8 as i32));
            local_8 = local_8.wrapping_add(1);
        }
    }
}
