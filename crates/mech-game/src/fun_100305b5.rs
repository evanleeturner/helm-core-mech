// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ada4c: i32;
    static mut DAT_100ada58: u8;
    static mut DAT_100ada5c: u8;
    static mut DAT_10168800: u8;
    static mut DAT_10168900: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100303b0(param_1: i32, param_2: *mut i32, param_3: *mut i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100305b5() -> *mut u8 {
    let mut local_8: *mut u8 = core::ptr::null_mut();
    // SAFETY: the globals and the callee are the game's own memory; the table
    // addresses are read and passed on exactly as the original does.
    unsafe {
        if -1 < DAT_100ada4c {
            let idx = (&raw mut DAT_10168800)
                .byte_add(DAT_100ada4c.wrapping_mul(4) as usize)
                .cast::<i32>()
                .read_unaligned();
            local_8 = (&raw mut DAT_10168900).byte_add(idx.wrapping_mul(0x34) as usize);
            FUN_100303b0(
                local_8 as i32,
                (&raw mut DAT_100ada58).cast::<i32>(),
                (&raw mut DAT_100ada5c).cast::<i32>(),
            );
            DAT_100ada4c = DAT_100ada4c.wrapping_sub(1);
        }
    }
    local_8
}
