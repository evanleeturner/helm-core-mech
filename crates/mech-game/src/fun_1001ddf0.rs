// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ab4c0: [i32; 0x5b];
    static mut DAT_100ab398: [*mut i8; 0x5b];
    static mut DAT_100ab2ac: [i8; 1];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001ddf0(param_1: i32) -> *mut i8 {
    mech_trace::enter!("1001ddf0", param_1);
    let mut local_8: i32 = 0;
    // SAFETY: the tables are the game's own memory; indices stay below 0x5b.
    unsafe {
        while local_8 < 0x5b
            && mech_trace::gr!("1001ddf0", "100ab4c0", DAT_100ab4c0[local_8 as usize]) != param_1
        {
            local_8 += 1;
        }
        mech_trace::rt!(
            "1001ddf0",
            if local_8 < 0x5b {
                mech_trace::gr!("1001ddf0", "100ab398", DAT_100ab398[local_8 as usize])
            } else {
                (&raw mut DAT_100ab2ac).cast::<i8>()
            }
        )
    }
}
