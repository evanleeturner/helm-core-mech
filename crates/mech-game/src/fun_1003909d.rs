// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ae6dc: [u8; 1];
    static mut DAT_100ae6e4: [u8; 1];
    static mut DAT_100ae6ec: [u8; 1];
    static mut DAT_100ae6f8: [u8; 1];
    static mut DAT_100ae704: [u8; 1];
    static mut DAT_100ae714: [u8; 1];
    static mut DAT_100ae720: [u8; 1];
    static mut DAT_100ae730: [u8; 1];
    static mut DAT_100ae738: [u8; 1];
}

unsafe extern "C" {
    fn strstr(a: *const i8, b: *const i8) -> *mut i8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003909d(param_1: i32, param_2: i32) {
    // SAFETY: param_1 and param_2 are the caller's structures; the string
    // globals are the game's own data, read and written where the original does.
    unsafe {
        let p1 = param_1 as *mut u8;
        let name = p1.byte_add(0xc) as *const i8;
        let u_var1 = ((param_2 + 0x60) as *const u32).read_unaligned();
        let dev3c = || ((param_2 + 0x3c) as *const i32).read_unaligned();

        let copy8 = |src: *const u8| {
            p1.byte_add(0x4c)
                .cast::<u32>()
                .write_unaligned(src.cast::<u32>().read_unaligned());
            p1.byte_add(0x50)
                .cast::<u32>()
                .write_unaligned(src.byte_add(4).cast::<u32>().read_unaligned());
        };
        let copy9 = |src: *const u8| {
            copy8(src);
            p1.byte_add(0x54).write(src.byte_add(8).read());
        };

        let mut r = strstr(name, (&raw mut DAT_100ae6dc) as *const i8);
        if !r.is_null() {
            copy8((&raw mut DAT_100ae6e4) as *const u8);
            return;
        }
        r = strstr(name, (&raw mut DAT_100ae6ec) as *const i8);
        if r.is_null()
            && ((u_var1 & 1) == 0
                || (u_var1 & 2) == 0
                || (u_var1 & 0x10) == 0
                || (u_var1 & 0x20) == 0
                || dev3c() != 8)
        {
            r = strstr(name, (&raw mut DAT_100ae704) as *const i8);
            if r.is_null()
                && ((u_var1 & 1) == 0
                    || (u_var1 & 0x10) == 0
                    || (u_var1 & 0x20) == 0
                    || dev3c() != 4)
            {
                r = strstr(name, (&raw mut DAT_100ae720) as *const i8);
                if r.is_null()
                    && ((u_var1 & 2) == 0
                        || (u_var1 & 0x10) == 0
                        || (u_var1 & 0x20) == 0
                        || dev3c() != 4)
                {
                    copy9((&raw mut DAT_100ae738) as *const u8);
                } else {
                    copy8((&raw mut DAT_100ae730) as *const u8);
                }
            } else {
                copy9((&raw mut DAT_100ae714) as *const u8);
            }
        } else {
            copy9((&raw mut DAT_100ae6f8) as *const u8);
        }
    }
}
