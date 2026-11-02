// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a40a4: u32;
    static mut DAT_100a40b0: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10083a59(
        param_1: u32,
        param_2: u32,
        param_3: u32,
        param_4: *mut i32,
        param_5: *mut u32,
        param_6: *mut u32,
        param_7: *mut i32,
    ) -> u8;
    fn FUN_10003cf8(param_1: i32) -> i32;
    fn FUN_10003d60(param_1: i32, param_2: i32) -> i32;
    fn FUN_10003783(
        param_1: i32,
        param_2: u32,
        param_3: u32,
        param_4: i32,
        param_5: u32,
        param_6: u16,
    ) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10003686(
    param_1: u32,
    param_2: u32,
    param_3: u32,
    param_4: u32,
    param_5: i32,
    param_6: u16,
) -> i32 {
    mech_trace::enter!(
        "10003686", param_1, param_2, param_3, param_4, param_5, param_6
    );
    let mut local_1c: i32 = 0;
    let mut local_18: i32 = 0;
    let mut local_10: i32 = 0;
    let mut local_8: u32 = 0;

    // SAFETY: the callees and globals are the game's own; the locals passed
    // by address are live for the calls.
    unsafe {
        mech_trace::cr!(
            "10003686",
            "10083a59",
            FUN_10083a59(
                param_1,
                param_2,
                param_3,
                &raw mut local_18,
                (&raw mut local_1c).cast::<u32>(),
                &raw mut local_8,
                &raw mut local_10,
            )
        );
        if (local_1c / 100 < mech_trace::gr!("10003686", "100a40b0", DAT_100a40b0) as i32)
            || (mech_trace::gr!("10003686", "100a40b0", DAT_100a40b0) == 0)
        {
            let mut local_14 =
                mech_trace::cr!("10003686", "10003cf8", FUN_10003cf8(local_1c / 100));
            if 0 < local_14 {
                if param_5 != 0 {
                    local_14 >>= 1;
                }
                let local_c = mech_trace::cr!(
                    "10003686",
                    "10003d60",
                    FUN_10003d60(
                        local_1c,
                        mech_trace::gr!("10003686", "100a40a4", DAT_100a40a4) as i32
                    )
                );
                mech_trace::cr!(
                    "10003686",
                    "10003783",
                    FUN_10003783(
                        local_c,
                        local_18 as u32,
                        param_4,
                        local_14,
                        0xffff_ffff,
                        param_6,
                    )
                );
            }
        }
    }
    mech_trace::rt!("10003686", local_1c)
}
