// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3904: u32;
    static mut DAT_100b3a20: u32;
    static mut DAT_100b3a28: u32;
    static mut DAT_100b3a30: u32;
    static mut DAT_100b3a34: u32;
    static mut DAT_100b3a8c: u32;
    static mut DAT_100b3a90: u32;
    static mut DAT_100b3a94: u32;
    static mut DAT_100b3a98: u32;
    static mut DAT_100eee64: u32;
    static s_Framerate__2_2ld__1_1ld_100b3aac: [i8; 0];
    static s__ld__ld_100b3ac4: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _sprintf(_Dest: *mut i8, _Format: *const i8, ...) -> i32;
    fn FUN_1005c0df(param_1: i32, param_2: i32, param_3: *mut i8, param_4: i32, param_5: i32)
    -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10069b2e() {
    mech_trace::enter!("10069b2e");
    let mut local_44: [i8; 64] = [0; 64];
    // SAFETY: the globals, strings and callees are the game's own; the
    // buffer is 64 bytes, as in the original.
    unsafe {
        if mech_trace::gr!("10069b2e", "100eee64", DAT_100eee64) != 0
            || mech_trace::gr!("10069b2e", "100b3a28", DAT_100b3a28) != 0
        {
            mech_trace::gw!(
                "10069b2e",
                "100b3a98",
                DAT_100b3a98,
                mech_trace::gr!("10069b2e", "100b3a98", DAT_100b3a98).wrapping_add(1)
            );
            mech_trace::gw!(
                "10069b2e",
                "100b3a94",
                DAT_100b3a94,
                mech_trace::gr!("10069b2e", "100b3a94", DAT_100b3a94)
                    .wrapping_add(mech_trace::gr!("10069b2e", "100b3904", DAT_100b3904))
            );
            if 9 < (mech_trace::gr!("10069b2e", "100b3a98", DAT_100b3a98) as i32) {
                if 0 < (mech_trace::gr!("10069b2e", "100b3a94", DAT_100b3a94) as i32) {
                    let i_var1 = (mech_trace::gr!("10069b2e", "100b3a98", DAT_100b3a98) as i32)
                        .wrapping_mul(0x712)
                        / (mech_trace::gr!("10069b2e", "100b3a94", DAT_100b3a94) as i32);
                    mech_trace::gw!("10069b2e", "100b3a8c", DAT_100b3a8c, (i_var1 / 10) as u32);
                    mech_trace::gw!("10069b2e", "100b3a90", DAT_100b3a90, (i_var1 % 10) as u32);
                    mech_trace::cr!(
                        "10069b2e",
                        "_sprintf",
                        _sprintf(
                            local_44.as_mut_ptr(),
                            (&raw const s_Framerate__2_2ld__1_1ld_100b3aac).cast::<i8>(),
                            mech_trace::gr!("10069b2e", "100b3a8c", DAT_100b3a8c),
                            mech_trace::gr!("10069b2e", "100b3a90", DAT_100b3a90),
                        )
                    );
                }
                mech_trace::gw!("10069b2e", "100b3a98", DAT_100b3a98, 0);
                mech_trace::gw!("10069b2e", "100b3a94", DAT_100b3a94, 0);
            }
            if mech_trace::gr!("10069b2e", "100b3a28", DAT_100b3a28) != 0
                && 0 < (mech_trace::gr!("10069b2e", "100b3a8c", DAT_100b3a8c) as i32)
            {
                mech_trace::cr!(
                    "10069b2e",
                    "_sprintf",
                    _sprintf(
                        local_44.as_mut_ptr(),
                        (&raw const s__ld__ld_100b3ac4).cast::<i8>(),
                        mech_trace::gr!("10069b2e", "100b3a8c", DAT_100b3a8c),
                        mech_trace::gr!("10069b2e", "100b3a90", DAT_100b3a90),
                    )
                );
                mech_trace::cr!(
                    "10069b2e",
                    "1005c0df",
                    FUN_1005c0df(
                        0x37,
                        1,
                        local_44.as_mut_ptr(),
                        mech_trace::gr!("10069b2e", "100b3a30", DAT_100b3a30) as i32,
                        mech_trace::gr!("10069b2e", "100b3a34", DAT_100b3a34) as i32,
                    )
                );
            }
            mech_trace::gw!("10069b2e", "100b3a20", DAT_100b3a20, 1);
        }
    }
}
