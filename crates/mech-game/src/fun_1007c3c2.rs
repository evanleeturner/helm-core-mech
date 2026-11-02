// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b6800: u8;
    static mut DAT_100cf0d0: u8;
    static mut DAT_100cf0d4: u8;
    static mut DAT_100cf0e0: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002f28d(param_1: i32) -> u32;
    fn FUN_1001cac9(param_1: i32, param_2: *mut u32, param_3: *mut u32, param_4: *mut u32) -> u32;
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
    fn FUN_1007cbc4(param_1: *mut i32) -> u8;
    fn FUN_1007c9fd(param_1: i32) -> i32;
    fn FUN_1007c97d(param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1007c3c2(param_1: i32) {
    mech_trace::enter!("1007c3c2", param_1);
    let i_var2 = param_1.wrapping_mul(0x14);
    let mut local_14: i32 = 0;
    let mut local_10: i32 = 0;
    let mut local_c: i32 = 0;
    // SAFETY: the globals are the game's own tables, 0x14-byte entries indexed
    // by param_1; callees are the game's functions; pointers read from the
    // tables are used as the original does.
    unsafe {
        let p1 = mech_trace::rd!(
            "1007c3c2",
            (&raw mut DAT_100cf0d4)
                .byte_add(i_var2 as usize)
                .cast::<*mut i32>(),
            read_unaligned
        );
        if !p1.is_null() {
            let i_var3 = mech_trace::cr!("1007c3c2", "1002f28d", FUN_1002f28d(p1 as i32));
            mech_trace::cr!(
                "1007c3c2",
                "1001cac9",
                FUN_1001cac9(
                    i_var3 as i32,
                    (&raw mut local_c).cast::<u32>(),
                    (&raw mut local_10).cast::<u32>(),
                    (&raw mut local_14).cast::<u32>(),
                )
            );
            mech_trace::cr!(
                "1007c3c2",
                "1004aa97",
                FUN_1004aa97(
                    0xfffffffe,
                    7,
                    local_c,
                    local_10,
                    local_14,
                    local_c,
                    local_10 as u32,
                    local_14,
                )
            );
            mech_trace::cr!("1007c3c2", "1007cbc4", FUN_1007cbc4(p1));
            let f_addr = mech_trace::rd!(
                "1007c3c2",
                (&raw mut DAT_100cf0e0)
                    .byte_add(i_var2 as usize)
                    .cast::<usize>(),
                read_unaligned
            );
            if f_addr != 0 {
                let f: unsafe extern "C" fn(*mut i32) = core::mem::transmute(f_addr);
                mech_trace::cr!("1007c3c2", "ind", f(p1));
            }
            let local_8 = mech_trace::cr!("1007c3c2", "1007c9fd", FUN_1007c9fd(p1 as i32));
            if local_8 != -1 {
                mech_trace::cr!("1007c3c2", "1007c97d", FUN_1007c97d(local_8));
            }
            let src = (&raw mut DAT_100b6800).cast::<u32>();
            let dst = (&raw mut DAT_100cf0d0)
                .byte_add(i_var2 as usize)
                .cast::<u32>();
            let mut i = 0usize;
            while i < 5 {
                mech_trace::wr!(
                    "1007c3c2",
                    dst.add(i),
                    write_unaligned,
                    mech_trace::rd!("1007c3c2", src.add(i), read_unaligned)
                );
                i += 1;
            }
        }
    }
}
