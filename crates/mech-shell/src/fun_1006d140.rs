// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a67b4: *mut i32;
    static mut DAT_100a67b8: *mut i32;
    static mut DAT_100aa6b8: *mut u32;
    static mut DAT_100aa6c0: *mut u32;
    static mut DAT_100aa6e0: *mut i32;
    static mut DAT_100aa6e4: *mut i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10017df0(param_1: *mut i32, param_2: u32) -> u32;
    fn FUN_10017ec0(param_1: i32, param_2: i32, param_3: i8);
    fn FUN_10018060(param_1: i32, param_2: i32) -> u32;
    fn FUN_100180f0(param_1: *mut i32, param_2: i8);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1006d140() {
    mech_trace::enter!("1006d140");
    // SAFETY: the globals and callees are the game's own; the collection
    // pointers and the copy buffers are valid in the original's memory.
    unsafe {
        let mut i_var1: i32 = mech_trace::rd!(
            "1006d140",
            mech_trace::gr!("1006d140", "100aa6e0", DAT_100aa6e0).byte_add(4),
            read_unaligned
        );
        while 0 < i_var1 {
            i_var1 = mech_trace::cr!(
                "1006d140",
                "10018060",
                FUN_10018060(
                    mech_trace::gr!("1006d140", "100aa6e0", DAT_100aa6e0) as i32,
                    0
                )
            ) as i32;
            FUN_10017ec0(
                mech_trace::gr!("1006d140", "100aa6e0", DAT_100aa6e0) as i32,
                i_var1,
                0,
            );
            mech_trace::cr!(
                "1006d140",
                "10017df0",
                FUN_10017df0(
                    mech_trace::gr!("1006d140", "100a67b4", DAT_100a67b4),
                    i_var1 as u32
                )
            );
            i_var1 = mech_trace::rd!(
                "1006d140",
                mech_trace::gr!("1006d140", "100aa6e0", DAT_100aa6e0).byte_add(4),
                read_unaligned
            );
        }
        FUN_100180f0(mech_trace::gr!("1006d140", "100aa6e0", DAT_100aa6e0), 0);
        i_var1 = mech_trace::rd!(
            "1006d140",
            mech_trace::gr!("1006d140", "100aa6e4", DAT_100aa6e4).byte_add(4),
            read_unaligned
        );
        while 0 < i_var1 {
            i_var1 = mech_trace::cr!(
                "1006d140",
                "10018060",
                FUN_10018060(
                    mech_trace::gr!("1006d140", "100aa6e4", DAT_100aa6e4) as i32,
                    0
                )
            ) as i32;
            FUN_10017ec0(
                mech_trace::gr!("1006d140", "100aa6e4", DAT_100aa6e4) as i32,
                i_var1,
                0,
            );
            mech_trace::cr!(
                "1006d140",
                "10017df0",
                FUN_10017df0(
                    mech_trace::gr!("1006d140", "100a67b8", DAT_100a67b8),
                    i_var1 as u32
                )
            );
            i_var1 = mech_trace::rd!(
                "1006d140",
                mech_trace::gr!("1006d140", "100aa6e4", DAT_100aa6e4).byte_add(4),
                read_unaligned
            );
        }
        FUN_100180f0(mech_trace::gr!("1006d140", "100aa6e4", DAT_100aa6e4), 0);
        let mut pu_var2 = mech_trace::gr!("1006d140", "100aa6c0", DAT_100aa6c0);
        let mut pu_var3 = mech_trace::gr!("1006d140", "100aa6b8", DAT_100aa6b8);
        let mut n: i32 = 0x214;
        while n != 0 {
            mech_trace::wr!(
                "1006d140",
                pu_var3,
                write_unaligned,
                mech_trace::rd!("1006d140", pu_var2, read_unaligned)
            );
            pu_var2 = pu_var2.byte_add(4);
            pu_var3 = pu_var3.byte_add(4);
            n -= 1;
        }
        mech_trace::wr!(
            "1006d140",
            pu_var3.cast::<u16>(),
            write_unaligned,
            mech_trace::rd!("1006d140", pu_var2.cast::<u16>(), read_unaligned)
        );
        mech_trace::wr!(
            "1006d140",
            pu_var3.byte_add(2).cast::<u8>(),
            write,
            mech_trace::rd!("1006d140", pu_var2.byte_add(2).cast::<u8>(), read)
        );
    }
}
