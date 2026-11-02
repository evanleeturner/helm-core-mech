// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a67b4: *mut i32;
    static mut DAT_100a67b8: *mut i32;
    static mut DAT_100a67cc: *mut i32;
    static mut DAT_100a67d0: *mut i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10018060(param_1: i32, param_2: i32) -> u32;
    fn FUN_10017df0(param_1: *mut i32, param_2: u32) -> u32;
    fn FUN_100180f0(param_1: *mut i32, param_2: i8) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1000c410() {
    mech_trace::enter!("1000c410");
    // SAFETY: the globals and callees are the game's own; the collection
    // pointers are read where the original reads them.
    unsafe {
        let mut i_var3: i32 = 0;
        if 0 < mech_trace::rd!(
            "1000c410",
            mech_trace::gr!("1000c410", "100a67cc", DAT_100a67cc).byte_add(4),
            read_unaligned
        ) {
            loop {
                let i_var2 = i_var3.wrapping_add(1);
                let u_var1 = mech_trace::cr!(
                    "1000c410",
                    "10018060",
                    FUN_10018060(
                        mech_trace::gr!("1000c410", "100a67cc", DAT_100a67cc) as i32,
                        i_var3
                    )
                );
                mech_trace::cr!(
                    "1000c410",
                    "10017df0",
                    FUN_10017df0(
                        mech_trace::gr!("1000c410", "100a67b4", DAT_100a67b4),
                        u_var1
                    )
                );
                i_var3 = i_var2;
                if !(i_var2
                    < mech_trace::rd!(
                        "1000c410",
                        mech_trace::gr!("1000c410", "100a67cc", DAT_100a67cc).byte_add(4),
                        read_unaligned
                    ))
                {
                    break;
                }
            }
        }
        mech_trace::cr!(
            "1000c410",
            "100180f0",
            FUN_100180f0(mech_trace::gr!("1000c410", "100a67cc", DAT_100a67cc), 0)
        );
        i_var3 = 0;
        if 0 < mech_trace::rd!(
            "1000c410",
            mech_trace::gr!("1000c410", "100a67d0", DAT_100a67d0).byte_add(4),
            read_unaligned
        ) {
            loop {
                let i_var2 = i_var3.wrapping_add(1);
                let u_var1 = mech_trace::cr!(
                    "1000c410",
                    "10018060",
                    FUN_10018060(
                        mech_trace::gr!("1000c410", "100a67d0", DAT_100a67d0) as i32,
                        i_var3
                    )
                );
                mech_trace::cr!(
                    "1000c410",
                    "10017df0",
                    FUN_10017df0(
                        mech_trace::gr!("1000c410", "100a67b8", DAT_100a67b8),
                        u_var1
                    )
                );
                i_var3 = i_var2;
                if !(i_var2
                    < mech_trace::rd!(
                        "1000c410",
                        mech_trace::gr!("1000c410", "100a67d0", DAT_100a67d0).byte_add(4),
                        read_unaligned
                    ))
                {
                    break;
                }
            }
        }
        mech_trace::cr!(
            "1000c410",
            "100180f0",
            FUN_100180f0(mech_trace::gr!("1000c410", "100a67d0", DAT_100a67d0), 0)
        );
    }
}
