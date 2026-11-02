// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1008375c: i32;
    static mut DAT_10083760: i32;
    static mut DAT_10083768: i32;
    static mut DAT_1008376c: i32;
    static mut DAT_10083c78: i32;
    static mut DAT_10083c7c: i32;
    static mut DAT_10083d0c: i32;
    static mut DAT_10082d10: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn __ftol() -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10004850() {
    mech_trace::enter!("10004850");
    // SAFETY: the globals are the game's own memory; the table pointers stay
    // within the game's array at the offsets the original uses.
    unsafe {
        let i_var2: i32 = if mech_trace::gr!("10004850", "1008375c", DAT_1008375c) == 0 {
            mech_trace::gr!("10004850", "10083760", DAT_10083760).wrapping_shl(4)
        } else {
            mech_trace::cr!("10004850", "__ftol", __ftol())
        };
        mech_trace::gw!(
            "10004850",
            "10083768",
            DAT_10083768,
            i_var2.wrapping_add(0x31) / 100
        );
        let mut pi_var3: *mut i32 = &raw mut DAT_10083c7c;
        mech_trace::gw!("10004850", "1008376c", DAT_1008376c, 0);
        loop {
            let prev = mech_trace::rd!("10004850", pi_var3.byte_offset(-4), read_unaligned);
            mech_trace::gw!(
                "10004850",
                "1008376c",
                DAT_1008376c,
                mech_trace::gr!("10004850", "1008376c", DAT_1008376c).wrapping_add(prev)
            );
            let cur = mech_trace::rd!("10004850", pi_var3, read_unaligned);
            if -1 < cur {
                mech_trace::gw!(
                    "10004850",
                    "1008376c",
                    DAT_1008376c,
                    mech_trace::gr!("10004850", "1008376c", DAT_1008376c).wrapping_add(cur)
                );
            }
            pi_var3 = pi_var3.byte_add(0x12);
            if !((pi_var3 as usize) < (&raw mut DAT_10083d0c as usize)) {
                break;
            }
        }
        if mech_trace::gr!("10004850", "10083768", DAT_10083768)
            < mech_trace::gr!("10004850", "1008376c", DAT_1008376c)
        {
            loop {
                let i_var1 =
                    mech_trace::gr!("10004850", "10082d10", DAT_10082d10).wrapping_mul(0x12);
                let p78: *mut i32 = (&raw mut DAT_10083c78).byte_offset(i_var1 as isize);
                let i_var2 = mech_trace::rd!("10004850", p78, read_unaligned);
                if 0 < i_var2 {
                    mech_trace::wr!("10004850", p78, write_unaligned, i_var2.wrapping_sub(1));
                    mech_trace::gw!(
                        "10004850",
                        "1008376c",
                        DAT_1008376c,
                        mech_trace::gr!("10004850", "1008376c", DAT_1008376c).wrapping_sub(1)
                    );
                }
                if mech_trace::gr!("10004850", "1008376c", DAT_1008376c)
                    <= mech_trace::gr!("10004850", "10083768", DAT_10083768)
                {
                    return;
                }
                let p7c: *mut i32 = (&raw mut DAT_10083c7c).byte_offset(i_var1 as isize);
                if 0 < mech_trace::rd!("10004850", p7c, read_unaligned) {
                    mech_trace::wr!(
                        "10004850",
                        p7c,
                        write_unaligned,
                        mech_trace::rd!("10004850", p7c, read_unaligned).wrapping_sub(1)
                    );
                    mech_trace::gw!(
                        "10004850",
                        "1008376c",
                        DAT_1008376c,
                        mech_trace::gr!("10004850", "1008376c", DAT_1008376c).wrapping_sub(1)
                    );
                }
                mech_trace::gw!(
                    "10004850",
                    "10082d10",
                    DAT_10082d10,
                    mech_trace::gr!("10004850", "10082d10", DAT_10082d10).wrapping_add(1)
                );
                if 7 < mech_trace::gr!("10004850", "10082d10", DAT_10082d10) {
                    mech_trace::gw!("10004850", "10082d10", DAT_10082d10, 0);
                }
                if !(mech_trace::gr!("10004850", "10083768", DAT_10083768)
                    < mech_trace::gr!("10004850", "1008376c", DAT_1008376c))
                {
                    break;
                }
            }
        }
    }
}
