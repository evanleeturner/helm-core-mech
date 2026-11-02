// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf38: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002a1e0(param_1: u32, param_2: i32) -> i32;
    fn FUN_1004d47f(param_1: i32) -> i32;
    fn FUN_10008156(param_1: i32, param_2: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10028fc6(param_1: i32, param_2: u32) -> u32 {
    mech_trace::enter!("10028fc6", param_1, param_2);
    let mut param_2 = param_2;
    // SAFETY: the callees and the global are the game's own; param_1 points
    // at a structure whose fields at offsets 8 and 0xc are read where the
    // original reads them.
    unsafe {
        let mut i_var1 = mech_trace::cr!("10028fc6", "1002a1e0", FUN_1002a1e0(param_2, 2));
        if i_var1 == 0 {
            param_2 = 0;
            i_var1 = 0;
        } else {
            i_var1 = mech_trace::cr!(
                "10028fc6",
                "1004d47f",
                FUN_1004d47f(mech_trace::rd!(
                    "10028fc6",
                    (param_1.wrapping_add(8) as *const i32),
                    read_unaligned
                ))
            );
            if i_var1 == mech_trace::gr!("10028fc6", "100adf38", DAT_100adf38) {
                i_var1 = mech_trace::cr!(
                    "10028fc6",
                    "10008156",
                    FUN_10008156(
                        5,
                        mech_trace::rd!(
                            "10028fc6",
                            (param_1.wrapping_add(0xc) as *const i32),
                            read_unaligned
                        ),
                    )
                );
            }
        }
        mech_trace::rt!(
            "10028fc6",
            ((i_var1 as u32) & 0xFFFF0000) | (param_2 & 0xFFFF)
        )
    }
}
