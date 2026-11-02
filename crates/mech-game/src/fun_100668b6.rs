// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf74: *mut c_void;
    static mut DAT_100b3940: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapAlloc(h: *mut c_void, flags: u32, size: u32) -> *mut u8;
    fn HeapFree(h: *mut c_void, flags: u32, p: *mut c_void) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100660e1(param_1: i32, param_2: *mut i32, param_3: *mut i32) -> *mut i32;
    fn FUN_1005aae4(param_1: i32) -> u32;
    fn FUN_1005aaf7(param_1: i32, param_2: i32) -> u32;
    fn FUN_1005acaa(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: *mut u8,
        param_6: i32,
    ) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100668b6(param_1: *mut i32, param_2: *const i8, param_3: i32) {
    mech_trace::enter!("100668b6", param_1, param_2, param_3);
    if param_1.is_null() || param_2.is_null() || param_3 == 0 {
        return;
    }
    // SAFETY: the pointers are the caller's valid arguments; the globals and
    // callees are the game's own; the heap buffer is sized for the string copy.
    unsafe {
        let mut len: u32 = 0;
        while mech_trace::dr!("100668b6", param_2.add(len as usize)) != 0 {
            len = len.wrapping_add(1);
        }
        let local_20 = len as i32;
        if local_20 == 0 {
            return;
        }
        let local_14 = mech_trace::cr!(
            "100668b6",
            "HeapAlloc",
            HeapAlloc(
                mech_trace::gr!("100668b6", "100adf74", DAT_100adf74),
                1,
                len.wrapping_add(1)
            )
        );
        let local_24 = local_14.wrapping_add(local_20 as usize);
        let mut i: usize = 0;
        while i <= len as usize {
            mech_trace::dw!(
                "100668b6",
                local_14.add(i),
                mech_trace::dr!("100668b6", param_2.add(i)) as u8
            );
            i += 1;
        }
        let mut local_10: *mut u8 = local_14;
        // Two adjacent ints: local_2c (x) then local_28 (y).
        let mut pt: [i32; 2] = [0, 0];
        mech_trace::cr!(
            "100668b6",
            "100660e1",
            FUN_100660e1(
                param_1 as i32,
                (&raw mut DAT_100b3940) as *mut i32,
                pt.as_mut_ptr(),
            )
        );
        let local_c = (mech_trace::dr!("100668b6", param_1.add(3)))
            .wrapping_sub(mech_trace::dr!("100668b6", param_1.add(1)));
        let local_8 = (mech_trace::dr!("100668b6", param_1.add(4)))
            .wrapping_sub(mech_trace::dr!("100668b6", param_1.add(2)));
        let local_18 = mech_trace::cr!("100668b6", "1005aae4", FUN_1005aae4(param_3)) as i32;
        loop {
            let pbvar5 = local_10;
            if !(local_10 <= local_24 && pt[1] < local_8) {
                break;
            }
            let mut local_34 = pt[0];
            while local_10 <= local_24 && mech_trace::dr!("100668b6", local_10) != 10 {
                let iv = mech_trace::cr!(
                    "100668b6",
                    "1005aaf7",
                    FUN_1005aaf7(
                        param_3,
                        (mech_trace::dr!("100668b6", local_10) as i8) as i32
                    )
                ) as i32;
                local_34 = local_34.wrapping_add(iv);
                if local_c < local_34 {
                    break;
                }
                local_10 = local_10.add(1);
            }
            let pbvar7 = local_10;
            if local_10 <= local_24 {
                if mech_trace::dr!("100668b6", local_10) == 10 {
                    mech_trace::dw!("100668b6", local_10, 0);
                } else {
                    while pbvar5 <= local_10
                        && mech_trace::dr!("100668b6", local_10) != 0x20
                        && mech_trace::dr!("100668b6", local_10) != 9
                    {
                        local_10 = local_10.wrapping_sub(1);
                    }
                    if local_10 < pbvar5 {
                        mech_trace::dw!("100668b6", pbvar7, 0);
                        local_10 = pbvar7;
                    } else {
                        mech_trace::dw!("100668b6", local_10, 0);
                    }
                }
            }
            if mech_trace::dr!("100668b6", pbvar5) != 0 {
                mech_trace::cr!(
                    "100668b6",
                    "1005acaa",
                    FUN_1005acaa(param_1, pt[0], pt[1], param_3, pbvar5, 0x100eebb0)
                );
            }
            local_10 = local_10.wrapping_add(1);
            pt[1] = pt[1].wrapping_add(local_18);
        }
        mech_trace::cr!(
            "100668b6",
            "HeapFree",
            HeapFree(
                mech_trace::gr!("100668b6", "100adf74", DAT_100adf74),
                1,
                local_14 as *mut c_void
            )
        );
    }
}
