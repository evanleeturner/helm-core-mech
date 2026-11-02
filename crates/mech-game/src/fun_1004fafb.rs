// SPDX-License-Identifier: MIT

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b0710: i32;
    static mut DAT_100b084c: u32;
    static mut DAT_100b0854: [i8; 0];
    static mut DAT_10123240: [u8; 0];
    static mut DAT_100b0858: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_fopen(filename: *const i8, mode: *const i8) -> *mut FILE;
    fn FUN_fwrite(s: *const u8, size: u32, count: u32, file: *mut FILE) -> u32;
    fn FUN_fclose(file: *mut FILE) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1004fafb() -> bool {
    let mut buf: [u32; 0x275] = [0; 0x275];
    // SAFETY: globals are the game's own memory; record pointers address the
    // game's mission data; the output buffer is written where the original does.
    unsafe {
        let mut local_8: i32 = 0;
        let local_10: *const u8 = (&raw mut DAT_10123240 as *mut u8)
            .byte_offset(DAT_100b0710.wrapping_mul(0x3c0a) as isize);
        let p = buf.as_mut_ptr();
        buf[0] = DAT_100b084c;
        buf[2] = local_10.byte_add(4).cast::<u32>().read_unaligned();
        buf[3] = local_10.byte_add(8).cast::<u32>().read_unaligned();
        buf[4] = u32::from(local_10.byte_add(0x38).read());
        let mut local_c: i32 = 0;
        loop {
            if local_10.cast::<i32>().read_unaligned() <= local_c {
                buf[1] = local_8 as u32;
                let file = FUN_fopen(
                    &raw mut DAT_100b0858 as *const i8,
                    &raw mut DAT_100b0854 as *const i8,
                );
                if !file.is_null() {
                    FUN_fwrite(p as *const u8, 0x9d4, 1, file);
                    FUN_fclose(file);
                }
                return !file.is_null();
            }
            let rec = local_10.byte_offset(local_c.wrapping_mul(0x13f) as isize);
            if rec.byte_add(0xae).cast::<i8>().read() != 0 {
                let ent = p.offset(5isize.wrapping_add((local_8.wrapping_mul(0xd)) as isize));
                if rec.byte_add(0x3a).read() == 5 {
                    ent.write_unaligned(1);
                } else {
                    ent.write_unaligned(0);
                }
                ent.add(1)
                    .write_unaligned(u32::from(rec.byte_add(0x3b).read()));
                ent.add(2)
                    .write_unaligned(rec.byte_add(0xa1).cast::<u32>().read_unaligned());
                ent.add(3)
                    .write_unaligned(rec.byte_add(0xa5).cast::<u32>().read_unaligned());
                ent.add(4)
                    .write_unaligned(rec.byte_add(0xaf).cast::<u32>().read_unaligned());
                let src = rec.byte_add(0xe7);
                let mut n: usize = 0;
                while src.add(n).read() != 0 {
                    n += 1;
                }
                n += 1;
                let dst = ent.add(5) as *mut u8;
                let mut i = 0;
                while i < n {
                    dst.add(i).write(src.add(i).read());
                    i += 1;
                }
                local_8 = local_8.wrapping_add(1);
            }
            local_c = local_c.wrapping_add(1);
        }
    }
}
