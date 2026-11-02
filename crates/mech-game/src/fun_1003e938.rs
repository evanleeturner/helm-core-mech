// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100af0a0: u32;
    static mut DAT_100af0a4: u32;
    static mut DAT_100c7b70: u32;
    static mut DAT_100c7b88: u32;
    static mut DAT_100c7bc0: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1003e938() {
    // SAFETY: the globals are the game's own memory; DAT_100c7bc0 starts a
    // buffer of at least 0x46 dwords, zeroed here as the original does.
    unsafe {
        let mut p = (&raw mut DAT_100c7bc0).cast::<u32>();
        let mut i: i32 = 0x46;
        while i != 0 {
            p.write_unaligned(0);
            p = p.add(1);
            i = i.wrapping_sub(1);
        }
        DAT_100af0a0 = 0;
        DAT_100c7b70 = 0;
        DAT_100c7b88 = 0;
        DAT_100af0a4 = 0;
    }
}
