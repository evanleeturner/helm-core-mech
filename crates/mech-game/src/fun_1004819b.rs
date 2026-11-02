// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b0578: u8;
    static mut DAT_100c7d08: [u8; 0];
    static mut DAT_100b06a8: [i8; 0];
}

unsafe extern "C" {
    fn sprintf(dest: *mut i8, format: *const i8, ...) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004819b(param_1: *mut i8) -> *mut u32 {
    // SAFETY: the globals are the game's own memory; param_1 points at a
    // NUL-terminated string, copied into the game's buffer as the original does.
    unsafe {
        let buf = (&raw mut DAT_100c7d08).cast::<u8>();
        let mut local_8: i32 = 0;
        while local_8 < 0x50 {
            buf.byte_add(local_8 as usize).write(0);
            local_8 = local_8.wrapping_add(1);
        }
        if DAT_100b0578 == 0 {
            // length including the terminating NUL
            let mut len: u32 = 0;
            loop {
                let c = param_1.byte_add(len as usize).read();
                len = len.wrapping_add(1);
                if c == 0 {
                    break;
                }
            }
            let mut src = param_1.cast_const().cast::<u8>();
            let mut dst = buf;
            let mut n = len >> 2;
            while n != 0 {
                let v = src.cast::<u32>().read_unaligned();
                dst.cast::<u32>().write_unaligned(v);
                src = src.byte_add(4);
                dst = dst.byte_add(4);
                n = n.wrapping_sub(1);
            }
            let mut r = len & 3;
            while r != 0 {
                dst.write(src.read());
                src = src.byte_add(1);
                dst = dst.byte_add(1);
                r = r.wrapping_sub(1);
            }
        } else {
            sprintf(
                buf.cast::<i8>(),
                (&raw mut DAT_100b06a8).cast::<i8>().cast_const(),
                &raw mut DAT_100b0578,
                param_1,
            );
        }
        buf.cast::<u32>()
    }
}
