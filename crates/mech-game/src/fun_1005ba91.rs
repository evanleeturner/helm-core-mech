// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b24f9: u8;
    static mut DAT_100b1af9: u8;
    static mut DAT_100b17f9: u8;
    static mut DAT_100b1bf9: u8;
    static mut DAT_100b27f9: u8;
    static mut DAT_100b12a0: unsafe extern "C" fn(u32, *mut u8);
    static mut DAT_100b12a4: unsafe extern "C" fn(u32, *mut u8);
    static mut DAT_100b1294: unsafe extern "C" fn();
}

unsafe extern "stdcall" {
    fn RaiseException(code: u32, flags: u32, n: u32, args: *const u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005ba91(param_1: *mut u32, param_2: i32, param_3: u32) {
    mech_trace::enter!("1005ba91", param_1, param_2, param_3);
    // SAFETY: all globals and the buffers behind param_1/param_2 belong to the
    // game; they are accessed exactly where the original does.
    unsafe {
        let a24: *mut u8 = &raw mut DAT_100b24f9;
        let a1a: *mut u8 = &raw mut DAT_100b1af9;
        let a17: *mut u8 = &raw mut DAT_100b17f9;
        let a1b: *mut u8 = &raw mut DAT_100b1bf9;
        let a27: *mut u8 = &raw mut DAT_100b27f9;

        let mut p11 = a24 as *mut u32;
        let mut i6: i32 = 0x40;
        while i6 != 0 {
            mech_trace::wr!("1005ba91", p11, write_unaligned, 0);
            p11 = p11.add(1);
            i6 = i6.wrapping_sub(1);
        }
        let mut i6: i32 = (mech_trace::rd!("1005ba91", param_1.add(2), read_unaligned)
            .wrapping_add(1))
        .wrapping_mul(mech_trace::rd!("1005ba91", param_1.add(1), read_unaligned).wrapping_add(1))
            as i32;
        let mut local_8: u32 = 0xffffffff;
        let mut pb9 = mech_trace::rd!("1005ba91", param_1, read_unaligned) as *const u8;
        let mut pb12 = a1a;
        loop {
            let b3 = mech_trace::dr!("1005ba91", pb9);
            let u7 = b3 as u32;
            let mut pb13 = pb12;
            if mech_trace::dr!("1005ba91", a24.add(u7 as usize)) == 0 {
                mech_trace::du!("1005ba91", a24.add(u7 as usize), |=, 1);
                pb13 = pb12.add(1);
                mech_trace::dw!("1005ba91", pb12, b3);
                local_8 = local_8.wrapping_add(1);
                let f = mech_trace::gr!("1005ba91", "100b12a0", DAT_100b12a0);
                mech_trace::cr!("1005ba91", "ind", f(u7, a17.add((u7 as usize) * 3)));
            }
            i6 = i6.wrapping_sub(1);
            pb9 = pb9.add(1);
            pb12 = pb13;
            if i6 == 0 {
                break;
            }
        }
        let mut local_c: u32 = 0;
        let mut u7: u32 = local_8;
        loop {
            let mut i6: i32 =
                (mech_trace::dr!("1005ba91", a1a.add(u7 as usize)) as i32).wrapping_mul(3);
            let mut i8: i32 = 2;
            loop {
                let src = mech_trace::dr!("1005ba91", a17.add(i6 as usize));
                let other = mech_trace::dr!("1005ba91", ((i6.wrapping_add(param_2)) as *const u8));
                let mut b3 = src.wrapping_sub(other);
                let mut u5: u8 = 0xff;
                if (src as i8) < (other as i8) {
                    b3 = 0u8.wrapping_sub(b3);
                    u5 = 1;
                }
                mech_trace::dw!("1005ba91", a24.add(i6 as usize), u5);
                mech_trace::dw!("1005ba91", a1b.add(i6 as usize), b3);
                if ((local_c as u8) as i8) < (b3 as i8) {
                    local_c = b3 as u32;
                }
                i6 = i6.wrapping_add(1);
                i8 = i8.wrapping_sub(1);
                if !(-1 < i8) {
                    break;
                }
            }
            u7 = u7.wrapping_sub(1);
            if !(-1 < (u7 as i32)) {
                break;
            }
        }
        let b3: u8 = (local_c as u8) >> 1;
        let mut pb9 = a27;
        let mut u7: u32 = local_8 >> 1;
        while u7 != 0 {
            mech_trace::dw!("1005ba91", pb9, b3);
            mech_trace::dw!("1005ba91", pb9.add(1), b3);
            pb9 = pb9.add(2);
            u7 = u7.wrapping_sub(1);
        }
        let mut u7: u32 = ((local_8 & 1) != 0) as u32;
        while u7 != 0 {
            mech_trace::dw!("1005ba91", pb9, b3);
            pb9 = pb9.add(1);
            u7 = u7.wrapping_sub(1);
        }
        if local_c != 0 {
            let mut local_14: u32 = 0x8000;
            let mut u7: u32 = local_8;
            let mut u10: u32 = local_c;
            loop {
                loop {
                    let i6: i32 =
                        (mech_trace::dr!("1005ba91", a1a.add(u7 as usize)) as i32).wrapping_mul(3);
                    let mut i8: i32 = 2;
                    loop {
                        let idx = i6.wrapping_add(i8) as usize;
                        let mut c4 = (mech_trace::dr!("1005ba91", a27.add(idx)))
                            .wrapping_add(mech_trace::dr!("1005ba91", a1b.add(idx)))
                            as i8;
                        if ((local_c as u8) as i8) <= c4 {
                            c4 = (c4 as u8).wrapping_sub(local_c as u8) as i8;
                            mech_trace::dw!(
                                "1005ba91",
                                a17.add(idx),
                                (mech_trace::dr!("1005ba91", a17.add(idx)))
                                    .wrapping_add(mech_trace::dr!("1005ba91", a24.add(idx)))
                            );
                        }
                        mech_trace::dw!("1005ba91", a27.add(idx), c4 as u8);
                        i8 = i8.wrapping_sub(1);
                        if !(-1 < i8) {
                            break;
                        }
                    }
                    let f = mech_trace::gr!("1005ba91", "100b12a4", DAT_100b12a4);
                    mech_trace::cr!(
                        "1005ba91",
                        "ind",
                        f(
                            mech_trace::dr!("1005ba91", a1a.add(u7 as usize)) as u32,
                            a17.add(i6 as usize)
                        )
                    );
                    u7 = u7.wrapping_sub(1);
                    if !(-1 < (u7 as i32)) {
                        break;
                    }
                }
                let hi = param_3 >> 0x10;
                let lo = param_3.wrapping_shl(0x10);
                if local_c == 0 || hi >= local_c {
                    RaiseException(0xC0000094, 0, 0, core::ptr::null());
                }
                let quot: u32;
                mech_trace::log::debug!(
                    f = "1005ba91", b = "div", a:% = mech_trace::Args([
                        mech_trace::__shown!(local_c),
                        mech_trace::__shown!(lo),
                        mech_trace::__shown!(hi),
                    ]); "asm.in"
                );
                core::arch::asm!(
                    "div {d:e}",
                    d = in(reg) local_c,
                    inout("eax") lo => quot,
                    inout("edx") hi => _,
                );
                mech_trace::log::debug!(
                    f = "1005ba91", b = "div", r = "eax",
                    v:% = mech_trace::__shown!(quot); "asm.out"
                );
                let i6: i32 = local_14.wrapping_add(quot) as i32;
                let b2 = 0 < ((i6 >> 0x10) as i16);
                local_14 = i6 as u32;
                if b2 {
                    loop {
                        let f = mech_trace::gr!("1005ba91", "100b1294", DAT_100b1294);
                        mech_trace::cr!("1005ba91", "ind", f());
                        let s1 = ((local_14 >> 0x10) as i16).wrapping_sub(1);
                        local_14 = ((s1 as u16 as u32) << 16) | (local_14 & 0xffff);
                        if s1 == 0 {
                            break;
                        }
                    }
                }
                u10 = u10.wrapping_sub(1);
                u7 = local_8;
                if u10 == 0 {
                    break;
                }
            }
        }
    }
}
