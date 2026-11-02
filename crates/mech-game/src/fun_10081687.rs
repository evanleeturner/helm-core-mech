// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10081687(param_1: i32, param_2: i32) -> u32 {
    // SAFETY: the caller passes addresses of records whose second dword
    // is readable; they are read where the original reads them.
    unsafe {
        let a = ((param_2 as u32).wrapping_add(4) as *const u32).read_unaligned();
        let b = ((param_1 as u32).wrapping_add(4) as *const u32).read_unaligned();
        let i_var1 = a.wrapping_sub(b) as i32;
        if i_var1 < 0 {
            0xffff_ffff
        } else if i_var1 < 1 {
            0
        } else {
            1
        }
    }
}
