// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_101e8540: u8;
    static mut DAT_101e8544: u8;
    static mut DAT_101e8546: u8;
    static mut DAT_101e856a: u8;
    static mut DAT_101e856e: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1001dfab(param_1: u32) -> u8;
    fn __strnset(_dest: *mut i8, _val: i32, _count: u32) -> *mut i8;
    fn __close(_file_handle: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001e3ac(param_1: i32) {
    let off = param_1.wrapping_mul(0x14a);
    // SAFETY: the globals and callees are the game's own memory; the offsets
    // index the game's per-slot records exactly as the original does.
    unsafe {
        let p46 = (&raw mut DAT_101e8546)
            .wrapping_byte_add(off as usize)
            .cast::<u32>();
        FUN_1001dfab(p46.read_unaligned());
        p46.write_unaligned(0);
        let mut i: u16 = 0;
        while i < 0x20 {
            let o = (i as u32).wrapping_mul(8).wrapping_add(off as u32) as i32 as usize;
            let pa = (&raw mut DAT_101e856a).wrapping_byte_add(o).cast::<i32>();
            if pa.read_unaligned() != 0 {
                FUN_1001dfab(pa.read_unaligned() as u32);
                pa.write_unaligned(0);
            }
            let pb = (&raw mut DAT_101e856e).wrapping_byte_add(o).cast::<i32>();
            if pb.read_unaligned() != 0 {
                FUN_1001dfab(pb.read_unaligned() as u32);
                pb.write_unaligned(0);
            }
            i = i.wrapping_add(1);
        }
        (&raw mut DAT_101e8544)
            .wrapping_byte_add(off as usize)
            .cast::<u16>()
            .write_unaligned(0);
        __strnset(
            (off.wrapping_add(0x101e866a) as u32 as usize) as *mut i8,
            0,
            0x20,
        );
        __close(
            (&raw mut DAT_101e8540)
                .wrapping_byte_add(off as usize)
                .cast::<i32>()
                .read_unaligned(),
        );
    }
}
