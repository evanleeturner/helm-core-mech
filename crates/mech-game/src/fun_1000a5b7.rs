// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7160: u32;
    static mut DAT_100a7b30: [u8; 0];
    static mut DAT_100adcd0: u32;
    static mut DAT_100b38fc: u32;
    static mut DAT_100b3904: u32;
    static mut DAT_100c4f30: u32;
    static mut DAT_100c4f34: u32;
    static mut DAT_10215658: u32;
    static mut DAT_10215668: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100498c5(param_1: i32) -> i32;
    fn FUN_10051f43(param_1: i32) -> i32;
    fn FUN_10003d80(param_1: *mut i32) -> u8;
    fn FUN_1000a3fd(param_1: i32, param_2: *mut u32, param_3: *mut i32, param_4: i32) -> u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1000898c() -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1000a5b7() {
    mech_trace::enter!("1000a5b7");
    // SAFETY: the globals and callees are the game's own; the pointers read
    // from the game's tables are dereferenced exactly where the original does.
    unsafe {
        let base = (&raw mut DAT_100a7b30).cast::<u8>();
        let puvar1 = mech_trace::rd!(
            "1000a5b7",
            base.byte_add(
                mech_trace::gr!("1000a5b7", "10215658", DAT_10215658).wrapping_mul(4) as usize
            )
            .cast::<*mut u32>(),
            read_unaligned
        );
        if puvar1.is_null() {
            return;
        }
        let pi_var2 = mech_trace::rd!(
            "1000a5b7",
            puvar1.byte_add(20).cast::<*mut i32>(),
            read_unaligned
        );
        if pi_var2.is_null() {
            return;
        }
        let mut i_var3 = mech_trace::cr!("1000a5b7", "100498c5", FUN_100498c5(4));
        if i_var3 != 0 || mech_trace::gr!("1000a5b7", "10215668", DAT_10215668) != 0 {
            mech_trace::gw!("1000a5b7", "10215668", DAT_10215668, 0);
            mech_trace::cr!("1000a5b7", "1000898c", FUN_1000898c());
            return;
        }
        let mut skip_to_end = false;
        if mech_trace::gr!("1000a5b7", "100a7160", DAT_100a7160) == 0 {
            let p = mech_trace::rd!("1000a5b7", pi_var2.byte_add(4), read_unaligned) as *const i32;
            i_var3 = mech_trace::cr!(
                "1000a5b7",
                "10051f43",
                FUN_10051f43(mech_trace::rd!("1000a5b7", p, read_unaligned))
            );
            mech_trace::gw!(
                "1000a5b7",
                "100c4f30",
                DAT_100c4f30,
                mech_trace::gr!("1000a5b7", "100b38fc", DAT_100b38fc).wrapping_add(i_var3 as u32)
            );
            mech_trace::gw!("1000a5b7", "100a7160", DAT_100a7160, 1);
        } else if mech_trace::gr!("1000a5b7", "100a7160", DAT_100a7160) != 1 {
            if mech_trace::gr!("1000a5b7", "100a7160", DAT_100a7160) != 2 {
                return;
            }
            skip_to_end = true;
        }
        if !skip_to_end {
            if mech_trace::gr!("1000a5b7", "100b38fc", DAT_100b38fc)
                < mech_trace::gr!("1000a5b7", "100c4f30", DAT_100c4f30)
            {
                mech_trace::cr!("1000a5b7", "1000898c", FUN_1000898c());
                return;
            }
            mech_trace::gw!("1000a5b7", "100a7160", DAT_100a7160, 2);
            let p = mech_trace::rd!("1000a5b7", pi_var2.byte_add(4), read_unaligned) as *const i32;
            i_var3 = mech_trace::cr!(
                "1000a5b7",
                "10051f43",
                FUN_10051f43(mech_trace::rd!("1000a5b7", p, read_unaligned))
            );
            mech_trace::gw!(
                "1000a5b7",
                "100c4f34",
                DAT_100c4f34,
                mech_trace::gr!("1000a5b7", "100b38fc", DAT_100b38fc).wrapping_add(i_var3 as u32)
            );
            mech_trace::cr!("1000a5b7", "10003d80", FUN_10003d80(pi_var2));
        }
        mech_trace::gw!("1000a5b7", "100adcd0", DAT_100adcd0, 0);
        let p = mech_trace::rd!("1000a5b7", pi_var2.byte_add(4), read_unaligned) as *const i32;
        let arg = mech_trace::rd!("1000a5b7", p, read_unaligned)
            .wrapping_sub(mech_trace::gr!("1000a5b7", "100b3904", DAT_100b3904) as i32)
            .wrapping_add(-1);
        i_var3 = mech_trace::cr!("1000a5b7", "10051f43", FUN_10051f43(arg));
        let q = mech_trace::rd!("1000a5b7", pi_var2, read_unaligned) as *mut u8;
        mech_trace::wr!(
            "1000a5b7",
            q.byte_add(8).cast::<i32>(),
            write_unaligned,
            i_var3
        );
        mech_trace::cr!("1000a5b7", "1000a3fd", FUN_1000a3fd(1, puvar1, pi_var2, 1));
        if mech_trace::gr!("1000a5b7", "100c4f34", DAT_100c4f34)
            < mech_trace::gr!("1000a5b7", "100b38fc", DAT_100b38fc)
        {
            mech_trace::gw!("1000a5b7", "100a7160", DAT_100a7160, 0);
        }
    }
}
