// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a9d78: u32;
    static mut DAT_100c4f50: u8;
    static mut DAT_100c4f60: u8;
    static mut DAT_100c5ef0: u8;
    static mut s_Too_many_project_file_entries_100a9d7c: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100103b0(param_1: *mut u8) -> i32;
    fn _strcmpi(_Str1: *mut i8, _Str2: *mut i8) -> i32;
    fn FUN_100108a0(param_1: u32, param_2: *mut i8) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10010010(param_1: *mut u8, param_2: i32) -> *mut u8 {
    // SAFETY: the callees and globals are the game's own memory; param_1 points
    // at a NUL-terminated string, and the tables are accessed as the original does.
    unsafe {
        let u_var2 = (FUN_100103b0(param_1) as u32) % 0x65;
        let bucket = (&raw mut DAT_100c5ef0)
            .byte_add((u_var2 as usize).wrapping_mul(4))
            .cast::<*mut u8>();
        let mut local_8: *mut u8 = bucket.read_unaligned();
        while !local_8.is_null() {
            let i_var3 = _strcmpi(local_8.cast::<i8>(), param_1.cast::<i8>());
            if i_var3 == 0 {
                return local_8;
            }
            local_8 = local_8.byte_add(0x10).cast::<*mut u8>().read_unaligned();
        }
        if param_2 == 0 {
            return core::ptr::null_mut();
        }
        if 199 < DAT_100a9d78 {
            FUN_100108a0(0x23, &raw mut s_Too_many_project_file_entries_100a9d7c);
        }
        let i_var3 = (DAT_100a9d78 as i32).wrapping_mul(0x14);
        let pbvar4 = (&raw mut DAT_100c4f50).byte_add(i_var3 as usize);
        DAT_100a9d78 = DAT_100a9d78.wrapping_add(1);
        let mut len: u32 = 0;
        let mut p = param_1;
        loop {
            len = len.wrapping_add(1);
            let b = p.read();
            p = p.add(1);
            if b == 0 {
                break;
            }
        }
        let mut src = param_1;
        let mut dst = pbvar4;
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
        let head = bucket.read_unaligned();
        (&raw mut DAT_100c4f60)
            .byte_add(i_var3 as usize)
            .cast::<*mut u8>()
            .write_unaligned(head);
        bucket.write_unaligned(pbvar4);
        pbvar4
    }
}
