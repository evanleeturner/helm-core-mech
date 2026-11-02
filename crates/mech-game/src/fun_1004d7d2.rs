// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38fc: u32;
    static mut DAT_100b9f60: u32;
    static mut DAT_10123120: u32;
    static mut DAT_10215684: u8;
    static mut DAT_10215688: u8;
    static mut DAT_1021568c: u8;
    static mut DAT_10215690: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1004d20b(param_1: i32, param_2: u32, param_3: u32, param_4: u32, param_5: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004d7d2(param_1: i32, param_2: i32) -> u32 {
    mech_trace::enter!("1004d7d2", param_1, param_2);
    let mut local_1c: u32 = 0;
    let mut local_18: u32 = 0;
    let mut local_10: u32 = 0;
    let mut local_c: u32 = 0;
    let mut local_8: u32 = 0;
    let p = param_1 as *mut u8;
    // SAFETY: the globals and callee are the game's own; param_1 points at
    // the caller's object, read and written at the original's offsets.
    unsafe {
        mech_trace::gw!(
            "1004d7d2",
            "10123120",
            DAT_10123120,
            mech_trace::gr!("1004d7d2", "100b38fc", DAT_100b38fc) / 0xb5
        );
        if mech_trace::rd!("1004d7d2", p.byte_add(0x128), read) != 0
            && mech_trace::rd!("1004d7d2", p.byte_add(0x12a), read) == 1
        {
            let u_var1 = mech_trace::rd!("1004d7d2", p.byte_add(0x129), read) as u16;
            if (u_var1 as i32) < (mech_trace::gr!("1004d7d2", "100b9f60", DAT_100b9f60) as i32) {
                let off = (u_var1 as usize).wrapping_mul(0x54);
                local_1c = mech_trace::rd!(
                    "1004d7d2",
                    (&raw mut DAT_10215688).byte_add(off).cast::<u32>(),
                    read_unaligned
                );
                local_c = mech_trace::rd!(
                    "1004d7d2",
                    (&raw mut DAT_1021568c).byte_add(off).cast::<u32>(),
                    read_unaligned
                );
                local_10 = mech_trace::rd!(
                    "1004d7d2",
                    (&raw mut DAT_10215690).byte_add(off).cast::<u32>(),
                    read_unaligned
                );
                local_18 = mech_trace::rd!(
                    "1004d7d2",
                    (&raw mut DAT_10215684).byte_add(off).cast::<u32>(),
                    read_unaligned
                );
                local_8 = 1;
            }
        }
        mech_trace::cr!(
            "1004d7d2",
            "1004d20b",
            FUN_1004d20b(param_2, local_1c, local_c, local_10, local_18)
        );
        mech_trace::wr!(
            "1004d7d2",
            p.byte_add(4).cast::<u32>(),
            write_unaligned,
            mech_trace::gr!("1004d7d2", "10123120", DAT_10123120)
        );
        mech_trace::wr!(
            "1004d7d2",
            p.byte_add(0xa1).cast::<u32>(),
            write_unaligned,
            mech_trace::gr!("1004d7d2", "10123120", DAT_10123120)
        );
    }
    mech_trace::rt!("1004d7d2", local_8)
}
