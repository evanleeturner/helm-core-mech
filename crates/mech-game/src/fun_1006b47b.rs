// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ab094: u32;
    static mut DAT_100adf38: u32;
    static mut DAT_100b3904: u32;
    static mut DAT_100aaf68: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1004ab11(param_1: u32, param_2: u32) -> u8;
    fn FUN_1002f322(param_1: i32, param_2: *mut u32, param_3: *mut u32, param_4: *mut u32) -> u8;
    fn FUN_10003742(param_1: u32, param_2: i32, param_3: u32, param_4: i32, param_5: u16) -> u8;
    fn FUN_100037f0(param_1: u32, param_2: u32, param_3: u32, param_4: u32, param_5: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1006b47b(param_1: *mut i32) {
    let mut local_10: u32 = 0;
    let mut local_14: i32 = 0;
    let mut local_18: u32 = 0;
    // SAFETY: param_1 points at the game's object; globals and callees are the
    // game's own; all reads and writes happen where the original does them.
    unsafe {
        let local_c: i32 = *param_1;
        let i_var1: i32 = param_1.add(0xec).read();
        if i_var1 != 0 {
            ((local_c as u32).wrapping_add(0x48) as *mut i32)
                .write_unaligned(param_1.add(0xec).read());
            FUN_1004ab11(0x19, *param_1 as u32);
            FUN_1002f322(
                ((local_c as u32).wrapping_add(0x48) as *const i32).read_unaligned(),
                &raw mut local_10,
                (&raw mut local_14).cast::<u32>(),
                &raw mut local_18,
            );
        }
        let i_var2: i32 = param_1.add(0xed).read();
        if i_var2 != 0 {
            ((local_c as u32).wrapping_add(0x48) as *mut i32)
                .write_unaligned(param_1.add(0xed).read());
            FUN_1004ab11(0x19, *param_1 as u32);
            FUN_1002f322(
                ((local_c as u32).wrapping_add(0x48) as *const i32).read_unaligned(),
                &raw mut local_10,
                (&raw mut local_14).cast::<u32>(),
                &raw mut local_18,
            );
        }
        if i_var2 != 0 || i_var1 != 0 {
            let base = DAT_100aaf68 as *const u8;
            local_10 = local_10.wrapping_sub(base.cast::<i32>().read_unaligned() as u32);
            local_14 = local_14.wrapping_sub(base.byte_add(4).cast::<i32>().read_unaligned());
            local_18 =
                local_18.wrapping_sub(base.byte_add(8).cast::<i32>().read_unaligned() as u32);
            let obj = *param_1 as u32;
            let v = ((obj.wrapping_add(4)) as *const i32).read_unaligned() as u32;
            let p = param_1.add(0xfd).read();
            if v == DAT_100adf38 && p < 0x1c4 && 0x1c4u32 < (p as u32).wrapping_add(DAT_100b3904) {
                FUN_10003742(0xf2, 100, 0x40, 5, 0x50);
            } else {
                FUN_100037f0(
                    local_10,
                    local_14 as u32,
                    local_18,
                    0xf3,
                    DAT_100ab094 as i32,
                );
            }
        }
    }
}
