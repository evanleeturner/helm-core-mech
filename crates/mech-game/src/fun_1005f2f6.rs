// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38fc: u32;
    static mut DAT_100f3170: u8;
    static mut DAT_10113170: u8;
    static mut DAT_10113172: u8;
    static mut DAT_10113174: u8;
    static mut DAT_10113176: u8;
    static mut DAT_10113178: u8;
    static mut DAT_1011317a: u8;
}

fn at(base: *mut u8, off: u32) -> *mut u8 {
    (base as usize).wrapping_add(off as usize) as *mut u8
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1005f2f6() {
    mech_trace::enter!("1005f2f6");
    // SAFETY: the globals are the game's own tables; every access reads or
    // writes at the offsets the original uses.
    unsafe {
        let i_var2 = mech_trace::gr!("1005f2f6", "100b38fc", DAT_100b38fc) as i32;
        let b70 = &raw mut DAT_10113170;
        let b72 = &raw mut DAT_10113172;
        let b74 = &raw mut DAT_10113174;
        let b76 = &raw mut DAT_10113176;
        let b78 = &raw mut DAT_10113178;
        let b7a = &raw mut DAT_1011317a;
        let tbl = &raw mut DAT_100f3170;
        let mut local_18: i32 = 0;
        while local_18 < 0x200 {
            let i_var1 = local_18.wrapping_mul(0xe) as u32;
            if mech_trace::rd!("1005f2f6", at(b78, i_var1).cast::<i16>(), read_unaligned) > -1
                && mech_trace::rd!("1005f2f6", at(b76, i_var1).cast::<i16>(), read_unaligned) != 0
                && mech_trace::rd!("1005f2f6", at(b74, i_var1).cast::<i16>(), read_unaligned) != 0
            {
                if mech_trace::rd!("1005f2f6", at(b7a, i_var1).cast::<i32>(), read_unaligned) == 0 {
                    mech_trace::wr!(
                        "1005f2f6",
                        at(b7a, i_var1).cast::<i32>(),
                        write_unaligned,
                        i_var2
                    );
                }
                let div = mech_trace::rd!("1005f2f6", at(b74, i_var1).cast::<u16>(), read_unaligned)
                    as i32;
                let num = i_var2.wrapping_sub(mech_trace::rd!(
                    "1005f2f6",
                    at(b7a, i_var1).cast::<i32>(),
                    read_unaligned
                ));
                let i_var3 = num / div;
                let mut local_14: u32 =
                    mech_trace::rd!("1005f2f6", at(b72, i_var1).cast::<i16>(), read_unaligned)
                        as i32 as u32;
                if 0 < i_var3 {
                    let mut local_20: i32 = 0;
                    while local_20 < i_var3 {
                        local_14 = local_14.wrapping_add(1) & 0x1f;
                        let sel = mech_trace::rd!(
                            "1005f2f6",
                            at(b70, i_var1).cast::<i16>(),
                            read_unaligned
                        ) as i32;
                        let off = local_14
                            .wrapping_mul(8)
                            .wrapping_add(sel.wrapping_mul(0x100) as u32);
                        if mech_trace::rd!("1005f2f6", at(tbl, off).cast::<i16>(), read_unaligned)
                            < 1
                        {
                            local_14 = 0;
                        }
                        local_20 = local_20.wrapping_add(1);
                    }
                    if mech_trace::rd!("1005f2f6", at(b76, i_var1).cast::<i16>(), read_unaligned)
                        == 2
                        && local_14
                            < (mech_trace::rd!(
                                "1005f2f6",
                                at(b72, i_var1).cast::<i16>(),
                                read_unaligned
                            ) as i32 as u32)
                    {
                        mech_trace::wr!(
                            "1005f2f6",
                            at(b76, i_var1).cast::<u16>(),
                            write_unaligned,
                            0
                        );
                        local_14 = 0;
                    }
                    mech_trace::wr!(
                        "1005f2f6",
                        at(b72, i_var1).cast::<i16>(),
                        write_unaligned,
                        local_14 as i16
                    );
                    let old =
                        mech_trace::rd!("1005f2f6", at(b7a, i_var1).cast::<i32>(), read_unaligned);
                    let w =
                        (mech_trace::rd!("1005f2f6", at(b74, i_var1).cast::<u16>(), read_unaligned)
                            as u32)
                            .wrapping_mul(i_var3 as u32);
                    let inner = (i_var2.wrapping_sub(old) as u32).wrapping_sub(w);
                    mech_trace::wr!(
                        "1005f2f6",
                        at(b7a, i_var1).cast::<u32>(),
                        write_unaligned,
                        (i_var2 as u32).wrapping_sub(inner)
                    );
                }
            }
            local_18 = local_18.wrapping_add(1);
        }
    }
}
