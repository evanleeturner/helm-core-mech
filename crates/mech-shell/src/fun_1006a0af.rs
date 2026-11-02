// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1006a0af(param_1: i32) -> i32 {
    let mut i_var1: i32 = param_1.wrapping_add(0xd);
    // SAFETY: param_1 is an address of the caller's buffer; reads occur
    // at the same offsets as the original.
    unsafe {
        let b: u8 = (param_1.wrapping_add(0xa) as *const u8).read();
        if (b & 0x80) != 0 {
            let shift: i32 = ((b & 7) as i32).wrapping_add(1);
            let add: i32 = 1i32.wrapping_shl(shift as u32).wrapping_mul(3);
            i_var1 = i_var1.wrapping_add(add);
        }
        (i_var1.wrapping_add(5) as *const i32).read_unaligned()
    }
}
