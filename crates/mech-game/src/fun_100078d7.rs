// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a5534: *mut i32;
    static mut DAT_100a5538: i32;
    static mut DAT_100b38fc: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100079a6(param_1: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100078d7() {
    mech_trace::enter!("100078d7");
    // SAFETY: the globals and callee are the game's own; the pointer chain
    // is the game's linked list, read and written where the original does.
    unsafe {
        if !mech_trace::gr!("100078d7", "100a5534", DAT_100a5534).is_null() {
            let mut b_var1: i32 = 0;
            let limit = mech_trace::rd!(
                "100078d7",
                mech_trace::gr!("100078d7", "100a5534", DAT_100a5534).byte_add(0x14),
                read_unaligned
            );
            if limit == 0 {
                if mech_trace::gr!("100078d7", "100a5538", DAT_100a5538) == 0 {
                    b_var1 = 1;
                }
            } else {
                b_var1 = (mech_trace::gr!("100078d7", "100b38fc", DAT_100b38fc) < limit) as i32;
            }
            if b_var1 != 0 {
                b_var1 = 0;
                loop {
                    mech_trace::wr!(
                        "100078d7",
                        mech_trace::gr!("100078d7", "100a5534", DAT_100a5534),
                        write_unaligned,
                        0
                    );
                    let next_ptr: i32 = mech_trace::rd!(
                        "100078d7",
                        mech_trace::gr!("100078d7", "100a5534", DAT_100a5534)
                            .byte_add(0x4f)
                            .cast::<i32>(),
                        read_unaligned
                    );
                    mech_trace::gw!(
                        "100078d7",
                        "100a5534",
                        DAT_100a5534,
                        next_ptr as usize as *mut i32
                    );
                    if !mech_trace::gr!("100078d7", "100a5534", DAT_100a5534).is_null()
                        && mech_trace::cr!(
                            "100078d7",
                            "100079a6",
                            FUN_100079a6(mech_trace::gr!("100078d7", "100a5534", DAT_100a5534)
                                as usize as i32)
                        ) != 0
                    {
                        b_var1 = 1;
                    }
                    if !(!mech_trace::gr!("100078d7", "100a5534", DAT_100a5534).is_null()
                        && b_var1 == 0)
                    {
                        break;
                    }
                }
            }
        }
    }
}
