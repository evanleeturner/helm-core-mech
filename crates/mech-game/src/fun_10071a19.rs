// SPDX-License-Identifier: MIT
use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf74: *mut c_void;
    static mut DAT_100b9a18: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapFree(h: *mut c_void, flags: u32, p: *mut c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10071a19(param_1: *mut i32) {
    mech_trace::enter!("10071a19", param_1);
    // SAFETY: param_1 points at the game's own record; the buffers, globals
    // and callee are the game's own, accessed in the original's order.
    unsafe {
        if mech_trace::rd!("10071a19", param_1.add(4), read_unaligned)
            < mech_trace::rd!("10071a19", param_1.add(3), read_unaligned)
        {
            let mut local_10: i32 = 0;
            let mut local_c: *mut i16 =
                mech_trace::rd!("10071a19", param_1.add(2), read_unaligned) as *mut i16;
            let mut local_14: *const i8 =
                mech_trace::rd!("10071a19", param_1.add(1), read_unaligned) as *const i8;
            let mut local_8: *mut i8 = (u32::from(mech_trace::rd!(
                "10071a19",
                param_1.byte_add(0x14).cast::<u8>(),
                read
            ))
            .wrapping_add(mech_trace::rd!("10071a19", param_1, read_unaligned) as u32))
                as *mut i8;
            while local_10
                < mech_trace::rd!(
                    "10071a19",
                    param_1.byte_add(0x15).cast::<i32>(),
                    read_unaligned
                )
                .wrapping_mul(3)
            {
                let v = (mech_trace::rd!("10071a19", local_14, read) as i16)
                    .wrapping_add(mech_trace::rd!("10071a19", local_c, read_unaligned));
                mech_trace::wr!("10071a19", local_c, write_unaligned, v);
                let n = mech_trace::rd!("10071a19", local_c, read_unaligned) as i32;
                let d = mech_trace::rd!("10071a19", param_1.add(3), read_unaligned);
                let q: i32;
                let r: i32;
                core::arch::asm!(
                    "idiv {d:e}",
                    d = in(reg) d,
                    inout("eax") n => q,
                    inout("edx") (n >> 31) => r,
                );
                mech_trace::wr!(
                    "10071a19",
                    local_8,
                    write,
                    (q as i8).wrapping_add(mech_trace::rd!("10071a19", local_8, read))
                );
                mech_trace::wr!("10071a19", local_c, write_unaligned, r as i16);
                local_c = local_c.add(1);
                local_14 = local_14.add(1);
                local_8 = local_8.add(1);
                local_10 = local_10.wrapping_add(1);
            }
            let tbl = mech_trace::gr!("10071a19", "100b9a18", DAT_100b9a18).wrapping_add(0x18)
                as *const u32;
            let f: unsafe extern "C" fn(i32) = core::mem::transmute::<u32, unsafe extern "C" fn(i32)>(
                mech_trace::rd!("10071a19", tbl, read_unaligned),
            );
            mech_trace::cr!(
                "10071a19",
                "ind",
                f(mech_trace::rd!("10071a19", param_1, read_unaligned))
            );
            mech_trace::wr!(
                "10071a19",
                param_1.add(4),
                write_unaligned,
                mech_trace::rd!("10071a19", param_1.add(4), read_unaligned).wrapping_add(1)
            );
            if mech_trace::rd!("10071a19", param_1.add(3), read_unaligned)
                <= mech_trace::rd!("10071a19", param_1.add(4), read_unaligned)
                && mech_trace::rd!("10071a19", param_1, read_unaligned) != 0
            {
                if mech_trace::rd!("10071a19", param_1, read_unaligned) != 0 {
                    mech_trace::cr!(
                        "10071a19",
                        "HeapFree",
                        HeapFree(
                            mech_trace::gr!("10071a19", "100adf74", DAT_100adf74),
                            1,
                            mech_trace::rd!("10071a19", param_1, read_unaligned) as *mut c_void
                        )
                    );
                    mech_trace::wr!("10071a19", param_1, write_unaligned, 0);
                }
                if mech_trace::rd!("10071a19", param_1.add(1), read_unaligned) != 0 {
                    mech_trace::wr!("10071a19", param_1.add(1), write_unaligned, 0);
                }
                if mech_trace::rd!("10071a19", param_1.add(2), read_unaligned) != 0 {
                    mech_trace::wr!("10071a19", param_1.add(2), write_unaligned, 0);
                }
            }
        }
    }
}
