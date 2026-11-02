// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c96b8: u32;
    static mut DAT_100c9ac0: [u8; 0];
    static mut DAT_100c9ed0: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1007cc00(param_1: *mut u32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1007cde0() -> *mut u32 {
    // SAFETY: the globals and the callee are the game's own; the table and
    // pool offsets are read and used as the original does.
    unsafe {
        let mut local_8: *mut u32 = core::ptr::null_mut();
        if -1 < (DAT_100c96b8 as i32) {
            let idx = DAT_100c96b8 as i32;
            let entry = (&raw mut DAT_100c9ac0)
                .cast::<u8>()
                .byte_offset((idx.wrapping_mul(4)) as isize)
                .cast::<i32>()
                .read_unaligned();
            local_8 = (&raw mut DAT_100c9ed0)
                .cast::<u8>()
                .byte_offset(entry.wrapping_mul(0x40) as isize)
                .cast::<u32>();
            FUN_1007cc00(local_8);
            DAT_100c96b8 = (DAT_100c96b8 as i32).wrapping_add(-1) as u32;
        }
        local_8
    }
}
