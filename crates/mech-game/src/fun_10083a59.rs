// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10006d3e() -> i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10006b20(param_1: i32) -> i32;
    fn FUN_10022b50(param_1: u32, param_2: i32) -> u32;
    fn FUN_10006b9a(param_1: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10083a59(
    param_1: u32,
    param_2: u32,
    param_3: u32,
    param_4: *mut i32,
    param_5: *mut u32,
    param_6: *mut u32,
    param_7: *mut i32,
) {
    mech_trace::enter!(
        "10083a59", param_1, param_2, param_3, param_4, param_5, param_6, param_7
    );
    let _ = param_2;
    // SAFETY: the callees are the game's own functions; the output pointers
    // are supplied by the caller and written where the original writes.
    unsafe {
        let i_var1 = mech_trace::cr!("10083a59", "10006d3e", FUN_10006d3e());
        let a3 = param_3 as i32;
        let a1 = param_1 as i32;
        let abs3 = (a3 ^ (a3 >> 0x1f)).wrapping_sub(a3 >> 0x1f);
        let abs1 = (a1 ^ (a1 >> 0x1f)).wrapping_sub(a1 >> 0x1f);
        let local_10: u32;
        if abs3 < abs1 {
            let i_var2 = mech_trace::cr!("10083a59", "10006b20", FUN_10006b20(i_var1));
            if i_var2 == 0 {
                local_10 = 0;
            } else {
                local_10 = mech_trace::cr!("10083a59", "10022b50", FUN_10022b50(param_1, i_var2));
            }
        } else {
            let i_var2 = mech_trace::cr!("10083a59", "10006b9a", FUN_10006b9a(i_var1));
            if i_var2 == 0 {
                local_10 = 0;
            } else {
                local_10 = mech_trace::cr!("10083a59", "10022b50", FUN_10022b50(param_3, i_var2));
            }
        }
        let i_var2 = mech_trace::cr!("10083a59", "10006d3e", FUN_10006d3e());
        let i_var3 = mech_trace::cr!("10083a59", "10006b9a", FUN_10006b9a(i_var2));
        let local_1c: u32;
        if i_var3 == 0 {
            local_1c = 0;
        } else {
            local_1c = mech_trace::cr!("10083a59", "10022b50", FUN_10022b50(local_10, i_var3));
        }
        mech_trace::dw!("10083a59", param_4, i_var1);
        mech_trace::dw!("10083a59", param_6, local_10);
        mech_trace::dw!("10083a59", param_5, local_1c);
        mech_trace::dw!("10083a59", param_7, i_var2);
    }
}
