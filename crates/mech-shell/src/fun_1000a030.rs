// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000a030(
    dest: *mut u32,
    source: *mut u32,
    byte_count: u32,
) -> *mut u8 {
    let mut dest = dest;
    let mut source_cursor = source;
    let mut remaining = byte_count >> 2;
    // SAFETY: dest and source point at the caller's buffers of byte_count
    // bytes; reads and writes follow the original's order.
    unsafe {
        while remaining != 0 {
            dest.write_unaligned(source_cursor.read_unaligned());
            source_cursor = source_cursor.wrapping_add(1);
            dest = dest.wrapping_add(1);
            remaining = remaining.wrapping_sub(1);
        }
        let mut dest_b = dest as *mut u8;
        let mut src_b = source_cursor as *const u8;
        remaining = byte_count & 3;
        while remaining != 0 {
            dest_b.write(src_b.read());
            src_b = src_b.wrapping_add(1);
            dest_b = dest_b.wrapping_add(1);
            remaining = remaining.wrapping_sub(1);
        }
    }
    (source as u32).wrapping_add(byte_count) as *mut u8
}
