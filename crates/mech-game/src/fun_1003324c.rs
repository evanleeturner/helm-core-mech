// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adaa8: u32;
    static mut DAT_100adaac: u32;
    static mut DAT_10168490: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10050a68(param_1: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1003324c() -> u32 {
    mech_trace::enter!("1003324c");
    // SAFETY: the globals and the callee are the game's own; the table at
    // DAT_10168490 is read at the index the original uses.
    unsafe {
        mech_trace::rt!(
            "1003324c",
            if mech_trace::gr!("1003324c", "100adaac", DAT_100adaac)
                < mech_trace::gr!("1003324c", "100adaa8", DAT_100adaa8)
            {
                let p = (&raw mut DAT_10168490)
                    .byte_add(
                        mech_trace::gr!("1003324c", "100adaac", DAT_100adaac).wrapping_mul(4)
                            as usize,
                    )
                    .cast::<i32>();
                let local_c = mech_trace::cr!(
                    "1003324c",
                    "10050a68",
                    FUN_10050a68(mech_trace::rd!("1003324c", p, read_unaligned))
                );
                mech_trace::gw!(
                    "1003324c",
                    "100adaac",
                    DAT_100adaac,
                    mech_trace::gr!("1003324c", "100adaac", DAT_100adaac).wrapping_add(1)
                );
                local_c
            } else {
                0
            }
        )
    }
}
