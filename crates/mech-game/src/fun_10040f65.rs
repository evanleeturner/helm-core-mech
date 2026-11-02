// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100af324: i32;
    static mut DAT_100af318: i32;
    static mut DAT_100af320: i32;
    static mut _DAT_100af32c: i32;
    static mut DAT_100af328: i32;
    static mut DAT_100af340: *mut u32;
    static mut DAT_100af33c: *mut u32;
    static mut DAT_100af344: *mut u32;
    static mut DAT_100af364: i32;
    static mut DAT_100af31c: i32;
    static mut DAT_100af350: *mut u32;
    static mut DAT_100af354: *mut u32;
    static mut DAT_100af348: *mut u32;
    static mut DAT_100af34c: *mut u32;
    static mut DAT_100af358: i32;
    static mut DAT_100af35c: i32;
    static mut _DAT_100af360: i32;
    static mut DAT_100af388: i32;
    static mut DAT_100af398: i32;
    static mut DAT_100af3a0: i32;
    static mut DAT_100af38c: i32;
    static mut DAT_100af39c: i32;
    static mut DAT_100af3a4: i32;
    static mut DAT_100af368: i32;
    static mut DAT_100af378: i32;
    static mut DAT_100af380: i32;
    static mut DAT_100af36c: i32;
    static mut DAT_100af37c: i32;
    static mut DAT_100af384: i32;
    static mut DAT_100af330: i32;
    static mut _DAT_100af3c4: unsafe extern "C" fn();
    static mut PTR_DAT_100414a8: [unsafe extern "C" fn(); 0];
    static mut DAT_100af3a8: i32;
    static mut DAT_100af3ac: i32;
    static mut DAT_100af3bc: i32;
    static mut DAT_100af3b4: u32;
    static mut DAT_100af3b8: u32;
    static mut _DAT_100af3e4: i32;
    static mut _DAT_100af3e0: i32;
    static mut _DAT_100af3d8: i32;
    static mut _DAT_100af3dc: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RaiseException(a: u32, b: u32, c: u32, d: *const u32);
}

#[inline(always)]
unsafe fn g(p: *const u32, i: usize) -> u32 {
    // SAFETY: caller supplies a valid vertex pointer.
    unsafe { mech_trace::rd!("10040f65", p.wrapping_add(i), read_unaligned) }
}

#[inline(always)]
fn ad(p: *const u32) -> i32 {
    p as usize as i32
}

#[inline(always)]
fn num_a(x: u32) -> i64 {
    ((((x.wrapping_mul(0x10000)) as i32 >> 16) as u32 as u64) << 32) as i64
}

#[inline(always)]
fn num_b(x: u32) -> i64 {
    (x as i32 as i64).wrapping_mul(65536)
}

#[inline(always)]
fn fmul(a: i32, b: i32) -> u32 {
    let p = (a as i64).wrapping_mul(b as i64);
    ((p as u32) >> 16) | ((((p as u64) >> 32) as u32) << 16)
}

#[inline(always)]
unsafe fn qdiv(n: i64, d: i32) -> i32 {
    let ld = (d as u32).wrapping_mul(0x10000) as i32 as i64;
    let mut q: i64 = 0;
    // SAFETY: RaiseException is the OS routine, called as the original does.
    unsafe {
        if ld == 0 {
            RaiseException(0xC0000094, 0, 0, core::ptr::null());
        } else {
            if ld == -1 {
                q = 0i64.wrapping_sub(n);
            } else {
                q = n / ld;
            }
            if q != (q as i32) as i64 {
                RaiseException(0xC0000094, 0, 0, core::ptr::null());
            }
        }
    }
    q as i32
}

