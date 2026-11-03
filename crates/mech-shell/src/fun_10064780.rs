// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10098224: *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10018060(param_1: i32, param_2: i32) -> u32;
    fn FUN_1006ec30(param_1: *mut core::ffi::c_void) -> u8;
}

#[allow(non_snake_case)]
unsafe extern "fastcall" {
    fn FUN_10064c50(param_1: i32) -> u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapFree(h: *mut core::ffi::c_void, flags: u32, mem: *mut core::ffi::c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "fastcall" fn FUN_10064780(param_1: *mut i32) {
    mech_trace::enter!("10064780", param_1);
    // SAFETY: param_1 points at the game's object whose first field is a
    // pointer to a block; the callees and globals are the game's own.
    unsafe {
        let mut i_var2: i32 = 0;
        if 0 < mech_trace::rd!(
            "10064780",
            (mech_trace::rd!("10064780", param_1, read_unaligned) as *const i32).byte_add(4),
            read_unaligned
        ) {
            loop {
                let pv_var1 = mech_trace::cr!(
                    "10064780",
                    "10018060",
                    FUN_10018060(mech_trace::rd!("10064780", param_1, read_unaligned), i_var2)
                ) as *mut core::ffi::c_void;
                if !pv_var1.is_null() {
                    mech_trace::cr!("10064780", "10064c50", FUN_10064c50(pv_var1 as i32));
                    mech_trace::cr!("10064780", "1006ec30", FUN_1006ec30(pv_var1));
                }
                i_var2 = i_var2.wrapping_add(1);
                if !(i_var2
                    < mech_trace::rd!(
                        "10064780",
                        (mech_trace::rd!("10064780", param_1, read_unaligned) as *const i32)
                            .byte_add(4),
                        read_unaligned
                    ))
                {
                    break;
                }
            }
        }
        let p11 = (mech_trace::rd!("10064780", param_1, read_unaligned) as *mut u8)
            .byte_add(0x11)
            .cast::<*mut core::ffi::c_void>();
        if !mech_trace::rd!("10064780", p11, read_unaligned).is_null() {
            mech_trace::cr!(
                "10064780",
                "HeapFree",
                HeapFree(
                    mech_trace::gr!("10064780", "10098224", DAT_10098224),
                    1,
                    mech_trace::rd!("10064780", p11, read_unaligned)
                )
            );
            let p11b = (mech_trace::rd!("10064780", param_1, read_unaligned) as *mut u8)
                .byte_add(0x11)
                .cast::<u32>();
            mech_trace::wr!("10064780", p11b, write_unaligned, 0);
        }
        if mech_trace::rd!("10064780", param_1, read_unaligned) != 0 {
            mech_trace::cr!(
                "10064780",
                "HeapFree",
                HeapFree(
                    mech_trace::gr!("10064780", "10098224", DAT_10098224),
                    1,
                    mech_trace::rd!("10064780", param_1, read_unaligned) as *mut core::ffi::c_void,
                )
            );
            mech_trace::wr!("10064780", param_1, write_unaligned, 0);
        }
    }
}
