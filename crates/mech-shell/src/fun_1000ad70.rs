// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10018060(param_1: i32, param_2: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000ad70(param_1: i32, param_2: i32) -> i32 {
    mech_trace::enter!("1000ad70", param_1, param_2);
    if param_1 == 0 {
        return mech_trace::rt!("1000ad70", 0);
    }
    let mut i_var2: i32 = 0;
    // SAFETY: param_1 points at the game's collection; the callee and the
    // item pointers it returns are the game's own memory.
    unsafe {
        if 0 < mech_trace::rd!(
            "1000ad70",
            ((param_1 as usize + 4) as *const i32),
            read_unaligned
        ) {
            loop {
                let i_var1 =
                    mech_trace::cr!("1000ad70", "10018060", FUN_10018060(param_1, i_var2)) as i32;
                if mech_trace::rd!(
                    "1000ad70",
                    ((i_var1 as usize + 0x80c) as *const i32),
                    read_unaligned
                ) == param_2
                {
                    return mech_trace::rt!("1000ad70", i_var1);
                }
                i_var2 = i_var2.wrapping_add(1);
                if i_var2
                    >= mech_trace::rd!(
                        "1000ad70",
                        ((param_1 as usize + 4) as *const i32),
                        read_unaligned
                    )
                {
                    break;
                }
            }
        }
    }
    mech_trace::rt!("1000ad70", 0)
}
