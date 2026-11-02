// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn __read(file_handle: i32, dst_buf: *mut u8, chunk_size: u32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10059f10(
    file_handle: i32,
    dest_buffer: *mut u8,
    byte_count: u32,
) -> u32 {
    let mut dest_buffer = dest_buffer;
    let mut bytes_left = byte_count;
    loop {
        if bytes_left == 0 {
            return byte_count;
        }
        let mut chunk_size = bytes_left;
        if 0x3fff < bytes_left {
            chunk_size = 0x4000;
        }
        // SAFETY: __read is the C runtime's function; dest_buffer points at
        // the caller's buffer with at least bytes_left bytes remaining.
        let bytes_read = unsafe { __read(file_handle, dest_buffer, chunk_size) } as u32;
        if bytes_read != chunk_size {
            break;
        }
        bytes_left = bytes_left.wrapping_sub(chunk_size);
        dest_buffer = dest_buffer.wrapping_add(chunk_size as usize);
    }
    0xffffffff
}
