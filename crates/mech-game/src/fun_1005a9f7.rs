// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005a9f7(param_1: i32, param_2: i32, param_3: *mut u32) {
    let l_var1: i64 = (param_1 as i64).wrapping_mul(param_2 as i64);
    let u_var1: u32 = l_var1 as u32;
    let u_var2: u16 = (u_var1.wrapping_add(0x8000) >> 0x10) as u16;
    let u_var3: u16 =
        (((l_var1 as u64) >> 0x20) as u16).wrapping_add((0xffff7fffu32 < u_var1) as u16);
    // SAFETY: param_3 points at the caller's writable u32 output slot.
    unsafe {
        param_3.write_unaligned(u32::from(u_var2) | (u32::from(u_var3) << 0x10));
    }
}
