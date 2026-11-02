// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c61b4: *mut u8;
    static mut DAT_100a50f0: i32;
    static mut DAT_10095af8: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005e170(param_1: *mut i8, param_2: u16) {
    // SAFETY: the globals are the game's own memory; param_1 points at a
    // NUL-terminated string and DAT_100c61b4 at a buffer with room for the
    // record, as the original assumes.
    unsafe {
        let mut len: u32 = 0;
        while *param_1.add(len as usize) != 0 {
            len = len.wrapping_add(1);
        }
        let total: u32 = len.wrapping_add(1);

        let u_var4: u32 = total.wrapping_add(0xD) & 0xFFFF_FFFC;
        let base = DAT_100c61b4;
        base.cast::<u32>().write_unaligned(DAT_10095af8);
        base.byte_add(4).cast::<u32>().write_unaligned(u_var4);
        base.byte_add(8).cast::<u16>().write_unaligned(param_2);

        let mut src = param_1 as *const u8;
        let mut dst = DAT_100c61b4.byte_add(10);
        let mut n = total >> 2;
        while n != 0 {
            dst.cast::<u32>()
                .write_unaligned(src.cast::<u32>().read_unaligned());
            src = src.byte_add(4);
            dst = dst.byte_add(4);
            n -= 1;
        }
        let mut r = total & 3;
        while r != 0 {
            *dst = *src;
            src = src.byte_add(1);
            dst = dst.byte_add(1);
            r -= 1;
        }

        DAT_100c61b4 = DAT_100c61b4.byte_add(u_var4 as usize);
        if DAT_100a50f0 <= (u_var4 as i32) {
            DAT_100a50f0 = u_var4 as i32;
        }
    }
}
