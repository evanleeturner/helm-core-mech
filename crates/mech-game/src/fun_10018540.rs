// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aae48: u32;
    static mut DAT_100aae50: u32;
    static mut DAT_100aae74: u32;
    static mut DAT_100aae78: u32;
    static mut DAT_100aae7c: u32;
    static mut DAT_100aafbc: u32;
    static mut DAT_10115100: [u8; 0];
    static mut DAT_10160ef0: u32;
    static mut DAT_10160f54: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10035f13();
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10017bc1(param_1: i32, param_2: u32, param_3: i32, param_4: i32) -> u32;
    fn FUN_1001793d(param_1: i32, param_2: *mut i32, param_3: i32, param_4: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10018540() {
    mech_trace::enter!("10018540");
    let mut buf = [0u32; 68];
    // SAFETY: the callee and the globals are the game's own memory; buf is
    // a local buffer standing in for the original's stack frame.
    unsafe {
        FUN_10035f13();
        let u_var1 = mech_trace::gr!("10018540", "100aafbc", DAT_100aafbc);
        let neg = mech_trace::gr!("10018540", "100aae50", DAT_100aae50).wrapping_neg();
        buf[3] = neg;
        buf[37] = mech_trace::gr!("10018540", "100aae50", DAT_100aae50);
        buf[20] = mech_trace::gr!("10018540", "100aae50", DAT_100aae50);
        buf[5] = neg;
        buf[39] = mech_trace::gr!("10018540", "100aae50", DAT_100aae50);
        buf[56] = mech_trace::gr!("10018540", "100aae50", DAT_100aae50);
        buf[55] = 200000;
        buf[38] = 200000;
        buf[21] = 200000;
        buf[4] = 200000;
        mech_trace::gw!(
            "10018540",
            "100aae74",
            DAT_100aae74,
            mech_trace::gr!("10018540", "100aae74", DAT_100aae74).wrapping_add(mech_trace::gr!(
                "10018540",
                "100aae7c",
                DAT_100aae7c
            ))
        );
        mech_trace::gw!(
            "10018540",
            "100aae78",
            DAT_100aae78,
            mech_trace::gr!("10018540", "100aae78", DAT_100aae78).wrapping_add(mech_trace::gr!(
                "10018540",
                "100aae7c",
                DAT_100aae7c
            ))
        );
        if 0x7f < mech_trace::gr!("10018540", "100aae74", DAT_100aae74) {
            mech_trace::gw!("10018540", "100aae74", DAT_100aae74, 0);
            mech_trace::gw!(
                "10018540",
                "100aae78",
                DAT_100aae78,
                mech_trace::gr!("10018540", "100aae48", DAT_100aae48)
            );
        }
        buf[57] = mech_trace::gr!("10018540", "100aae74", DAT_100aae74);
        buf[6] = mech_trace::gr!("10018540", "100aae74", DAT_100aae74);
        buf[40] = mech_trace::gr!("10018540", "100aae78", DAT_100aae78);
        buf[23] = mech_trace::gr!("10018540", "100aae78", DAT_100aae78);
        buf[24] = mech_trace::gr!("10018540", "100aae74", DAT_100aae74);
        buf[7] = mech_trace::gr!("10018540", "100aae74", DAT_100aae74);
        buf[41] = mech_trace::gr!("10018540", "100aae78", DAT_100aae78);
        buf[58] = mech_trace::gr!("10018540", "100aae78", DAT_100aae78);
        buf[60] = 0;
        buf[43] = 0;
        buf[26] = 0;
        buf[9] = 0;
        mech_trace::gw!("10018540", "100aafbc", DAT_100aafbc, 0);
        buf[22] = buf[5];
        buf[54] = buf[3];
        let local_8 = mech_trace::cr!(
            "10018540",
            "10017bc1",
            FUN_10017bc1(
                buf.as_mut_ptr() as usize as i32,
                4,
                mech_trace::gr!("10018540", "10160f54", DAT_10160f54) as i32,
                mech_trace::gr!("10018540", "10160ef0", DAT_10160ef0) as i32,
            )
        ) as i32;
        if local_8 != 0 {
            mech_trace::cr!(
                "10018540",
                "1001793d",
                FUN_1001793d(8, (&raw mut DAT_10115100).cast::<i32>(), local_8, 0)
            );
        }
        mech_trace::gw!("10018540", "100aafbc", DAT_100aafbc, u_var1);
    }
}
