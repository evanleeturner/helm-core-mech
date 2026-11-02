// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100af098: i32;
    static mut DAT_100a7b78: i32;
    static mut DAT_100a7b7c: i32;
    static mut DAT_10160c00: [i32; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1003cc75();
    fn FUN_1001d5cb(param_1: i32) -> i32;
    fn FUN_1002f8af(param_1: i32);
    fn FUN_1003dfde(param_1: *mut u16);
    fn FUN_1003df3a(param_1: *mut u16);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1000e7cf() {
    mech_trace::enter!("1000e7cf");
    // SAFETY: the globals and callees are the game's own; the list nodes are
    // game memory walked exactly as the original does.
    unsafe {
        let mut local_10: i32 = 0;
        if mech_trace::gr!("1000e7cf", "100af098", DAT_100af098) != 0 {
            FUN_1003cc75();
            let base = (&raw mut DAT_10160c00).cast::<i32>();
            let mut local_8: i32 = mech_trace::rd!("1000e7cf", base, read_unaligned);
            while local_8 != 0 {
                local_10 = (local_10 as u32).wrapping_add(1) as i32;
                let mut local_c: *mut u16 = mech_trace::rd!(
                    "1000e7cf",
                    ((local_8 as u32).wrapping_add(8) as *const u32),
                    read_unaligned
                ) as *mut u16;
                while !local_c.is_null() {
                    let w = mech_trace::rd!("1000e7cf", local_c.byte_add(2), read_unaligned);
                    if ((w as u8) & 0xf0) == 0x90 {
                        mech_trace::gw!(
                            "1000e7cf",
                            "100a7b78",
                            DAT_100a7b78,
                            mech_trace::cr!("1000e7cf", "1001d5cb", FUN_1001d5cb(local_c as i32))
                        );
                        FUN_1002f8af(mech_trace::gr!("1000e7cf", "100a7b78", DAT_100a7b78));
                        break;
                    }
                    local_c = mech_trace::rd!(
                        "1000e7cf",
                        local_c.byte_add(8).cast::<u32>(),
                        read_unaligned
                    ) as *mut u16;
                }
                local_c = mech_trace::rd!(
                    "1000e7cf",
                    ((local_8 as u32).wrapping_add(8) as *const u32),
                    read_unaligned
                ) as *mut u16;
                while !local_c.is_null() {
                    let w = mech_trace::rd!("1000e7cf", local_c.byte_add(2), read_unaligned);
                    if ((w as u8) & 0xf0) == 0xa0 {
                        mech_trace::gw!(
                            "1000e7cf",
                            "100a7b7c",
                            DAT_100a7b7c,
                            mech_trace::cr!("1000e7cf", "1001d5cb", FUN_1001d5cb(local_c as i32))
                        );
                        break;
                    }
                    local_c = mech_trace::rd!(
                        "1000e7cf",
                        local_c.byte_add(8).cast::<u32>(),
                        read_unaligned
                    ) as *mut u16;
                }
                let mut p_var1: *mut u16 = mech_trace::rd!(
                    "1000e7cf",
                    ((local_8 as u32).wrapping_add(8) as *const u32),
                    read_unaligned
                ) as *mut u16;
                loop {
                    local_c = p_var1;
                    if local_c.is_null() {
                        break;
                    }
                    p_var1 =
                        mech_trace::rd!("1000e7cf", local_c.add(4).cast::<u32>(), read_unaligned)
                            as *mut u16;
                    if ((mech_trace::rd!("1000e7cf", local_c.add(1), read_unaligned) as u8) & 0xf0)
                        == 0x70
                    {
                        FUN_1003dfde(local_c);
                        FUN_1003df3a(local_c);
                    }
                    if mech_trace::rd!("1000e7cf", local_c.add(0x12).cast::<i32>(), read_unaligned)
                        == 4
                    {
                        FUN_1003df3a(local_c);
                    }
                }
                local_8 =
                    mech_trace::rd!("1000e7cf", base.offset(local_10 as isize), read_unaligned);
            }
        }
    }
}
