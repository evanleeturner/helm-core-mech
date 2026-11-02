// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002954c(param_1: u16) -> i32;
    fn FUN_1002883b(param_1: i32, param_2: u16, param_3: i16) -> u32;
    fn FUN_1002c354(param_1: i32) -> u8;
    fn FUN_1002a89b(param_1: i32, param_2: i16) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002872e(param_1: i32, param_2: u16, param_3: i16) -> i16 {
    // SAFETY: the callees are the game's own; param_1 is an address of the
    // game's object, read and written at the offsets the original uses.
    unsafe {
        let local_8: i16;
        if param_3 == 0 {
            let i_var1 = FUN_1002954c(param_2);
            let u_var2 = FUN_1002883b(param_1, param_2, (i_var1 / 100) as i16);
            local_8 = u_var2 as i16;
            if local_8 != 0 && (param_2 & 0x100) != 0 {
                if (param_1.wrapping_add(0x1a2) as *const i32).read_unaligned()
                    == (param_2 as i16) as i32
                {
                    FUN_1002c354(param_1);
                }
                if (param_1.wrapping_add(0x176) as *const i16).read_unaligned() == 7 {
                    FUN_1002a89b(param_1, param_2 as i16);
                    let v = (param_1.wrapping_add(0xdc) as *const u32).read_unaligned();
                    (param_1.wrapping_add(0x178) as *mut i16).write_unaligned(v as i16);
                    if ((param_1.wrapping_add(0xdd) as *const u8).read() & 0x10) != 0 {
                        FUN_1002a89b(param_1, 0x100);
                    }
                } else {
                    FUN_1002a89b(param_1, param_2 as i16);
                }
            }
        } else {
            let u_var2 = FUN_1002883b(param_1, param_2, param_3);
            local_8 = u_var2 as i16;
        }
        local_8
    }
}
