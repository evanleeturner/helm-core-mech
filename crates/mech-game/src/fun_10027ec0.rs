// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ab0a0: i32;
    static mut DAT_100b38d8: i32;
    static mut DAT_100eef40: [u32; 0];
    static mut DAT_100ad464: [i8; 0];
}

unsafe extern "C" {
    fn sprintf(dest: *mut i8, format: *const i8, ...) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10027ec0() {
    let mut local_6c = [0i8; 100];
    let mut local_70: i32 = 0;
    // SAFETY: the globals and the entry table are the game's own memory; each
    // table entry points at a live record read at the original's offsets.
    unsafe {
        while local_70 < DAT_100b38d8 {
            let local_8 = (&raw mut DAT_100eef40)
                .cast::<u32>()
                .offset(local_70 as isize)
                .read_unaligned() as i32;
            let base = local_8 as usize as *const u8;
            let flag: u32 = if local_8 == DAT_100ab0a0 {
                0xffff_ffff
            } else {
                0
            };
            let a = ((flag & 0x1e) as i32).wrapping_add(0x20);
            let b = base.byte_add(4).cast::<u32>().read_unaligned();
            let c = base.byte_add(8).cast::<u32>().read_unaligned();
            let d = base.byte_add(0x187).cast::<i8>().read() as i32;
            let e = base.byte_add(0x186).cast::<i8>().read() as i32;
            let f = local_8.wrapping_add(0x110);
            sprintf(
                local_6c.as_mut_ptr(),
                (&raw mut DAT_100ad464).cast::<i8>(),
                a,
                b,
                c,
                d,
                e,
                f,
            );
            local_70 = local_70.wrapping_add(1);
        }
    }
}
