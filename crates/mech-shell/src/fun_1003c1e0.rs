// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut LAB_1003c460: [u8; 1];
    static mut LAB_1003c4b0: [u8; 1];
    static mut LAB_1003c510: [u8; 1];
    static mut LAB_1003c560: [u8; 1];
    static mut LAB_1003c5e0: [u8; 1];
    static mut LAB_1003c680: [u8; 1];
    static mut DAT_10098224: *mut u8;
    static mut DAT_100a6744: u32;
    static mut DAT_100a67b4: u32;
    static mut DAT_100c48d0: [u8; 1];
    static mut DAT_100c49d0: [u8; 1];
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapAlloc(h: *mut u8, flags: u32, size: u32) -> *mut u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003c1e0(param_1: i32) -> *mut u32 {
    mech_trace::enter!("1003c1e0", param_1);
    if param_1 == 0 {
        return mech_trace::rt!("1003c1e0", core::ptr::null_mut());
    }
    // SAFETY: param_1 points at the game's structure; the globals and the
    // heap block are the game's own memory, accessed where the original does.
    unsafe {
        let mut i_var5: i32 = 0;
        let mut pi_var4 = (param_1.wrapping_add(0x2f0)) as *const i32;
        let mut i_var2: i32 = 0xe;
        loop {
            if -1 < mech_trace::rd!("1003c1e0", pi_var4, read_unaligned) {
                i_var5 = i_var5.wrapping_add(1);
            }
            pi_var4 = pi_var4.add(2);
            i_var2 = i_var2.wrapping_sub(1);
            if i_var2 == 0 {
                break;
            }
        }
        let p3 = mech_trace::cr!(
            "1003c1e0",
            "HeapAlloc",
            HeapAlloc(
                mech_trace::gr!("1003c1e0", "10098224", DAT_10098224),
                1,
                (i_var5.wrapping_add(2)).wrapping_mul(0x70) as u32,
            )
        );
        mech_trace::wr!("1003c1e0", p3, write_unaligned, 0x172);
        let mut local_10: i32 = 1;
        mech_trace::wr!("1003c1e0", p3.add(1), write_unaligned, 0x59);
        mech_trace::wr!("1003c1e0", p3.add(2), write_unaligned, 0xffffffff);
        mech_trace::wr!("1003c1e0", p3.add(3), write_unaligned, 0xffffffff);
        mech_trace::wr!("1003c1e0", p3.add(4), write_unaligned, 0);
        mech_trace::wr!(
            "1003c1e0",
            p3.add(5),
            write_unaligned,
            (&raw mut DAT_100c48d0) as u32
        );
        mech_trace::wr!("1003c1e0", p3.add(6), write_unaligned, 0);
        mech_trace::wr!("1003c1e0", p3.add(8), write_unaligned, 0);
        mech_trace::wr!("1003c1e0", p3.add(9), write_unaligned, 0xffffffff);
        mech_trace::wr!("1003c1e0", p3.add(10), write_unaligned, 0xffffffff);
        mech_trace::wr!("1003c1e0", p3.add(0xb), write_unaligned, 0xffffffff);
        if mech_trace::rd!(
            "1003c1e0",
            ((mech_trace::gr!("1003c1e0", "100a67b4", DAT_100a67b4).wrapping_add(4)) as *const i32),
            read_unaligned
        ) == 0
        {
            mech_trace::wr!(
                "1003c1e0",
                p3.add(7),
                write_unaligned,
                (&raw mut LAB_1003c510) as u32
            );
        } else {
            mech_trace::wr!(
                "1003c1e0",
                p3.add(7),
                write_unaligned,
                (&raw mut LAB_1003c460) as u32
            );
            local_10 = 2;
            mech_trace::wr!(
                "1003c1e0",
                p3.add(0x15),
                write_unaligned,
                (&raw mut LAB_1003c4b0) as u32
            );
            mech_trace::wr!("1003c1e0", p3.add(0xe), write_unaligned, 0x22d);
            mech_trace::wr!("1003c1e0", p3.add(0xf), write_unaligned, 0x59);
            mech_trace::wr!("1003c1e0", p3.add(0x10), write_unaligned, 0xffffffff);
            mech_trace::wr!("1003c1e0", p3.add(0x11), write_unaligned, 0xffffffff);
            mech_trace::wr!("1003c1e0", p3.add(0x12), write_unaligned, 0);
            mech_trace::wr!(
                "1003c1e0",
                p3.add(0x13),
                write_unaligned,
                (&raw mut DAT_100c48d0) as u32
            );
            mech_trace::wr!("1003c1e0", p3.add(0x14), write_unaligned, 0);
            mech_trace::wr!("1003c1e0", p3.add(0x16), write_unaligned, 0);
            mech_trace::wr!("1003c1e0", p3.add(0x17), write_unaligned, 0xffffffff);
            mech_trace::wr!("1003c1e0", p3.add(0x18), write_unaligned, 0xffffffff);
            mech_trace::wr!("1003c1e0", p3.add(0x19), write_unaligned, 0xffffffff);
        }
        i_var5 = 0;
        let mut local_c: i32 = local_10.wrapping_sub(1);
        i_var2 = 0;
        let mut p7 = (param_1.wrapping_add(0x2f4)) as *mut u32;
        loop {
            let prev = mech_trace::rd!("1003c1e0", p7.offset(-1), read_unaligned) as i32;
            if -1 < prev {
                let idx = local_10.wrapping_add(i_var5).wrapping_mul(0xe);
                let p1 = p3.offset(idx as isize);
                mech_trace::wr!("1003c1e0", p1, write_unaligned, 0x172);
                let row = mech_trace::rd!(
                    "1003c1e0",
                    ((mech_trace::gr!("1003c1e0", "100a6744", DAT_100a6744).wrapping_add(0x40c))
                        as *const i32),
                    read_unaligned
                );
                mech_trace::wr!(
                    "1003c1e0",
                    p1.add(1),
                    write_unaligned,
                    row.wrapping_mul(local_10.wrapping_add(local_c))
                        .wrapping_add(0x59) as u32
                );
                mech_trace::wr!("1003c1e0", p1.add(2), write_unaligned, 200);
                mech_trace::wr!("1003c1e0", p1.add(3), write_unaligned, 0xffffffff);
                mech_trace::wr!("1003c1e0", p1.add(4), write_unaligned, 0);
                mech_trace::wr!(
                    "1003c1e0",
                    p1.add(5),
                    write_unaligned,
                    (&raw mut DAT_100c49d0) as u32
                );
                mech_trace::wr!("1003c1e0", p1.add(6), write_unaligned, 0);
                mech_trace::wr!(
                    "1003c1e0",
                    p1.add(7),
                    write_unaligned,
                    (&raw mut LAB_1003c680) as u32
                );
                mech_trace::wr!(
                    "1003c1e0",
                    p1.add(8),
                    write_unaligned,
                    (&raw mut LAB_1003c560) as u32
                );
                if 4 < (((mech_trace::rd!("1003c1e0", p7, read_unaligned) >> 8) as u8) & 7) {
                    mech_trace::wr!(
                        "1003c1e0",
                        p7,
                        write_unaligned,
                        mech_trace::rd!("1003c1e0", p7, read_unaligned) & 0xfffff0ff
                    );
                }
                let i_var6 = i_var5.wrapping_add(1);
                mech_trace::wr!("1003c1e0", p1.add(9), write_unaligned, (prev / 100) as u32);
                mech_trace::wr!("1003c1e0", p1.add(10), write_unaligned, i_var2 as u32);
                mech_trace::wr!("1003c1e0", p1.add(0xb), write_unaligned, 0xffffffff);
                mech_trace::wr!("1003c1e0", p1.add(0xd), write_unaligned, 0xffffffff);
                mech_trace::wr!("1003c1e0", p1.add(0xc), write_unaligned, param_1 as u32);
                i_var5 = i_var5.wrapping_add(2);
                let idx2 = local_10.wrapping_add(i_var6).wrapping_mul(0xe);
                let p1 = p3.offset(idx2 as isize);
                mech_trace::wr!("1003c1e0", p1, write_unaligned, 0x22d);
                let row = mech_trace::rd!(
                    "1003c1e0",
                    ((mech_trace::gr!("1003c1e0", "100a6744", DAT_100a6744).wrapping_add(0x40c))
                        as *const i32),
                    read_unaligned
                );
                mech_trace::wr!(
                    "1003c1e0",
                    p1.add(1),
                    write_unaligned,
                    row.wrapping_mul(local_10.wrapping_add(local_c))
                        .wrapping_add(0x59) as u32
                );
                mech_trace::wr!("1003c1e0", p1.add(2), write_unaligned, 0xffffffff);
                mech_trace::wr!("1003c1e0", p1.add(3), write_unaligned, 0xffffffff);
                mech_trace::wr!("1003c1e0", p1.add(4), write_unaligned, 0);
                mech_trace::wr!(
                    "1003c1e0",
                    p1.add(5),
                    write_unaligned,
                    (&raw mut DAT_100c49d0) as u32
                );
                mech_trace::wr!("1003c1e0", p1.add(6), write_unaligned, 0);
                mech_trace::wr!(
                    "1003c1e0",
                    p1.add(7),
                    write_unaligned,
                    (&raw mut LAB_1003c5e0) as u32
                );
                mech_trace::wr!("1003c1e0", p1.add(8), write_unaligned, 0);
                mech_trace::wr!("1003c1e0", p1.add(9), write_unaligned, 0xffffffff);
                mech_trace::wr!("1003c1e0", p1.add(10), write_unaligned, i_var2 as u32);
                mech_trace::wr!("1003c1e0", p1.add(0xb), write_unaligned, 0xffffffff);
                mech_trace::wr!(
                    "1003c1e0",
                    p1.add(0xd),
                    write_unaligned,
                    (mech_trace::rd!("1003c1e0", p7, read_unaligned) & 0x700) >> 8
                );
                mech_trace::wr!("1003c1e0", p1.add(0xc), write_unaligned, param_1 as u32);
                local_c = local_c.wrapping_add(1);
            }
            p7 = p7.add(2);
            i_var2 = i_var2.wrapping_add(1);
            if i_var2 >= 0xe {
                break;
            }
        }
        let idx = local_10.wrapping_add(i_var5).wrapping_mul(0xe);
        mech_trace::wr!(
            "1003c1e0",
            p3.offset(idx as isize),
            write_unaligned,
            0xffffffff
        );
        mech_trace::rt!("1003c1e0", p3)
    }
}
