// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10167250: u8;
    static mut DAT_10167254: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10006b20(param_1: i32) -> i32 {
    mech_trace::enter!("10006b20", param_1);
    let prod = (param_1 as i64).wrapping_mul(0x5b05b05b);
    let lo = prod as u32;
    let hi = ((prod as u64) >> 32) as u32;
    let u_var3 = ((lo >> 0x1d) | hi.wrapping_shl(3)).wrapping_add(((lo >> 0x1c) & 1 != 0) as u32);
    let mut u_var5 = (u_var3 >> 0xe) & 0x3fc;
    let mut u_var2 = u_var3;
    if (u_var3 & 0x1000000) != 0 {
        u_var2 = !u_var3;
        u_var5 ^= 0x3fc;
    }
    // SAFETY: the tables are the game's own memory; u_var5 is a masked
    // byte offset into them, read where the original reads.
    unsafe {
        let a = mech_trace::rd!(
            "10006b20",
            (&raw mut DAT_10167250)
                .cast::<u8>()
                .add(u_var5 as usize)
                .cast::<i32>(),
            read_unaligned
        );
        let b = mech_trace::rd!(
            "10006b20",
            (&raw mut DAT_10167254)
                .cast::<u8>()
                .add(u_var5 as usize)
                .cast::<i32>(),
            read_unaligned
        );
        let diff = (b as u32).wrapping_sub(a as u32);
        let l_var1 = (diff as u64).wrapping_mul((u_var2 & 0xffff) as u64);
        let lo2 = l_var1 as u32;
        let hi2 = (l_var1 >> 32) as u32;
        let a2 = mech_trace::rd!(
            "10006b20",
            (&raw mut DAT_10167250)
                .cast::<u8>()
                .add(u_var5 as usize)
                .cast::<i32>(),
            read_unaligned
        );
        let mut i_var4 = (a2 as u32)
            .wrapping_add((lo2 >> 0x10) | hi2.wrapping_shl(0x10))
            .wrapping_add(((lo2 >> 0xf) & 1 != 0) as u32) as i32;
        if (u_var3 & 0x2000000) != 0 {
            i_var4 = i_var4.wrapping_neg();
        }
        mech_trace::rt!("10006b20", i_var4)
    }
}
