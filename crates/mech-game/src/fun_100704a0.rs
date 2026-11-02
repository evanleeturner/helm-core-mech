// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b4638: [u32; 1];
    static mut DAT_100eeda0: [u32; 1];
    static mut DAT_100eedac: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100704a0(_param_1: u32, param_2: u32, param_3: i32) {
    mech_trace::enter!("100704a0", _param_1, param_2, param_3);
    let mut local_8: u32 = (param_2 >> 0x10) & 0xff;
    // SAFETY: the globals are the game's own memory; the table index is
    // bounded below 0x59 and the bitmap index is computed as the original does.
    unsafe {
        if local_8 != 0 && ((param_2 >> 0x10) & 0x100) != 0 && local_8 < 0x59 {
            local_8 = mech_trace::rd!(
                "100704a0",
                (&raw mut DAT_100b4638)
                    .cast::<u32>()
                    .offset(local_8 as isize),
                read
            );
        }
        if local_8 != 0 {
            let i_var1: i32 = (local_8 as i32).wrapping_sub(1);
            let idx: i32 = i_var1.wrapping_add(((i_var1 >> 0x1f) as u32 & 0x1f) as i32) >> 5;
            let p = (&raw mut DAT_100eeda0).cast::<u32>().offset(idx as isize);
            let bit: u32 = 1i32.wrapping_shl((i_var1 as u32) & 0x1f) as u32;
            if param_3 == 0 {
                mech_trace::wr!(
                    "100704a0",
                    p,
                    write,
                    mech_trace::rd!("100704a0", p, read) & !bit
                );
            } else {
                mech_trace::wr!(
                    "100704a0",
                    p,
                    write,
                    mech_trace::rd!("100704a0", p, read) | bit
                );
            }
            match local_8 {
                0x1d | 0x70 => {
                    if param_3 == 0 {
                        mech_trace::gu!("100704a0", "100eedac", DAT_100eedac, &=, 0xff7fffff);
                    } else {
                        mech_trace::gu!("100704a0", "100eedac", DAT_100eedac, |=, 0x800000);
                    }
                }
                0x2a | 0x36 => {
                    if param_3 == 0 {
                        mech_trace::gu!("100704a0", "100eedac", DAT_100eedac, &=, 0xffbfffff);
                    } else {
                        mech_trace::gu!("100704a0", "100eedac", DAT_100eedac, |=, 0x400000);
                    }
                }
                0x38 | 0x71 => {
                    if param_3 == 0 {
                        mech_trace::gu!("100704a0", "100eedac", DAT_100eedac, &=, 0xfeffffff);
                    } else {
                        mech_trace::gu!("100704a0", "100eedac", DAT_100eedac, |=, 0x1000000);
                    }
                }
                _ => {}
            }
        }
    }
}
