// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b0714: i32;
    static mut DAT_1015f2e0: [u8; 0];
    static mut DAT_101600f8: [u8; 0];
    static mut DAT_101604c0: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn __strcmpi(_Str1: *const i8, _Str2: *const i8) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004cf27(param_1: i32, str2: *mut i8) -> u32 {
    mech_trace::enter!("1004cf27", param_1, str2);
    if param_1 < 0x10 {
        let mut local_c: i32 = 0;
        // SAFETY: the globals are the game's own tables; offsets follow the original's
        // strides and the copy keeps the original's 0x70-byte length.
        unsafe {
            while local_c < mech_trace::gr!("1004cf27", "100b0714", DAT_100b0714) {
                let entry = (&raw mut DAT_1015f2e0)
                    .cast::<u8>()
                    .byte_offset(local_c.wrapping_mul(0x1c) as isize);
                let r = mech_trace::cr!(
                    "1004cf27",
                    "__strcmpi",
                    __strcmpi(entry.cast::<i8>() as *const i8, str2 as *const i8)
                );
                if r == 0 {
                    mech_trace::wr!(
                        "1004cf27",
                        (&raw mut DAT_101600f8)
                            .cast::<u8>()
                            .byte_offset(param_1.wrapping_mul(0x3c) as isize)
                            .cast::<i32>(),
                        write_unaligned,
                        local_c
                    );
                    let mut src = (&raw mut DAT_1015f2e0)
                        .cast::<u8>()
                        .byte_offset(local_c.wrapping_mul(0x1c) as isize)
                        .cast::<u32>();
                    let mut dst = (&raw mut DAT_101604c0)
                        .cast::<u8>()
                        .byte_offset(param_1.wrapping_mul(0x70) as isize)
                        .cast::<u32>();
                    let mut n: i32 = 0x1c;
                    while n != 0 {
                        mech_trace::wr!(
                            "1004cf27",
                            dst,
                            write_unaligned,
                            mech_trace::rd!("1004cf27", src, read_unaligned)
                        );
                        src = src.wrapping_add(1);
                        dst = dst.wrapping_add(1);
                        n = n.wrapping_sub(1);
                    }
                    return mech_trace::rt!("1004cf27", 1);
                }
                local_c = local_c.wrapping_add(1);
            }
        }
    }
    mech_trace::rt!("1004cf27", 0)
}
