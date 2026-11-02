// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38d8: u32;
    static mut DAT_100eef40: [u32; 1];
    static mut DAT_101604d0: [u8; 1];
    static mut DAT_101604f0: [u8; 1];
    static mut DAT_10160510: [u8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1004d47f(param_1: i32) -> u32;
    fn FUN_1002fc9d(param_1: i32) -> i32;
    fn FUN_10089060(
        param_1: *mut i32,
        param_2: *mut i32,
        param_3: *mut i32,
        param_4: *mut i32,
    ) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004d34a(
    param_1: u32,
    param_2: *mut i32,
    param_3: *mut i32,
    param_4: *mut u32,
) -> u32 {
    // SAFETY: the globals and callees are the game's own; the pointers are
    // the caller's and are read and written where the original does.
    unsafe {
        let mut local_18: i32 = 0;
        if param_1 < DAT_100b38d8 {
            let tab = (&raw mut DAT_100eef40).cast::<u32>();
            let i_var1 = ((tab.add(param_1 as usize).read() as usize).wrapping_add(8)
                as *const i32)
                .read_unaligned();
            let local_c = FUN_1004d47f(i_var1);
            let local_8 = ((tab.add(param_1 as usize).read() as usize).wrapping_add(0xc)
                as *const i32)
                .read_unaligned();
            if param_1 == local_c {
                let base = tab.add(local_c as usize).read() as usize;
                param_2.write_unaligned((base.wrapping_add(0x50) as *const i32).read_unaligned());
                let base = tab.add(local_c as usize).read() as usize;
                param_3.write_unaligned((base.wrapping_add(0x58) as *const i32).read_unaligned());
                let base = tab.add(local_c as usize).read() as usize;
                param_4.write_unaligned((base.wrapping_add(0x60) as *const u32).read_unaligned());
            } else {
                let off = local_8
                    .wrapping_mul(4)
                    .wrapping_add(i_var1.wrapping_mul(0x70)) as usize;
                param_2.write_unaligned(
                    ((&raw mut DAT_101604d0 as usize).wrapping_add(off) as *const i32)
                        .read_unaligned(),
                );
                param_3.write_unaligned(
                    ((&raw mut DAT_101604f0 as usize).wrapping_add(off) as *const i32)
                        .read_unaligned(),
                );
                param_4.write_unaligned(
                    ((&raw mut DAT_10160510 as usize).wrapping_add(off) as *const u32)
                        .read_unaligned(),
                );
                let base = tab.add(local_c as usize).read() as usize;
                let local_14 = (base.wrapping_add(0x40) as *const i32).read_unaligned();
                let local_10 = FUN_1002fc9d(local_14) as usize as *mut i32;
                FUN_10089060(local_10, param_2, &raw mut local_18, param_3);
            }
            1
        } else {
            0
        }
    }
}
