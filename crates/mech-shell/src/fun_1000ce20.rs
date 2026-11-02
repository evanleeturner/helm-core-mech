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
pub unsafe extern "stdcall" fn FUN_1000ce20() -> i32 {
    mech_trace::enter!("1000ce20");
    // SAFETY: the global and the callee are the game's own; the collection
    // pointer and the item pointers it yields are read where the original does.
    unsafe {
        let mut i_var2: i32 = 0;
        if 0 < mech_trace::rd!(
            "1000ce20",
            ((mech_trace::gr!("1000ce20", "100a67a4", DAT_100a67a4) as usize + 4) as *const i32),
            read_unaligned
        ) {
            loop {
                let i_var1 = mech_trace::cr!(
                    "1000ce20",
                    "10018060",
                    FUN_10018060(
                        mech_trace::gr!("1000ce20", "100a67a4", DAT_100a67a4) as i32,
                        i_var2
                    )
                ) as i32;
                if mech_trace::rd!(
                    "1000ce20",
                    ((i_var1 as u32).wrapping_add(0x2064) as usize as *const i8),
                    read
                ) == 1
                {
                    return mech_trace::rt!("1000ce20", i_var1);
                }
                i_var2 = i_var2.wrapping_add(1);
                if !(i_var2
                    < mech_trace::rd!(
                        "1000ce20",
                        ((mech_trace::gr!("1000ce20", "100a67a4", DAT_100a67a4) as usize + 4)
                            as *const i32),
                        read_unaligned
                    ))
                {
                    break;
                }
            }
        }
        mech_trace::rt!("1000ce20", 0)
    }
}
