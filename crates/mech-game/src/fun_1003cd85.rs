// SPDX-License-Identifier: MIT

use core::arch::asm;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aefc0: i32;
    static mut DAT_100af098: i32;
    static mut DAT_100af088: i32;
    static mut DAT_100af08c: i32;
    static mut DAT_100af090: i32;
    static mut DAT_100af094: i32;
    static mut DAT_10160c00: i32;
    static mut DAT_10160c04: i32;
    static mut DAT_10160e48: i32;
    static mut DAT_10160e50: i32;
    static mut DAT_10160e54: i32;
    static mut DAT_10160e58: i32;
    static mut DAT_10160e5c: i32;
    static mut DAT_10160e60: i32;
    static mut DAT_10160e64: i32;
    static mut DAT_10160e68: i32;
    static mut DAT_10160e70: i32;
    static mut DAT_100aefc8: i32;
    static mut DAT_100aefe8: i32;
    static mut DAT_100af028: i32;
    static mut DAT_100af068: i32;
    static mut DAT_100c6d5c: u8;
}

#[inline(always)]
unsafe fn rem_idiv(a: i32, b: i32) -> i32 {
    let r: i32;
    // SAFETY: the machine's own idiv; faults on zero divisor as the original does.
    unsafe {
        mech_trace::log::debug!(
            f = "1003cd85", b = "idiv", a:% = mech_trace::Args([
                mech_trace::__shown!(b),
                mech_trace::__shown!(a),
            ]); "asm.in"
        );
        asm!(
            "cdq",
            "idiv {d:e}",
            d = in(reg) b,
            inlateout("eax") a => _,
            out("edx") r,
        );
        mech_trace::log::debug!(
            f = "1003cd85", b = "idiv", r = "edx", v:% = mech_trace::__shown!(r); "asm.out"
        );
    }
    r
}

#[inline(always)]
unsafe fn cell(x: i32, y: i32) -> i32 {
    let off = (y as u32)
        .wrapping_mul(0x18)
        .wrapping_add((x as u32).wrapping_mul(0x120)) as i32;
    // SAFETY: the table is the game's own memory; the value is read where the original reads it.
    unsafe {
        mech_trace::rd!(
            "1003cd85",
            (&raw mut DAT_100c6d5c)
                .byte_offset(off as isize)
                .cast::<i32>(),
            read_unaligned
        )
    }
}

