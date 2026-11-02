// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c61b4: *mut u8;
    static mut DAT_100a50f0: u32;
    static mut DAT_10095b71: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005e910(param_1: *mut i8) {
    // SAFETY: the globals are the game's own; param_1 points at a
    // NUL-terminated string and DAT_100c61b4 at a buffer large enough for
    // the header and the copied string, as in the original.
    unsafe {
        let pc_var2 = DAT_100c61b4;
        pc_var2.cast::<u32>().write_unaligned(DAT_10095b71);

        let mut len: u32 = 0;
        loop {
            let c = param_1.add(len as usize).read();
            len = len.wrapping_add(1);
            if c == 0 {
                break;
            }
        }

        let mut src = param_1 as *const u8;
        let mut dst = pc_var2.add(8);
        let mut n = len >> 2;
        while n != 0 {
            dst.cast::<u32>()
                .write_unaligned(src.cast::<u32>().read_unaligned());
            src = src.add(4);
            dst = dst.add(4);
            n -= 1;
        }
        let mut r = len & 3;
        while r != 0 {
            dst.write(src.read());
            src = src.add(1);
            dst = dst.add(1);
            r -= 1;
        }

        let mut u_var3 = len.wrapping_add(0xb) & 0xffff_fffc;
        pc_var2.add(4).cast::<u32>().write_unaligned(u_var3);
        if (u_var3 as i32) <= (DAT_100a50f0 as i32) {
            u_var3 = DAT_100a50f0;
        }
        DAT_100a50f0 = u_var3;
        let off = pc_var2.add(4).cast::<i32>().read_unaligned();
        DAT_100c61b4 = DAT_100c61b4.byte_offset(off as isize);
    }
}
