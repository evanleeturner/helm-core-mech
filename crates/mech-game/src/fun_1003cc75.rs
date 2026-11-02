// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aefc0: i32;
    static mut DAT_100af098: i32;
    static mut DAT_100aefc4: i32;
    static mut DAT_10160c00: i32;
    static mut DAT_10160c04: i32;
    static mut DAT_100c6d5c: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1003cc75() {
    mech_trace::enter!("1003cc75");
    // SAFETY: the globals are the game's own memory; pointer offsets are
    // scaled in ints exactly as the C's pointer arithmetic on `int` objects,
    // and the loaded pointers refer to game structures.
    unsafe {
        let mut local_14: i32 = 0;
        if mech_trace::gr!("1003cc75", "100aefc0", DAT_100aefc0) == 0 {
            mech_trace::gw!(
                "1003cc75",
                "10160c00",
                DAT_10160c00,
                mech_trace::gr!("1003cc75", "100af098", DAT_100af098)
            );
            mech_trace::gw!("1003cc75", "10160c04", DAT_10160c04, 0);
        } else {
            if mech_trace::gr!("1003cc75", "100aefc4", DAT_100aefc4) != 0
                && mech_trace::rd!(
                    "1003cc75",
                    ((mech_trace::gr!("1003cc75", "100aefc4", DAT_100aefc4).wrapping_add(8))
                        as *const i32),
                    read_unaligned
                ) != 0
            {
                mech_trace::gw!(
                    "1003cc75",
                    "10160c00",
                    DAT_10160c00,
                    mech_trace::gr!("1003cc75", "100aefc4", DAT_100aefc4)
                );
                mech_trace::gw!("1003cc75", "10160c04", DAT_10160c04, 0);
                local_14 = 1;
            }
            let mut local_c: i32 = 0;
            while local_c < 0xc {
                let mut local_10: i32 = 0;
                while local_10 < 0xc {
                    let off = local_10
                        .wrapping_mul(0x18)
                        .wrapping_add(local_c.wrapping_mul(0x120));
                    let p = (&raw mut DAT_100c6d5c).wrapping_offset(off as isize);
                    let i_var1 = mech_trace::rd!("1003cc75", p, read_unaligned);
                    if i_var1 != 0
                        && mech_trace::rd!(
                            "1003cc75",
                            ((i_var1.wrapping_add(8)) as *const i32),
                            read_unaligned
                        ) != 0
                    {
                        let o = local_14.wrapping_mul(4) as isize;
                        mech_trace::wr!(
                            "1003cc75",
                            (&raw mut DAT_10160c00).wrapping_offset(o),
                            write_unaligned,
                            i_var1
                        );
                        mech_trace::wr!(
                            "1003cc75",
                            (&raw mut DAT_10160c04).wrapping_offset(o),
                            write_unaligned,
                            0
                        );
                        local_14 = local_14.wrapping_add(1);
                    }
                    local_10 = local_10.wrapping_add(1);
                }
                local_c = local_c.wrapping_add(1);
            }
            mech_trace::wr!(
                "1003cc75",
                (&raw mut DAT_10160c00).wrapping_offset(local_14.wrapping_mul(4) as isize),
                write_unaligned,
                0
            );
        }
    }
}
