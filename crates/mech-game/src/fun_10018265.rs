// SPDX-License-Identifier: MIT

use core::arch::asm;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aae5c: i32;
    static mut DAT_100aae60: i32;
    static mut DAT_100aaf68: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10017f0b(param_1: i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32) -> u8;
}

/// Machine signed divide: returns (quotient, remainder).
#[inline(always)]
unsafe fn idiv(a: i32, d: i32) -> (i32, i32) {
    let q: i32;
    let r: i32;
    // SAFETY: executes the machine's idiv, faulting as the original does on
    // a zero divisor or overflow.
    unsafe {
        asm!(
            "cdq",
            "idiv ecx",
            inout("eax") a => q,
            in("ecx") d,
            out("edx") r,
            options(nostack)
        );
    }
    (q, r)
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10018265(param_1: i32, param_2: i32, param_3: i32) {
    // SAFETY: the globals and callee are the game's own; the pointer held in
    // DAT_100aaf68 points at the game's position data.
    unsafe {
        let p = DAT_100aaf68 as *const u8;
        let i_var1: i32 = p.cast::<i32>().read_unaligned();
        let i_var2: i32 = p.byte_add(8).cast::<i32>().read_unaligned();

        let local_1c: i32 = if i_var1 < 0 {
            idiv(i_var1, DAT_100aae5c).0.wrapping_sub(1)
        } else {
            idiv(i_var1, DAT_100aae5c).0
        };
        let local_20: i32 = if i_var2 < 0 {
            idiv(i_var2, DAT_100aae5c).0.wrapping_sub(1)
        } else {
            idiv(i_var2, DAT_100aae5c).0
        };
        FUN_10017f0b(param_1, param_2, param_3, local_1c, local_20);

        let local_28: i32 = if i_var1 < 0 { -1 } else { 1 };
        let local_c: i32 = if i_var2 < 0 { -1 } else { 1 };

        let r1 = idiv(i_var1, DAT_100aae5c).1;
        let u_var3 = idiv(r1, DAT_100aae60).0 as u32;
        let u_var5 = ((u_var3 as i32) >> 0x1f) as u32;
        let r2 = idiv(i_var2, DAT_100aae5c).1;
        let u_var4 = idiv(r2, DAT_100aae60).0 as u32;
        let u_var6 = ((u_var4 as i32) >> 0x1f) as u32;

        let sel: u32 = ((u_var3 ^ u_var5).wrapping_sub(u_var5))
            .wrapping_mul(2)
            .wrapping_add((u_var4 ^ u_var6).wrapping_sub(u_var6));

        match sel {
            0 => {
                FUN_10017f0b(
                    param_1,
                    param_2,
                    param_3,
                    local_1c.wrapping_sub(local_28),
                    local_20,
                );
                FUN_10017f0b(
                    param_1,
                    param_2,
                    param_3,
                    local_1c.wrapping_sub(local_28),
                    local_20.wrapping_sub(local_c),
                );
                FUN_10017f0b(
                    param_1,
                    param_2,
                    param_3,
                    local_1c,
                    local_20.wrapping_sub(local_c),
                );
            }
            1 => {
                FUN_10017f0b(
                    param_1,
                    param_2,
                    param_3,
                    local_1c.wrapping_sub(local_28),
                    local_20,
                );
                FUN_10017f0b(
                    param_1,
                    param_2,
                    param_3,
                    local_1c.wrapping_sub(local_28),
                    local_c.wrapping_add(local_20),
                );
                FUN_10017f0b(
                    param_1,
                    param_2,
                    param_3,
                    local_1c,
                    local_c.wrapping_add(local_20),
                );
            }
            2 => {
                FUN_10017f0b(
                    param_1,
                    param_2,
                    param_3,
                    local_28.wrapping_add(local_1c),
                    local_20,
                );
                FUN_10017f0b(
                    param_1,
                    param_2,
                    param_3,
                    local_28.wrapping_add(local_1c),
                    local_20.wrapping_sub(local_c),
                );
                FUN_10017f0b(
                    param_1,
                    param_2,
                    param_3,
                    local_1c,
                    local_20.wrapping_sub(local_c),
                );
            }
            3 => {
                FUN_10017f0b(
                    param_1,
                    param_2,
                    param_3,
                    local_28.wrapping_add(local_1c),
                    local_20,
                );
                FUN_10017f0b(
                    param_1,
                    param_2,
                    param_3,
                    local_28.wrapping_add(local_1c),
                    local_c.wrapping_add(local_20),
                );
                FUN_10017f0b(
                    param_1,
                    param_2,
                    param_3,
                    local_1c,
                    local_c.wrapping_add(local_20),
                );
            }
            _ => {}
        }
    }
}
