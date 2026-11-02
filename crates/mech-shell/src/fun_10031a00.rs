// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10092000: u32;
    static mut DAT_1009201c: u8;
    static mut DAT_100b51e8: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000a890(param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10031a00(param_1: i32, param_2: i32) {
    mech_trace::enter!("10031a00", param_1, param_2);
    let mut i_var1: i32 = 0;
    // SAFETY: param_1 is the caller's object address; the globals and the
    // callee are the game's own. Reads and writes follow the original.
    unsafe {
        if mech_trace::gr!("10031a00", "1009201c", DAT_1009201c) != 1 {
            let mut pi_var3 = param_1.wrapping_add(0x368) as *mut i32;
            let mut local_4: i32 = 8;
            loop {
                i_var1 = 0xc;
                loop {
                    if mech_trace::rd!("10031a00", pi_var3, read_unaligned) / 10 == param_2 / 10 {
                        mech_trace::wr!("10031a00", pi_var3, write_unaligned, 0);
                    }
                    pi_var3 = pi_var3.wrapping_add(1);
                    i_var1 = i_var1.wrapping_sub(1);
                    if i_var1 == 0 {
                        break;
                    }
                }
                local_4 = local_4.wrapping_sub(1);
                if local_4 == 0 {
                    break;
                }
            }
            if param_2 == 9000 {
                mech_trace::wr!(
                    "10031a00",
                    ((param_1.wrapping_add(0x240)) as *mut u32),
                    write_unaligned,
                    0
                );
                mech_trace::cr!(
                    "10031a00",
                    "1000a890",
                    FUN_1000a890(mech_trace::gr!("10031a00", "10092000", DAT_10092000) as i32)
                );
                return;
            }
            if param_2 == 0x251c {
                mech_trace::wr!(
                    "10031a00",
                    ((param_1.wrapping_add(0x24c)) as *mut u32),
                    write_unaligned,
                    0
                );
                mech_trace::cr!(
                    "10031a00",
                    "1000a890",
                    FUN_1000a890(mech_trace::gr!("10031a00", "10092000", DAT_10092000) as i32)
                );
                return;
            }
            return;
        }
        let mut i_var2: i32 = 0;
        let idx = mech_trace::rd!(
            "10031a00",
            ((param_1.wrapping_add(0x364)) as *const i32),
            read_unaligned
        );
        let mut pi_var3 = idx
            .wrapping_mul(0x30)
            .wrapping_add(0x368)
            .wrapping_add(param_1) as *mut i32;
        loop {
            if mech_trace::rd!("10031a00", pi_var3, read_unaligned) == param_2 {
                let idx2 = mech_trace::rd!(
                    "10031a00",
                    ((param_1.wrapping_add(0x364)) as *const i32),
                    read_unaligned
                );
                let off = i_var2.wrapping_add(idx2.wrapping_mul(0xc)).wrapping_mul(4);
                mech_trace::wr!(
                    "10031a00",
                    (param_1.wrapping_add(0x368).wrapping_add(off) as *mut u32),
                    write_unaligned,
                    0
                );
                i_var1 = 1;
                break;
            }
            pi_var3 = pi_var3.wrapping_add(1);
            i_var2 = i_var2.wrapping_add(1);
            if i_var2 >= 0xc {
                break;
            }
        }
        mech_trace::gw!(
            "10031a00",
            "100b51e8",
            DAT_100b51e8,
            mech_trace::gr!("10031a00", "100b51e8", DAT_100b51e8).wrapping_add(i_var1 as u32)
        );
    }
}
