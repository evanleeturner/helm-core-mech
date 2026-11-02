// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c96b4: i32;
    static mut DAT_100c96b8: u32;
    static mut DAT_100c9ac0: [u8; 0];
    static mut DAT_100c9ed0: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1007cc83(param_1: *mut i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1007cd68(param_1: i32) -> *mut i32 {
    let mut local_8: *mut i32 = core::ptr::null_mut();
    // SAFETY: the globals and the callee are the game's own memory; the
    // table pointers are offset and written exactly as the original does.
    unsafe {
        if (DAT_100c96b4.wrapping_sub(1) as u32) != DAT_100c96b8
            && -1 < param_1
            && param_1 < DAT_100c96b4
        {
            local_8 = (&raw mut DAT_100c9ed0)
                .cast::<u8>()
                .byte_add(param_1.wrapping_mul(0x40) as u32 as usize)
                .cast::<i32>();
            DAT_100c96b8 = DAT_100c96b8.wrapping_add(1);
            (&raw mut DAT_100c9ac0)
                .cast::<u8>()
                .byte_add(DAT_100c96b8.wrapping_mul(4) as usize)
                .cast::<i32>()
                .write_unaligned(param_1);
            FUN_1007cc83(local_8);
        }
    }
    local_8
}
