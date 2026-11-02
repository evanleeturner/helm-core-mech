// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10215678: u8;
    static mut DAT_10215694: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002a5d4(param_1: i32) -> u8;
    fn FUN_1004d34a(param_1: u32, param_2: *mut i32, param_3: *mut i32, param_4: *mut u32) -> u32;
    fn FUN_10082490(param_1: u32, param_2: u32, param_3: u32, param_4: u32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002a8fe(param_1: i32) {
    mech_trace::enter!("1002a8fe", param_1);
    let mut local_10: i32 = 0;
    let mut local_c: u32 = 0;
    let mut local_8: i32 = 0;
    // SAFETY: the callees and globals are the game's own; param_1 points at
    // the caller's object, and the tables at DAT_10215678/DAT_10215694 are
    // indexed in 0x54-byte records exactly as the original does.
    unsafe {
        mech_trace::cr!("1002a8fe", "1002a5d4", FUN_1002a5d4(param_1));
        let key = mech_trace::rd!(
            "1002a8fe",
            ((param_1.wrapping_add(4)) as *const u32),
            read_unaligned
        );
        let r = mech_trace::cr!(
            "1002a8fe",
            "1004d34a",
            FUN_1004d34a(key, &raw mut local_8, &raw mut local_10, &raw mut local_c)
        );
        if r != 0 {
            let key = mech_trace::rd!(
                "1002a8fe",
                ((param_1.wrapping_add(4)) as *const u32),
                read_unaligned
            );
            let i_var1 = mech_trace::cr!(
                "1002a8fe",
                "10082490",
                FUN_10082490(key, local_8 as u32, local_c, local_10 as u32)
            );
            if i_var1 != -1 {
                let off = i_var1.wrapping_mul(0x54) as usize;
                let p16 = (&raw mut DAT_10215694).byte_add(off).cast::<u16>();
                mech_trace::wr!(
                    "1002a8fe",
                    p16,
                    write_unaligned,
                    mech_trace::rd!("1002a8fe", p16, read_unaligned) | 1
                );
                let key = mech_trace::rd!(
                    "1002a8fe",
                    ((param_1.wrapping_add(4)) as *const u32),
                    read_unaligned
                );
                mech_trace::wr!(
                    "1002a8fe",
                    (&raw mut DAT_10215678).byte_add(off).cast::<u32>(),
                    write_unaligned,
                    key | 0x200
                );
                mech_trace::wr!(
                    "1002a8fe",
                    (param_1.wrapping_add(0x178) as *mut u16),
                    write_unaligned,
                    (i_var1 as u16) | 0x100
                );
            }
        }
    }
}
