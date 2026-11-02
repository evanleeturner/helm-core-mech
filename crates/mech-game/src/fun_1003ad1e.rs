// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3904: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002f322(param_1: i32, param_2: *mut u32, param_3: *mut u32, param_4: *mut u32) -> u8;
    fn FUN_10051f43(param_1: i32) -> i32;
    fn FUN_1002f3a7(param_1: i32, param_2: i32, param_3: i32, param_4: i32) -> u8;
    fn FUN_1002fbc0(param_1: *mut i32) -> u8;
    fn FUN_1002f727(param_1: i32) -> u8;
    fn FUN_100399ec(param_1: i32, param_2: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003ad1e(param_1: i32) -> bool {
    let base = param_1 as usize as *mut u8;
    // SAFETY: param_1 points at the game's object; the offsets read and written
    // are those of the original, and the callees and globals are the game's own.
    unsafe {
        let local_c = base.byte_add(0x1f6).cast::<*mut i32>().read_unaligned();
        let mut local_10: u32 = 0;
        let mut local_14: i32 = 0;
        let mut local_18: u32 = 0;
        FUN_1002f322(
            base.byte_add(0x1d2).cast::<i32>().read_unaligned(),
            &raw mut local_10,
            (&raw mut local_14).cast::<u32>(),
            &raw mut local_18,
        );
        if local_14 < 1 {
            let p = base.byte_add(0x10).cast::<u16>();
            p.write_unaligned(p.read_unaligned() & 0xfffe);
            FUN_1002f727(base.byte_add(0x1d2).cast::<i32>().read_unaligned());
            FUN_100399ec(local_c.add(8).read_unaligned(), local_c.read_unaligned());
        } else {
            let scale = DAT_100b3904 as i32;
            let i_var2 = local_c
                .add(5)
                .read_unaligned()
                .wrapping_add(base.byte_add(0x1ea).cast::<i32>().read_unaligned())
                .wrapping_mul(scale);
            let i_var1 = base.byte_add(0x1ee).cast::<i32>().read_unaligned();
            let i_var3 = FUN_10051f43(10);
            let local_8 = base
                .byte_add(0x1f2)
                .cast::<i32>()
                .read_unaligned()
                .wrapping_add(local_c.add(7).read_unaligned())
                .wrapping_mul(scale)
                / 0xb5;
            let third = i_var1
                .wrapping_sub(i_var3)
                .wrapping_add(local_c.add(6).read_unaligned())
                .wrapping_mul(scale)
                / 0xb5;
            FUN_1002f3a7(
                base.byte_add(0x1d2).cast::<i32>().read_unaligned(),
                i_var2 / 0xb5,
                third,
                local_8,
            );
            FUN_1002fbc0(base.byte_add(0x1d2).cast::<*mut i32>().read_unaligned());
            base.byte_add(0x1e6).cast::<u32>().write_unaligned(0xdf);
        }
        0 < local_14
    }
}
