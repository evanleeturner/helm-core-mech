// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10058e74(param_1: *mut i32, param_2: u8) -> u32;
    fn FUN_10058f53(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: *mut i32,
        param_5: i32,
        param_6: i32,
        param_7: u32,
    ) -> u32;
}

unsafe fn idiv_rem(a: i32, b: i32) -> i32 {
    let r: i32;
    // SAFETY: the divisor is nonzero per the caller's checks; the machine's own
    // idiv semantics (including its fault) are used.
    unsafe {
        core::arch::asm!(
            "cdq",
            "idiv {b:e}",
            b = in(reg) b,
            inout("eax") a => _,
            out("edx") r,
            options(nostack)
        );
    }
    r
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100592ef(
    param_1: *mut *mut u32,
    param_2: u32,
    param_3: u32,
    param_4: i32,
    param_5: u32,
) -> i32 {
    let mut param_2 = param_2;
    let mut param_3 = param_3;
    let p = param_1 as *mut i32;
    // SAFETY: param_1 points at the caller's structure of pointer-sized fields;
    // the callees are the game's own, and the locals live through each call.
    unsafe {
        let f1 = p.add(1).read_unaligned();
        let f2 = p.add(2).read_unaligned();
        let f3 = p.add(3).read_unaligned();
        let f4 = p.add(4).read_unaligned();
        let mut i_var1 = f3.wrapping_add(1).wrapping_sub(f1);
        if i_var1 != 0 && f1 <= f3.wrapping_add(1) {
            let i_var3 = f4.wrapping_add(1).wrapping_sub(f2);
            if i_var3 != 0 && f2 <= f4.wrapping_add(1) {
                let i_var2 = i_var1.wrapping_neg();
                let i_var4 = i_var3.wrapping_neg();
                let mut local_18: u32 = 0;
                let mut local_blk: [i32; 5] = [0; 5];
                let local_30: *mut i32;
                let local_48: u32;
                if param_4 == 1 {
                    if param_5 == 0 {
                        return i_var1.wrapping_mul(i_var3);
                    }
                    local_18 = param_5;
                    local_blk[0] = (&raw mut local_18) as usize as i32;
                    local_blk[1] = 0;
                    local_blk[2] = 0;
                    local_blk[3] = i_var1.wrapping_sub(1);
                    local_blk[4] = i_var3.wrapping_sub(1);
                    local_30 = local_blk.as_mut_ptr();
                    FUN_10058f53(p, 0, 0, local_30, 0, 0, 0xffff_ffff);
                    param_2 = idiv_rem(param_2 as i32, i_var1) as u32;
                    param_3 = idiv_rem(param_3 as i32, i_var3) as u32;
                    local_48 = 0xffff_ffff;
                } else {
                    let a2 = param_2 as i32;
                    let a3 = param_3 as i32;
                    let abs2 = (a2 ^ (a2 >> 0x1f)).wrapping_sub(a2 >> 0x1f);
                    let abs3 = (a3 ^ (a3 >> 0x1f)).wrapping_sub(a3 >> 0x1f);
                    if i_var1 <= abs2 || i_var3 <= abs3 {
                        i_var1 = FUN_10058e74(p, param_5 as u8) as i32;
                        return i_var1;
                    }
                    local_30 = p;
                    local_48 = param_5;
                }
                if param_2 != 0 || param_3 != 0 {
                    let a = param_2 as i32;
                    let b = param_3 as i32;
                    FUN_10058f53(local_30, 0, 0, p, a, b, 0xffff_ffff);
                    FUN_10058f53(local_30, i_var1, i_var3, p, a, b, local_48);
                    FUN_10058f53(local_30, i_var1, 0, p, a, b, local_48);
                    FUN_10058f53(local_30, i_var1, i_var4, p, a, b, local_48);
                    FUN_10058f53(local_30, 0, i_var3, p, a, b, local_48);
                    FUN_10058f53(local_30, 0, i_var4, p, a, b, local_48);
                    FUN_10058f53(local_30, i_var2, i_var3, p, a, b, local_48);
                    FUN_10058f53(local_30, i_var2, 0, p, a, b, local_48);
                    FUN_10058f53(local_30, i_var2, i_var4, p, a, b, local_48);
                }
                return 0;
            }
        }
    }
    -2
}
