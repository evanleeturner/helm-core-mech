// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10094434: [u8; 1];
    static mut DAT_10098224: *mut c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    #[link_name = "strncpy"]
    fn _strncpy(record: *mut i8, source: *const i8, count: u32) -> *mut i8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapAlloc(h: *mut c_void, flags: u32, size: u32) -> *mut c_void;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10037fc0(chunk_buffer: i32) -> *mut i8 {
    let mut record: *mut i8 = core::ptr::null_mut();
    if chunk_buffer == 0 {
        return record;
    }
    // SAFETY: the buffer, the globals and the callees are the game's own;
    // reads and writes follow the original's offsets and order.
    unsafe {
        let mut chunk_offset: i32 = 0xc;
        let total_size = ((chunk_buffer as u32).wrapping_add(4) as *const i32).read_unaligned();
        let mut chunk: *const u8 = (chunk_buffer as u32).wrapping_add(0xc) as *const u8;
        if 0xc < total_size {
            loop {
                let tag = chunk.cast::<i32>().read_unaligned();
                let len = chunk.add(4).cast::<i32>().read_unaligned();
                if tag == 0x52535344 {
                    _strncpy(
                        record.wrapping_add(100),
                        chunk.add(8).cast::<i8>(),
                        len.wrapping_sub(1) as u32,
                    );
                    record
                        .wrapping_offset(len.wrapping_add(100) as isize)
                        .write(0);
                    chunk_offset = chunk_offset.wrapping_add(len);
                } else if tag == 0x544c4950 {
                    record = HeapAlloc(DAT_10098224, 1, 0x2066).cast::<i8>();
                    core::ptr::write_bytes(record, 0, 0x2066);
                    _strncpy(record, chunk.add(8).cast::<i8>(), 0x1f);
                    record.add(0x20).write(0);

                    let mut src: *const u8 = (&raw mut DAT_10094434).cast::<u8>();
                    let mut dst: *mut i8 = record.add(100);
                    loop {
                        let c = *src;
                        *dst = c as i8;
                        src = src.add(1);
                        dst = dst.add(1);
                        if c == 0 {
                            break;
                        }
                    }

                    for i in 0x20..0x28 {
                        record.add(i).write(-1);
                    }

                    let mut src: *const u8 = chunk.add(0x3c);
                    let mut dst: *mut i8 = record.add(0x44);
                    loop {
                        let c = *src;
                        *dst = c as i8;
                        src = src.add(1);
                        dst = dst.add(1);
                        if c == 0 {
                            break;
                        }
                    }

                    let mut src: *const u8 = chunk.add(0x4c);
                    let mut dst: *mut i8 = record.add(0x54);
                    loop {
                        let c = *src;
                        *dst = c as i8;
                        src = src.add(1);
                        dst = dst.add(1);
                        if c == 0 {
                            break;
                        }
                    }

                    record.add(0x38).cast::<i32>().write_unaligned(0);
                    record
                        .add(0x3c)
                        .cast::<i32>()
                        .write_unaligned(chunk.add(0x38).cast::<i32>().read_unaligned());
                    record.add(0x40).cast::<i32>().write_unaligned(0);
                    record.add(0x2064).write(0);

                    let mut fsrc: *const u8 = chunk.add(0x28);
                    let mut cur: *mut i8 = record.add(0x28);
                    let mut n = 8;
                    while n != 0 {
                        let w = fsrc.cast::<i32>().read_unaligned();
                        fsrc = fsrc.add(2);
                        cur.cast::<i16>().write_unaligned(w as i16);
                        cur = cur.add(2);
                        n -= 1;
                    }
                    chunk_offset = chunk_offset.wrapping_add(0x5c);
                } else {
                    chunk_offset = chunk_offset.wrapping_add(len);
                }
                chunk = (chunk_buffer as u32).wrapping_add(chunk_offset as u32) as *const u8;
                if !(chunk_offset < total_size) {
                    break;
                }
            }
        }
    }
    record
}
