// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10090078: *mut u8;
    static mut DAT_100901f4: [i8; 1];
    static mut DAT_100a6798: u32;
    static mut DAT_100a67a4: u32;
}

#[allow(non_snake_case)]
unsafe extern "fastcall" {
    fn FUN_10010d80(param_1: *mut u8) -> u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10018060(param_1: i32, param_2: i32) -> u32;
    fn FUN_1000ad70(param_1: i32, param_2: i32) -> i32;
    fn FUN_1001d1b0(param_1: *mut i8) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001d130(param_1: i32) {
    mech_trace::enter!("1001d130", param_1);
    if param_1 != 0 {
        // SAFETY: param_1 points at the caller's structure; the callees and
        // globals are the game's own, used in the original's order.
        unsafe {
            let p = param_1 as u32 as *mut u8;
            if -1 < mech_trace::rd!("1001d130", p.byte_add(0x24).cast::<i32>(), read_unaligned) {
                mech_trace::cr!(
                    "1001d130",
                    "10010d80",
                    FUN_10010d80(mech_trace::gr!("1001d130", "10090078", DAT_10090078))
                );
                let item = mech_trace::cr!(
                    "1001d130",
                    "10018060",
                    FUN_10018060(
                        mech_trace::gr!("1001d130", "100a67a4", DAT_100a67a4) as i32,
                        mech_trace::rd!("1001d130", p.byte_add(0x24).cast::<i32>(), read_unaligned),
                    )
                );
                let field = mech_trace::rd!(
                    "1001d130",
                    (item as usize as *const u8).byte_add(0x24).cast::<i32>(),
                    read_unaligned
                );
                let i_var1 = mech_trace::cr!(
                    "1001d130",
                    "1000ad70",
                    FUN_1000ad70(
                        mech_trace::gr!("1001d130", "100a6798", DAT_100a6798) as i32,
                        field
                    )
                );
                if i_var1 == 0 {
                    mech_trace::cr!(
                        "1001d130",
                        "1001d1b0",
                        FUN_1001d1b0((&raw mut DAT_100901f4).cast::<i8>())
                    );
                    mech_trace::wr!(
                        "1001d130",
                        p.byte_add(0x2c).cast::<u32>(),
                        write_unaligned,
                        2
                    );
                    return;
                }
                mech_trace::cr!(
                    "1001d130",
                    "1001d1b0",
                    FUN_1001d1b0(i_var1.wrapping_add(0x81c) as u32 as *mut i8)
                );
                mech_trace::wr!(
                    "1001d130",
                    p.byte_add(0x2c).cast::<u32>(),
                    write_unaligned,
                    2
                );
            }
        }
    }
}
