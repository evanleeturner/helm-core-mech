// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ab374: i32;
    static mut DAT_1016acb4: u32;
    static mut DAT_1016bc60: [u8; 0];
    static mut DAT_1016cc60: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100239a4(param_1: *mut i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10023a8c(param_1: i32) -> *mut i32 {
    let mut local_8: *mut i32 = core::ptr::null_mut();
    // SAFETY: the globals and the callee are the game's own memory; the
    // pointers are computed and written exactly where the original does.
    unsafe {
        if (DAT_100ab374.wrapping_sub(1) as u32) != DAT_1016acb4
            && -1 < param_1
            && param_1 < DAT_100ab374
        {
            local_8 = (&raw mut DAT_1016cc60)
                .cast::<u8>()
                .byte_add(param_1.wrapping_mul(0x1fa) as usize)
                .cast::<i32>();
            DAT_1016acb4 = DAT_1016acb4.wrapping_add(1);
            (&raw mut DAT_1016bc60)
                .cast::<u8>()
                .byte_add(DAT_1016acb4.wrapping_mul(4) as usize)
                .cast::<i32>()
                .write_unaligned(param_1);
            FUN_100239a4(local_8);
        }
    }
    local_8
}
