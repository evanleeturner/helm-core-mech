// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000ffd0(param_1: u32) -> u32 {
    mech_trace::enter!("1000ffd0", param_1);
    let mut i_var2: i32 = 0x1f;
    if param_1 != 0 {
        while param_1.wrapping_shr(i_var2 as u32) == 0 {
            i_var2 = i_var2.wrapping_add(-1);
        }
    }
    mech_trace::rt!(
        "1000ffd0",
        if (i_var2 as u32).wrapping_sub(0xf) == 0 || i_var2 < 0xf {
            let mut s_var1: i16 = 0xf;
            if (param_1 as u16) != 0 {
                while ((param_1 as u16 as i32).wrapping_shr(s_var1 as i32 as u32) as u16) == 0 {
                    s_var1 = s_var1.wrapping_add(-1);
                }
            }
            let amt = ((0x10u32.wrapping_sub(s_var1 as i32 as u32) as u16) >> 1) as u8 & 0x1f;
            param_1.wrapping_shl(amt as u32)
        } else {
            let amt = (((i_var2 as u32).wrapping_sub(0xf)) >> 1) as u8 & 0x1f;
            param_1.wrapping_shr(amt as u32)
        }
    )
}
