// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a9a38: u32;
    static mut DAT_100b0700: u32;
    static mut DAT_100c7e08: u32;
    static mut DAT_100c7e2c: u32;
    static mut DAT_100c7e30: u32;
    static mut DAT_100c7e34: u32;
    static mut DAT_100c7e38: u32;
    static mut DAT_101f26f0: u8;
    static mut DAT_101f2714: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1006065d(param_1: i32, param_2: u32, param_3: i32) -> u32;
    fn FUN_1004aa97(
        param_1: u32,
        param_2: u32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
        param_7: u32,
        param_8: i32,
    ) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004c173(param_1: i32) {
    // SAFETY: the globals and callees are the game's own; param_1 points at
    // a game structure, and the table entries are 0x2c bytes apart as in the
    // original.
    unsafe {
        let u_var2: u32 = DAT_100a9a38.wrapping_mul(2);
        let mut local_8: i32 = 0xff;
        loop {
            let i_var1 = local_8.wrapping_sub(1);
            if local_8 == 0 {
                break;
            }
            local_8 = i_var1;
            let off = (i_var1.wrapping_mul(0x2c)) as isize;
            let a = (&raw mut DAT_101f2714)
                .byte_offset(off)
                .cast::<i32>()
                .read_unaligned();
            if a != 0x16 {
                continue;
            }
            let p = (&raw mut DAT_101f26f0)
                .byte_offset(off)
                .cast::<i32>()
                .read_unaligned();
            if p == 0 {
                continue;
            }
            let q = ((p as u32).wrapping_add(0x6c) as usize as *const i32).read_unaligned();
            if q == 0 {
                continue;
            }
            break;
        }
        DAT_100c7e08 = 10000;
        FUN_1006065d(0x12, u_var2, 2);
        DAT_100c7e30 = (param_1.wrapping_add(0x50) as usize as *const i32).read_unaligned() as u32;
        DAT_100c7e34 = (param_1.wrapping_add(0x54) as usize as *const i32).read_unaligned() as u32;
        DAT_100c7e38 = (param_1.wrapping_add(0x58) as usize as *const i32).read_unaligned() as u32;
        DAT_100c7e2c = 0;
        DAT_100b0700 = u_var2;
        FUN_1004aa97(
            0xfffffffe,
            0x16,
            DAT_100c7e30 as i32,
            DAT_100c7e34 as i32,
            DAT_100c7e38 as i32,
            DAT_100c7e30 as i32,
            DAT_100c7e34,
            DAT_100c7e38 as i32,
        );
    }
}
