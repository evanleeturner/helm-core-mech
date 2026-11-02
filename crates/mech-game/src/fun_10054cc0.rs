// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1011515c: i32;
    static mut DAT_100b0f1c: i32;
    static mut DAT_100b0f20: i32;
    static mut DAT_10115178: i32;
    static mut DAT_10115150: i32;
    static mut DAT_10115154: i32;
    static mut DAT_100aafc8: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10054330(param_1: i32, param_2: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10054cc0(param_1: i32) {
    mech_trace::enter!("10054cc0", param_1);
    // SAFETY: globals and callees are the game's own; param_1 points at a
    // game object whose fields are read at the same offsets as the original.
    unsafe {
        if param_1 != 0
            && mech_trace::gr!("10054cc0", "1011515c", DAT_1011515c) != 0
            && mech_trace::gr!("10054cc0", "100b0f1c", DAT_100b0f1c)
                < mech_trace::gr!("10054cc0", "100b0f20", DAT_100b0f20)
        {
            let p = mech_trace::rd!(
                "10054cc0",
                ((param_1 as usize).wrapping_add(0x6c) as *const *mut u16),
                read_unaligned
            );
            if !p.is_null() {
                let w1 = mech_trace::rd!("10054cc0", p.byte_add(2), read_unaligned);
                if ((w1 as u8) & 0xf0) != 0x20 {
                    mech_trace::gw!(
                        "10054cc0",
                        "10115178",
                        DAT_10115178,
                        mech_trace::gr!("10054cc0", "10115178", DAT_10115178).wrapping_add(1)
                    );
                    if (mech_trace::rd!("10054cc0", p, read_unaligned) & 0x1000) == 0 {
                        let f: extern "C" fn(*mut u16) -> i32 =
                            core::mem::transmute::<usize, extern "C" fn(*mut u16) -> i32>(
                                mech_trace::gr!("10054cc0", "100aafc8", DAT_100aafc8) as u32
                                    as usize,
                            );
                        let r = mech_trace::cr!("10054cc0", "ind", f(p));
                        if r == 0 {
                            mech_trace::gw!(
                                "10054cc0",
                                "10115150",
                                DAT_10115150,
                                mech_trace::gr!("10054cc0", "10115150", DAT_10115150)
                                    .wrapping_add(1)
                            );
                            FUN_10054330(
                                p as usize as i32,
                                mech_trace::gr!("10054cc0", "10115154", DAT_10115154),
                            );
                        }
                    }
                }
            }
            let mut local_8 = mech_trace::rd!(
                "10054cc0",
                ((param_1 as usize).wrapping_add(4) as *const i32),
                read_unaligned
            );
            while local_8 != 0 {
                FUN_10054cc0(local_8);
                local_8 = mech_trace::rd!(
                    "10054cc0",
                    ((local_8 as usize).wrapping_add(8) as *const i32),
                    read_unaligned
                );
            }
        }
    }
}
