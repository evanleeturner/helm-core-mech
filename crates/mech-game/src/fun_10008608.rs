// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7178: u32;
    static mut DAT_100a717c: u32;
    static mut DAT_100a7180: u32;
    static mut DAT_100a7b30: [u8; 0];
    static mut DAT_10215650: u32;
    static mut DAT_10215658: u32;
    static mut DAT_10215660: u32;
    static mut DAT_100a761c: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100083d0(param_1: i32, param_2: *mut u32) -> u8;
    fn FUN_100660e1(param_1: i32, param_2: *mut i32, param_3: *mut i32) -> *mut i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10008608() {
    // SAFETY: the globals and callees are the game's own memory and code;
    // the table at DAT_100a7b30 holds six pointers, read as the original does.
    unsafe {
        let mut local_8: i32 = 0;
        while local_8 < 6 {
            let p = (&raw mut DAT_100a7b30)
                .cast::<u8>()
                .byte_add(local_8.wrapping_mul(4) as usize)
                .cast::<*mut u32>()
                .read_unaligned();
            FUN_100083d0(local_8, p);
            local_8 = local_8.wrapping_add(1);
        }
        DAT_10215658 = 0;
        DAT_10215660 = 1;
        DAT_10215650 = 0;
        let q = DAT_100a761c.wrapping_add(0x14) as *mut i32;
        FUN_100660e1(0x101f52e0, q, q);
        DAT_100a7178 = DAT_100a7178.wrapping_add(-1000i32 as u32);
        DAT_100a7180 = DAT_100a717c.wrapping_sub(DAT_100a7178);
    }
}
