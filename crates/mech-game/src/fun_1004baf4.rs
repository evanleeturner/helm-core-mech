// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b06f0: u32;
    static mut DAT_100c7e10: [u8; 0];
    static mut DAT_101ec7a0: [u8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1004baf4() -> *mut u8 {
    // SAFETY: the globals are the game's own memory; the table entry is read
    // at the same offset the original computes.
    unsafe {
        let idx = DAT_100b06f0;
        if idx == 0xFFFF_FFFF
            || (&raw mut DAT_101ec7a0)
                .cast::<u8>()
                .wrapping_byte_add(idx.wrapping_mul(0x60) as usize)
                .cast::<i32>()
                .read_unaligned()
                == 0
        {
            DAT_100b06f0 = 0xFFFF_FFFF;
            core::ptr::null_mut()
        } else {
            (&raw mut DAT_100c7e10).cast::<u8>()
        }
    }
}
