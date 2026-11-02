// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10003a28: [u8; 0];
    static mut DAT_10082058: i32;
    static mut DAT_1008205c: i32;
    static mut DAT_10082060: i32;
    static mut DAT_10082064: i32;
    static mut DAT_10082068: i32;
    static mut _DAT_1008206c: i32;
    static mut DAT_10082070: i32;
    static mut DAT_1008207c: *mut u32;
    static mut DAT_10082080: *mut u32;
    static mut DAT_10082084: *mut u32;
    static mut DAT_10082088: *mut u32;
    static mut DAT_1008208c: *mut u32;
    static mut DAT_10082090: *mut u32;
    static mut DAT_10082094: *mut u32;
    static mut DAT_10082098: i32;
    static mut DAT_1008209c: i32;
    static mut _DAT_100820a0: i32;
    static mut DAT_100820a4: u32;
    static mut DAT_100820a8: i32;
    static mut DAT_100820ac: i32;
    static mut DAT_100820b8: i32;
    static mut DAT_100820bc: i32;
    static mut DAT_100820c0: i32;
    static mut DAT_100820c4: i32;
    static mut DAT_100820c8: i32;
    static mut DAT_100820cc: i32;
    static mut DAT_100820d8: i32;
    static mut DAT_100820dc: i32;
    static mut DAT_100820e0: i32;
    static mut DAT_100820e4: i32;
    static mut DAT_100820e8: u32;
    static mut DAT_100820ec: i32;
    static mut DAT_100820f4: u32;
    static mut DAT_100820f8: u32;
    static mut DAT_100820fc: i32;
    static mut _DAT_10082104: unsafe extern "C" fn(*mut c_void);
    static mut _DAT_10082118: i32;
    static mut _DAT_1008211c: i32;
    static mut _DAT_10082120: i32;
    static mut _DAT_10082124: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RaiseException(
        dwExceptionCode: u32,
        dwExceptionFlags: u32,
        nNumberOfArguments: u32,
        lpArguments: *const u32,
    );
}

#[inline(always)]
unsafe fn r(p: *mut u32, i: usize) -> u32 {
    // SAFETY: p points at the caller's vertex array, read where the original reads.
    unsafe { mech_trace::rd!("100034e5", p.add(i), read_unaligned) }
}

#[inline(always)]
fn na(ud: u32) -> i64 {
    ((((ud << 16) as i32 >> 16) as u32 as u64) << 32) as i64
}

#[inline(always)]
fn nb(ud: u32) -> i64 {
    ((((ud as i32 >> 16) as u32 as u64) << 32) | ((ud << 16) as u64)) as i64
}

#[inline(always)]
unsafe fn dv(num: i64, den: i32) -> i32 {
    let mut q: i64 = 0;
    // SAFETY: RaiseException is the system API, called as the original does.
    unsafe {
        if den == 0 {
            RaiseException(0xC0000094, 0, 0, core::ptr::null());
        } else {
            q = num.wrapping_div(den as i64);
            if q > 0x7fffffff || q < -0x80000000 {
                RaiseException(0xC0000094, 0, 0, core::ptr::null());
            }
        }
    }
    q as i32
}

#[inline(always)]
fn mulsh(a: i32, b: i32) -> u32 {
    (((a as i64).wrapping_mul(b as i64) as u64) >> 16) as u32
}

