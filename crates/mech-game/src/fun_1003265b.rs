// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adab8: *mut i8;
    static mut DAT_100adabc: *mut i8;
    static mut DAT_100b0710: i32;
    static mut DAT_101600ec: u8;
    static mut DAT_101600f0: u8;
    static mut DAT_101600f4: u8;
    static mut DAT_101604a0: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1004cf27(param_1: i32, param_2: *mut i8) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003265b(param_1: i32) {
    // SAFETY: param_1 is the caller's buffer and the globals are the game's own
    // tables; reads and writes happen at the same offsets and order as the original.
    unsafe {
        let i_var1 = ((param_1.wrapping_add(4)) as *const i32).read_unaligned();
        let mut local_10: i32 = 0;
        while local_10 < ((i_var1 as u32).wrapping_sub(8) / 0x18) as i32 {
            let rec = param_1.wrapping_add(local_10.wrapping_mul(0x18));
            let t = local_10.wrapping_mul(0x3c);
            let a = (rec.wrapping_add(8) as *const i32).read_unaligned();
            ((&raw mut DAT_101600ec).byte_add(t as usize).cast::<i32>()).write_unaligned(a);
            let b = (rec.wrapping_add(0xc) as *const i32).read_unaligned();
            ((&raw mut DAT_101600f4).byte_add(t as usize).cast::<i32>()).write_unaligned(b);
            let idx = (rec.wrapping_add(8) as *const i32).read_unaligned();
            let b2 = (rec.wrapping_add(0xc) as *const i32).read_unaligned();
            ((&raw mut DAT_101604a0)
                .byte_add(idx.wrapping_mul(4) as usize)
                .cast::<i32>())
            .write_unaligned(b2);
            if local_10 == DAT_100b0710 && !DAT_100adab8.is_null() {
                FUN_1004cf27(local_10, DAT_100adab8);
            } else if local_10 == DAT_100b0710 || DAT_100adabc.is_null() {
                FUN_1004cf27(local_10, rec.wrapping_add(0x10) as *mut i8);
            } else {
                FUN_1004cf27(local_10, DAT_100adabc);
            }
            ((&raw mut DAT_101600f0).byte_add(t as usize).cast::<i32>()).write_unaligned(0);
            local_10 = local_10.wrapping_add(1);
        }
    }
}
