// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002950f(param_1: i32, param_2: i16) -> u8;
    fn FUN_1002a132(param_1: i32) -> i32;
    fn FUN_10044bba(param_1: i32, param_2: i32) -> u32;
    fn FUN_1004403d(param_1: i32, param_2: u8) -> u8;
}

#[inline(always)]
unsafe fn rd<T>(addr: i32) -> T {
    // SAFETY: the caller guarantees the address is readable game memory.
    unsafe { mech_trace::rd!("10044a83", (addr as usize as *const T), read_unaligned) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10044a83(param_1: i32) {
    mech_trace::enter!("10044a83", param_1);
    // SAFETY: param_1 points at the game's object; all offsets read are the
    // original's, and the callees are the game's own.
    unsafe {
        let mut i_var3: i32 = rd::<i32>(param_1.wrapping_add(0x20));
        if (rd::<u8>(param_1.wrapping_add(0x1f2)) & 1) != 0
            && rd::<i16>(param_1.wrapping_add(0x1e4)) == 0
            && rd::<i16>(param_1.wrapping_add(0x1c4)) != 10
        {
            mech_trace::cr!(
                "10044a83",
                "1002950f",
                FUN_1002950f(param_1, rd::<i16>(param_1.wrapping_add(0x178)))
            );
            let u_var1 = mech_trace::cr!("10044a83", "1002a132", FUN_1002a132(param_1)) as u32;
            let sign = (u_var1 as i32) >> 0x1f;
            let i_var2 = (u_var1 ^ (sign as u32)).wrapping_sub(sign as u32) as i32;
            let limit = rd::<i32>(i_var3.wrapping_add(0x414));
            if i_var2 < limit
                && (limit.wrapping_neg() < i_var2
                    || rd::<i32>(rd::<i32>(param_1.wrapping_add(0x4c)).wrapping_add(8)) == 0)
            {
                if rd::<i32>(param_1.wrapping_add(0x1ea)) == 0
                    && rd::<i8>(rd::<i32>(param_1.wrapping_add(0x4c)).wrapping_add(0x1d)) != 0
                    && rd::<i32>(param_1.wrapping_add(0x78)) != 0
                {
                    mech_trace::cr!("10044a83", "1004403d", FUN_1004403d(param_1, 0));
                }
            } else {
                i_var3 =
                    mech_trace::cr!("10044a83", "10044bba", FUN_10044bba(param_1, 0x14)) as i32;
                if i_var3 != 0
                    && rd::<i32>(param_1.wrapping_add(0x78)) != 0
                    && rd::<i8>(rd::<i32>(param_1.wrapping_add(0x4c)).wrapping_add(0x1d)) == 0
                {
                    mech_trace::cr!("10044a83", "1004403d", FUN_1004403d(param_1, 1));
                }
            }
        }
    }
}
