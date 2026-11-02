// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ab384: u32;
    static mut DAT_1016acb4: i32;
    static mut DAT_1016bc60: [u8; 0];
    static mut DAT_1016cc60: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10023920(param_1: *mut u32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10023b0d() -> *mut u32 {
    let mut local_8: *mut u32 = core::ptr::null_mut();
    // SAFETY: the globals and the callee are the game's own memory; the index
    // table and record array are read at the offsets the original uses.
    unsafe {
        if -1 < DAT_1016acb4 && DAT_100ab384 != 0 {
            let idx = (&raw mut DAT_1016bc60)
                .cast::<u8>()
                .byte_add((DAT_1016acb4 as usize).wrapping_mul(4))
                .cast::<i32>()
                .read_unaligned();
            local_8 = (&raw mut DAT_1016cc60)
                .cast::<u8>()
                .byte_offset((idx.wrapping_mul(0x1fa)) as isize)
                .cast::<u32>();
            FUN_10023920(local_8);
            DAT_1016acb4 = DAT_1016acb4.wrapping_sub(1);
        }
    }
    local_8
}
