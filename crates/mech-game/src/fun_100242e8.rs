// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ab374: u32;
    static mut DAT_100ab378: u32;
    static mut DAT_1016cc60: [u8; 0];
    static mut DAT_1016cc70: [u8; 0];
    static mut DAT_1016ce3a: [u8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100242e8(param_1: u32) {
    // SAFETY: the globals are the game's own memory; the offsets index its
    // record table, and the function pointer read from it is the game's callback.
    unsafe {
        if DAT_100ab378 != 0 && param_1 < DAT_100ab374 {
            let off = param_1.wrapping_mul(0x1fa) as usize;
            let flags = (&raw mut DAT_1016cc70)
                .cast::<u8>()
                .byte_add(off)
                .cast::<u16>()
                .read_unaligned();
            if (flags & 1) != 0 {
                let f = (&raw mut DAT_1016ce3a)
                    .cast::<u8>()
                    .byte_add(off)
                    .cast::<unsafe extern "C" fn(*mut u8)>()
                    .read_unaligned();
                let arg = (&raw mut DAT_1016cc60).cast::<u8>().byte_add(off);
                f(arg);
            }
        }
    }
}
