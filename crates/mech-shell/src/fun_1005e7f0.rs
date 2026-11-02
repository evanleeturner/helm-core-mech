// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c61b4: *mut i8;
    static mut DAT_10095b28: i32;
    static mut DAT_100a50f0: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005e7f0(param_1: *mut i8) {
    // SAFETY: the globals are the game's own; param_1 is a NUL-terminated
    // string and DAT_100c61b4 points at a buffer large enough for the record.
    unsafe {
        let pcvar2 = DAT_100c61b4;
        pcvar2.cast::<i32>().write_unaligned(DAT_10095b28);

        let mut n: u32 = 0;
        loop {
            let c = *param_1.byte_add(n as usize);
            n = n.wrapping_add(1);
            if c == 0 {
                break;
            }
        }

        let mut src = param_1 as *const i8;
        let mut dst = pcvar2.byte_add(8);
        let mut q = n >> 2;
        while q != 0 {
            let v = src.cast::<i32>().read_unaligned();
            dst.cast::<i32>().write_unaligned(v);
            src = src.byte_add(4);
            dst = dst.byte_add(4);
            q -= 1;
        }
        let mut r = n & 3;
        while r != 0 {
            *dst = *src;
            src = src.byte_add(1);
            dst = dst.byte_add(1);
            r -= 1;
        }

        let size = n.wrapping_add(0xb) & 0xffff_fffc;
        pcvar2.byte_add(4).cast::<u32>().write_unaligned(size);
        let mut v = size as i32;
        if v <= DAT_100a50f0 {
            v = DAT_100a50f0;
        }
        DAT_100a50f0 = v;
        let off = pcvar2.byte_add(4).cast::<i32>().read_unaligned();
        DAT_100c61b4 = DAT_100c61b4.byte_offset(off as isize);
    }
}
