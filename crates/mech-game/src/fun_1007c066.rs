// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38fc: u32;
    static mut DAT_100cf0d0: u8;
    static mut DAT_100cf0d4: u8;
    static mut DAT_100cf0d8: u8;
    static mut DAT_100cf0dc: u8;
    static mut DAT_100cf0e0: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1001ca37(param_1: i32, param_2: u16) -> u8;
    fn FUN_1002fcce(param_1: *mut i32) -> u8;
    fn FUN_1007c28e(param_1: *mut i32, param_2: *mut u8) -> u8;
    fn FUN_1007be21(param_1: *mut i32, param_2: u32) -> i32;
    fn FUN_1002fa2b(param_1: i32, param_2: u32) -> u8;
    fn FUN_1002f90b(param_1: i32, param_2: u32) -> u8;
    fn FUN_1002f969(param_1: i32, param_2: u32) -> u8;
    fn FUN_1007bf28(param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1007c066(param_1: *mut i32, param_2: *mut u8, param_3: i32) {
    mech_trace::enter!("1007c066", param_1, param_2, param_3);
    if param_1.is_null() {
        return;
    }
    // SAFETY: param_1 points at the game's object; the globals and callees are
    // the game's own; the table has 0x80 entries of 0x14 bytes.
    unsafe {
        let obj = mech_trace::rd!("1007c066", param_1.add(0x1b), read_unaligned);
        if obj == 0 {
            return;
        }
        let mut b_var2 = false;
        if ((mech_trace::rd!(
            "1007c066",
            ((obj.wrapping_add(2)) as *const u16),
            read_unaligned
        ) as u8)
            & 0xf0)
            == 0x70
            || (param_3 != 0
                && mech_trace::rd!(
                    "1007c066",
                    ((obj.wrapping_add(0x16)) as *const i16),
                    read_unaligned
                ) == 0)
        {
            mech_trace::cr!("1007c066", "1001ca37", FUN_1001ca37(obj, 0x50));
            b_var2 = true;
        }
        let mut local_14: i32 = 0;
        while local_14 < 0x80 {
            let off = local_14.wrapping_mul(0x14) as usize;
            if mech_trace::rd!(
                "1007c066",
                (&raw mut DAT_100cf0d0).byte_add(off).cast::<i32>(),
                read_unaligned
            ) == 0
            {
                break;
            }
            local_14 = local_14.wrapping_add(1);
        }
        if local_14 == 0x80 || b_var2 {
            mech_trace::cr!("1007c066", "1002fcce", FUN_1002fcce(param_1));
            mech_trace::cr!("1007c066", "1007c28e", FUN_1007c28e(param_1, param_2));
        } else {
            let i_var1 = local_14.wrapping_mul(0x14) as usize;
            let i_var3 = mech_trace::cr!("1007c066", "1007be21", FUN_1007be21(param_1, 2));
            if i_var3 < 0 {
                mech_trace::cr!("1007c066", "1007c28e", FUN_1007c28e(param_1, param_2));
            } else {
                mech_trace::wr!(
                    "1007c066",
                    (&raw mut DAT_100cf0d0).byte_add(i_var1).cast::<u32>(),
                    write_unaligned,
                    1
                );
                mech_trace::wr!(
                    "1007c066",
                    (&raw mut DAT_100cf0d4).byte_add(i_var1).cast::<*mut i32>(),
                    write_unaligned,
                    param_1
                );
                mech_trace::wr!(
                    "1007c066",
                    (&raw mut DAT_100cf0e0).byte_add(i_var1).cast::<*mut u8>(),
                    write_unaligned,
                    param_2
                );
                mech_trace::wr!(
                    "1007c066",
                    (&raw mut DAT_100cf0d8).byte_add(i_var1).cast::<u32>(),
                    write_unaligned,
                    mech_trace::gr!("1007c066", "100b38fc", DAT_100b38fc)
                );
                mech_trace::wr!(
                    "1007c066",
                    (&raw mut DAT_100cf0dc).byte_add(i_var1).cast::<u32>(),
                    write_unaligned,
                    0x100000
                );
                mech_trace::cr!("1007c066", "1002fa2b", FUN_1002fa2b(param_1 as i32, 0x300));
                mech_trace::cr!("1007c066", "1002f90b", FUN_1002f90b(param_1 as i32, 0x50));
                mech_trace::cr!(
                    "1007c066",
                    "1002f969",
                    FUN_1002f969(param_1 as i32, local_14 as u32)
                );
                mech_trace::cr!("1007c066", "1007bf28", FUN_1007bf28(i_var3));
            }
        }
    }
}
