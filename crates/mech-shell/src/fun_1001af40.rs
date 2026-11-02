// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b12c0: u32;
    static mut DAT_100b12c4: u32;
    static mut DAT_100b50d8: u32;
    static mut DAT_100b12a8: u32;
    static mut DAT_100b1290: u32;
    static mut DAT_100b50f0: u32;
    static mut DAT_1008f984: u32;
    static mut DAT_1008fb14: u32;
    static mut DAT_1008fb18: u32;
    static mut DAT_1008fb1c: u32;
    static mut DAT_1008fb20: u32;
    static mut DAT_1008fb24: u32;
    static mut _DAT_1008fb28: u32;
    static mut DAT_1008fb2c: u32;
    static mut DAT_1008fb30: u32;
    static mut DAT_1008fb34: u32;
    static mut DAT_1008fb38: u32;
    static mut DAT_1008fb3c: u32;
    static mut DAT_1008fb40: u32;
    static mut DAT_1008f968: u32;
    static mut PTR_DAT_1008fb44: *mut u32;
    static mut DAT_1008f988: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1001af40() {
    mech_trace::enter!("1001af40");
    // SAFETY: all globals are the game's own memory; the three 6-word
    // blocks are written within their bounds, as the original does.
    unsafe {
        mech_trace::gw!("1001af40", "100b12c0", DAT_100b12c0, 0);
        mech_trace::gw!("1001af40", "100b12c4", DAT_100b12c4, 0);
        let mut p = &raw mut DAT_100b50d8;
        let mut i: i32 = 6;
        while i != 0 {
            mech_trace::wr!("1001af40", p, write_unaligned, 0);
            p = p.add(1);
            i -= 1;
        }
        p = &raw mut DAT_100b12a8;
        i = 6;
        while i != 0 {
            mech_trace::wr!("1001af40", p, write_unaligned, 0);
            p = p.add(1);
            i -= 1;
        }
        p = &raw mut DAT_100b1290;
        i = 6;
        while i != 0 {
            mech_trace::wr!("1001af40", p, write_unaligned, 0);
            p = p.add(1);
            i -= 1;
        }
        mech_trace::gw!("1001af40", "100b50f0", DAT_100b50f0, 0);
        mech_trace::gw!("1001af40", "1008f984", DAT_1008f984, 0);
        mech_trace::gw!("1001af40", "1008fb14", DAT_1008fb14, 0);
        mech_trace::gw!("1001af40", "1008fb18", DAT_1008fb18, 0);
        mech_trace::gw!("1001af40", "1008fb1c", DAT_1008fb1c, 0);
        mech_trace::gw!("1001af40", "1008fb20", DAT_1008fb20, 0);
        mech_trace::gw!("1001af40", "1008fb24", DAT_1008fb24, 0);
        mech_trace::gw!("1001af40", "1008fb28", _DAT_1008fb28, 0);
        mech_trace::gw!("1001af40", "1008fb2c", DAT_1008fb2c, 0);
        mech_trace::gw!("1001af40", "1008fb30", DAT_1008fb30, 0);
        mech_trace::gw!("1001af40", "1008fb34", DAT_1008fb34, 0);
        mech_trace::gw!("1001af40", "1008fb38", DAT_1008fb38, 0);
        mech_trace::gw!("1001af40", "1008fb3c", DAT_1008fb3c, 0);
        mech_trace::gw!("1001af40", "1008fb40", DAT_1008fb40, 0);
        mech_trace::gw!("1001af40", "1008f968", DAT_1008f968, 0);
        mech_trace::gw!(
            "1001af40",
            "1008fb44",
            PTR_DAT_1008fb44,
            &raw mut DAT_1008f988
        );
    }
}
