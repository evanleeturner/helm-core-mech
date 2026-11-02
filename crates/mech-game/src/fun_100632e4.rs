// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7bcc: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10003742(param_1: u32, param_2: i32, param_3: u32, param_4: i32, param_5: u16);
    fn FUN_100082c4(param_1: i32, param_2: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100632e4(param_1: i32) -> u32 {
    // SAFETY: param_1 points at the game's own structure; every offset read
    // or written is the one the original uses, and the globals and callees
    // are the game's own.
    unsafe {
        let rd = |a: i32| (a as *const i32).read_unaligned();
        let mut local_8: i32 = 0;
        if rd(param_1.wrapping_add(0x3e0)) == -1 {
            return 0;
        }
        let i_var2 = rd(param_1.wrapping_add(0x3e0))
            .wrapping_mul(0x40)
            .wrapping_add(rd(param_1.wrapping_add(0x54)));
        if rd(i_var2.wrapping_add(0x20)) < rd(param_1.wrapping_add(0x60)) {
            local_8 = rd(i_var2.wrapping_add(0x20))
                .wrapping_mul(0x3a)
                .wrapping_add(param_1)
                .wrapping_add(100);
        }
        if local_8 == 0 {
            return 0;
        }
        let idx = rd(i_var2.wrapping_add(4)).wrapping_mul(0x68);
        let dat = (&raw mut DAT_100a7bcc)
            .byte_offset(idx as isize)
            .cast::<i32>()
            .read_unaligned();
        if dat < 1 || rd(local_8.wrapping_add(2)) < 1 {
            return 0;
        }
        FUN_10003742(0xa7, 100, 0x40, 5, 0x50);
        FUN_10003742(0xa7, 100, 0x40, 5, 0x32);
        FUN_100082c4(0x22, -1);
        let mut local_14 = rd(param_1.wrapping_add(0x54));
        let mut local_10: i32 = 0;
        while local_10 < 0xe {
            let q = rd(local_14.wrapping_add(0x3c)) / 100;
            let w = (local_8 as *const u16).read_unaligned();
            if (q as u32) == (w as u32) {
                ((local_14.wrapping_add(8)) as *mut u32).write_unaligned(0xffffffff);
            }
            local_14 = local_14.wrapping_add(0x40);
            local_10 = local_10.wrapping_add(1);
        }
        local_10 = 0;
        while local_10 < rd(local_8.wrapping_add(6)) {
            let e = rd(local_8
                .wrapping_add(14)
                .wrapping_add(local_10.wrapping_mul(4)));
            let addr = e
                .wrapping_mul(0x1e)
                .wrapping_add(2)
                .wrapping_add(rd(param_1.wrapping_add(0x5c)));
            (addr as *mut u16).write_unaligned(0);
            local_10 = local_10.wrapping_add(1);
        }
        (local_8.wrapping_add(2) as *mut u16).write_unaligned(0);
        (local_8.wrapping_add(4) as *mut u16).write_unaligned(0);
        1
    }
}
