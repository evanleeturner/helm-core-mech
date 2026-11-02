// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a67a4: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10018060(param_1: i32, param_2: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1004b0f0() -> i32 {
    mech_trace::enter!("1004b0f0");
    // SAFETY: the globals and callee are the game's own; the collection
    // pointer and item pointers are read at the offsets the original uses.
    unsafe {
        let mut i_var2: i32 = 0;
        let mut i_var3: i32 = 0;
        if 0 < mech_trace::rd!(
            "1004b0f0",
            ((mech_trace::gr!("1004b0f0", "100a67a4", DAT_100a67a4).wrapping_add(4)) as *const i32),
            read_unaligned
        ) {
            loop {
                let i_var1 = mech_trace::cr!(
                    "1004b0f0",
                    "10018060",
                    FUN_10018060(
                        mech_trace::gr!("1004b0f0", "100a67a4", DAT_100a67a4) as i32,
                        i_var3
                    )
                );
                if mech_trace::rd!(
                    "1004b0f0",
                    ((i_var1.wrapping_add(0x2064)) as *const i8),
                    read
                ) == 0
                {
                    i_var2 = i_var2.wrapping_add(mech_trace::rd!(
                        "1004b0f0",
                        ((i_var1.wrapping_add(0x3c)) as *const i32),
                        read_unaligned
                    ));
                }
                i_var3 = i_var3.wrapping_add(1);
                if !(i_var3
                    < mech_trace::rd!(
                        "1004b0f0",
                        ((mech_trace::gr!("1004b0f0", "100a67a4", DAT_100a67a4).wrapping_add(4))
                            as *const i32),
                        read_unaligned
                    ))
                {
                    break;
                }
            }
        }
        mech_trace::rt!("1004b0f0", i_var2)
    }
}
