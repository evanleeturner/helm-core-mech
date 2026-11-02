// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10093900: u8;
    static mut DAT_10093a18: u8;
    static mut DAT_100b07fc: u32;
    static mut DAT_100b0800: u8;
    static mut DAT_100b0914: u8;
    static mut PTR_DAT_10093b2c: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10035eb0() {
    // SAFETY: the globals are the game's own memory; the source and
    // destination blocks are 0x45 dwords each, copied in the original's order.
    unsafe {
        let mut src = &raw mut DAT_10093900 as *const u32;
        DAT_100b07fc = (PTR_DAT_10093b2c == (src as usize as u32)) as u32;
        let mut dst = &raw mut DAT_100b0800 as *mut u32;
        let mut i: i32 = 0x45;
        while i != 0 {
            dst.write_unaligned(src.read_unaligned());
            src = src.wrapping_add(1);
            dst = dst.wrapping_add(1);
            i = i.wrapping_sub(1);
        }
        src = &raw mut DAT_10093a18 as *const u32;
        dst = &raw mut DAT_100b0914 as *mut u32;
        i = 0x45;
        while i != 0 {
            dst.write_unaligned(src.read_unaligned());
            src = src.wrapping_add(1);
            dst = dst.wrapping_add(1);
            i = i.wrapping_sub(1);
        }
    }
}