#[inline(always)]
fn addr(p: *mut u32) -> i32 {
    p as usize as i32
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100034e5(
    param_1: *mut i32,
    param_2: i32,
    param_3: *mut u32,
    param_4: *mut i32,
    param_5: i32,
) {
    mech_trace::enter!("100034e5", param_1, param_2, param_3, param_4, param_5);
    // SAFETY: the globals and callees are the game's own; the pointer
    // parameters are the caller's buffers, accessed where the original does.
    unsafe {
        let pi1 = mech_trace::rd!("100034e5", param_1, read_unaligned) as usize as *mut i32;
        mech_trace::gw!(
            "100034e5",
            "10082064",
            DAT_10082064,
            mech_trace::rd!("100034e5", pi1.add(1), read_unaligned).wrapping_add(1)
        );
        let mut i_var9 = mech_trace::rd!("100034e5", pi1.add(1), read_unaligned);
        if mech_trace::rd!("100034e5", param_1.add(3), read_unaligned)
            <= mech_trace::rd!("100034e5", pi1.add(1), read_unaligned)
        {
            i_var9 = mech_trace::rd!("100034e5", param_1.add(3), read_unaligned);
        }
        let mut u_var17: u32 = 0;
        if (mech_trace::rd!("100034e5", param_1.add(1), read_unaligned) as u32) < 0x80000000 {
            u_var17 = mech_trace::rd!("100034e5", param_1.add(1), read_unaligned) as u32;
        }
        let i_var6 = (i_var9 as u32).wrapping_sub(u_var17) as i32;
        if (u_var17 as i32) > i_var9 {
            return;
        }

        i_var9 = mech_trace::rd!("100034e5", pi1.add(2), read_unaligned);
        if mech_trace::rd!("100034e5", param_1.add(4), read_unaligned)
            <= mech_trace::rd!("100034e5", pi1.add(2), read_unaligned)
        {
            i_var9 = mech_trace::rd!("100034e5", param_1.add(4), read_unaligned);
        }
        let mut u_var21: u32 = 0;
        if (mech_trace::rd!("100034e5", param_1.add(2), read_unaligned) as u32) < 0x80000000 {
            u_var21 = mech_trace::rd!("100034e5", param_1.add(2), read_unaligned) as u32;
        }
        let mut i_var7 = (i_var9 as u32).wrapping_sub(u_var21) as i32;
        mech_trace::gw!("100034e5", "10082058", DAT_10082058, i_var6);
        if (u_var21 as i32) > i_var9 {
            return;
        }

        mech_trace::gw!(
            "100034e5",
            "10082060",
            DAT_10082060,
            u_var21
                .wrapping_mul(mech_trace::gr!("100034e5", "10082064", DAT_10082064) as u32)
                .wrapping_add(u_var17)
                .wrapping_add(mech_trace::rd!("100034e5", pi1, read_unaligned) as u32)
                as i32
        );
        mech_trace::gw!(
            "100034e5",
            "_DAT_1008206c",
            _DAT_1008206c,
            mech_trace::rd!("100034e5", param_4, read_unaligned)
        );
        mech_trace::gw!(
            "100034e5",
            "10082068",
            DAT_10082068,
            mech_trace::rd!("100034e5", param_4.add(1), read_unaligned).wrapping_add(1)
        );
        mech_trace::gw!(
            "100034e5",
            "10082080",
            DAT_10082080,
            param_3.wrapping_offset(param_2.wrapping_mul(6) as isize)
        );
        mech_trace::gw!("100034e5", "1008207c", DAT_1008207c, param_3);
        u_var17 = 0x7fff;
        u_var21 = 0xffff8000;
        let mut u_var8: u32 = 0xf;
        let mut pu14 = param_3;
        loop {
            let u_var2 = r(pu14, 1);
            if (u_var2 as i32) <= (u_var17 as i32) {
                u_var17 = u_var2;
                mech_trace::gw!("100034e5", "10082084", DAT_10082084, pu14);
            }
            if (u_var21 as i32) <= (u_var2 as i32) {
                u_var21 = u_var2;
            }
            let p0 = r(pu14, 0);
            let p1 = r(pu14, 1);
            u_var8 &= (((((((p0 >> 31) << 1) | ((i_var6 as u32).wrapping_sub(p0) >> 31)) << 1)
                | (p1 >> 31))
                << 1)
                | ((i_var7 as u32).wrapping_sub(p1) >> 31));
            pu14 = pu14.wrapping_add(6);
            if pu14 == mech_trace::gr!("100034e5", "10082080", DAT_10082080) {
                break;
            }
        }
        mech_trace::gw!("100034e5", "1008205c", DAT_1008205c, i_var7);
        if u_var8 != 0 {
            return;
        }

        mech_trace::gw!(
            "100034e5",
            "10082090",
            DAT_10082090,
            mech_trace::gr!("100034e5", "10082084", DAT_10082084)
        );
        mech_trace::gw!(
            "100034e5",
            "10082094",
            DAT_10082094,
            mech_trace::gr!("100034e5", "10082084", DAT_10082084)
        );
        pu14 = mech_trace::gr!("100034e5", "10082090", DAT_10082090);
        mech_trace::gw!("100034e5", "100820a4", DAT_100820a4, u_var17);
        if u_var21 == u_var17 {
            return;
        }

        // left edge
        loop {
            mech_trace::gw!("100034e5", "10082090", DAT_10082090, pu14);
            mech_trace::gw!(
                "100034e5",
                "10082088",
                DAT_10082088,
                mech_trace::gr!("100034e5", "10082090", DAT_10082090)
            );
            pu14 = mech_trace::gr!("100034e5", "10082090", DAT_10082090).wrapping_sub(6);
            if addr(pu14) < addr(param_3) {
                pu14 = mech_trace::gr!("100034e5", "10082080", DAT_10082080).wrapping_sub(6);
            }
            if ((r(mech_trace::gr!("100034e5", "10082090", DAT_10082090), 1) as i32) < 0
                && (r(pu14, 1) as i32) < 1)
            {
                continue;
            }
            mech_trace::gw!(
                "100034e5",
                "10082098",
                DAT_10082098,
                r(pu14, 1).wrapping_sub(r(mech_trace::gr!("100034e5", "10082090", DAT_10082090), 1))
                    as i32
            );
            if mech_trace::gr!("100034e5", "10082098", DAT_10082098) == 0 {
                continue;
            }
            break;
        }
        let mut i_den =
            ((mech_trace::gr!("100034e5", "10082098", DAT_10082098) as u32) << 16) as i32;

        let mut u_d =
            r(pu14, 0).wrapping_sub(r(mech_trace::gr!("100034e5", "10082090", DAT_10082090), 0));
        mech_trace::gw!("100034e5", "100820c8", DAT_100820c8, dv(na(u_d), i_den));
        u_d = r(pu14, 3).wrapping_sub(r(mech_trace::gr!("100034e5", "10082090", DAT_10082090), 3));
        mech_trace::gw!("100034e5", "100820d8", DAT_100820d8, dv(nb(u_d), i_den));
        u_d = r(pu14, 4).wrapping_sub(r(mech_trace::gr!("100034e5", "10082090", DAT_10082090), 4));
        mech_trace::gw!("100034e5", "100820e0", DAT_100820e0, dv(nb(u_d), i_den));

        mech_trace::gw!(
            "100034e5",
            "100820a8",
            DAT_100820a8,
            r(mech_trace::gr!("100034e5", "10082090", DAT_10082090), 0)
                .wrapping_mul(0x10000)
                .wrapping_add(0x8000) as i32
        );
        mech_trace::gw!(
            "100034e5",
            "100820b8",
            DAT_100820b8,
            r(mech_trace::gr!("100034e5", "10082090", DAT_10082090), 3).wrapping_add(0x8000) as i32
        );
        mech_trace::gw!(
            "100034e5",
            "100820c0",
            DAT_100820c0,
            r(mech_trace::gr!("100034e5", "10082090", DAT_10082090), 4).wrapping_add(0x8000) as i32
        );

        // right edge
        let mut pu18 = mech_trace::gr!("100034e5", "10082094", DAT_10082094);
        loop {
            mech_trace::gw!("100034e5", "10082094", DAT_10082094, pu18);
            mech_trace::gw!(
                "100034e5",
                "1008208c",
                DAT_1008208c,
                mech_trace::gr!("100034e5", "10082094", DAT_10082094)
            );
            pu18 = mech_trace::gr!("100034e5", "10082094", DAT_10082094).wrapping_add(6);
            if addr(mech_trace::gr!("100034e5", "10082080", DAT_10082080))
                <= addr(mech_trace::gr!("100034e5", "10082094", DAT_10082094).wrapping_add(6))
            {
                pu18 = param_3;
            }
            if ((r(mech_trace::gr!("100034e5", "10082094", DAT_10082094), 1) as i32) < 0
                && (r(pu18, 1) as i32) < 1)
            {
                continue;
            }
            mech_trace::gw!(
                "100034e5",
                "1008209c",
                DAT_1008209c,
                r(pu18, 1).wrapping_sub(r(mech_trace::gr!("100034e5", "10082094", DAT_10082094), 1))
                    as i32
            );
            if mech_trace::gr!("100034e5", "1008209c", DAT_1008209c) == 0 {
                continue;
            }
            break;
        }
        i_den = ((mech_trace::gr!("100034e5", "1008209c", DAT_1008209c) as u32) << 16) as i32;

        u_d = r(pu18, 0).wrapping_sub(r(mech_trace::gr!("100034e5", "10082094", DAT_10082094), 0));
        mech_trace::gw!("100034e5", "100820cc", DAT_100820cc, dv(na(u_d), i_den));
        u_d = r(pu18, 3).wrapping_sub(r(mech_trace::gr!("100034e5", "10082094", DAT_10082094), 3));
        mech_trace::gw!("100034e5", "100820dc", DAT_100820dc, dv(nb(u_d), i_den));
        u_d = r(pu18, 4).wrapping_sub(r(mech_trace::gr!("100034e5", "10082094", DAT_10082094), 4));
        mech_trace::gw!("100034e5", "100820e4", DAT_100820e4, dv(nb(u_d), i_den));

        mech_trace::gw!(
            "100034e5",
            "100820ac",
            DAT_100820ac,
            r(mech_trace::gr!("100034e5", "10082094", DAT_10082094), 0)
                .wrapping_mul(0x10000)
                .wrapping_add(0x8000) as i32
        );
        mech_trace::gw!(
            "100034e5",
            "100820bc",
            DAT_100820bc,
            r(mech_trace::gr!("100034e5", "10082094", DAT_10082094), 3).wrapping_add(0x8000) as i32
        );
        mech_trace::gw!(
            "100034e5",
            "100820c4",
            DAT_100820c4,
            r(mech_trace::gr!("100034e5", "10082094", DAT_10082094), 4).wrapping_add(0x8000) as i32
        );

        mech_trace::gw!(
            "100034e5",
            "_DAT_100820a0",
            _DAT_100820a0,
            (i_var7 as u32).wrapping_sub(u_var17) as i32
        );
        if u_var21.wrapping_sub(i_var7 as u32) == 0 || (u_var21 as i32) < i_var7 {
            mech_trace::gw!(
                "100034e5",
                "_DAT_100820a0",
                _DAT_100820a0,
                (mech_trace::gr!("100034e5", "_DAT_100820a0", _DAT_100820a0) as u32)
                    .wrapping_add(u_var21.wrapping_sub(i_var7 as u32)) as i32
            );
        }
        if 0x7fffffff < u_var17 {
            mech_trace::gw!(
                "100034e5",
                "_DAT_100820a0",
                _DAT_100820a0,
                (mech_trace::gr!("100034e5", "_DAT_100820a0", _DAT_100820a0) as u32)
                    .wrapping_add(u_var17) as i32
            );
            mech_trace::gw!("100034e5", "100820a4", DAT_100820a4, 0);
            mech_trace::gw!(
                "100034e5",
                "10082098",
                DAT_10082098,
                (mech_trace::gr!("100034e5", "10082098", DAT_10082098) as u32)
                    .wrapping_add(r(mech_trace::gr!("100034e5", "10082090", DAT_10082090), 1))
                    as i32
            );
            i_var9 = r(mech_trace::gr!("100034e5", "10082090", DAT_10082090), 1)
                .wrapping_mul(0xffff0000) as i32;
            mech_trace::gw!(
                "100034e5",
                "100820a8",
                DAT_100820a8,
                (mech_trace::gr!("100034e5", "100820a8", DAT_100820a8) as u32).wrapping_add(mulsh(
                    mech_trace::gr!("100034e5", "100820c8", DAT_100820c8),
                    i_var9
                )) as i32
            );
            mech_trace::gw!(
                "100034e5",
                "100820b8",
                DAT_100820b8,
                (mech_trace::gr!("100034e5", "100820b8", DAT_100820b8) as u32).wrapping_add(mulsh(
                    mech_trace::gr!("100034e5", "100820d8", DAT_100820d8),
                    i_var9
                )) as i32
            );
            mech_trace::gw!(
                "100034e5",
                "100820c0",
                DAT_100820c0,
                (mech_trace::gr!("100034e5", "100820c0", DAT_100820c0) as u32).wrapping_add(mulsh(
                    mech_trace::gr!("100034e5", "100820e0", DAT_100820e0),
                    i_var9
                )) as i32
            );
            mech_trace::gw!(
                "100034e5",
                "1008209c",
                DAT_1008209c,
                (mech_trace::gr!("100034e5", "1008209c", DAT_1008209c) as u32)
                    .wrapping_add(r(mech_trace::gr!("100034e5", "10082094", DAT_10082094), 1))
                    as i32
            );
            i_var9 = r(mech_trace::gr!("100034e5", "10082094", DAT_10082094), 1)
                .wrapping_mul(0xffff0000) as i32;
            mech_trace::gw!(
                "100034e5",
                "100820ac",
                DAT_100820ac,
                (mech_trace::gr!("100034e5", "100820ac", DAT_100820ac) as u32).wrapping_add(mulsh(
                    mech_trace::gr!("100034e5", "100820cc", DAT_100820cc),
                    i_var9
                )) as i32
            );
            mech_trace::gw!(
                "100034e5",
                "100820bc",
                DAT_100820bc,
                (mech_trace::gr!("100034e5", "100820bc", DAT_100820bc) as u32).wrapping_add(mulsh(
                    mech_trace::gr!("100034e5", "100820dc", DAT_100820dc),
                    i_var9
                )) as i32
            );
            mech_trace::gw!(
                "100034e5",
                "100820c4",
                DAT_100820c4,
                (mech_trace::gr!("100034e5", "100820c4", DAT_100820c4) as u32).wrapping_add(mulsh(
                    mech_trace::gr!("100034e5", "100820e4", DAT_100820e4),
                    i_var9
                )) as i32
            );
        }
        mech_trace::gw!(
            "100034e5",
            "10082070",
            DAT_10082070,
            mech_trace::gr!("100034e5", "100820a4", DAT_100820a4)
                .wrapping_mul(mech_trace::gr!("100034e5", "10082064", DAT_10082064) as u32)
                .wrapping_add(mech_trace::gr!("100034e5", "10082060", DAT_10082060) as u32)
                as i32
        );
        mech_trace::gw!(
            "100034e5",
            "_DAT_10082104",
            _DAT_10082104,
            mech_trace::rd!(
                "100034e5",
                (&raw const DAT_10003a28)
                    .cast::<unsafe extern "C" fn(*mut c_void)>()
                    .offset(param_5 as isize),
                read_unaligned
            )
        );
        i_var9 = mech_trace::gr!("100034e5", "100820a8", DAT_100820a8);
        i_var7 = mech_trace::gr!("100034e5", "100820b8", DAT_100820b8);
        let mut i_var13 = mech_trace::gr!("100034e5", "100820bc", DAT_100820bc);
        let mut i_var16 = mech_trace::gr!("100034e5", "100820ac", DAT_100820ac);
        let mut i_var20 = mech_trace::gr!("100034e5", "100820c0", DAT_100820c0);
        let mut i_var23 = mech_trace::gr!("100034e5", "100820c4", DAT_100820c4);
        mech_trace::gw!("100034e5", "10082090", DAT_10082090, pu14);
        mech_trace::gw!("100034e5", "10082094", DAT_10082094, pu18);

        loop {
            let mut i_var5 = i_var9;
            let mut i_var10 = i_var7;
            let mut i_var12 = i_var13;
            let mut i_var15 = i_var16;
            let mut i_var19 = i_var20;
            let mut i_var22 = i_var23;
            if i_var16 <= i_var9 {
                i_var5 = i_var16;
                i_var10 = i_var13;
                i_var12 = i_var7;
                i_var15 = i_var9;
                i_var19 = i_var23;
                i_var22 = i_var20;
            }
            u_var17 = (i_var5 >> 16) as u32;
            let mut enter = false;
            if (u_var17 as i32) <= i_var6 {
                i_var15 >>= 16;
                if -1 < i_var15 {
                    enter = true;
                }
            }
            if enter {
                let i_span = (i_var15 as u32).wrapping_sub(u_var17) as i32;
                mech_trace::gw!("100034e5", "100820e8", DAT_100820e8, u_var17);
                mech_trace::gw!("100034e5", "100820ec", DAT_100820ec, i_var15);
                mech_trace::gw!("100034e5", "100820fc", DAT_100820fc, i_var10);
                if i_span != 0 {
                    i_den = ((i_span as u32) << 16) as i32;

                    u_d = (i_var12 as u32).wrapping_sub(i_var10 as u32);
                    let mut u_q = dv(nb(u_d), i_den) as u32;
                    mech_trace::gw!("100034e5", "100820f4", DAT_100820f4, u_q);
                    u_var21 = mech_trace::gr!("100034e5", "100820f4", DAT_100820f4) >> 16;
                    let mut i_step: i32 = 1;
                    if (u_var21 & 0x8000) != 0 {
                        i_step = -1;
                        u_var21 = (u_var21 | 0xffff0000)
                            .wrapping_add(1)
                            .wrapping_sub((u_q & 0xffff == 0) as u32);
                    }

                    u_d = (i_var22 as u32).wrapping_sub(i_var19 as u32);
                    u_q = dv(nb(u_d), i_den) as u32;
                    mech_trace::gw!("100034e5", "100820f8", DAT_100820f8, u_q);
                    let mut s_var11 = (u_q >> 16) as i16;
                    mech_trace::gw!(
                        "100034e5",
                        "_DAT_10082124",
                        _DAT_10082124,
                        mech_trace::gr!("100034e5", "10082068", DAT_10082068)
                    );
                    if ((mech_trace::gr!("100034e5", "100820f8", DAT_100820f8) >> 16) & 0x8000) != 0
                    {
                        mech_trace::gw!(
                            "100034e5",
                            "_DAT_10082124",
                            _DAT_10082124,
                            0u32.wrapping_sub(
                                mech_trace::gr!("100034e5", "10082068", DAT_10082068) as u32
                            ) as i32
                        );
                        s_var11 = s_var11
                            .wrapping_add(1)
                            .wrapping_sub((u_q & 0xffff == 0) as i16);
                    }
                    mech_trace::gw!(
                        "100034e5",
                        "_DAT_10082120",
                        _DAT_10082120,
                        (mech_trace::gr!("100034e5", "10082068", DAT_10082068) as i16)
                            .wrapping_mul(s_var11) as i32
                    );
                    mech_trace::gw!(
                        "100034e5",
                        "_DAT_10082118",
                        _DAT_10082118,
                        u_var21.wrapping_add(mech_trace::gr!(
                            "100034e5",
                            "_DAT_10082120",
                            _DAT_10082120
                        ) as u32) as i32
                    );
                    mech_trace::gw!(
                        "100034e5",
                        "_DAT_1008211c",
                        _DAT_1008211c,
                        (mech_trace::gr!("100034e5", "_DAT_10082118", _DAT_10082118) as u32)
                            .wrapping_add(mech_trace::gr!(
                                "100034e5",
                                "_DAT_10082124",
                                _DAT_10082124
                            ) as u32) as i32
                    );
                    mech_trace::gw!(
                        "100034e5",
                        "_DAT_10082120",
                        _DAT_10082120,
                        (i_step as u32)
                            .wrapping_add(u_var21)
                            .wrapping_add(mech_trace::gr!(
                                "100034e5",
                                "_DAT_10082120",
                                _DAT_10082120
                            ) as u32) as i32
                    );
                    mech_trace::gw!(
                        "100034e5",
                        "_DAT_10082124",
                        _DAT_10082124,
                        (mech_trace::gr!("100034e5", "_DAT_10082120", _DAT_10082120) as u32)
                            .wrapping_add(mech_trace::gr!(
                                "100034e5",
                                "_DAT_10082124",
                                _DAT_10082124
                            ) as u32) as i32
                    );
                    if 0x7fffffff < u_var17 {
                        mech_trace::gw!("100034e5", "100820e8", DAT_100820e8, 0);
                        mech_trace::gw!(
                            "100034e5",
                            "100820fc",
                            DAT_100820fc,
                            (i_var10 as u32).wrapping_add(mulsh(
                                mech_trace::gr!("100034e5", "100820f4", DAT_100820f4) as i32,
                                u_var17.wrapping_mul(0xffff0000) as i32,
                            )) as i32
                        );
                    }
                    if i_var15.wrapping_sub(i_var6) != 0 && i_var6 <= i_var15 {
                        mech_trace::gw!(
                            "100034e5",
                            "100820ec",
                            DAT_100820ec,
                            i_var15.wrapping_sub(i_var15.wrapping_sub(i_var6))
                        );
                    }
                }
                mech_trace::cr!("100034e5", "ind", _DAT_10082104(core::ptr::null_mut()));
                return;
            }
            mech_trace::gw!(
                "100034e5",
                "10082070",
                DAT_10082070,
                (mech_trace::gr!("100034e5", "10082070", DAT_10082070) as u32)
                    .wrapping_add(mech_trace::gr!("100034e5", "10082064", DAT_10082064) as u32)
                    as i32
            );
            mech_trace::gw!(
                "100034e5",
                "_DAT_100820a0",
                _DAT_100820a0,
                (mech_trace::gr!("100034e5", "_DAT_100820a0", _DAT_100820a0) as u32).wrapping_sub(1)
                    as i32
            );
            if mech_trace::gr!("100034e5", "_DAT_100820a0", _DAT_100820a0) < 0 {
                break;
            }
            if mech_trace::gr!("100034e5", "_DAT_100820a0", _DAT_100820a0) == 0 {
                i_var9 = i_var9.wrapping_add(mech_trace::gr!("100034e5", "100820c8", DAT_100820c8));
                i_var7 = i_var7.wrapping_add(mech_trace::gr!("100034e5", "100820d8", DAT_100820d8));
                i_var20 =
                    i_var20.wrapping_add(mech_trace::gr!("100034e5", "100820e0", DAT_100820e0));
                i_var16 =
                    i_var16.wrapping_add(mech_trace::gr!("100034e5", "100820cc", DAT_100820cc));
                i_var13 =
                    i_var13.wrapping_add(mech_trace::gr!("100034e5", "100820dc", DAT_100820dc));
                i_var23 =
                    i_var23.wrapping_add(mech_trace::gr!("100034e5", "100820e4", DAT_100820e4));
            } else {
                mech_trace::gw!(
                    "100034e5",
                    "10082098",
                    DAT_10082098,
                    mech_trace::gr!("100034e5", "10082098", DAT_10082098).wrapping_sub(1)
                );
                if mech_trace::gr!("100034e5", "10082098", DAT_10082098) == 0 {
                    mech_trace::gw!(
                        "100034e5",
                        "10082088",
                        DAT_10082088,
                        mech_trace::gr!("100034e5", "10082090", DAT_10082090)
                    );
                    pu14 = mech_trace::gr!("100034e5", "10082090", DAT_10082090).wrapping_sub(6);
                    if addr(pu14) < addr(param_3) {
                        pu14 =
                            mech_trace::gr!("100034e5", "10082080", DAT_10082080).wrapping_sub(6);
                    }
                    let dd = r(pu14, 1)
                        .wrapping_sub(r(mech_trace::gr!("100034e5", "10082090", DAT_10082090), 1));
                    mech_trace::gw!(
                        "100034e5",
                        "10082098",
                        DAT_10082098,
                        dd.wrapping_add((dd == 0) as u32) as i32
                    );
                    i_den = ((mech_trace::gr!("100034e5", "10082098", DAT_10082098) as u32) << 16)
                        as i32;

                    u_d = r(pu14, 0)
                        .wrapping_sub(r(mech_trace::gr!("100034e5", "10082090", DAT_10082090), 0));
                    mech_trace::gw!("100034e5", "100820c8", DAT_100820c8, dv(na(u_d), i_den));
                    u_d = r(pu14, 3)
                        .wrapping_sub(r(mech_trace::gr!("100034e5", "10082090", DAT_10082090), 3));
                    mech_trace::gw!("100034e5", "100820d8", DAT_100820d8, dv(nb(u_d), i_den));
                    u_d = r(pu14, 4)
                        .wrapping_sub(r(mech_trace::gr!("100034e5", "10082090", DAT_10082090), 4));
                    mech_trace::gw!("100034e5", "100820e0", DAT_100820e0, dv(nb(u_d), i_den));

                    i_var9 = r(mech_trace::gr!("100034e5", "10082090", DAT_10082090), 0)
                        .wrapping_mul(0x10000)
                        .wrapping_add(0x8000) as i32;
                    i_var7 = r(mech_trace::gr!("100034e5", "10082090", DAT_10082090), 3)
                        .wrapping_add(0x8000) as i32;
                    i_var20 = r(mech_trace::gr!("100034e5", "10082090", DAT_10082090), 4)
                        .wrapping_add(0x8000) as i32;
                    mech_trace::gw!("100034e5", "10082090", DAT_10082090, pu14);
                } else {
                    i_var9 =
                        i_var9.wrapping_add(mech_trace::gr!("100034e5", "100820c8", DAT_100820c8));
                    i_var7 =
                        i_var7.wrapping_add(mech_trace::gr!("100034e5", "100820d8", DAT_100820d8));
                    i_var20 =
                        i_var20.wrapping_add(mech_trace::gr!("100034e5", "100820e0", DAT_100820e0));
                }
                mech_trace::gw!(
                    "100034e5",
                    "1008209c",
                    DAT_1008209c,
                    mech_trace::gr!("100034e5", "1008209c", DAT_1008209c).wrapping_sub(1)
                );
                if mech_trace::gr!("100034e5", "1008209c", DAT_1008209c) == 0 {
                    mech_trace::gw!(
                        "100034e5",
                        "1008208c",
                        DAT_1008208c,
                        mech_trace::gr!("100034e5", "10082094", DAT_10082094)
                    );
                    pu14 = mech_trace::gr!("100034e5", "10082094", DAT_10082094).wrapping_add(6);
                    if addr(mech_trace::gr!("100034e5", "10082080", DAT_10082080))
                        <= addr(
                            mech_trace::gr!("100034e5", "10082094", DAT_10082094).wrapping_add(6),
                        )
                    {
                        pu14 = param_3;
                    }
                    let dd = r(pu14, 1)
                        .wrapping_sub(r(mech_trace::gr!("100034e5", "10082094", DAT_10082094), 1));
                    mech_trace::gw!(
                        "100034e5",
                        "1008209c",
                        DAT_1008209c,
                        dd.wrapping_add((dd == 0) as u32) as i32
                    );
                    i_den = ((mech_trace::gr!("100034e5", "1008209c", DAT_1008209c) as u32) << 16)
                        as i32;

                    u_d = r(pu14, 0)
                        .wrapping_sub(r(mech_trace::gr!("100034e5", "10082094", DAT_10082094), 0));
                    mech_trace::gw!("100034e5", "100820cc", DAT_100820cc, dv(na(u_d), i_den));
                    u_d = r(pu14, 3)
                        .wrapping_sub(r(mech_trace::gr!("100034e5", "10082094", DAT_10082094), 3));
                    mech_trace::gw!("100034e5", "100820dc", DAT_100820dc, dv(nb(u_d), i_den));
                    u_d = r(pu14, 4)
                        .wrapping_sub(r(mech_trace::gr!("100034e5", "10082094", DAT_10082094), 4));
                    mech_trace::gw!("100034e5", "100820e4", DAT_100820e4, dv(nb(u_d), i_den));

                    i_var13 = r(mech_trace::gr!("100034e5", "10082094", DAT_10082094), 3)
                        .wrapping_add(0x8000) as i32;
                    i_var23 = r(mech_trace::gr!("100034e5", "10082094", DAT_10082094), 4)
                        .wrapping_add(0x8000) as i32;
                    i_var16 = r(mech_trace::gr!("100034e5", "10082094", DAT_10082094), 0)
                        .wrapping_mul(0x10000)
                        .wrapping_add(0x8000) as i32;
                    mech_trace::gw!("100034e5", "10082094", DAT_10082094, pu14);
                } else {
                    i_var16 =
                        i_var16.wrapping_add(mech_trace::gr!("100034e5", "100820cc", DAT_100820cc));
                    i_var13 =
                        i_var13.wrapping_add(mech_trace::gr!("100034e5", "100820dc", DAT_100820dc));
                    i_var23 =
                        i_var23.wrapping_add(mech_trace::gr!("100034e5", "100820e4", DAT_100820e4));
                }
            }
        }
    }
}
