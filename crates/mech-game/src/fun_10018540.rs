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
    let mut buf = [0u32; 68];
    // SAFETY: the callee and the globals are the game's own memory; buf is
    // a local buffer standing in for the original's stack frame.
    unsafe {
        FUN_10035f13();
        let u_var1 = DAT_100aafbc;
        let neg = DAT_100aae50.wrapping_neg();
        buf[3] = neg;
        buf[37] = DAT_100aae50;
        buf[20] = DAT_100aae50;
        buf[5] = neg;
        buf[39] = DAT_100aae50;
        buf[56] = DAT_100aae50;
        buf[55] = 200000;
        buf[38] = 200000;
        buf[21] = 200000;
        buf[4] = 200000;
        DAT_100aae74 = DAT_100aae74.wrapping_add(DAT_100aae7c);
        DAT_100aae78 = DAT_100aae78.wrapping_add(DAT_100aae7c);
        if 0x7f < DAT_100aae74 {
            DAT_100aae74 = 0;
            DAT_100aae78 = DAT_100aae48;
        }
        buf[57] = DAT_100aae74;
        buf[6] = DAT_100aae74;
        buf[40] = DAT_100aae78;
        buf[23] = DAT_100aae78;
        buf[24] = DAT_100aae74;
        buf[7] = DAT_100aae74;
        buf[41] = DAT_100aae78;
        buf[58] = DAT_100aae78;
        buf[60] = 0;
        buf[43] = 0;
        buf[26] = 0;
        buf[9] = 0;
        DAT_100aafbc = 0;
        buf[22] = buf[5];
        buf[54] = buf[3];
        let local_8 = FUN_10017bc1(
            buf.as_mut_ptr() as usize as i32,
            4,
            DAT_10160f54 as i32,
            DAT_10160ef0 as i32,
        ) as i32;
        if local_8 != 0 {
            FUN_1001793d(8, (&raw mut DAT_10115100).cast::<i32>(), local_8, 0);
        }
        DAT_100aafbc = u_var1;
    }
}
