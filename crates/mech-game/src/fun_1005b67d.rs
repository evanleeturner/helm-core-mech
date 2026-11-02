// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005b67d(param_1: i32, param_2: i32, param_3: *mut u32) -> *mut u32 {
    // SAFETY: the caller supplies a table at param_1 and a source buffer at
    // param_3; reads and writes happen in the same order as the original.
    unsafe {
        let i_var1 = ((param_1
            .wrapping_add(param_2.wrapping_mul(8))
            .wrapping_add(0xc)) as usize as *const i32)
            .read_unaligned();
        if i_var1 == 0 {
            return core::ptr::null_mut();
        }
        let mut pi_var5 = i_var1.wrapping_add(param_1) as usize as *mut i32;
        let pu_var3 = pi_var5.read_unaligned() as u32;
        let mut pu_var4: u32 = pu_var3;
        let mut src = param_3;
        let mut pu_var2: u32 = param_3 as usize as u32;
        while pu_var2 != 0 {
            pi_var5 = pi_var5.wrapping_add(1);
            pi_var5.write_unaligned(src.read_unaligned() as i32);
            pu_var4 = pu_var4.wrapping_sub(1);
            src = src.wrapping_add(1);
            pu_var2 = pu_var4;
        }
        pu_var3 as usize as *mut u32
    }
}
