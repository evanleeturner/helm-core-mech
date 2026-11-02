// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3aa8: i32;
    static mut DAT_100b3aa4: i32;
    static mut DAT_100b3a14: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1006a338();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1006a38c(param_1: *mut i8) {
    mech_trace::enter!("1006a38c", param_1);
    let mut b_var2: i32 = 0;
    let mut u_var3: u32 = 0xFFFF_FFFF;
    let mut pc_var4: *mut i8 = param_1;
    // SAFETY: param_1 is a NUL-terminated string from the caller; the globals
    // and callee are the game's own.
    unsafe {
        loop {
            if u_var3 == 0 {
                break;
            }
            u_var3 = u_var3.wrapping_sub(1);
            let c_var1 = mech_trace::rd!("1006a38c", pc_var4, read);
            pc_var4 = pc_var4.wrapping_add(1);
            if c_var1 == 0 {
                break;
            }
        }
        let limit = (!u_var3).wrapping_sub(1) as i32;
        let mut local_10: i32 = 0;
        while local_10 < limit {
            if mech_trace::rd!("1006a38c", param_1.offset(local_10 as isize), read) == b'\n' as i8 {
                b_var2 = 1;
            }
            mech_trace::gw!(
                "1006a38c",
                "100b3aa8",
                DAT_100b3aa8,
                mech_trace::gr!("1006a38c", "100b3aa8", DAT_100b3aa8).wrapping_add(1)
            );
            if 0x4f < mech_trace::gr!("1006a38c", "100b3aa8", DAT_100b3aa8) || b_var2 != 0 {
                mech_trace::gw!("1006a38c", "100b3aa8", DAT_100b3aa8, 0);
                mech_trace::gw!(
                    "1006a38c",
                    "100b3aa4",
                    DAT_100b3aa4,
                    mech_trace::gr!("1006a38c", "100b3aa4", DAT_100b3aa4).wrapping_add(1)
                );
                if 0x17 < mech_trace::gr!("1006a38c", "100b3aa4", DAT_100b3aa4) {
                    FUN_1006a338();
                    mech_trace::gw!("1006a38c", "100b3aa4", DAT_100b3aa4, 3);
                }
                b_var2 = 0;
            }
            local_10 = local_10.wrapping_add(1);
        }
        mech_trace::gw!(
            "1006a38c",
            "100b3a14",
            DAT_100b3a14,
            mech_trace::gr!("1006a38c", "100b3aa4", DAT_100b3aa4)
        );
    }
}
