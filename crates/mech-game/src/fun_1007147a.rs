// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b4840: u8;
    static mut DAT_100b4844: u8;
    static mut DAT_100b484d: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1007147a(
    param_1: *mut i32,
    param_2: *const i8,
    param_3: u32,
    param_4: *const i8,
) {
    if param_1.is_null() {
        return;
    }
    // SAFETY: param_1 and the pointers it holds are the caller's; the globals
    // are the game's own table; strings are NUL-terminated as in the original.
    unsafe {
        let p = param_1.read_unaligned();
        if p == 0 {
            return;
        }
        let i_var2 = ((p as u32).wrapping_add(4) as *const i32).read_unaligned();
        if i_var2 >= 0x3d {
            return;
        }
        let off = i_var2.wrapping_mul(0x16) as isize;

        (&raw mut DAT_100b4840)
            .byte_offset(off)
            .cast::<u32>()
            .write_unaligned(param_3);

        let mut len: usize = 0;
        while param_2.add(len).read() != 0 {
            len += 1;
        }
        len += 1;
        let mut src = param_2.cast::<u8>();
        let mut dst = (&raw mut DAT_100b4844).byte_offset(off);
        let mut n = len >> 2;
        while n != 0 {
            dst.cast::<u32>()
                .write_unaligned(src.cast::<u32>().read_unaligned());
            src = src.add(4);
            dst = dst.add(4);
            n -= 1;
        }
        let mut n = len & 3;
        while n != 0 {
            *dst = *src;
            src = src.add(1);
            dst = dst.add(1);
            n -= 1;
        }

        let mut len: usize = 0;
        while param_4.add(len).read() != 0 {
            len += 1;
        }
        len += 1;
        let mut src = param_4.cast::<u8>();
        let mut dst = (&raw mut DAT_100b484d).byte_offset(off);
        let mut n = len >> 2;
        while n != 0 {
            dst.cast::<u32>()
                .write_unaligned(src.cast::<u32>().read_unaligned());
            src = src.add(4);
            dst = dst.add(4);
            n -= 1;
        }
        let mut n = len & 3;
        while n != 0 {
            *dst = *src;
            src = src.add(1);
            dst = dst.add(1);
            n -= 1;
        }
    }
}
