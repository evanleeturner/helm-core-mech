// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa2c8: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1006602f(param_1: *const u32) {
    // SAFETY: the global is the game's own buffer of at least 0x100 bytes;
    // param_1 points at the caller's 0x40 dwords, read in order.
    unsafe {
        let mut src = param_1;
        let mut dst = (&raw mut DAT_100aa2c8).cast::<u32>();
        let mut count: i32 = 0x40;
        while count != 0 {
            dst.write_unaligned(src.read_unaligned());
            src = src.add(1);
            dst = dst.add(1);
            count -= 1;
        }
    }
}
