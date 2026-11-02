// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b06f0: u32;
    static mut DAT_101ec758: u8;
    static mut DAT_101ec780: u8;
    static mut DAT_101ec794: u8;
    static mut DAT_101ec7a0: u8;
    static mut DAT_101ec7a8: u8;
    static mut DAT_101ec7ac: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002f727(param_1: i32) -> u8;
    fn FUN_1002f7f7(param_1: i32) -> u8;
    fn FUN_1007d08d(param_1: i32) -> u8;
    fn FUN_10030fd2(param_1: i32) -> u8;
    fn FUN_10049a9b(param_1: i32) -> u8;
    fn FUN_1004ab46(
        param_1: u32,
        param_2: i32,
        param_3: u32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
        param_7: i32,
        param_8: u32,
        param_9: i32,
        param_10: i32,
        param_11: i32,
        param_12: i32,
    ) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004a9bd(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
    param_7: i32,
    param_8: u32,
    param_9: i32,
) {
    let i_var2 = param_2.wrapping_mul(0x60) as usize;
    // SAFETY: the globals are the game's own tables, indexed by byte offset
    // as the original does; the callees are the game's functions.
    unsafe {
        if (&raw mut DAT_101ec7a0)
            .byte_add(i_var2)
            .cast::<i32>()
            .read_unaligned()
            != 0
        {
            DAT_100b06f0 = 0xffff_ffff;
        }
        FUN_1002f727(
            (&raw mut DAT_101ec780)
                .byte_add(i_var2)
                .cast::<i32>()
                .read_unaligned(),
        );
        FUN_1002f7f7(
            (&raw mut DAT_101ec780)
                .byte_add(i_var2)
                .cast::<i32>()
                .read_unaligned(),
        );
        let u_var1 = (&raw mut DAT_101ec758)
            .byte_add(i_var2)
            .cast::<u32>()
            .read_unaligned();
        FUN_1007d08d(
            (&raw mut DAT_101ec7a8)
                .byte_add(i_var2)
                .cast::<i32>()
                .read_unaligned(),
        );
        FUN_10030fd2(
            (&raw mut DAT_101ec7ac)
                .byte_add(i_var2)
                .cast::<i32>()
                .read_unaligned(),
        );
        (&raw mut DAT_101ec7a8)
            .byte_add(i_var2)
            .cast::<u32>()
            .write_unaligned(0xffff_ffff);
        (&raw mut DAT_101ec7ac)
            .byte_add(i_var2)
            .cast::<u32>()
            .write_unaligned(0xffff_ffff);
        FUN_10049a9b(param_2);
        if param_3 != 0 {
            FUN_1004ab46(
                (&raw mut DAT_101ec794)
                    .byte_add(i_var2)
                    .cast::<u32>()
                    .read_unaligned(),
                param_1,
                u_var1,
                param_4,
                param_5,
                param_6,
                param_7,
                param_8,
                param_9,
                0,
                0,
                0,
            );
        }
    }
}
