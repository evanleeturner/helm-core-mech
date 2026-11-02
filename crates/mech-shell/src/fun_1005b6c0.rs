// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c40c0: u32;
    static mut DAT_100c40c4: u32;
    static mut DAT_100c6080: [u8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005b6c0(param_1: *mut u16) -> u32 {
    // SAFETY: the globals are the game's own memory; param_1 points at the
    // caller's u16 output slot, written where the original does.
    unsafe {
        if DAT_100c40c0 == DAT_100c40c4 {
            param_1.write_unaligned(0);
            return 0;
        }
        let v = (&raw mut DAT_100c6080)
            .cast::<u8>()
            .byte_add(DAT_100c40c0.wrapping_mul(2) as usize)
            .cast::<u16>()
            .read_unaligned();
        param_1.write_unaligned(v);
        DAT_100c40c0 = DAT_100c40c0.wrapping_add(1);
        if DAT_100c40c0 == 0x40 {
            DAT_100c40c0 = 0;
        }
        0
    }
}
