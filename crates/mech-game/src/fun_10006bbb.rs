// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10167250: i32;
    static mut DAT_10167450: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RaiseException(a: u32, b: u32, c: u32, d: *const u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10006bbb(param_1: u32) -> i32 {
    let mut param_1 = param_1;
    let mut local_c: i32;

    if param_1 == 0 {
        local_c = 0;
    } else {
        let b_var5 = (param_1 as i32) < 0;
        if b_var5 {
            param_1 = 0u32.wrapping_sub(param_1);
        }
        if (param_1 as i32) < 0x20000000 {
            // SAFETY: the game's own lookup table starts at DAT_10167250 and
            // spans the offsets read here; the callee is the system routine.
            unsafe {
                let mut p3: *const u32 = (&raw mut DAT_10167250) as *const u32;
                let mut u_var2: u32 = 0;
                if DAT_10167450 <= param_1 {
                    p3 = (&raw mut DAT_10167450) as *const u32;
                    u_var2 = 0x800000;
                }
                if p3.add(0x40).read_unaligned() <= param_1 {
                    p3 = p3.add(0x40);
                    u_var2 |= 0x400000;
                }
                if p3.add(0x20).read_unaligned() <= param_1 {
                    p3 = p3.add(0x20);
                    u_var2 |= 0x200000;
                }
                if p3.add(0x10).read_unaligned() <= param_1 {
                    p3 = p3.add(0x10);
                    u_var2 |= 0x100000;
                }
                if p3.add(8).read_unaligned() <= param_1 {
                    p3 = p3.add(8);
                    u_var2 |= 0x80000;
                }
                if p3.add(4).read_unaligned() <= param_1 {
                    p3 = p3.add(4);
                    u_var2 |= 0x40000;
                }
                if p3.add(2).read_unaligned() <= param_1 {
                    p3 = p3.add(2);
                    u_var2 |= 0x20000;
                }
                if p3.add(1).read_unaligned() <= param_1 {
                    p3 = p3.add(1);
                    u_var2 |= 0x10000;
                }
                let u_var1 = param_1.wrapping_sub(p3.read_unaligned());
                if u_var1 != 0 {
                    let u_var4 = p3.add(1).read_unaligned().wrapping_sub(p3.read_unaligned());
                    if u_var4 == 0 || u_var1 < u_var4 {
                        let n: i64 = ((u_var1 as i32 as i64) as u64).wrapping_shl(16) as i64;
                        let d: i32 = u_var4 as i32;
                        let mut q: i64 = 0;
                        if d == 0 {
                            RaiseException(0xC0000094, 0, 0, core::ptr::null());
                        } else {
                            q = n / (d as i64);
                            if q != (q as i32 as i64) {
                                RaiseException(0xC0000094, 0, 0, core::ptr::null());
                            }
                        }
                        u_var2 = (((u_var2 >> 16) & 0xFFFF) << 16) | ((q as u32) & 0xFFFF);
                    } else {
                        u_var2 = u_var2.wrapping_add(0x10000);
                    }
                }
                local_c = ((u_var2 as i32 as i64).wrapping_mul(0x5a000000i64) >> 32) as i32;
            }
        } else {
            local_c = 0x5a0000;
        }
        if b_var5 {
            local_c = 0u32.wrapping_sub(local_c as u32) as i32;
        }
    }
    local_c
}
