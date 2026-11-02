// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adcdc: u32;
    static mut DAT_100b2ce8: u32;
    static mut DAT_100b3958: u32;
    static mut DAT_100b3960: [i8; 0];
    static mut DAT_100eeee0: u32;
    static mut DAT_100eeef0: [u32; 7];
    static mut DAT_100eef0c: u32;
    static mut DAT_100eef10: u32;
    static mut DAT_100eef20: [u32; 7];
    static mut DAT_10215658: u32;
    static mut s_mercs_hud_100b3964: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_100681c0();
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _fopen(filename: *const i8, mode: *const i8) -> *mut u8;
    fn _fwrite(buf: *const u8, size: u32, count: u32, file: *mut u8) -> u32;
    fn _fclose(file: *mut u8) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10067d90() {
    // SAFETY: the globals and the C runtime functions are the game's own;
    // the file handle is checked for null before use.
    unsafe {
        DAT_100b3958 = 1;
        FUN_100681c0();
        let mut local_8: i32 = 0;
        while local_8 < 7 {
            let src = (&raw mut DAT_100eeef0).cast::<u32>().add(local_8 as usize);
            let dst = (&raw mut DAT_100eef20).cast::<u32>().add(local_8 as usize);
            dst.write_unaligned(src.read_unaligned());
            local_8 = local_8.wrapping_add(1);
        }
        DAT_100eeee0 = DAT_100b2ce8;
        DAT_100eef0c = DAT_100adcdc;
        DAT_100eef10 = DAT_10215658;
        let file = _fopen(
            (&raw mut s_mercs_hud_100b3964).cast::<i8>(),
            (&raw mut DAT_100b3960).cast::<i8>(),
        );
        if !file.is_null() {
            _fwrite((&raw mut DAT_100eef20).cast::<u8>(), 4, 7, file);
            _fwrite((&raw mut DAT_100eeee0).cast::<u8>(), 4, 1, file);
            _fwrite((&raw mut DAT_100eef0c).cast::<u8>(), 4, 1, file);
            _fwrite((&raw mut DAT_100eef10).cast::<u8>(), 4, 1, file);
            _fclose(file);
        }
    }
}
