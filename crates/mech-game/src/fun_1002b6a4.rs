// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10123130: [i32; 0];
    static mut DAT_10123369: [u8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002b6a4(param_1: i32, param_2: u32, param_3: i32) -> u32 {
    let _ = param_2;
    // SAFETY: the tables are the game's own memory; indices follow the original's
    // arithmetic and the 16-bit read is unaligned.
    unsafe {
        let base = (&raw mut DAT_10123130).cast::<i32>();
        let entry = base.offset(param_1 as isize).read_unaligned();
        let i_var1 = ((entry as i16) as i32 as u32)
            .wrapping_mul(0x13f)
            .wrapping_add((param_3 as u32).wrapping_mul(2));
        let off = i_var1.wrapping_add((param_1 as u32).wrapping_mul(0x3c0a)) as i32;
        let p = (&raw mut DAT_10123369).cast::<u8>().offset(off as isize);
        let value = p.cast::<u16>().read_unaligned();
        (i_var1 & 0xffff0000) | u32::from(value)
    }
}
