// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7590: u32;
    static mut DAT_100a7594: u32;
    static mut DAT_102155a0: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100108a0(param_1: u32, param_2: *mut i8) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1000cfbc() {
    // SAFETY: the globals and the callee are the game's own memory; the
    // table read happens only when the index is below 0x80000000, as in the original.
    unsafe {
        DAT_100a7594 = DAT_100a7594.wrapping_sub(1);
        if DAT_100a7594 < 0x80000000 {
            let p = (&raw mut DAT_102155a0)
                .cast::<u8>()
                .byte_add(DAT_100a7594.wrapping_mul(4) as usize);
            DAT_100a7590 = p.cast::<u32>().read_unaligned();
        } else {
            FUN_100108a0(0x22, core::ptr::null_mut());
        }
    }
}
