// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa1ac: u32;
    static mut DAT_100adf34: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100065c4(param_1: i32, param_2: *mut i32, param_3: i32, param_4: u32) -> u8;
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
    fn FUN_1004bbc3(param_1: u32) -> u8;
    fn FUN_1000fc30(param_1: i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_10002570(param_1: u32, param_2: i32) -> u32;
    fn FUN_1007fcc3(param_1: *mut i32, param_2: *mut i32, param_3: i32) -> u8;
    fn FUN_1004bd53(
        param_1: u32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
    ) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1007fecd(param_1: *mut i32, param_2: i32) {
    // SAFETY: param_1 and param_2 point at the game's own structures, read
    // at the original's offsets; globals and callees are the game's own.
    unsafe {
        let flags = if param_2 != 0 {
            ((param_2.wrapping_add(2)) as *const u16).read_unaligned()
        } else {
            0
        };
        if DAT_100aa1ac == 0 || param_2 == 0 || ((flags as u8) & 0xf0) != 0xb0 {
            let mut i_var1 = FUN_1000fc30(
                param_1.add(0x10d).read_unaligned(),
                param_1.add(0x10e).read_unaligned(),
                param_1.add(0x10f).read_unaligned(),
            );
            if 200000 < i_var1 {
                i_var1 = FUN_10002570(i_var1.wrapping_sub(200000) as u32, 1300000) as i32;
                if ((DAT_100adf34.wrapping_add(3)) as *const i8).read() != 0 {
                    FUN_1007fcc3(param_1, core::ptr::null_mut(), i_var1.wrapping_mul(3));
                }
                if param_2 != 0
                    && (((param_2.wrapping_add(2)) as *const u16).read_unaligned() & 0x200) != 0
                {
                    let base = param_1.read_unaligned();
                    FUN_1004bd53(
                        (base.wrapping_add(4) as *const u32).read_unaligned(),
                        param_2,
                        ((i_var1 as u32).wrapping_mul(3) as i32) >> 0x10,
                        (param_2.wrapping_add(0x34) as *const i32).read_unaligned(),
                        (param_2.wrapping_add(0x38) as *const i32).read_unaligned(),
                        (param_2.wrapping_add(0x3c) as *const i32).read_unaligned(),
                    );
                }
            }
        } else {
            FUN_100065c4(-2, param_1, 0x320000, 7);
            FUN_100065c4(-2, param_1, 0x320000, 8);
            let base = param_1.read_unaligned();
            FUN_1004aa97(
                (base.wrapping_add(4) as *const u32).read_unaligned(),
                0xd,
                (param_2.wrapping_add(0x34) as *const i32).read_unaligned(),
                (param_2.wrapping_add(0x38) as *const i32).read_unaligned(),
                (param_2.wrapping_add(0x3c) as *const i32).read_unaligned(),
                (param_2.wrapping_add(0x34) as *const i32).read_unaligned(),
                (param_2.wrapping_add(0x38) as *const u32).read_unaligned(),
                (param_2.wrapping_add(0x3c) as *const i32).read_unaligned(),
            );
            FUN_1004bbc3(u32::from(
                (param_2.wrapping_add(0x14) as *const u16).read_unaligned(),
            ));
        }
    }
}
