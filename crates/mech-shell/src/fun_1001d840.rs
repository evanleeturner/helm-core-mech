// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a6798: u32;
    static mut DAT_100b5124: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10018060(param_1: i32, param_2: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1001d840() -> i32 {
    mech_trace::enter!("1001d840");
    // SAFETY: the globals and callee are the game's own; the collection
    // pointer and items it returns are valid game memory read as the original does.
    unsafe {
        let mut i_var3: i32 = 0;
        let mut i_var2: i32 = 0;
        if 0 < mech_trace::rd!(
            "1001d840",
            ((mech_trace::gr!("1001d840", "100a6798", DAT_100a6798).wrapping_add(4)) as *const i32),
            read_unaligned
        ) {
            loop {
                let i_var1 = mech_trace::cr!(
                    "1001d840",
                    "10018060",
                    FUN_10018060(
                        mech_trace::gr!("1001d840", "100a6798", DAT_100a6798) as i32,
                        i_var2
                    )
                ) as i32;
                if mech_trace::rd!(
                    "1001d840",
                    ((i_var1.wrapping_add(0x80c)) as *const i32),
                    read_unaligned
                ) == -1
                    && mech_trace::gr!("1001d840", "100b5124", DAT_100b5124) != i_var1
                {
                    i_var3 = i_var3.wrapping_add(1);
                }
                i_var2 = i_var2.wrapping_add(1);
                if i_var2
                    >= mech_trace::rd!(
                        "1001d840",
                        ((mech_trace::gr!("1001d840", "100a6798", DAT_100a6798).wrapping_add(4))
                            as *const i32),
                        read_unaligned
                    )
                {
                    break;
                }
            }
        }
        mech_trace::rt!("1001d840", i_var3)
    }
}
