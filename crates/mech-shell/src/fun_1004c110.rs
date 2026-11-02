// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a678c: *mut i32;
    static mut DAT_100a6798: *mut i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10017c90(
        param_1: *mut u32,
        param_2: i32,
        param_3: u32,
        param_4: u8,
        param_5: u32,
    ) -> u32;
    fn FUN_10018060(param_1: i32, param_2: i32) -> u32;
    fn FUN_10017df0(param_1: *mut i32, param_2: u32) -> u32;
    fn FUN_1000ad30(param_1: i32, param_2: i32) -> i32;
    fn FUN_100180f0(param_1: *mut i32, param_2: i8) -> u8;
    fn FUN_10018090(param_1: *mut i32, param_2: *mut i32) -> u8;
    fn FUN_10017d70(param_1: *mut i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1004c110() {
    mech_trace::enter!("1004c110");
    let mut local_8: *mut i32 = core::ptr::null_mut();
    let mut local_4: *mut i32 = core::ptr::null_mut();
    let mut i_var2: i32 = 0;
    // SAFETY: the callees and globals are the game's own; the collection and
    // item pointers come from the game's collection functions and are used
    // as the original does.
    unsafe {
        mech_trace::cr!(
            "1004c110",
            "10017c90",
            FUN_10017c90((&raw mut local_8).cast::<u32>(), 10, 0, 4, 0)
        );
        mech_trace::cr!(
            "1004c110",
            "10017c90",
            FUN_10017c90((&raw mut local_4).cast::<u32>(), 10, 0, 4, 0)
        );
        if 0 < mech_trace::rd!(
            "1004c110",
            mech_trace::gr!("1004c110", "100a6798", DAT_100a6798).byte_add(4),
            read_unaligned
        ) {
            loop {
                let item = mech_trace::cr!(
                    "1004c110",
                    "10018060",
                    FUN_10018060(
                        mech_trace::gr!("1004c110", "100a6798", DAT_100a6798) as i32,
                        i_var2
                    )
                );
                let count = mech_trace::rd!(
                    "1004c110",
                    ((item as usize).wrapping_add(0x814) as *const i32),
                    read_unaligned
                );
                if 0 < count {
                    mech_trace::cr!("1004c110", "10017df0", FUN_10017df0(local_8, item));
                    let key = mech_trace::rd!(
                        "1004c110",
                        ((item as usize).wrapping_add(0x810) as *const i32),
                        read_unaligned
                    );
                    let found = mech_trace::cr!(
                        "1004c110",
                        "1000ad30",
                        FUN_1000ad30(
                            mech_trace::gr!("1004c110", "100a678c", DAT_100a678c) as i32,
                            key
                        )
                    );
                    mech_trace::cr!("1004c110", "10017df0", FUN_10017df0(local_4, found as u32));
                }
                i_var2 = i_var2.wrapping_add(1);
                if !(i_var2
                    < mech_trace::rd!(
                        "1004c110",
                        mech_trace::gr!("1004c110", "100a6798", DAT_100a6798).byte_add(4),
                        read_unaligned
                    ))
                {
                    break;
                }
            }
        }
        mech_trace::cr!(
            "1004c110",
            "100180f0",
            FUN_100180f0(mech_trace::gr!("1004c110", "100a6798", DAT_100a6798), 0)
        );
        mech_trace::cr!(
            "1004c110",
            "100180f0",
            FUN_100180f0(mech_trace::gr!("1004c110", "100a678c", DAT_100a678c), 0)
        );
        mech_trace::cr!(
            "1004c110",
            "10018090",
            FUN_10018090(
                mech_trace::gr!("1004c110", "100a6798", DAT_100a6798),
                local_8
            )
        );
        mech_trace::cr!(
            "1004c110",
            "10018090",
            FUN_10018090(
                mech_trace::gr!("1004c110", "100a678c", DAT_100a678c),
                local_4
            )
        );
        mech_trace::cr!("1004c110", "10017d70", FUN_10017d70(local_8));
        mech_trace::cr!("1004c110", "10017d70", FUN_10017d70(local_4));
    }
}
