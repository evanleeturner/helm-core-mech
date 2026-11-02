// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b0710: i32;
    static mut DAT_1015f2e0: u8;
    static mut DAT_101600f8: u8;
    static mut DAT_101604c0: u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_100082ad() -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004d01b(param_1: i32, param_2: i32) {
    if param_1 < 0x10 && param_2 < 0x20 {
        // SAFETY: the globals and the callee are the game's own memory; the
        // arrays are indexed and copied exactly as the original does.
        unsafe {
            if param_1 == DAT_100b0710 {
                FUN_100082ad();
            }
            (&raw mut DAT_101600f8)
                .byte_offset(param_1.wrapping_mul(0x3c) as isize)
                .cast::<i32>()
                .write_unaligned(param_2);
            let mut src = (&raw mut DAT_1015f2e0)
                .byte_offset(param_2.wrapping_mul(0x1c) as isize)
                .cast::<u32>();
            let mut dst = (&raw mut DAT_101604c0)
                .byte_offset(param_1.wrapping_mul(0x70) as isize)
                .cast::<u32>();
            let mut count: i32 = 0x1c;
            while count != 0 {
                dst.write_unaligned(src.read_unaligned());
                src = src.add(1);
                dst = dst.add(1);
                count -= 1;
            }
        }
    }
}
