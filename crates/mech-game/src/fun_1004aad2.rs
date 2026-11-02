// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1004ab46(
        a: i32,
        b: i32,
        c: u32,
        d: i32,
        e: i32,
        f: i32,
        g: i32,
        h: i32,
        i: i32,
        j: i32,
        k: i32,
        l: i32,
    ) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004aad2(
    param_1: u32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
    param_7: i32,
) -> u32 {
    mech_trace::enter!(
        "1004aad2", param_1, param_2, param_3, param_4, param_5, param_6, param_7
    );
    // SAFETY: the callee is the game's own function; all arguments are plain values.
    unsafe {
        mech_trace::cr!(
            "1004aad2",
            "1004ab46",
            FUN_1004ab46(
                0xfffffffe_u32 as i32,
                0,
                param_1,
                param_5,
                param_6,
                param_7,
                param_5,
                param_6,
                param_7,
                param_2,
                param_3,
                param_4,
            )
        );
    }
    mech_trace::rt!("1004aad2", 0)
}
