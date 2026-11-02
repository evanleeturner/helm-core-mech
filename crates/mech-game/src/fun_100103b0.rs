// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100103b0(param_1: *const u8) -> i32 {
    let mut p = param_1;
    let mut u_var2: u32 = 0;
    // SAFETY: param_1 points at a NUL-terminated byte string owned by the caller.
    unsafe {
        while *p != 0 {
            u_var2 = u_var2.wrapping_add(u32::from(*p | 0x20));
            let s_var1 = u_var2 as u16;
            let rot = s_var1.wrapping_shl(1) | s_var1.wrapping_shr(15);
            u_var2 = (u_var2 & 0xffff0000) | u32::from(rot);
            p = p.add(1);
        }
    }
    u_var2 as i32
}
