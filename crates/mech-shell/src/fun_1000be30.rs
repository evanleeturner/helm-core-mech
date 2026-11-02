// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10098224: *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapAlloc(h: *mut core::ffi::c_void, flags: u32, size: u32) -> *mut u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000be30(source: *const u32) {
    // SAFETY: the heap handle is the game's own; source points at a record of
    // 0x853 bytes, copied into the fresh allocation in the original's order.
    unsafe {
        let mut copy_cursor = HeapAlloc(DAT_10098224, 1, 0x853);
        let mut source = source;
        if !copy_cursor.is_null() {
            let mut dwords_left: i32 = 0x214;
            while dwords_left != 0 {
                copy_cursor.write_unaligned(source.read_unaligned());
                source = source.add(1);
                copy_cursor = copy_cursor.add(1);
                dwords_left = dwords_left.wrapping_sub(1);
            }
            copy_cursor
                .cast::<u16>()
                .write_unaligned(source.cast::<u16>().read_unaligned());
            copy_cursor
                .cast::<u8>()
                .byte_add(2)
                .write(source.cast::<u8>().byte_add(2).read());
        }
    }
}
