// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002950f(param_1: i32, param_2: i16) -> u8;
    fn FUN_1002a132(param_1: i32) -> i32;
    fn FUN_10022b70(param_1: i32, param_2: i32) -> i32;
    fn FUN_10044a2b(param_1: i32) -> i32;
    fn FUN_100449ef(param_1: i32, param_2: i16) -> u8;
    fn FUN_1004403d(param_1: i32, param_2: u8) -> u8;
    fn FUN_10044bba(param_1: i32, param_2: i32) -> u32;
}

#[inline(always)]
unsafe fn rd(addr: i32) -> i32 {
    // SAFETY: the caller guarantees addr points at a readable i32 in the game's memory.
    unsafe { (addr as *const i32).read_unaligned() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004398b(param_1: i32, param_2: i16) -> u32 {
    // SAFETY: param_1 points at the game's object; all offsets and callees are the game's own.
    unsafe {
        FUN_1002950f(param_1, param_2);
        let u_var1 = FUN_1002a132(param_1);
        FUN_10022b70(param_1, u_var1);
        let mut local_c: u32;
        if rd(param_1.wrapping_add(0x78)) == 0
            && rd(rd(param_1.wrapping_add(0x20)).wrapping_add(0x3d8)) == 0
        {
            local_c = 0;
        } else {
            local_c = 1;
        }
        if rd(param_1.wrapping_add(0x1d4)) != 2 {
            let sign = u_var1 >> 0x1f;
            let i_var2 = (u_var1 ^ sign).wrapping_sub(sign);
            let lim = rd(rd(param_1.wrapping_add(0x20)).wrapping_add(0x414));
            let lim3 = rd(rd(param_1.wrapping_add(0x20)).wrapping_add(0x414)).wrapping_mul(3);
            let local_14: i16 = if lim < i_var2 && i_var2 < lim3 { 1 } else { 0 };
            if rd(param_1.wrapping_add(0xc0)) < 0x3e9 {
                let p4c = rd(param_1.wrapping_add(0x4c));
                if (p4c.wrapping_add(0x1d) as *const i8).read() != 0 {
                    let v = rd(param_1.wrapping_add(0x1d4)).wrapping_add(1);
                    (param_1.wrapping_add(0x1d4) as *mut i32).write_unaligned(v);
                }
                FUN_1004403d(param_1, 0);
                let p4c = rd(param_1.wrapping_add(0x4c));
                (p4c.wrapping_add(0x20) as *mut u8).write(0);
                let p4c = rd(param_1.wrapping_add(0x4c));
                (p4c.wrapping_add(0x21) as *mut u8).write(0);
            } else {
                let i_var2 = FUN_10044a2b(param_1);
                let c0 = rd(param_1.wrapping_add(0xc0));
                let thr = ((((c0 < 0x1771) as i32).wrapping_sub(1)) & 0x1b).wrapping_add(9);
                if thr < i_var2 {
                    FUN_100449ef(param_1, local_14);
                } else {
                    FUN_100449ef(param_1, (local_14 == 0) as i16);
                }
                FUN_1004403d(param_1, 1);
                let r = FUN_10044bba(param_1, 0x28);
                if r as i32 == 0 {
                    local_c = 1;
                }
            }
        }
        local_c
    }
}
