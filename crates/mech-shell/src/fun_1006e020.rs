// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ab618: i32;
    static mut DAT_100ab61c: i32;
    static mut DAT_100ab620: i32;
    static mut DAT_100ab624: i32;
    static mut DAT_100ab628: i32;
    static mut DAT_100ab62c: i32;
    static mut DAT_100ab630: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1006e020() {
    mech_trace::enter!("1006e020");
    // SAFETY: the globals are the game's own memory; each is written by value.
    unsafe {
        mech_trace::gw!("1006e020", "100ab618", DAT_100ab618, 0);
        mech_trace::gw!("1006e020", "100ab61c", DAT_100ab61c, 0);
        mech_trace::gw!("1006e020", "100ab620", DAT_100ab620, 0);
        mech_trace::gw!("1006e020", "100ab624", DAT_100ab624, 0);
        mech_trace::gw!("1006e020", "100ab628", DAT_100ab628, 0);
        mech_trace::gw!("1006e020", "100ab62c", DAT_100ab62c, 0);
        mech_trace::gw!("1006e020", "100ab630", DAT_100ab630, 0);
    }
}
