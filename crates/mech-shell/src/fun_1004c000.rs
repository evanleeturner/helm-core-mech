// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10083508: u32;
    static mut DAT_1009b6d4: [i8; 1];
    static mut DAT_1009b6d8: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FID_conflict___wfopen(_Filename: *mut i8, _Mode: *mut i8) -> *mut u8;
    fn _fread(_DstBuf: *mut u8, _ElementSize: u32, _Count: u32, _File: *mut u8) -> u32;
    fn _fclose(_File: *mut u8) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004c000(param_1: *mut u32) {
    // SAFETY: the callees and globals are the game's own; param_1 points at a
    // caller buffer of at least 0x9d4 bytes, and the destination is the game's
    // memory at DAT_10083508 + 0x23d3.
    unsafe {
        let file = FID_conflict___wfopen(
            (&raw mut DAT_1009b6d8).cast::<i8>(),
            (&raw mut DAT_1009b6d4).cast::<i8>(),
        );
        if !file.is_null() {
            _fread(param_1.cast::<u8>(), 0x9d4, 1, file);
            _fclose(file);
            let mut src = param_1;
            let mut dst = DAT_10083508.wrapping_add(0x23d3) as *mut u32;
            let mut count: i32 = 0x275;
            while count != 0 {
                dst.write_unaligned(src.read_unaligned());
                src = src.wrapping_add(1);
                dst = dst.wrapping_add(1);
                count -= 1;
            }
        }
    }
}
