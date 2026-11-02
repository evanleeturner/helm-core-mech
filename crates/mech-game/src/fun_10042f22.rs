// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10215678: u8;
    static mut DAT_10215694: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10042fcf(
        param_1: u32,
        param_2: i16,
        param_3: *mut i32,
        param_4: *mut i32,
        param_5: *mut u32,
        param_6: i16,
    ) -> u8;
    fn FUN_10082490(param_1: u32, param_2: u32, param_3: u32, param_4: u32) -> i32;
    fn FUN_1002a89b(param_1: i32, param_2: i16) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10042f22(param_1: i32, param_2: u32, param_3: i16, param_4: i16) {
    let mut local_8: i32 = 0;
    let mut local_10: i32 = 0;
    let mut local_c: u32 = 0;
    // SAFETY: the callees and globals are the game's own; param_1 points at the
    // caller's structure, and the tables at the globals' addresses are indexed
    // by the entry the callee returned, as the original does.
    unsafe {
        FUN_10042fcf(
            param_2,
            param_3,
            &raw mut local_8,
            &raw mut local_10,
            &raw mut local_c,
            param_4,
        );
        let field = ((param_1.wrapping_add(4)) as *const u32).read_unaligned();
        let i_var1 = FUN_10082490(field, local_8 as u32, local_c, local_10 as u32);
        if i_var1 != -1 {
            let off = i_var1.wrapping_mul(0x54) as usize;
            let p = (&raw mut DAT_10215694).byte_add(off).cast::<u16>();
            p.write_unaligned(p.read_unaligned() | 1);
            let field2 = ((param_1.wrapping_add(4)) as *const u32).read_unaligned();
            (&raw mut DAT_10215678)
                .byte_add(off)
                .cast::<u32>()
                .write_unaligned(field2 | 0x200);
            FUN_1002a89b(param_1, 0x100);
        }
    }
}