#[inline(always)]
unsafe fn has_item(p: i32) -> bool {
    // SAFETY: p is a pointer stored in the game's table.
    unsafe {
        mech_trace::rd!(
            "1003cd85",
            (p as u32 as usize as *const u8).byte_add(8).cast::<i32>(),
            read_unaligned
        ) != 0
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003cd85(param_1: *mut i32) {
    mech_trace::enter!("1003cd85", param_1);
    // SAFETY: globals and tables are the game's own; param_1 is the caller's
    // record, read where the original reads it.
    unsafe {
        let list0 = &raw mut DAT_10160c00;
        let list1 = &raw mut DAT_10160c04;
        let e70 = &raw mut DAT_10160e70;
        let mut local_18: i32 = 0;
        if mech_trace::gr!("1003cd85", "100aefc0", DAT_100aefc0) == 0 || param_1.is_null() {
            mech_trace::wr!(
                "1003cd85",
                list0,
                write_unaligned,
                mech_trace::gr!("1003cd85", "100af098", DAT_100af098)
            );
            mech_trace::wr!("1003cd85", list1, write_unaligned, 0);
            return;
        }
        let mut local_c: i32 =
            (mech_trace::rd!("1003cd85", param_1.add(3), read_unaligned) >> 0x10) % 0x168;
        if local_c < 0 {
            local_c = (local_c as u32).wrapping_add(0x168) as i32;
        }
        let u_var1: u32 = (((local_c as u32).wrapping_add(0x16) as i32 % 0x168) / 0x2d) as u32;
        let u_var2: u32 = ((u_var1 as i32) >> 0x1f) as u32;
        if (((u_var1 ^ u_var2).wrapping_sub(u_var2) & 1) ^ u_var2) == u_var2 {
            mech_trace::wr!("1003cd85", e70.add(2), write_unaligned, 3);
            mech_trace::wr!("1003cd85", e70.add(3), write_unaligned, 3);
        } else {
            mech_trace::wr!("1003cd85", e70.add(2), write_unaligned, 5);
            mech_trace::wr!("1003cd85", e70.add(3), write_unaligned, 2);
        }
        mech_trace::wr!("1003cd85", e70.add(1), write_unaligned, 4);
        mech_trace::wr!("1003cd85", e70, write_unaligned, 2);
        match u_var1 {
            0 => {
                mech_trace::gw!("1003cd85", "10160e5c", DAT_10160e5c, 0);
                mech_trace::gw!("1003cd85", "10160e50", DAT_10160e50, 0);
                mech_trace::gw!("1003cd85", "10160e54", DAT_10160e54, 0);
                mech_trace::gw!("1003cd85", "10160e58", DAT_10160e58, 1);
            }
            1 => {
                mech_trace::gw!("1003cd85", "10160e50", DAT_10160e50, 1);
                mech_trace::gw!("1003cd85", "10160e54", DAT_10160e54, 1);
                mech_trace::gw!("1003cd85", "10160e58", DAT_10160e58, 1);
                mech_trace::gw!("1003cd85", "10160e5c", DAT_10160e5c, 1);
            }
            2 => {
                mech_trace::gw!("1003cd85", "10160e50", DAT_10160e50, 1);
                mech_trace::gw!("1003cd85", "10160e54", DAT_10160e54, 2);
                mech_trace::gw!("1003cd85", "10160e58", DAT_10160e58, 3);
                mech_trace::gw!("1003cd85", "10160e5c", DAT_10160e5c, 1);
            }
            3 => {
                mech_trace::gw!("1003cd85", "10160e50", DAT_10160e50, 2);
                mech_trace::gw!("1003cd85", "10160e54", DAT_10160e54, 3);
                mech_trace::gw!("1003cd85", "10160e58", DAT_10160e58, 3);
                mech_trace::gw!("1003cd85", "10160e5c", DAT_10160e5c, 2);
            }
            4 => {
                mech_trace::gw!("1003cd85", "10160e50", DAT_10160e50, 2);
                mech_trace::gw!("1003cd85", "10160e54", DAT_10160e54, 4);
                mech_trace::gw!("1003cd85", "10160e58", DAT_10160e58, 5);
                mech_trace::gw!("1003cd85", "10160e5c", DAT_10160e5c, 2);
            }
            5 => {
                mech_trace::gw!("1003cd85", "10160e50", DAT_10160e50, 3);
                mech_trace::gw!("1003cd85", "10160e54", DAT_10160e54, 5);
                mech_trace::gw!("1003cd85", "10160e58", DAT_10160e58, 5);
                mech_trace::gw!("1003cd85", "10160e5c", DAT_10160e5c, 3);
            }
            6 => {
                mech_trace::gw!("1003cd85", "10160e50", DAT_10160e50, 3);
                mech_trace::gw!("1003cd85", "10160e54", DAT_10160e54, 6);
                mech_trace::gw!("1003cd85", "10160e58", DAT_10160e58, 7);
                mech_trace::gw!("1003cd85", "10160e5c", DAT_10160e5c, 3);
            }
            _ => {
                mech_trace::gw!("1003cd85", "10160e5c", DAT_10160e5c, 0);
                mech_trace::gw!("1003cd85", "10160e50", DAT_10160e50, 0);
                mech_trace::gw!("1003cd85", "10160e58", DAT_10160e58, 7);
                mech_trace::gw!("1003cd85", "10160e54", DAT_10160e54, 7);
            }
        }
        mech_trace::gw!(
            "1003cd85",
            "10160e60",
            DAT_10160e60,
            ((mech_trace::rd!("1003cd85", param_1, read_unaligned) as u32).wrapping_add(150000)
                as i32)
                / 25000
        );
        mech_trace::gw!(
            "1003cd85",
            "10160e68",
            DAT_10160e68,
            ((mech_trace::rd!("1003cd85", param_1.add(2), read_unaligned) as u32)
                .wrapping_add(150000) as i32)
                / 25000
        );
        mech_trace::gw!("1003cd85", "10160e64", DAT_10160e64, 0);
        mech_trace::gw!("1003cd85", "10160e48", DAT_10160e48, 0);
        let mut local_10: i32 = 0;
        let mut local_14: i32 = 0;
        while mech_trace::gr!("1003cd85", "10160e64", DAT_10160e64) < 4 {
            match mech_trace::gr!("1003cd85", "10160e64", DAT_10160e64) {
                0 => {
                    let idx = rem_idiv(
                        mech_trace::gr!("1003cd85", "10160e50", DAT_10160e50)
                            .wrapping_add(mech_trace::gr!("1003cd85", "10160e48", DAT_10160e48)),
                        mech_trace::gr!("1003cd85", "100af088", DAT_100af088),
                    );
                    let t = &raw mut DAT_100aefc8;
                    local_10 = mech_trace::rd!(
                        "1003cd85",
                        t.offset(idx.wrapping_mul(2) as isize),
                        read_unaligned
                    )
                    .wrapping_add(mech_trace::gr!(
                        "1003cd85",
                        "10160e60",
                        DAT_10160e60
                    ));
                    local_14 = mech_trace::rd!(
                        "1003cd85",
                        t.offset(idx.wrapping_mul(2).wrapping_add(1) as isize),
                        read_unaligned
                    )
                    .wrapping_add(mech_trace::gr!(
                        "1003cd85",
                        "10160e68",
                        DAT_10160e68
                    ));
                }
                1 => {
                    let idx = rem_idiv(
                        mech_trace::gr!("1003cd85", "10160e54", DAT_10160e54)
                            .wrapping_add(mech_trace::gr!("1003cd85", "10160e48", DAT_10160e48)),
                        mech_trace::gr!("1003cd85", "100af08c", DAT_100af08c),
                    );
                    let t = &raw mut DAT_100aefe8;
                    local_10 = mech_trace::rd!(
                        "1003cd85",
                        t.offset(idx.wrapping_mul(2) as isize),
                        read_unaligned
                    )
                    .wrapping_add(mech_trace::gr!(
                        "1003cd85",
                        "10160e60",
                        DAT_10160e60
                    ));
                    local_14 = mech_trace::rd!(
                        "1003cd85",
                        t.offset(idx.wrapping_mul(2).wrapping_add(1) as isize),
                        read_unaligned
                    )
                    .wrapping_add(mech_trace::gr!(
                        "1003cd85",
                        "10160e68",
                        DAT_10160e68
                    ));
                }
                2 => {
                    let idx = rem_idiv(
                        mech_trace::gr!("1003cd85", "10160e58", DAT_10160e58)
                            .wrapping_add(mech_trace::gr!("1003cd85", "10160e48", DAT_10160e48)),
                        mech_trace::gr!("1003cd85", "100af090", DAT_100af090),
                    );
                    let t = &raw mut DAT_100af028;
                    local_10 = mech_trace::rd!(
                        "1003cd85",
                        t.offset(idx.wrapping_mul(2) as isize),
                        read_unaligned
                    )
                    .wrapping_add(mech_trace::gr!(
                        "1003cd85",
                        "10160e60",
                        DAT_10160e60
                    ));
                    local_14 = mech_trace::rd!(
                        "1003cd85",
                        t.offset(idx.wrapping_mul(2).wrapping_add(1) as isize),
                        read_unaligned
                    )
                    .wrapping_add(mech_trace::gr!(
                        "1003cd85",
                        "10160e68",
                        DAT_10160e68
                    ));
                }
                3 => {
                    let idx = rem_idiv(
                        mech_trace::gr!("1003cd85", "10160e5c", DAT_10160e5c)
                            .wrapping_add(mech_trace::gr!("1003cd85", "10160e48", DAT_10160e48)),
                        mech_trace::gr!("1003cd85", "100af094", DAT_100af094),
                    );
                    let t = &raw mut DAT_100af068;
                    local_10 = mech_trace::rd!(
                        "1003cd85",
                        t.offset(idx.wrapping_mul(2) as isize),
                        read_unaligned
                    )
                    .wrapping_add(mech_trace::gr!(
                        "1003cd85",
                        "10160e60",
                        DAT_10160e60
                    ));
                    local_14 = mech_trace::rd!(
                        "1003cd85",
                        t.offset(idx.wrapping_mul(2).wrapping_add(1) as isize),
                        read_unaligned
                    )
                    .wrapping_add(mech_trace::gr!(
                        "1003cd85",
                        "10160e68",
                        DAT_10160e68
                    ));
                }
                _ => {}
            }
            mech_trace::gw!(
                "1003cd85",
                "10160e48",
                DAT_10160e48,
                mech_trace::gr!("1003cd85", "10160e48", DAT_10160e48).wrapping_add(1)
            );
            if mech_trace::rd!(
                "1003cd85",
                e70.offset(mech_trace::gr!("1003cd85", "10160e64", DAT_10160e64) as isize),
                read_unaligned
            ) <= mech_trace::gr!("1003cd85", "10160e48", DAT_10160e48)
            {
                mech_trace::gw!("1003cd85", "10160e48", DAT_10160e48, 0);
                mech_trace::gw!(
                    "1003cd85",
                    "10160e64",
                    DAT_10160e64,
                    mech_trace::gr!("1003cd85", "10160e64", DAT_10160e64).wrapping_add(1)
                );
            }
            if -1 < local_10 && local_10 < 0xc && -1 < local_14 && local_14 < 0xc {
                let p = cell(local_10, local_14);
                if has_item(p) {
                    let p2 = cell(local_10, local_14);
                    mech_trace::wr!(
                        "1003cd85",
                        list0.offset(local_18 as isize),
                        write_unaligned,
                        p2
                    );
                    mech_trace::wr!(
                        "1003cd85",
                        list1.offset(local_18 as isize),
                        write_unaligned,
                        0
                    );
                    local_18 = local_18.wrapping_add(1);
                }
            }
        }
        let p = cell(
            mech_trace::gr!("1003cd85", "10160e60", DAT_10160e60),
            mech_trace::gr!("1003cd85", "10160e68", DAT_10160e68),
        );
        if has_item(p) {
            let p2 = cell(
                mech_trace::gr!("1003cd85", "10160e60", DAT_10160e60),
                mech_trace::gr!("1003cd85", "10160e68", DAT_10160e68),
            );
            mech_trace::wr!(
                "1003cd85",
                list0.offset(local_18 as isize),
                write_unaligned,
                p2
            );
            mech_trace::wr!(
                "1003cd85",
                list1.offset(local_18 as isize),
                write_unaligned,
                0
            );
        }
    }
}
