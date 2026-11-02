// SPDX-License-Identifier: MIT

use core::arch::asm;

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10067a14(param_1: *mut i32, param_2: u8) -> u32;
    fn FUN_10067af3(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: *mut i32,
        param_5: i32,
        param_6: i32,
        param_7: u32,
    ) -> u32;
}

#[inline(always)]
unsafe fn machine_rem(a: i32, d: i32) -> i32 {
    let rem: i32;
    // SAFETY: plain idiv; faults like the original machine on a bad divisor.
    unsafe {
        asm!(
            "cdq",
            "idiv {d:e}",
            d = in(reg) d,
            inout("eax") a => _,
            out("edx") rem,
        );
    }
    rem
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10067e8f(
    param_1: *mut *mut u32,
    param_2: u32,
    param_3: u32,
    param_4: i32,
    param_5: u32,
) -> i32 {
    mech_trace::enter!("10067e8f", param_1, param_2, param_3, param_4, param_5);
    let mut param_2 = param_2;
    let mut param_3 = param_3;
    let p = param_1 as *mut i32;
    // SAFETY: param_1 points at the caller's rect structure; the callees are
    // the game's own, and the local buffers outlive every call using them.
    unsafe {
        let f1 = mech_trace::rd!("10067e8f", p.byte_add(4), read_unaligned);
        let f2 = mech_trace::rd!("10067e8f", p.byte_add(8), read_unaligned);
        let f3 = mech_trace::rd!("10067e8f", p.byte_add(12), read_unaligned);
        let f4 = mech_trace::rd!("10067e8f", p.byte_add(16), read_unaligned);
        let mut i_var1 = f3.wrapping_add(1).wrapping_sub(f1);
        if !(i_var1 != 0 && f1 <= f3.wrapping_add(1)) {
            return mech_trace::rt!("10067e8f", -2);
        }
        let i_var3 = f4.wrapping_add(1).wrapping_sub(f2);
        if !(i_var3 != 0 && f2 <= f4.wrapping_add(1)) {
            return mech_trace::rt!("10067e8f", -2);
        }
        let i_var2 = i_var1.wrapping_neg();
        let i_var4 = i_var3.wrapping_neg();
        let mut local_18: u32 = 0;
        let mut local_buf: [u32; 5] = [0; 5];
        let local_30: *mut i32;
        let local_48: u32;
        if param_4 == 1 {
            if param_5 == 0 {
                return mech_trace::rt!("10067e8f", i_var1.wrapping_mul(i_var3));
            }
            local_18 = param_5;
            local_buf[0] = (&raw mut local_18) as u32;
            local_buf[1] = 0;
            local_buf[2] = 0;
            local_buf[3] = i_var1.wrapping_sub(1) as u32;
            local_buf[4] = i_var3.wrapping_sub(1) as u32;
            local_30 = local_buf.as_mut_ptr() as *mut i32;
            mech_trace::cr!(
                "10067e8f",
                "10067af3",
                FUN_10067af3(p, 0, 0, local_30, 0, 0, 0xffff_ffff)
            );
            param_2 = machine_rem(param_2 as i32, i_var1) as u32;
            param_3 = machine_rem(param_3 as i32, i_var3) as u32;
            local_48 = 0xffff_ffff;
        } else {
            let a2 = param_2 as i32;
            let a3 = param_3 as i32;
            let abs2 = ((a2 ^ (a2 >> 31)).wrapping_sub(a2 >> 31)) as i32;
            let abs3 = ((a3 ^ (a3 >> 31)).wrapping_sub(a3 >> 31)) as i32;
            if i_var1 <= abs2 || i_var3 <= abs3 {
                return mech_trace::rt!(
                    "10067e8f",
                    mech_trace::cr!("10067e8f", "10067a14", FUN_10067a14(p, param_5 as u8)) as i32
                );
            }
            local_30 = p;
            local_48 = param_5;
        }
        if param_2 != 0 || param_3 != 0 {
            let x = param_2 as i32;
            let y = param_3 as i32;
            mech_trace::cr!(
                "10067e8f",
                "10067af3",
                FUN_10067af3(local_30, 0, 0, p, x, y, 0xffff_ffff)
            );
            mech_trace::cr!(
                "10067e8f",
                "10067af3",
                FUN_10067af3(local_30, i_var1, i_var3, p, x, y, local_48)
            );
            mech_trace::cr!(
                "10067e8f",
                "10067af3",
                FUN_10067af3(local_30, i_var1, 0, p, x, y, local_48)
            );
            mech_trace::cr!(
                "10067e8f",
                "10067af3",
                FUN_10067af3(local_30, i_var1, i_var4, p, x, y, local_48)
            );
            mech_trace::cr!(
                "10067e8f",
                "10067af3",
                FUN_10067af3(local_30, 0, i_var3, p, x, y, local_48)
            );
            mech_trace::cr!(
                "10067e8f",
                "10067af3",
                FUN_10067af3(local_30, 0, i_var4, p, x, y, local_48)
            );
            mech_trace::cr!(
                "10067e8f",
                "10067af3",
                FUN_10067af3(local_30, i_var2, i_var3, p, x, y, local_48)
            );
            mech_trace::cr!(
                "10067e8f",
                "10067af3",
                FUN_10067af3(local_30, i_var2, 0, p, x, y, local_48)
            );
            mech_trace::cr!(
                "10067e8f",
                "10067af3",
                FUN_10067af3(local_30, i_var2, i_var4, p, x, y, local_48)
            );
        }
        i_var1 = 0;
        mech_trace::rt!("10067e8f", i_var1)
    }
}
