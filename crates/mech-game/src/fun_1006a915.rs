// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3c48: i32;
    static mut DAT_100b3c4c: i32;
    static mut DAT_100b3c50: i32;
    static mut DAT_100b3c54: i32;
    static mut _DAT_100b3c58: i32;
    static mut DAT_100b3c9c: [i32; 5];
    static mut DAT_100aa1ac: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100470be();
    fn FUN_1006ab7b();
    fn FUN_1006ac3b();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1006a915(param_1: i32) {
    mech_trace::enter!("1006a915", param_1);
    if -1 < param_1 {
        // SAFETY: the globals and callees are the game's own memory and code;
        // the array is read by raw pointer at the original's indices.
        unsafe {
            let arr = (&raw mut DAT_100b3c9c).cast::<i32>();
            match param_1 {
                0 => {
                    mech_trace::gw!(
                        "1006a915",
                        "100b3c4c",
                        DAT_100b3c4c,
                        mech_trace::rd!("1006a915", arr.add(1), read)
                    );
                }
                1 => {
                    mech_trace::gw!(
                        "1006a915",
                        "100b3c50",
                        DAT_100b3c50,
                        mech_trace::rd!("1006a915", arr.add(2), read)
                    );
                    if mech_trace::gr!("1006a915", "100b3c50", DAT_100b3c50) == 0 {
                        mech_trace::gw!(
                            "1006a915",
                            "100b3c58",
                            _DAT_100b3c58,
                            (mech_trace::gr!("1006a915", "100b3c58", _DAT_100b3c58) as u32
                                & 0xfffffffd) as i32
                        );
                    } else {
                        mech_trace::gw!(
                            "1006a915",
                            "100b3c58",
                            _DAT_100b3c58,
                            (mech_trace::gr!("1006a915", "100b3c58", _DAT_100b3c58) as u32 | 2)
                                as i32
                        );
                    }
                }
                2 => {
                    mech_trace::gw!(
                        "1006a915",
                        "100b3c54",
                        DAT_100b3c54,
                        mech_trace::rd!("1006a915", arr.add(3), read)
                    );
                    if mech_trace::gr!("1006a915", "100b3c54", DAT_100b3c54) == 0 {
                        mech_trace::gw!(
                            "1006a915",
                            "100b3c58",
                            _DAT_100b3c58,
                            (mech_trace::gr!("1006a915", "100b3c58", _DAT_100b3c58) as u32
                                & 0xfffffff3) as i32
                        );
                        FUN_1006ac3b();
                    } else {
                        mech_trace::gw!(
                            "1006a915",
                            "100b3c58",
                            _DAT_100b3c58,
                            (mech_trace::gr!("1006a915", "100b3c58", _DAT_100b3c58) as u32 | 0xc)
                                as i32
                        );
                        FUN_100470be();
                        if mech_trace::gr!("1006a915", "100aa1ac", DAT_100aa1ac) == 0 {
                            FUN_1006ab7b();
                        }
                    }
                }
                3 => {
                    mech_trace::gw!(
                        "1006a915",
                        "100b3c48",
                        DAT_100b3c48,
                        mech_trace::rd!("1006a915", arr, read)
                    );
                }
                4 => {
                    mech_trace::gw!(
                        "1006a915",
                        "100b3c58",
                        _DAT_100b3c58,
                        mech_trace::rd!("1006a915", arr.add(4), read)
                    );
                }
                _ => {}
            }
        }
    }
}
