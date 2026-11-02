// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf74: *mut core::ffi::c_void;
    static mut DAT_100b32b8: u32;
    static mut DAT_100b38fc: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapFree(h: *mut core::ffi::c_void, flags: u32, mem: *mut core::ffi::c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005d9bf(param_1: *mut i32, param_2: *mut u32) {
    mech_trace::enter!("1005d9bf", param_1, param_2);
    if param_2.is_null() {
        return;
    }
    // SAFETY: param_1 is the caller's list head and param_2 a node in that
    // list; the globals, the node's callback and HeapFree are the game's own.
    unsafe {
        if (mech_trace::rd!("1005d9bf", param_1, read_unaligned) as u32 as usize as *mut u32)
            == param_2
        {
            mech_trace::wr!(
                "1005d9bf",
                param_1,
                write_unaligned,
                mech_trace::rd!("1005d9bf", param_2.add(5), read_unaligned) as i32
            );
        } else {
            let mut local_8: i32 = mech_trace::rd!("1005d9bf", param_1, read_unaligned);
            while local_8 != 0 {
                let next = (local_8 as u32 as usize).wrapping_add(0x14) as *mut u32;
                if (mech_trace::rd!("1005d9bf", next, read_unaligned) as usize as *mut u32)
                    == param_2
                {
                    mech_trace::wr!(
                        "1005d9bf",
                        next,
                        write_unaligned,
                        mech_trace::rd!("1005d9bf", param_2.add(5), read_unaligned)
                    );
                    break;
                }
                local_8 = mech_trace::rd!("1005d9bf", next, read_unaligned) as i32;
            }
        }
        mech_trace::gw!(
            "1005d9bf",
            "100b32b8",
            DAT_100b32b8,
            param_2 as usize as u32
        );
        let cb: unsafe extern "C" fn(i32, i32, u32, u32) =
            core::mem::transmute(mech_trace::rd!("1005d9bf", param_2, read_unaligned) as usize);
        mech_trace::cr!(
            "1005d9bf",
            "ind",
            cb(
                2,
                0,
                mech_trace::gr!("1005d9bf", "100b38fc", DAT_100b38fc),
                mech_trace::rd!("1005d9bf", param_2.add(2), read_unaligned)
            )
        );
        mech_trace::cr!(
            "1005d9bf",
            "HeapFree",
            HeapFree(
                mech_trace::gr!("1005d9bf", "100adf74", DAT_100adf74),
                1,
                mech_trace::rd!("1005d9bf", param_2.add(1), read_unaligned) as usize
                    as *mut core::ffi::c_void,
            )
        );
    }
}
