// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10083508: u32;
    static mut DAT_100a67ac: u32;
    static mut DAT_100a67b0: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10018060(param_1: i32, param_2: i32) -> u32;
    #[link_name = "_rand"]
    fn _rand() -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1004c460() {
    mech_trace::enter!("1004c460");
    // SAFETY: globals and callees are the game's own; the pointers they yield
    // address game structures read and written at the original's offsets.
    unsafe {
        if mech_trace::rd!(
            "1004c460",
            ((mech_trace::gr!("1004c460", "10083508", DAT_10083508).wrapping_add(0x23b7))
                as *const i8),
            read
        ) != 1
        {
            let mut i_var3: i32 = 0;
            if 0 < mech_trace::rd!(
                "1004c460",
                ((mech_trace::gr!("1004c460", "100a67ac", DAT_100a67ac).wrapping_add(4))
                    as *const i32),
                read_unaligned
            ) {
                loop {
                    let i_var1 = mech_trace::cr!(
                        "1004c460",
                        "10018060",
                        FUN_10018060(
                            mech_trace::gr!("1004c460", "100a67ac", DAT_100a67ac) as i32,
                            i_var3
                        )
                    );
                    let i_var2 = mech_trace::cr!("1004c460", "_rand", _rand());
                    let p = i_var1 as *mut u8;
                    mech_trace::wr!("1004c460", p.byte_add(9), write, 1);
                    if mech_trace::rd!("1004c460", p.byte_add(0x12).cast::<i32>(), read_unaligned)
                        < i_var2 % 100
                    {
                        mech_trace::wr!("1004c460", p.byte_add(9), write, 0);
                    }
                    i_var3 = i_var3.wrapping_add(1);
                    if !(i_var3
                        < mech_trace::rd!(
                            "1004c460",
                            ((mech_trace::gr!("1004c460", "100a67ac", DAT_100a67ac).wrapping_add(4))
                                as *const i32),
                            read_unaligned
                        ))
                    {
                        break;
                    }
                }
            }
            i_var3 = 0;
            if 0 < mech_trace::rd!(
                "1004c460",
                ((mech_trace::gr!("1004c460", "100a67b0", DAT_100a67b0).wrapping_add(4))
                    as *const i32),
                read_unaligned
            ) {
                loop {
                    let i_var1 = mech_trace::cr!(
                        "1004c460",
                        "10018060",
                        FUN_10018060(
                            mech_trace::gr!("1004c460", "100a67b0", DAT_100a67b0) as i32,
                            i_var3
                        )
                    );
                    let i_var2 = mech_trace::cr!("1004c460", "_rand", _rand());
                    let p = i_var1 as *mut u8;
                    mech_trace::wr!("1004c460", p.byte_add(0x84e), write, 1);
                    if mech_trace::rd!("1004c460", p.byte_add(0x847).cast::<i32>(), read_unaligned)
                        < i_var2 % 100
                    {
                        mech_trace::wr!("1004c460", p.byte_add(0x84e), write, 0);
                    }
                    i_var3 = i_var3.wrapping_add(1);
                    if !(i_var3
                        < mech_trace::rd!(
                            "1004c460",
                            ((mech_trace::gr!("1004c460", "100a67b0", DAT_100a67b0).wrapping_add(4))
                                as *const i32),
                            read_unaligned
                        ))
                    {
                        break;
                    }
                }
            }
        }
    }
}
