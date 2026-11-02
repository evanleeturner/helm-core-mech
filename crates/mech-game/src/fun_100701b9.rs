// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c7fd8: u32;
    static mut DAT_100c7fdc: u32;
    static mut DAT_100eedb0: [u8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100701b9(param_1: *mut u16) -> u32 {
    // SAFETY: the globals are the game's own memory; param_1 points at the
    // caller's u16, and the ring buffer at DAT_100eedb0 holds 0x40 u16 entries.
    unsafe {
        if DAT_100c7fd8 == DAT_100c7fdc {
            param_1.write_unaligned(0);
        } else {
            let base = (&raw mut DAT_100eedb0).cast::<u8>();
            let v = base
                .byte_add(DAT_100c7fd8.wrapping_mul(2) as usize)
                .cast::<u16>()
                .read_unaligned();
            param_1.write_unaligned(v);
            DAT_100c7fd8 = DAT_100c7fd8.wrapping_add(1);
            if DAT_100c7fd8 == 0x40 {
                DAT_100c7fd8 = 0;
            }
        }
    }
    0
}
