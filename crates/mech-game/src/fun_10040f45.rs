// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100af3f8: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10040f45(param_1: *mut u32) {
    // SAFETY: param_1 points at 0x40 readable dwords; the destination is the
    // game's own buffer at DAT_100af3f8, which holds at least 0x40 dwords.
    unsafe {
        let mut src = param_1;
        let mut dst = (&raw mut DAT_100af3f8).cast::<u32>();
        let mut count: i32 = 0x40;
        while count != 0 {
            dst.write_unaligned(src.read_unaligned());
            src = src.add(1);
            dst = dst.add(1);
            count -= 1;
        }
    }
}
