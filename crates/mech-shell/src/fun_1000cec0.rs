// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10018060(param_1: i32, param_2: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000cec0(param_1: i32) -> u32 {
    mech_trace::enter!("1000cec0", param_1);
    let mut i_var2: i32 = 0;
    // SAFETY: param_1 points at the game's collection object; the callee and
    // the item pointers it returns are the game's own, read where the original does.
    unsafe {
        if mech_trace::rd!(
            "1000cec0",
            (param_1.wrapping_add(4) as *const i32),
            read_unaligned
        ) > 0
        {
            loop {
                let i_var1 = mech_trace::cr!("1000cec0", "10018060", FUN_10018060(param_1, i_var2));
                if mech_trace::rd!("1000cec0", (i_var1.wrapping_add(0x2064) as *const i8), read)
                    == 1
                    && mech_trace::rd!(
                        "1000cec0",
                        (i_var1.wrapping_add(0x20) as *const i32),
                        read_unaligned
                    ) != -1
                {
                    return mech_trace::rt!("1000cec0", ((i_var1 >> 8) | 1) as u32);
                }
                i_var2 = i_var2.wrapping_add(1);
                let in_eax = mech_trace::rd!(
                    "1000cec0",
                    (param_1.wrapping_add(4) as *const u32),
                    read_unaligned
                );
                if !(i_var2 < in_eax as i32) {
                    break;
                }
            }
        }
    }
    mech_trace::rt!("1000cec0", 0)
}
