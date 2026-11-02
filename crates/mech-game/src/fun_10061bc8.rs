// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b36fc: u32;
    static mut DAT_100f30cc: [u8; 20];
    static mut DAT_10114ec8: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10003d80(param_1: *mut i32) -> u8;
    fn FUN_10003fc7(param_1: i32, param_2: *mut i32) -> *mut u32;
    fn FUN_1006147f(param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10061bc8(param_1: i32) {
    mech_trace::enter!("10061bc8", param_1);
    let mut local_34: [u32; 5] = [0; 5];
    let mut local_20: [u32; 5] = [0; 5];
    // SAFETY: the globals, callees and the object at param_1 are the game's
    // own memory; copies are 5 dwords between the places the original uses.
    unsafe {
        if (mech_trace::rd!("10061bc8", ((param_1 + 4) as *const i16), read_unaligned) != 0)
            && mech_trace::gr!("10061bc8", "100b36fc", DAT_100b36fc) != 0
        {
            let local_c = mech_trace::rd!(
                "10061bc8",
                ((param_1 + 0xac) as *const *mut i32),
                read_unaligned
            );
            if !local_c.is_null() {
                let s =
                    mech_trace::rd!("10061bc8", ((param_1 + 0xb0) as *const i32), read_unaligned);
                if s != 0 && s != 3 && s != 4 {
                    mech_trace::cr!("10061bc8", "10003d80", FUN_10003d80(local_c));
                }
                let local_8 = mech_trace::cr!("10061bc8", "10003fc7", FUN_10003fc7(1, local_c));
                if !local_8.is_null() {
                    let g = &raw mut DAT_100f30cc as *mut u32;
                    for i in 0..5 {
                        local_20[i] = mech_trace::rd!("10061bc8", g.add(i), read_unaligned);
                    }
                    let p = mech_trace::rd!(
                        "10061bc8",
                        ((param_1 + 0xa4) as *const *mut u32),
                        read_unaligned
                    );
                    for i in 0..5 {
                        local_34[i] = mech_trace::rd!("10061bc8", p.add(i), read_unaligned);
                    }
                    for i in 0..5 {
                        mech_trace::wr!(
                            "10061bc8",
                            g.add(i),
                            write_unaligned,
                            mech_trace::rd!("10061bc8", local_8.add(i), read_unaligned)
                        );
                    }
                    let p = mech_trace::rd!(
                        "10061bc8",
                        ((param_1 + 0xa4) as *const *mut u32),
                        read_unaligned
                    );
                    for i in 0..5 {
                        mech_trace::wr!(
                            "10061bc8",
                            p.add(i),
                            write_unaligned,
                            mech_trace::rd!("10061bc8", local_8.add(i), read_unaligned)
                        );
                    }
                    mech_trace::cr!("10061bc8", "1006147f", FUN_1006147f(param_1));
                    for i in 0..5 {
                        mech_trace::wr!("10061bc8", g.add(i), write_unaligned, local_20[i]);
                    }
                    let p = mech_trace::rd!(
                        "10061bc8",
                        ((param_1 + 0xa4) as *const *mut u32),
                        read_unaligned
                    );
                    for i in 0..5 {
                        mech_trace::wr!("10061bc8", p.add(i), write_unaligned, local_34[i]);
                    }
                }
            }
            mech_trace::wr!(
                "10061bc8",
                ((param_1 + 0xb0) as *mut u32),
                write_unaligned,
                mech_trace::gr!("10061bc8", "10114ec8", DAT_10114ec8)
            );
        }
    }
}
