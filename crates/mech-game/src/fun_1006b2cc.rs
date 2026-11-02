// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
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
    fn FUN_10051f43(param_1: i32) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10051f89() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1006b2cc(param_1: *mut i32) {
    // SAFETY: param_1 points at the game's object whose first field is a
    // pointer to a record; reads happen where the original does, and the
    // callees are the game's own.
    unsafe {
        let base = param_1.read_unaligned() as usize as *const u8;
        let mut i_var1 = base.byte_add(0x50).cast::<i32>().read_unaligned();
        let i_var2 = base.byte_add(0x54).cast::<i32>().read_unaligned();
        let mut i_var3 = base.byte_add(0x58).cast::<i32>().read_unaligned();
        if param_1.add(0xef).read_unaligned() == 0 {
            let b = param_1.read_unaligned() as usize as *const u8;
            FUN_1004aa97(
                b.byte_add(0x13c).cast::<u32>().read_unaligned(),
                0xd,
                i_var1,
                i_var2,
                i_var3,
                i_var1,
                i_var3 as u32,
                0,
            );
        } else {
            let mut i_var4 = FUN_10051f43(100);
            if i_var4 < 0x15 {
                i_var4 = FUN_10051f89() as i32;
                i_var1 = i_var1.wrapping_add(i_var4 / 2);
                i_var4 = FUN_10051f89() as i32;
                i_var3 = i_var3.wrapping_add(i_var4 / 2);
                i_var4 = FUN_10051f43(100);
                let b = param_1.read_unaligned() as usize as *const u8;
                let code: u32 = if i_var4 < 0x3c { 3 } else { 0x10b };
                FUN_1004aa97(
                    b.byte_add(0x13c).cast::<u32>().read_unaligned(),
                    code,
                    i_var1,
                    i_var2,
                    i_var3,
                    i_var1,
                    i_var2 as u32,
                    i_var3,
                );
            }
        }
    }
}