#[inline(always)]
unsafe fn qstore(n: i64, d: i32, dst: *mut i32) {
    // SAFETY: dst is the address of one of the game's globals.
    unsafe {
        let q = qdiv(n, d);
        mech_trace::wr!("10040f65", dst, write_unaligned, q);
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10040f65(
    param_1: *mut i32,
    param_2: i32,
    param_3: *mut u32,
    param_4: *mut u32,
    param_5: i32,
) {
    mech_trace::enter!("10040f65", param_1, param_2, param_3, param_4, param_5);
    // SAFETY: the globals and the tables are the game's own; the pointer
    // parameters point at the caller's structures, accessed as the original does.
    unsafe {
        let p1 = |i: usize| -> i32 {
            mech_trace::rd!("10040f65", param_1.wrapping_add(i), read_unaligned)
        };
        let pi1 = (p1(0)) as u32 as usize as *mut i32;
        let pi1_at =
            |i: usize| -> i32 { mech_trace::rd!("10040f65", pi1.wrapping_add(i), read_unaligned) };

        mech_trace::gw!(
            "10040f65",
            "100af324",
            DAT_100af324,
            pi1_at(1).wrapping_add(1)
        );
        let mut i_var9 = pi1_at(1);
        if p1(3) <= pi1_at(1) {
            i_var9 = p1(3);
        }
        let mut u_var17: u32 = 0;
        if (p1(1) as u32) < 0x80000000 {
            u_var17 = p1(1) as u32;
        }
        let i_var6 = (i_var9 as u32).wrapping_sub(u_var17) as i32;
        if !((u_var17 as i32) <= i_var9) {
            return;
        }
        i_var9 = pi1_at(2);
        if p1(4) <= pi1_at(2) {
            i_var9 = p1(4);
        }
        let mut u_var21: u32 = 0;
        if (p1(2) as u32) < 0x80000000 {
            u_var21 = p1(2) as u32;
        }
        let mut i_var7 = (i_var9 as u32).wrapping_sub(u_var21) as i32;
        mech_trace::gw!("10040f65", "100af318", DAT_100af318, i_var6);
        if !((u_var21 as i32) <= i_var9) {
            return;
        }
        mech_trace::gw!(
            "10040f65",
            "100af320",
            DAT_100af320,
            u_var21
                .wrapping_mul(mech_trace::gr!("10040f65", "100af324", DAT_100af324) as u32)
                .wrapping_add(u_var17)
                .wrapping_add(mech_trace::rd!("10040f65", pi1, read_unaligned) as u32)
                as i32
        );
        mech_trace::gw!(
            "10040f65",
            "_DAT_100af32c",
            _DAT_100af32c,
            mech_trace::rd!("10040f65", param_4, read_unaligned) as i32
        );
        mech_trace::gw!(
            "10040f65",
            "100af328",
            DAT_100af328,
            g(param_4, 1).wrapping_add(1) as i32
        );
        mech_trace::gw!(
            "10040f65",
            "100af340",
            DAT_100af340,
            param_3.wrapping_offset(param_2.wrapping_mul(6) as isize)
        );
        mech_trace::gw!("10040f65", "100af33c", DAT_100af33c, param_3);
        u_var17 = 0x7fff;
        u_var21 = 0xffff8000;
        let mut u_var8: u32 = 0xf;
        let mut pu14 = param_3;
        loop {
            let u_var2 = g(pu14, 1);
            if (u_var2 as i32) <= (u_var17 as i32) {
                u_var17 = u_var2;
                mech_trace::gw!("10040f65", "100af344", DAT_100af344, pu14);
            }
            if (u_var21 as i32) <= (u_var2 as i32) {
                u_var21 = u_var2;
            }
            u_var8 = u_var8
                & (((((((g(pu14, 0) >> 0x1f) << 1)
                    | ((i_var6 as u32).wrapping_sub(g(pu14, 0)) >> 0x1f))
                    << 1)
                    | (g(pu14, 1) >> 0x1f))
                    << 1)
                    | ((i_var7 as u32).wrapping_sub(g(pu14, 1)) >> 0x1f));
            pu14 = pu14.wrapping_add(6);
            if pu14 == mech_trace::gr!("10040f65", "100af340", DAT_100af340) {
                break;
            }
        }
        mech_trace::gw!("10040f65", "100af31c", DAT_100af31c, i_var7);
        if u_var8 != 0 {
            return;
        }
        mech_trace::gw!(
            "10040f65",
            "100af350",
            DAT_100af350,
            mech_trace::gr!("10040f65", "100af344", DAT_100af344)
        );
        mech_trace::gw!(
            "10040f65",
            "100af354",
            DAT_100af354,
            mech_trace::gr!("10040f65", "100af344", DAT_100af344)
        );
        pu14 = mech_trace::gr!("10040f65", "100af350", DAT_100af350);
        mech_trace::gw!("10040f65", "100af364", DAT_100af364, u_var17 as i32);
        if u_var21 == u_var17 {
            return;
        }
        loop {
            mech_trace::gw!("10040f65", "100af350", DAT_100af350, pu14);
            mech_trace::gw!(
                "10040f65",
                "100af348",
                DAT_100af348,
                mech_trace::gr!("10040f65", "100af350", DAT_100af350)
            );
            pu14 = mech_trace::gr!("10040f65", "100af350", DAT_100af350).wrapping_sub(6);
            if ad(pu14) < ad(param_3) {
                pu14 = mech_trace::gr!("10040f65", "100af340", DAT_100af340).wrapping_sub(6);
            }
            if (g(mech_trace::gr!("10040f65", "100af350", DAT_100af350), 1) as i32) < 0
                && (g(pu14, 1) as i32) < 1
            {
                continue;
            }
            mech_trace::gw!(
                "10040f65",
                "100af358",
                DAT_100af358,
                g(pu14, 1).wrapping_sub(g(mech_trace::gr!("10040f65", "100af350", DAT_100af350), 1))
                    as i32
            );
            if mech_trace::gr!("10040f65", "100af358", DAT_100af358) == 0 {
                continue;
            }
            break;
        }

        qstore(
            num_a(
                g(pu14, 0)
                    .wrapping_sub(g(mech_trace::gr!("10040f65", "100af350", DAT_100af350), 0)),
            ),
            mech_trace::gr!("10040f65", "100af358", DAT_100af358),
            &raw mut DAT_100af388,
        );
        qstore(
            num_b(
                g(pu14, 3)
                    .wrapping_sub(g(mech_trace::gr!("10040f65", "100af350", DAT_100af350), 3)),
            ),
            mech_trace::gr!("10040f65", "100af358", DAT_100af358),
            &raw mut DAT_100af398,
        );
        qstore(
            num_b(
                g(pu14, 4)
                    .wrapping_sub(g(mech_trace::gr!("10040f65", "100af350", DAT_100af350), 4)),
            ),
            mech_trace::gr!("10040f65", "100af358", DAT_100af358),
            &raw mut DAT_100af3a0,
        );

        mech_trace::gw!(
            "10040f65",
            "100af368",
            DAT_100af368,
            g(mech_trace::gr!("10040f65", "100af350", DAT_100af350), 0)
                .wrapping_mul(0x10000)
                .wrapping_add(0x8000) as i32
        );
        mech_trace::gw!(
            "10040f65",
            "100af378",
            DAT_100af378,
            g(mech_trace::gr!("10040f65", "100af350", DAT_100af350), 3).wrapping_add(0x8000) as i32
        );
        mech_trace::gw!(
            "10040f65",
            "100af380",
            DAT_100af380,
            g(mech_trace::gr!("10040f65", "100af350", DAT_100af350), 4).wrapping_add(0x8000) as i32
        );
        let mut pu18 = mech_trace::gr!("10040f65", "100af354", DAT_100af354);
        loop {
            mech_trace::gw!("10040f65", "100af354", DAT_100af354, pu18);
            mech_trace::gw!(
                "10040f65",
                "100af34c",
                DAT_100af34c,
                mech_trace::gr!("10040f65", "100af354", DAT_100af354)
            );
            pu18 = mech_trace::gr!("10040f65", "100af354", DAT_100af354).wrapping_add(6);
            if ad(mech_trace::gr!("10040f65", "100af340", DAT_100af340))
                <= ad(mech_trace::gr!("10040f65", "100af354", DAT_100af354).wrapping_add(6))
            {
                pu18 = param_3;
            }
            if (g(mech_trace::gr!("10040f65", "100af354", DAT_100af354), 1) as i32) < 0
                && (g(pu18, 1) as i32) < 1
            {
                continue;
            }
            mech_trace::gw!(
                "10040f65",
                "100af35c",
                DAT_100af35c,
                g(pu18, 1).wrapping_sub(g(mech_trace::gr!("10040f65", "100af354", DAT_100af354), 1))
                    as i32
            );
            if mech_trace::gr!("10040f65", "100af35c", DAT_100af35c) == 0 {
                continue;
            }
            break;
        }

        qstore(
            num_a(
                g(pu18, 0)
                    .wrapping_sub(g(mech_trace::gr!("10040f65", "100af354", DAT_100af354), 0)),
            ),
            mech_trace::gr!("10040f65", "100af35c", DAT_100af35c),
            &raw mut DAT_100af38c,
        );
        qstore(
            num_b(
                g(pu18, 3)
                    .wrapping_sub(g(mech_trace::gr!("10040f65", "100af354", DAT_100af354), 3)),
            ),
            mech_trace::gr!("10040f65", "100af35c", DAT_100af35c),
            &raw mut DAT_100af39c,
        );
        qstore(
            num_b(
                g(pu18, 4)
                    .wrapping_sub(g(mech_trace::gr!("10040f65", "100af354", DAT_100af354), 4)),
            ),
            mech_trace::gr!("10040f65", "100af35c", DAT_100af35c),
            &raw mut DAT_100af3a4,
        );

        mech_trace::gw!(
            "10040f65",
            "100af36c",
            DAT_100af36c,
            g(mech_trace::gr!("10040f65", "100af354", DAT_100af354), 0)
                .wrapping_mul(0x10000)
                .wrapping_add(0x8000) as i32
        );
        mech_trace::gw!(
            "10040f65",
            "100af37c",
            DAT_100af37c,
            g(mech_trace::gr!("10040f65", "100af354", DAT_100af354), 3).wrapping_add(0x8000) as i32
        );
        mech_trace::gw!(
            "10040f65",
            "100af384",
            DAT_100af384,
            g(mech_trace::gr!("10040f65", "100af354", DAT_100af354), 4).wrapping_add(0x8000) as i32
        );
        mech_trace::gw!(
            "10040f65",
            "_DAT_100af360",
            _DAT_100af360,
            (i_var7 as u32).wrapping_sub(u_var17) as i32
        );
        if u_var21.wrapping_sub(i_var7 as u32) == 0 || (u_var21 as i32) < i_var7 {
            mech_trace::gw!(
                "10040f65",
                "_DAT_100af360",
                _DAT_100af360,
                (mech_trace::gr!("10040f65", "_DAT_100af360", _DAT_100af360) as u32)
                    .wrapping_add(u_var21.wrapping_sub(i_var7 as u32)) as i32
            );
        }
        if 0x7fffffffu32 < u_var17 {
            mech_trace::gw!(
                "10040f65",
                "_DAT_100af360",
                _DAT_100af360,
                (mech_trace::gr!("10040f65", "_DAT_100af360", _DAT_100af360) as u32)
                    .wrapping_add(u_var17) as i32
            );
            mech_trace::gw!("10040f65", "100af364", DAT_100af364, 0);
            mech_trace::gw!(
                "10040f65",
                "100af358",
                DAT_100af358,
                (mech_trace::gr!("10040f65", "100af358", DAT_100af358) as u32)
                    .wrapping_add(g(mech_trace::gr!("10040f65", "100af350", DAT_100af350), 1))
                    as i32
            );
            i_var9 = g(mech_trace::gr!("10040f65", "100af350", DAT_100af350), 1)
                .wrapping_mul(0xffff0000) as i32;
            mech_trace::gw!(
                "10040f65",
                "100af368",
                DAT_100af368,
                (mech_trace::gr!("10040f65", "100af368", DAT_100af368) as u32).wrapping_add(fmul(
                    mech_trace::gr!("10040f65", "100af388", DAT_100af388),
                    i_var9
                )) as i32
            );
            mech_trace::gw!(
                "10040f65",
                "100af378",
                DAT_100af378,
                (mech_trace::gr!("10040f65", "100af378", DAT_100af378) as u32).wrapping_add(fmul(
                    mech_trace::gr!("10040f65", "100af398", DAT_100af398),
                    i_var9
                )) as i32
            );
            mech_trace::gw!(
                "10040f65",
                "100af380",
                DAT_100af380,
                (mech_trace::gr!("10040f65", "100af380", DAT_100af380) as u32).wrapping_add(fmul(
                    mech_trace::gr!("10040f65", "100af3a0", DAT_100af3a0),
                    i_var9
                )) as i32
            );
            mech_trace::gw!(
                "10040f65",
                "100af35c",
                DAT_100af35c,
                (mech_trace::gr!("10040f65", "100af35c", DAT_100af35c) as u32)
                    .wrapping_add(g(mech_trace::gr!("10040f65", "100af354", DAT_100af354), 1))
                    as i32
            );
            i_var9 = g(mech_trace::gr!("10040f65", "100af354", DAT_100af354), 1)
                .wrapping_mul(0xffff0000) as i32;
            mech_trace::gw!(
                "10040f65",
                "100af36c",
                DAT_100af36c,
                (mech_trace::gr!("10040f65", "100af36c", DAT_100af36c) as u32).wrapping_add(fmul(
                    mech_trace::gr!("10040f65", "100af38c", DAT_100af38c),
                    i_var9
                )) as i32
            );
            mech_trace::gw!(
                "10040f65",
                "100af37c",
                DAT_100af37c,
                (mech_trace::gr!("10040f65", "100af37c", DAT_100af37c) as u32).wrapping_add(fmul(
                    mech_trace::gr!("10040f65", "100af39c", DAT_100af39c),
                    i_var9
                )) as i32
            );
            mech_trace::gw!(
                "10040f65",
                "100af384",
                DAT_100af384,
                (mech_trace::gr!("10040f65", "100af384", DAT_100af384) as u32).wrapping_add(fmul(
                    mech_trace::gr!("10040f65", "100af3a4", DAT_100af3a4),
                    i_var9
                )) as i32
            );
        }
        mech_trace::gw!(
            "10040f65",
            "100af330",
            DAT_100af330,
            (mech_trace::gr!("10040f65", "100af364", DAT_100af364) as u32)
                .wrapping_mul(mech_trace::gr!("10040f65", "100af324", DAT_100af324) as u32)
                .wrapping_add(mech_trace::gr!("10040f65", "100af320", DAT_100af320) as u32)
                as i32
        );
        mech_trace::gw!(
            "10040f65",
            "_DAT_100af3c4",
            _DAT_100af3c4,
            mech_trace::rd!(
                "10040f65",
                (&raw mut PTR_DAT_100414a8)
                    .cast::<unsafe extern "C" fn()>()
                    .offset(param_5 as isize),
                read_unaligned
            )
        );
        i_var9 = mech_trace::gr!("10040f65", "100af368", DAT_100af368);
        i_var7 = mech_trace::gr!("10040f65", "100af378", DAT_100af378);
        let mut i_var13 = mech_trace::gr!("10040f65", "100af37c", DAT_100af37c);
        let mut i_var16 = mech_trace::gr!("10040f65", "100af36c", DAT_100af36c);
        let mut i_var20 = mech_trace::gr!("10040f65", "100af380", DAT_100af380);
        let mut i_var23 = mech_trace::gr!("10040f65", "100af384", DAT_100af384);
        mech_trace::gw!("10040f65", "100af350", DAT_100af350, pu14);
        mech_trace::gw!("10040f65", "100af354", DAT_100af354, pu18);
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
            u_var17 = (i_var5 >> 0x10) as u32;
            if (u_var17 as i32) <= i_var6 {
                i_var15 >>= 0x10;
                if -1 < i_var15 {
                    i_var9 = (i_var15 as u32).wrapping_sub(u_var17) as i32;
                    mech_trace::gw!("10040f65", "100af3a8", DAT_100af3a8, u_var17 as i32);
                    mech_trace::gw!("10040f65", "100af3ac", DAT_100af3ac, i_var15);
                    mech_trace::gw!("10040f65", "100af3bc", DAT_100af3bc, i_var10);
                    if i_var9 != 0 {
                        let u_var3 =
                            qdiv(num_b((i_var12 as u32).wrapping_sub(i_var10 as u32)), i_var9)
                                as u32;
                        mech_trace::gw!("10040f65", "100af3b4", DAT_100af3b4, u_var3);
                        u_var21 = mech_trace::gr!("10040f65", "100af3b4", DAT_100af3b4) >> 0x10;
                        i_var7 = 1;
                        if (u_var21 & 0x8000) != 0 {
                            i_var7 = -1;
                            u_var21 = (u_var21 | 0xffff0000)
                                .wrapping_add(1)
                                .wrapping_sub((((u_var3 & 0xffff) == 0) as u32));
                        }
                        let u_var3 =
                            qdiv(num_b((i_var22 as u32).wrapping_sub(i_var19 as u32)), i_var9)
                                as u32;
                        mech_trace::gw!("10040f65", "100af3b8", DAT_100af3b8, u_var3);
                        let mut s_var11 = (u_var3 >> 0x10) as i16;
                        mech_trace::gw!(
                            "10040f65",
                            "_DAT_100af3e4",
                            _DAT_100af3e4,
                            mech_trace::gr!("10040f65", "100af328", DAT_100af328)
                        );
                        if ((mech_trace::gr!("10040f65", "100af3b8", DAT_100af3b8) >> 0x10)
                            & 0x8000)
                            != 0
                        {
                            mech_trace::gw!(
                                "10040f65",
                                "_DAT_100af3e4",
                                _DAT_100af3e4,
                                0u32.wrapping_sub(mech_trace::gr!(
                                    "10040f65",
                                    "100af328",
                                    DAT_100af328
                                ) as u32) as i32
                            );
                            s_var11 = s_var11
                                .wrapping_add(1)
                                .wrapping_sub(((u_var3 & 0xffff) == 0) as u16 as i16);
                        }
                        mech_trace::gw!(
                            "10040f65",
                            "_DAT_100af3e0",
                            _DAT_100af3e0,
                            (mech_trace::gr!("10040f65", "100af328", DAT_100af328) as i16)
                                .wrapping_mul(s_var11) as i32
                        );
                        mech_trace::gw!(
                            "10040f65",
                            "_DAT_100af3d8",
                            _DAT_100af3d8,
                            u_var21.wrapping_add(mech_trace::gr!(
                                "10040f65",
                                "_DAT_100af3e0",
                                _DAT_100af3e0
                            ) as u32) as i32
                        );
                        mech_trace::gw!(
                            "10040f65",
                            "_DAT_100af3dc",
                            _DAT_100af3dc,
                            (mech_trace::gr!("10040f65", "_DAT_100af3d8", _DAT_100af3d8) as u32)
                                .wrapping_add(mech_trace::gr!(
                                    "10040f65",
                                    "_DAT_100af3e4",
                                    _DAT_100af3e4
                                ) as u32) as i32
                        );
                        mech_trace::gw!(
                            "10040f65",
                            "_DAT_100af3e0",
                            _DAT_100af3e0,
                            (i_var7 as u32)
                                .wrapping_add(u_var21)
                                .wrapping_add(mech_trace::gr!(
                                    "10040f65",
                                    "_DAT_100af3e0",
                                    _DAT_100af3e0
                                ) as u32) as i32
                        );
                        mech_trace::gw!(
                            "10040f65",
                            "_DAT_100af3e4",
                            _DAT_100af3e4,
                            (mech_trace::gr!("10040f65", "_DAT_100af3e0", _DAT_100af3e0) as u32)
                                .wrapping_add(mech_trace::gr!(
                                    "10040f65",
                                    "_DAT_100af3e4",
                                    _DAT_100af3e4
                                ) as u32) as i32
                        );
                        if 0x7fffffffu32 < u_var17 {
                            mech_trace::gw!("10040f65", "100af3a8", DAT_100af3a8, 0);
                            mech_trace::gw!(
                                "10040f65",
                                "100af3bc",
                                DAT_100af3bc,
                                (i_var10 as u32).wrapping_add(fmul(
                                    mech_trace::gr!("10040f65", "100af3b4", DAT_100af3b4) as i32,
                                    u_var17.wrapping_mul(0xffff0000) as i32,
                                )) as i32
                            );
                        }
                        if i_var15.wrapping_sub(i_var6) != 0 && i_var6 <= i_var15 {
                            mech_trace::gw!(
                                "10040f65",
                                "100af3ac",
                                DAT_100af3ac,
                                i_var15.wrapping_sub(i_var15.wrapping_sub(i_var6))
                            );
                        }
                    }
                    mech_trace::cr!(
                        "10040f65",
                        "ind",
                        (mech_trace::gr!("10040f65", "_DAT_100af3c4", _DAT_100af3c4))()
                    );
                    return;
                }
            }
            mech_trace::gw!(
                "10040f65",
                "100af330",
                DAT_100af330,
                (mech_trace::gr!("10040f65", "100af330", DAT_100af330) as u32)
                    .wrapping_add(mech_trace::gr!("10040f65", "100af324", DAT_100af324) as u32)
                    as i32
            );
            mech_trace::gw!(
                "10040f65",
                "_DAT_100af360",
                _DAT_100af360,
                mech_trace::gr!("10040f65", "_DAT_100af360", _DAT_100af360).wrapping_sub(1)
            );
            if mech_trace::gr!("10040f65", "_DAT_100af360", _DAT_100af360) < 0 {
                break;
            }
            if mech_trace::gr!("10040f65", "_DAT_100af360", _DAT_100af360) == 0 {
                i_var9 = (i_var9 as u32).wrapping_add(mech_trace::gr!(
                    "10040f65",
                    "100af388",
                    DAT_100af388
                ) as u32) as i32;
                i_var7 = (i_var7 as u32).wrapping_add(mech_trace::gr!(
                    "10040f65",
                    "100af398",
                    DAT_100af398
                ) as u32) as i32;
                i_var20 = (i_var20 as u32).wrapping_add(mech_trace::gr!(
                    "10040f65",
                    "100af3a0",
                    DAT_100af3a0
                ) as u32) as i32;
                i_var16 = (i_var16 as u32).wrapping_add(mech_trace::gr!(
                    "10040f65",
                    "100af38c",
                    DAT_100af38c
                ) as u32) as i32;
                i_var13 = (i_var13 as u32).wrapping_add(mech_trace::gr!(
                    "10040f65",
                    "100af39c",
                    DAT_100af39c
                ) as u32) as i32;
                i_var23 = (i_var23 as u32).wrapping_add(mech_trace::gr!(
                    "10040f65",
                    "100af3a4",
                    DAT_100af3a4
                ) as u32) as i32;
            } else {
                mech_trace::gw!(
                    "10040f65",
                    "100af358",
                    DAT_100af358,
                    mech_trace::gr!("10040f65", "100af358", DAT_100af358).wrapping_sub(1)
                );
                if mech_trace::gr!("10040f65", "100af358", DAT_100af358) == 0 {
                    mech_trace::gw!(
                        "10040f65",
                        "100af348",
                        DAT_100af348,
                        mech_trace::gr!("10040f65", "100af350", DAT_100af350)
                    );
                    pu14 = mech_trace::gr!("10040f65", "100af350", DAT_100af350).wrapping_sub(6);
                    if ad(pu14) < ad(param_3) {
                        pu14 =
                            mech_trace::gr!("10040f65", "100af340", DAT_100af340).wrapping_sub(6);
                    }
                    let d = g(pu14, 1)
                        .wrapping_sub(g(mech_trace::gr!("10040f65", "100af350", DAT_100af350), 1));
                    mech_trace::gw!(
                        "10040f65",
                        "100af358",
                        DAT_100af358,
                        d.wrapping_add((d == 0) as u32) as i32
                    );
                    qstore(
                        num_a(g(pu14, 0).wrapping_sub(g(
                            mech_trace::gr!("10040f65", "100af350", DAT_100af350),
                            0,
                        ))),
                        mech_trace::gr!("10040f65", "100af358", DAT_100af358),
                        &raw mut DAT_100af388,
                    );
                    qstore(
                        num_b(g(pu14, 3).wrapping_sub(g(
                            mech_trace::gr!("10040f65", "100af350", DAT_100af350),
                            3,
                        ))),
                        mech_trace::gr!("10040f65", "100af358", DAT_100af358),
                        &raw mut DAT_100af398,
                    );
                    qstore(
                        num_b(g(pu14, 4).wrapping_sub(g(
                            mech_trace::gr!("10040f65", "100af350", DAT_100af350),
                            4,
                        ))),
                        mech_trace::gr!("10040f65", "100af358", DAT_100af358),
                        &raw mut DAT_100af3a0,
                    );
                    i_var9 = g(mech_trace::gr!("10040f65", "100af350", DAT_100af350), 0)
                        .wrapping_mul(0x10000)
                        .wrapping_add(0x8000) as i32;
                    i_var7 = g(mech_trace::gr!("10040f65", "100af350", DAT_100af350), 3)
                        .wrapping_add(0x8000) as i32;
                    i_var20 = g(mech_trace::gr!("10040f65", "100af350", DAT_100af350), 4)
                        .wrapping_add(0x8000) as i32;
                    mech_trace::gw!("10040f65", "100af350", DAT_100af350, pu14);
                } else {
                    i_var9 = (i_var9 as u32).wrapping_add(mech_trace::gr!(
                        "10040f65",
                        "100af388",
                        DAT_100af388
                    ) as u32) as i32;
                    i_var7 = (i_var7 as u32).wrapping_add(mech_trace::gr!(
                        "10040f65",
                        "100af398",
                        DAT_100af398
                    ) as u32) as i32;
                    i_var20 = (i_var20 as u32).wrapping_add(mech_trace::gr!(
                        "10040f65",
                        "100af3a0",
                        DAT_100af3a0
                    ) as u32) as i32;
                }
                mech_trace::gw!(
                    "10040f65",
                    "100af35c",
                    DAT_100af35c,
                    mech_trace::gr!("10040f65", "100af35c", DAT_100af35c).wrapping_sub(1)
                );
                if mech_trace::gr!("10040f65", "100af35c", DAT_100af35c) == 0 {
                    mech_trace::gw!(
                        "10040f65",
                        "100af34c",
                        DAT_100af34c,
                        mech_trace::gr!("10040f65", "100af354", DAT_100af354)
                    );
                    pu14 = mech_trace::gr!("10040f65", "100af354", DAT_100af354).wrapping_add(6);
                    if ad(mech_trace::gr!("10040f65", "100af340", DAT_100af340))
                        <= ad(mech_trace::gr!("10040f65", "100af354", DAT_100af354).wrapping_add(6))
                    {
                        pu14 = param_3;
                    }
                    let d = g(pu14, 1)
                        .wrapping_sub(g(mech_trace::gr!("10040f65", "100af354", DAT_100af354), 1));
                    mech_trace::gw!(
                        "10040f65",
                        "100af35c",
                        DAT_100af35c,
                        d.wrapping_add((d == 0) as u32) as i32
                    );
                    qstore(
                        num_a(g(pu14, 0).wrapping_sub(g(
                            mech_trace::gr!("10040f65", "100af354", DAT_100af354),
                            0,
                        ))),
                        mech_trace::gr!("10040f65", "100af35c", DAT_100af35c),
                        &raw mut DAT_100af38c,
                    );
                    qstore(
                        num_b(g(pu14, 3).wrapping_sub(g(
                            mech_trace::gr!("10040f65", "100af354", DAT_100af354),
                            3,
                        ))),
                        mech_trace::gr!("10040f65", "100af35c", DAT_100af35c),
                        &raw mut DAT_100af39c,
                    );
                    qstore(
                        num_b(g(pu14, 4).wrapping_sub(g(
                            mech_trace::gr!("10040f65", "100af354", DAT_100af354),
                            4,
                        ))),
                        mech_trace::gr!("10040f65", "100af35c", DAT_100af35c),
                        &raw mut DAT_100af3a4,
                    );
                    i_var13 = g(mech_trace::gr!("10040f65", "100af354", DAT_100af354), 3)
                        .wrapping_add(0x8000) as i32;
                    i_var23 = g(mech_trace::gr!("10040f65", "100af354", DAT_100af354), 4)
                        .wrapping_add(0x8000) as i32;
                    i_var16 = g(mech_trace::gr!("10040f65", "100af354", DAT_100af354), 0)
                        .wrapping_mul(0x10000)
                        .wrapping_add(0x8000) as i32;
                    mech_trace::gw!("10040f65", "100af354", DAT_100af354, pu14);
                } else {
                    i_var16 = (i_var16 as u32).wrapping_add(mech_trace::gr!(
                        "10040f65",
                        "100af38c",
                        DAT_100af38c
                    ) as u32) as i32;
                    i_var13 = (i_var13 as u32).wrapping_add(mech_trace::gr!(
                        "10040f65",
                        "100af39c",
                        DAT_100af39c
                    ) as u32) as i32;
                    i_var23 = (i_var23 as u32).wrapping_add(mech_trace::gr!(
                        "10040f65",
                        "100af3a4",
                        DAT_100af3a4
                    ) as u32) as i32;
                }
            }
        }
    }
}
