// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ada48: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1008975d(param_1: *mut u32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002f19a(param_1: i32, param_2: *mut i32) -> *mut i32 {
    mech_trace::enter!("1002f19a", param_1, param_2);
    // SAFETY: param_1 is zero or the address of a valid parent node, param_2
    // points at a caller-owned node; the callee and the global are the game's own.
    unsafe {
        mech_trace::wr!("1002f19a", param_2, write_unaligned, param_1);
        mech_trace::wr!("1002f19a", param_2.add(1), write_unaligned, 0);
        if param_1 == 0 {
            mech_trace::wr!("1002f19a", param_2.add(2), write_unaligned, 0);
        } else {
            let p1 = param_1 as *mut i32;
            mech_trace::wr!(
                "1002f19a",
                param_2.add(2),
                write_unaligned,
                mech_trace::rd!("1002f19a", p1.add(1), read_unaligned)
            );
            mech_trace::wr!("1002f19a", p1.add(1), write_unaligned, param_2 as i32);
        }
        mech_trace::cr!(
            "1002f19a",
            "1008975d",
            FUN_1008975d(param_2.add(3).cast::<u32>())
        );
        if param_1 == 0 {
            mech_trace::cr!(
                "1002f19a",
                "1008975d",
                FUN_1008975d(param_2.add(0xf).cast::<u32>())
            );
        } else {
            let mut src = (param_1 as usize).wrapping_add(0x3c) as *const i32;
            let mut dst = param_2.add(0xf);
            let mut n: i32 = 0xc;
            while n != 0 {
                mech_trace::wr!(
                    "1002f19a",
                    dst,
                    write_unaligned,
                    mech_trace::rd!("1002f19a", src, read_unaligned)
                );
                src = src.add(1);
                dst = dst.add(1);
                n -= 1;
            }
        }
        mech_trace::wr!("1002f19a", param_2.add(0x1b), write_unaligned, 0);
        mech_trace::wr!("1002f19a", param_2.add(0x1d), write_unaligned, 1);
        mech_trace::wr!("1002f19a", param_2.add(0x1c), write_unaligned, 0);
        mech_trace::wr!(
            "1002f19a",
            param_2.add(0x1e),
            write_unaligned,
            mech_trace::gr!("1002f19a", "100ada48", DAT_100ada48) as i32
        );
        mech_trace::gw!(
            "1002f19a",
            "100ada48",
            DAT_100ada48,
            (mech_trace::gr!("1002f19a", "100ada48", DAT_100ada48).wrapping_add(7) & 0x7f)
                .wrapping_add(0x40)
        );
    }
    mech_trace::rt!("1002f19a", param_2)
}
