// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aefc4: u32;
    static mut DAT_100c6d5c: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003d6bf(param_1: i32, param_2: i32) -> u32 {
    mech_trace::enter!("1003d6bf", param_1, param_2);
    let i_var1 = param_1.wrapping_add(150000) / 25000;
    let i_var2 = param_2.wrapping_add(150000) / 25000;
    // SAFETY: the globals are the game's own memory; the table index is
    // bounds-checked by the conditions below before it is read.
    unsafe {
        let mut u_var3 = mech_trace::gr!("1003d6bf", "100aefc4", DAT_100aefc4);
        if (-1 < i_var1) && (i_var1 < 0xc) && (-1 < i_var2) && (i_var2 < 0xc) {
            let off = (i_var2.wrapping_mul(0x18)).wrapping_add(i_var1.wrapping_mul(0x120));
            u_var3 = mech_trace::rd!(
                "1003d6bf",
                (&raw mut DAT_100c6d5c).byte_add(off as usize).cast::<u32>(),
                read_unaligned
            );
        }
        mech_trace::rt!("1003d6bf", u_var3)
    }
}
