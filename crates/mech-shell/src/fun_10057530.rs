// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100986b4: u32;
    static mut DAT_100a1ae8: u8;
    static mut DAT_100a3650: i8;
    static mut DAT_100a3640: i8;
    static mut PTR_DAT_100985ec: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100425b0(record_index: i32, logo_number: u32, param_3: *mut i32) -> *mut u16;
    fn sprintf(dest: *mut i8, format: *const i8, ...) -> i32;
    fn FID_conflict___wfopen(filename: *const i8, mode: *const i8) -> *mut c_void;
    fn fread(buf: *mut c_void, element_size: usize, count: usize, file: *mut c_void) -> usize;
    fn fclose(file: *mut c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10057530(record_index: i16, logo_number: i16) -> u32 {
    let mut cel_file_name = [0i8; 16];
    // SAFETY: globals and callees are the game's own; the table lookup and
    // buffers are used exactly as the original does.
    unsafe {
        let off = (record_index as i32).wrapping_mul(0xe);
        let id = (&raw mut DAT_100a1ae8)
            .byte_add(off as usize)
            .cast::<i16>()
            .read_unaligned() as i32;
        let cel_buffer = FUN_100425b0(
            DAT_100986b4 as i32,
            id as u32,
            PTR_DAT_100985ec as usize as *mut i32,
        );
        if cel_buffer.is_null() {
            return 3;
        }
        sprintf(
            cel_file_name.as_mut_ptr(),
            &raw const DAT_100a3640,
            logo_number as i32,
        );
        let cel_file = FID_conflict___wfopen(cel_file_name.as_ptr(), &raw const DAT_100a3650);
        if cel_file.is_null() {
            return 1;
        }
        fread(cel_buffer.cast::<c_void>(), 1, 0x1002, cel_file);
        let close_result = fclose(cel_file) as u32;
        close_result & 0xffff0000
    }
}
