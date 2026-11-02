// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut LAB_1000bde0: u8;
    static mut DAT_100a67b4: *mut i32;
    static mut DAT_100a67bc: *mut i32;
    static mut DAT_100a67c0: *mut i32;
    static mut DAT_100a67c4: *mut i32;
    static mut DAT_100a67c8: *mut i32;
    static mut DAT_100a67cc: *mut i32;
    static mut DAT_100a67d4: *mut i32;
    static mut DAT_100a67d8: *mut i32;
    static mut DAT_100a67dc: *mut i32;
    static mut DAT_100a67e0: *mut i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100180f0(param_1: *mut i32, param_2: i8);
    fn FUN_10017c90(
        param_1: *mut u32,
        param_2: i32,
        param_3: u32,
        param_4: u8,
        param_5: u32,
    ) -> u32;
    fn FUN_10018060(param_1: i32, param_2: i32) -> u32;
    fn FUN_1000c770(param_1: *mut i32, param_2: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1000c990() {
    mech_trace::enter!("1000c990");
    // SAFETY: all globals and callees are the game's own; the collection
    // pointers are read where the original reads them.
    unsafe {
        let lab = (&raw mut LAB_1000bde0) as usize as u32;

        FUN_100180f0(mech_trace::gr!("1000c990", "100a67bc", DAT_100a67bc), 1);
        mech_trace::cr!(
            "1000c990",
            "10017c90",
            FUN_10017c90((&raw mut DAT_100a67bc) as *mut u32, 10, 0, 1, lab)
        );
        FUN_100180f0(mech_trace::gr!("1000c990", "100a67c0", DAT_100a67c0), 1);
        mech_trace::cr!(
            "1000c990",
            "10017c90",
            FUN_10017c90((&raw mut DAT_100a67c0) as *mut u32, 10, 0, 1, lab)
        );
        FUN_100180f0(mech_trace::gr!("1000c990", "100a67c4", DAT_100a67c4), 1);
        mech_trace::cr!(
            "1000c990",
            "10017c90",
            FUN_10017c90((&raw mut DAT_100a67c4) as *mut u32, 10, 0, 1, lab)
        );
        FUN_100180f0(mech_trace::gr!("1000c990", "100a67c8", DAT_100a67c8), 1);
        mech_trace::cr!(
            "1000c990",
            "10017c90",
            FUN_10017c90((&raw mut DAT_100a67c8) as *mut u32, 10, 0, 1, lab)
        );
        FUN_100180f0(mech_trace::gr!("1000c990", "100a67d4", DAT_100a67d4), 1);
        mech_trace::cr!(
            "1000c990",
            "10017c90",
            FUN_10017c90((&raw mut DAT_100a67d4) as *mut u32, 10, 0, 1, lab)
        );
        FUN_100180f0(mech_trace::gr!("1000c990", "100a67d8", DAT_100a67d8), 1);
        mech_trace::cr!(
            "1000c990",
            "10017c90",
            FUN_10017c90((&raw mut DAT_100a67d8) as *mut u32, 10, 0, 1, lab)
        );
        FUN_100180f0(mech_trace::gr!("1000c990", "100a67dc", DAT_100a67dc), 1);
        mech_trace::cr!(
            "1000c990",
            "10017c90",
            FUN_10017c90((&raw mut DAT_100a67dc) as *mut u32, 10, 0, 1, lab)
        );
        FUN_100180f0(mech_trace::gr!("1000c990", "100a67e0", DAT_100a67e0), 1);
        mech_trace::cr!(
            "1000c990",
            "10017c90",
            FUN_10017c90((&raw mut DAT_100a67e0) as *mut u32, 10, 0, 1, lab)
        );

        let mut item_index: i32 = 0;
        if 0 < mech_trace::rd!(
            "1000c990",
            mech_trace::gr!("1000c990", "100a67b4", DAT_100a67b4).add(1),
            read_unaligned
        ) {
            loop {
                let next_index = item_index.wrapping_add(1);
                let item = mech_trace::cr!(
                    "1000c990",
                    "10018060",
                    FUN_10018060(
                        mech_trace::gr!("1000c990", "100a67b4", DAT_100a67b4) as usize as i32,
                        item_index
                    )
                ) as usize as *mut i32;
                FUN_1000c770(item, 0);
                item_index = next_index;
                if !(next_index
                    < mech_trace::rd!(
                        "1000c990",
                        mech_trace::gr!("1000c990", "100a67b4", DAT_100a67b4).add(1),
                        read_unaligned
                    ))
                {
                    break;
                }
            }
        }
        FUN_100180f0(mech_trace::gr!("1000c990", "100a67b4", DAT_100a67b4), 0);

        if !mech_trace::gr!("1000c990", "100a67cc", DAT_100a67cc).is_null() {
            item_index = 0;
            if 0 < mech_trace::rd!(
                "1000c990",
                mech_trace::gr!("1000c990", "100a67cc", DAT_100a67cc).add(1),
                read_unaligned
            ) {
                loop {
                    let next_index = item_index.wrapping_add(1);
                    let item = mech_trace::cr!(
                        "1000c990",
                        "10018060",
                        FUN_10018060(
                            mech_trace::gr!("1000c990", "100a67cc", DAT_100a67cc) as usize as i32,
                            item_index
                        )
                    ) as usize as *mut i32;
                    FUN_1000c770(item, 1);
                    item_index = next_index;
                    if !(next_index
                        < mech_trace::rd!(
                            "1000c990",
                            mech_trace::gr!("1000c990", "100a67cc", DAT_100a67cc).add(1),
                            read_unaligned
                        ))
                    {
                        break;
                    }
                }
            }
            FUN_100180f0(mech_trace::gr!("1000c990", "100a67cc", DAT_100a67cc), 0);
        }
    }
}
