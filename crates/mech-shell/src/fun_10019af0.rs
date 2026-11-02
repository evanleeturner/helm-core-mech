// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut LAB_10019f40: u8;
    static mut LAB_10019ff0: u8;
    static mut LAB_1001a0b0: u8;
    static mut LAB_1001a100: u8;
    static mut LAB_1001a150: u8;
    static mut LAB_1001a1a0: u8;
    static mut LAB_1001a200: u8;
    static mut LAB_1001a260: u8;
    static mut LAB_1001a2c0: u8;
    static mut LAB_1001a320: u8;
    static mut LAB_1001a3d0: u8;
    static mut LAB_1001a430: u8;
    static mut DAT_1008f624: u8;
    static mut DAT_1008f628: u8;
    static mut DAT_10098224: *mut c_void;
    static mut DAT_100a6744: u32;
    static mut DAT_100c49d0: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10017ff0(param_1: i32) -> u8;
    fn FUN_1000cd20(param_1: i32) -> i32;
    fn FUN_10018060(param_1: i32, param_2: i32) -> u32;
    fn FUN_10050490(param_1: i32) -> u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapAlloc(h: *mut c_void, flags: u32, size: u32) -> *mut u32;
}

#[inline(always)]
unsafe fn w(p: *mut u32, i: usize, v: u32) {
    // SAFETY: p points into the heap block sized by the caller.
    unsafe { mech_trace::wr!("10019af0", p.wrapping_add(i), write_unaligned, v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10019af0(param_1: i32) -> *mut u32 {
    mech_trace::enter!("10019af0", param_1);
    if param_1 == 0 {
        return mech_trace::rt!("10019af0", core::ptr::null_mut());
    }
    // SAFETY: all globals and callees are the game's own; param_1 and the
    // items it yields point at the game's structures, accessed as the original does.
    unsafe {
        let cnt = |_: ()| {
            mech_trace::rd!(
                "10019af0",
                ((param_1 as u32).wrapping_add(4) as *const i32),
                read_unaligned
            )
        };
        mech_trace::cr!("10019af0", "10017ff0", FUN_10017ff0(param_1));
        let i_var3 = mech_trace::cr!("10019af0", "1000cd20", FUN_1000cd20(param_1));
        let p4 = mech_trace::cr!(
            "10019af0",
            "HeapAlloc",
            HeapAlloc(
                mech_trace::gr!("10019af0", "10098224", DAT_10098224),
                1,
                (i_var3 as u32).wrapping_mul(0xa8).wrapping_add(0x118),
            )
        );
        let dat = (&raw mut DAT_100c49d0) as usize as u32;
        let lab = |p: *mut u8| p as usize as u32;
        w(p4, 0, 0x213);
        w(p4, 7, lab(&raw mut LAB_1001a3d0));
        w(p4, 1, 0x5a);
        w(p4, 2, 0xffffffff);
        w(p4, 3, 0xffffffff);
        w(p4, 4, 0);
        let mut local_c: i32 = 2;
        w(p4, 5, dat);
        w(p4, 6, 0);
        w(p4, 8, 0);
        w(p4, 9, 0xffffffff);
        w(p4, 10, 0xffffffff);
        w(p4, 0xb, 0xffffffff);
        let i_var3 = mech_trace::rd!(
            "10019af0",
            ((mech_trace::gr!("10019af0", "100a6744", DAT_100a6744).wrapping_add(0x40c))
                as *const i32),
            read_unaligned
        );
        w(p4, 0xe, 0x52);
        let i_var3 = i_var3.wrapping_mul(2).wrapping_add(0x5a);
        w(p4, 0xf, i_var3 as u32);
        w(p4, 0x10, 0xffffffff);
        w(p4, 0x11, 0xffffffff);
        w(p4, 0x12, 0);
        w(p4, 0x13, dat);
        w(p4, 0x14, 0);
        w(p4, 0x16, 0);
        w(p4, 0x17, 0xffffffff);
        w(p4, 0x18, 0xffffffff);
        w(p4, 0x19, 0xffffffff);
        let c24 = mech_trace::gr!("10019af0", "1008f624", DAT_1008f624);
        if cnt(()) == 0 {
            w(p4, 0x15, lab(&raw mut LAB_1001a430));
        } else {
            if mech_trace::gr!("10019af0", "1008f628", DAT_1008f628) == 1 {
                w(p4, 0x15, lab(&raw mut LAB_1001a0b0));
            } else {
                w(p4, 0x15, lab(&raw mut LAB_1001a150));
                if c24 != 1 {
                    w(p4, 0x15, lab(&raw mut LAB_1001a100));
                }
            }
            w(p4, 0x1c, 0xb6);
            w(p4, 0x1d, i_var3 as u32);
            w(p4, 0x1e, 0xffffffff);
            w(p4, 0x1f, 0xffffffff);
            w(p4, 0x20, 0);
            w(p4, 0x21, dat);
            w(p4, 0x22, 0);
            w(p4, 0x24, 0);
            w(p4, 0x25, 0xffffffff);
            w(p4, 0x26, 0xffffffff);
            w(p4, 0x27, 0xffffffff);
            let c28 = mech_trace::gr!("10019af0", "1008f628", DAT_1008f628);
            w(p4, 0x23, lab(&raw mut LAB_1001a1a0));
            if c28 != 1 {
                w(p4, 0x23, lab(&raw mut LAB_1001a200));
            }
            local_c = 4;
            w(p4, 0x2a, 0xf2);
            w(p4, 0x2b, i_var3 as u32);
            w(p4, 0x2c, 0xffffffff);
            w(p4, 0x2d, 0xffffffff);
            w(p4, 0x2e, 0);
            w(p4, 0x2f, dat);
            w(p4, 0x30, 0);
            w(p4, 0x32, 0);
            w(p4, 0x33, 0xffffffff);
            w(p4, 0x34, 0xffffffff);
            w(p4, 0x35, 0xffffffff);
            let c28 = mech_trace::gr!("10019af0", "1008f628", DAT_1008f628);
            w(p4, 0x31, lab(&raw mut LAB_1001a260));
            if c28 != 1 {
                w(p4, 0x31, lab(&raw mut LAB_1001a2c0));
            }
        }
        let mut i_var5: i32 = 0;
        if 0 < cnt(()) {
            loop {
                let i_var9 = i_var5.wrapping_add(1);
                let it = mech_trace::cr!("10019af0", "10018060", FUN_10018060(param_1, i_var5));
                mech_trace::wr!("10019af0", (it.wrapping_add(9) as *mut u8), write, 0);
                i_var5 = i_var9;
                if !(i_var9 < cnt(())) {
                    break;
                }
            }
        }
        let mut local_14: i32 = 0;
        let mut local_4: i32 = 0;
        let mut local_8: i32 = local_c.wrapping_sub(1);
        if 0 < cnt(()) {
            loop {
                let pv6 = mech_trace::cr!("10019af0", "10018060", FUN_10018060(param_1, local_4))
                    as *mut i32;
                if mech_trace::rd!("10019af0", (pv6 as *mut u8).wrapping_add(9), read) != 1 {
                    let mut i_var5: i32 = 1;
                    mech_trace::wr!("10019af0", (pv6 as *mut u8).wrapping_add(9), write, 1);
                    let mut i_var9 = local_4.wrapping_add(1);
                    if i_var9 < cnt(()) {
                        loop {
                            let pv7 = mech_trace::cr!(
                                "10019af0",
                                "10018060",
                                FUN_10018060(param_1, i_var9)
                            ) as *mut i32;
                            if mech_trace::rd!("10019af0", pv7, read_unaligned)
                                == mech_trace::rd!("10019af0", pv6, read_unaligned)
                            {
                                mech_trace::wr!(
                                    "10019af0",
                                    (pv7 as *mut u8).wrapping_add(9),
                                    write,
                                    1
                                );
                                i_var5 = i_var5.wrapping_add(1);
                            }
                            i_var9 = i_var9.wrapping_add(1);
                            if !(i_var9 < cnt(())) {
                                break;
                            }
                        }
                    }
                    let h = mech_trace::rd!(
                        "10019af0",
                        ((mech_trace::gr!("10019af0", "100a6744", DAT_100a6744).wrapping_add(0x40c))
                            as *const i32),
                        read_unaligned
                    );
                    let y = h.wrapping_mul(local_8).wrapping_add(i_var3) as u32;

                    let b = (local_c.wrapping_add(local_14) as usize).wrapping_mul(0xe);
                    w(p4, b, 0x52);
                    w(p4, b + 1, y);
                    w(p4, b + 2, 0xd2);
                    w(p4, b + 3, 0xffffffff);
                    w(p4, b + 4, 0);
                    w(p4, b + 5, dat);
                    w(p4, b + 6, 0);
                    w(p4, b + 7, lab(&raw mut LAB_1001a320));
                    w(
                        p4,
                        b + 9,
                        mech_trace::rd!("10019af0", pv6, read_unaligned) as u32,
                    );
                    w(p4, b + 10, i_var5 as u32);
                    w(p4, b + 0xb, 0xffffffff);
                    w(p4, b + 8, 0);
                    let u =
                        mech_trace::cr!("10019af0", "10050490", FUN_10050490(pv6 as usize as i32));
                    w(p4, b + 0xc, u);

                    let h = mech_trace::rd!(
                        "10019af0",
                        ((mech_trace::gr!("10019af0", "100a6744", DAT_100a6744).wrapping_add(0x40c))
                            as *const i32),
                        read_unaligned
                    );
                    let y = h.wrapping_mul(local_8).wrapping_add(i_var3) as u32;
                    let b =
                        (local_c.wrapping_add(local_14).wrapping_add(1) as usize).wrapping_mul(0xe);
                    w(p4, b, 0xb6);
                    w(p4, b + 1, y);
                    w(p4, b + 2, 0xffffffff);
                    w(p4, b + 3, 0xffffffff);
                    w(p4, b + 4, 0);
                    w(p4, b + 5, dat);
                    w(p4, b + 6, 0);
                    w(p4, b + 7, lab(&raw mut LAB_10019f40));
                    w(p4, b + 9, 0xffffffff);
                    w(p4, b + 10, i_var5 as u32);
                    w(p4, b + 0xb, 0xffffffff);
                    w(p4, b + 0xd, 0xffffffff);
                    w(p4, b + 8, 0);
                    let u =
                        mech_trace::cr!("10019af0", "10050490", FUN_10050490(pv6 as usize as i32));
                    w(p4, b + 0xc, u);

                    let h = mech_trace::rd!(
                        "10019af0",
                        ((mech_trace::gr!("10019af0", "100a6744", DAT_100a6744).wrapping_add(0x40c))
                            as *const i32),
                        read_unaligned
                    );
                    let y = h.wrapping_mul(local_8).wrapping_add(i_var3) as u32;
                    let b =
                        (local_c.wrapping_add(local_14).wrapping_add(2) as usize).wrapping_mul(0xe);
                    w(p4, b, 0xf2);
                    w(p4, b + 1, y);
                    w(p4, b + 2, 0xffffffff);
                    w(p4, b + 3, 0xffffffff);
                    w(p4, b + 4, 0);
                    w(p4, b + 5, dat);
                    w(p4, b + 6, 0);
                    w(p4, b + 7, lab(&raw mut LAB_10019ff0));
                    w(p4, b + 9, 0xffffffff);
                    w(p4, b + 10, 0xffffffff);
                    w(p4, b + 0xb, 0xffffffff);
                    w(p4, b + 0xd, 0xffffffff);
                    w(p4, b + 8, 0);
                    let u =
                        mech_trace::cr!("10019af0", "10050490", FUN_10050490(pv6 as usize as i32));
                    local_14 = local_14.wrapping_add(3);
                    local_8 = local_8.wrapping_add(1);
                    w(p4, b + 0xc, u);
                }
                local_4 = local_4.wrapping_add(1);
                if !(local_4 < cnt(())) {
                    break;
                }
            }
        }
        w(
            p4,
            (local_c.wrapping_add(local_14) as usize).wrapping_mul(0xe),
            0xffffffff,
        );
        mech_trace::rt!("10019af0", p4)
    }
}
