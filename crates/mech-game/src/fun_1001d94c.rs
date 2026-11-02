// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ab238: u32;
    static mut DAT_101e89b0: [u8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001d94c(param_1: *mut u8, param_2: *mut u8) {
    // SAFETY: the globals are the game's own memory; param_1 and param_2
    // point at the caller's buffers of at least three bytes each.
    unsafe {
        let i_var1 = DAT_100ab238;
        let table = (&raw mut DAT_101e89b0).cast::<u8>();
        let base = i_var1.wrapping_mul(0x40);
        *param_2 = table
            .byte_add(u32::from(*param_1).wrapping_add(base) as usize)
            .read();
        *param_2.add(1) = table
            .byte_add(u32::from(*param_1.add(1)).wrapping_add(base) as usize)
            .read();
        *param_2.add(2) = table
            .byte_add(u32::from(*param_1.add(2)).wrapping_add(base) as usize)
            .read();
    }
}
