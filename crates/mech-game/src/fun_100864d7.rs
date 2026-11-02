// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf74: *mut c_void;
    static mut DAT_100ba7bc: u32;
    static mut DAT_100ba7c0: u32;
    static mut DAT_100ba824: [i8; 0];
    static mut DAT_100c9054: *mut u16;
    static mut DAT_100ba828: [i8; 0];
    static mut DAT_100ba850: [i8; 0];
    static mut DAT_100ba8a0: [i8; 0];
    static mut DAT_100ba878: [i8; 0];
    static mut DAT_100ba814: [i8; 0];
}

unsafe extern "C" {
    fn sprintf(dest: *mut i8, format: *const i8, ...) -> i32;
    fn fopen(filename: *const i8, mode: *const i8) -> *mut c_void;
    fn fprintf(file: *mut c_void, format: *const i8, ...) -> i32;
    fn fclose(file: *mut c_void) -> i32;
}

unsafe extern "stdcall" {
    fn HeapSize(h: *mut c_void, flags: u32, p: *const c_void) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100864d7() {
    let mut name = [0i8; 100];
    let mut tag = [0u8; 8];
    // SAFETY: globals, the C runtime and HeapSize are the game's own; the
    // list nodes are the game's cache entries, read where the original does.
    unsafe {
        let i_var1 = DAT_100ba7c0;
        DAT_100ba7c0 = DAT_100ba7c0.wrapping_add(1);
        sprintf(
            name.as_mut_ptr(),
            (&raw const DAT_100ba814).cast::<i8>(),
            i_var1 as i32,
        );
        let file = fopen(name.as_ptr(), (&raw const DAT_100ba824).cast::<i8>());
        tag[4] = 0;
        fprintf(file, (&raw const DAT_100ba828).cast::<i8>());
        let mut i: i32 = 0;
        while i < 0x3f1 {
            let mut node = ((DAT_100ba7bc as usize).wrapping_add((i as usize).wrapping_mul(4))
                as *const *mut u16)
                .read_unaligned();
            while !node.is_null() {
                let t = node.byte_add(4).cast::<u32>().read_unaligned();
                tag[0..4].copy_from_slice(&t.to_le_bytes());
                let sz = HeapSize(DAT_100adf74, 1, node as *const c_void);
                fprintf(
                    file,
                    (&raw const DAT_100ba850).cast::<i8>(),
                    node.read_unaligned() as u32,
                    tag.as_ptr(),
                    node.byte_add(2).cast::<i16>().read_unaligned() as i32,
                    sz,
                );
                node = node.byte_add(8).cast::<*mut u16>().read_unaligned();
            }
            i = i.wrapping_add(1);
        }
        fprintf(file, (&raw const DAT_100ba878).cast::<i8>());
        let mut node = DAT_100c9054;
        while !node.is_null() {
            let t = node.byte_add(4).cast::<u32>().read_unaligned();
            tag[0..4].copy_from_slice(&t.to_le_bytes());
            let sz = HeapSize(DAT_100adf74, 1, node as *const c_void);
            fprintf(
                file,
                (&raw const DAT_100ba8a0).cast::<i8>(),
                node.read_unaligned() as u32,
                tag.as_ptr(),
                node.byte_add(2).cast::<i16>().read_unaligned() as i32,
                sz,
            );
            node = node.byte_add(12).cast::<*mut u16>().read_unaligned();
        }
        fclose(file);
    }
}
