// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10160eb4: u8;
    static mut DAT_10160eb8: u8;
    static mut DAT_10160ec4: i32;
    static mut DAT_10160ec0: i32;
    static mut DAT_10160edc: i32;
    static mut DAT_10160ee8: i32;
    static mut DAT_10160ed0: i32;
    static mut DAT_10160ee0: i32;
    static mut DAT_101150f8: u8;
    static mut DAT_10115168: u8;
    static mut DAT_10115160: i32;
    static mut DAT_1011515c: i32;
    static mut DAT_10115100: [*mut u32; 20];
}

#[inline(always)]
unsafe fn idiv64(num: i64, div: i32) -> i32 {
    let lo = num as u32;
    let hi = (num >> 32) as u32;
    let q: u32;
    // SAFETY: the machine's idiv faults on a zero divisor or an overflowing
    // quotient, exactly as the original does.
    unsafe {
        mech_trace::log::debug!(
            f = "10080639", b = "idiv", a:% = mech_trace::Args([
                mech_trace::__shown!(div),
                mech_trace::__shown!(lo),
                mech_trace::__shown!(hi),
            ]); "asm.in"
        );
        core::arch::asm!(
            "idiv {d:e}",
            d = in(reg) div,
            inout("eax") lo => q,
            inout("edx") hi => _,
            options(nomem, nostack)
        );
        mech_trace::log::debug!(
            f = "10080639", b = "idiv", r = "eax", v:% = mech_trace::__shown!(q); "asm.out"
        );
    }
    q as i32
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10080639(param_1: *mut u32) -> *mut u32 {
    mech_trace::enter!("10080639", param_1);
    // SAFETY: param_1 points at the caller's point record; the globals and
    // the table are the game's own memory.
    unsafe {
        let p8 = param_1.cast::<u8>();
        if (mech_trace::rd!("10080639", p8.add(0x1d), read) & 1) == 0 {
            let sh = (mech_trace::gr!("10080639", "10160eb4", DAT_10160eb4) & 0x1f) as u32;
            let num = ((mech_trace::rd!("10080639", param_1, read_unaligned) as i32 as i64) as u64)
                .wrapping_shl(sh) as i64;
            let div = mech_trace::rd!("10080639", param_1.add(2), read_unaligned) as i32;
            let q = idiv64(num, div);
            let x = (q.wrapping_add(2) >> 2).wrapping_add(mech_trace::gr!(
                "10080639",
                "10160ec4",
                DAT_10160ec4
            ));
            mech_trace::wr!("10080639", param_1.add(3), write_unaligned, x as u32);

            let mut b: u8 = 0;
            if mech_trace::gr!("10080639", "10160edc", DAT_10160edc) < x {
                b = 2;
            }
            if x < mech_trace::gr!("10080639", "10160ec0", DAT_10160ec0) {
                b |= 1;
            }

            let sh = (mech_trace::gr!("10080639", "10160eb8", DAT_10160eb8) & 0x1f) as u32;
            let num = ((mech_trace::rd!("10080639", param_1.add(1), read_unaligned) as i32 as i64)
                as u64)
                .wrapping_shl(sh) as i64;
            let div = mech_trace::rd!("10080639", param_1.add(2), read_unaligned) as i32;
            let q = idiv64(num, div);
            let y = mech_trace::gr!("10080639", "10160ee8", DAT_10160ee8)
                .wrapping_sub(q.wrapping_add(2) >> 2);
            mech_trace::wr!("10080639", param_1.add(4), write_unaligned, y as u32);

            if mech_trace::gr!("10080639", "10160ed0", DAT_10160ed0) < y {
                b |= 8;
            }
            if y < mech_trace::gr!("10080639", "10160ee0", DAT_10160ee0) {
                b |= 4;
            }
            mech_trace::wr!("10080639", p8.add(0x1c), write, b);
            let f = mech_trace::rd!("10080639", p8.add(0x1d), read);
            mech_trace::wr!("10080639", p8.add(0x1d), write, f | 1);
        }

        let oc = mech_trace::rd!("10080639", p8.add(0x1c), read);
        mech_trace::gu!("10080639", "101150f8", DAT_101150f8, |=, oc);
        mech_trace::gu!("10080639", "10115168", DAT_10115168, &=, mech_trace::rd!("10080639", p8.add(0x1c), read));

        if mech_trace::gr!("10080639", "10115160", DAT_10115160) < 0x14 {
            mech_trace::wr!(
                "10080639",
                (&raw mut DAT_10115100)
                    .cast::<*mut u32>()
                    .offset(mech_trace::gr!("10080639", "10115160", DAT_10115160) as isize),
                write,
                param_1
            );
            mech_trace::gw!(
                "10080639",
                "10115160",
                DAT_10115160,
                mech_trace::gr!("10080639", "10115160", DAT_10115160).wrapping_add(1)
            );
        } else {
            mech_trace::gw!("10080639", "1011515c", DAT_1011515c, 0);
        }
        mech_trace::rt!("10080639", param_1)
    }
}
