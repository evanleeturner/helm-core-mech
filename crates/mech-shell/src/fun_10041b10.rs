// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10083508: *mut i8;
    static mut DAT_10098224: *mut core::ffi::c_void;
    static mut DAT_100983cc: *mut i8;
    static mut DAT_100983d0: i32;
    static mut DAT_100a6798: i32;
    static mut DAT_100a67a4: i32;
    static mut DAT_100ba1d8: [i8; 1];
    static mut DAT_100ba1f8: [i8; 1];
    static mut DAT_10098494: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000be10() -> i32;
    fn FUN_10017df0(param_1: i32, param_2: *mut i8);
    fn FUN_10018060(param_1: i32, param_2: i32) -> i32;
    fn FUN_1005bbb0(param_1: *mut i8);
    fn FUN_1000c2b0();
    fn FUN_1000c250();
    fn FUN_1000c410();
}

unsafe extern "stdcall" {
    fn HeapAlloc(
        h_heap: *mut core::ffi::c_void,
        dw_flags: u32,
        dw_bytes: u32,
    ) -> *mut core::ffi::c_void;
}

/// Copies a NUL-terminated string including the terminator.
unsafe fn copy_str(src: *const i8, dst: *mut i8) {
    // SAFETY: caller guarantees src is NUL-terminated and dst has room.
    unsafe {
        let mut n: usize = 0;
        while mech_trace::dr!("10041b10", src.add(n)) != 0 {
            n += 1;
        }
        n += 1;
        let mut i = 0;
        while i < n {
            mech_trace::dw!(
                "10041b10",
                dst.add(i),
                mech_trace::dr!("10041b10", src.add(i))
            );
            i += 1;
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10041b10() -> u32 {
    mech_trace::enter!("10041b10");
    // SAFETY: all globals and callees are the game's own memory; pointers
    // are read and written at the offsets the original uses.
    unsafe {
        mech_trace::dw!(
            "10041b10",
            mech_trace::gr!("10041b10", "10083508", DAT_10083508).add(1),
            1
        );
        mech_trace::dw!(
            "10041b10",
            mech_trace::gr!("10041b10", "10083508", DAT_10083508),
            1
        );
        mech_trace::gw!(
            "10041b10",
            "100983cc",
            DAT_100983cc,
            mech_trace::cr!(
                "10041b10",
                "HeapAlloc",
                HeapAlloc(
                    mech_trace::gr!("10041b10", "10098224", DAT_10098224),
                    1,
                    0x2066
                )
            ) as *mut i8
        );
        FUN_1005bbb0(&raw mut DAT_100ba1d8 as *mut i8);

        copy_str(
            &raw const DAT_100ba1d8 as *const i8,
            mech_trace::gr!("10041b10", "100983cc", DAT_100983cc),
        );

        let p = mech_trace::gr!("10041b10", "100983cc", DAT_100983cc);
        mech_trace::dw!("10041b10", p.add(0x20), -1);
        mech_trace::dw!("10041b10", p.add(0x21), -1);
        mech_trace::dw!("10041b10", p.add(0x22), -1);
        mech_trace::dw!("10041b10", p.add(0x23), -1);
        let u_var2 = mech_trace::cr!("10041b10", "1000be10", FUN_1000be10());
        mech_trace::wr!(
            "10041b10",
            mech_trace::gr!("10041b10", "100983cc", DAT_100983cc)
                .add(0x24)
                .cast::<i32>(),
            write_unaligned,
            u_var2
        );

        let mut i_var3: i32 = 0;
        loop {
            let i_var4 = i_var3.wrapping_add(2);
            let q = mech_trace::gr!("10041b10", "100983cc", DAT_100983cc)
                .offset(i_var3 as isize)
                .add(0x28);
            mech_trace::dw!("10041b10", q, -1);
            mech_trace::dw!("10041b10", q.add(1), -1);
            i_var3 = i_var4;
            if i_var4 >= 0x10 {
                break;
            }
        }

        let p = mech_trace::gr!("10041b10", "100983cc", DAT_100983cc);
        mech_trace::dw!("10041b10", p.add(0x38), 0);
        mech_trace::dw!("10041b10", p.add(0x39), 0);
        mech_trace::dw!("10041b10", p.add(0x3a), 0);
        mech_trace::dw!("10041b10", p.add(0x3b), 0);
        let p = mech_trace::gr!("10041b10", "100983cc", DAT_100983cc);
        mech_trace::dw!("10041b10", p.add(0x3c), 0);
        mech_trace::dw!("10041b10", p.add(0x3d), 0);
        mech_trace::dw!("10041b10", p.add(0x3e), 0);
        mech_trace::dw!("10041b10", p.add(0x3f), 0);
        let p = mech_trace::gr!("10041b10", "100983cc", DAT_100983cc);
        mech_trace::dw!("10041b10", p.add(0x40), 0);
        mech_trace::dw!("10041b10", p.add(0x41), 0);
        mech_trace::dw!("10041b10", p.add(0x42), 0);
        mech_trace::dw!("10041b10", p.add(0x43), 0);
        mech_trace::dw!(
            "10041b10",
            mech_trace::gr!("10041b10", "100983cc", DAT_100983cc).add(0x2064),
            1
        );

        copy_str(
            &raw const DAT_10098494 as *const i8,
            mech_trace::gr!("10041b10", "100983cc", DAT_100983cc).add(100),
        );

        FUN_10017df0(
            mech_trace::gr!("10041b10", "100a67a4", DAT_100a67a4),
            mech_trace::gr!("10041b10", "100983cc", DAT_100983cc),
        );

        copy_str(
            &raw const DAT_100ba1f8 as *const i8,
            mech_trace::gr!("10041b10", "10083508", DAT_10083508).add(0x102),
        );

        let i_var3 = mech_trace::rd!(
            "10041b10",
            mech_trace::gr!("10041b10", "10083508", DAT_10083508)
                .add(0x23bb)
                .cast::<i32>(),
            read_unaligned
        );
        if i_var3 == 0 {
            FUN_1000c2b0();
            FUN_1000c250();
            FUN_1000c410();
            return mech_trace::rt!("10041b10", 0xc026);
        }
        if i_var3 == 1 {
            mech_trace::gw!(
                "10041b10",
                "100983d0",
                DAT_100983d0,
                mech_trace::cr!(
                    "10041b10",
                    "10018060",
                    FUN_10018060(mech_trace::gr!("10041b10", "100a6798", DAT_100a6798), 0)
                )
            );
            if mech_trace::gr!("10041b10", "100983d0", DAT_100983d0) != 0 {
                let d = mech_trace::gr!("10041b10", "100983d0", DAT_100983d0) as usize as *mut u8;
                let v = mech_trace::rd!(
                    "10041b10",
                    mech_trace::gr!("10041b10", "100983cc", DAT_100983cc)
                        .add(0x24)
                        .cast::<i32>(),
                    read_unaligned
                );
                mech_trace::wr!("10041b10", d.add(0x80c).cast::<i32>(), write_unaligned, v);
                let w = mech_trace::rd!("10041b10", d.add(0x810).cast::<i32>(), read_unaligned);
                mech_trace::wr!(
                    "10041b10",
                    mech_trace::gr!("10041b10", "100983cc", DAT_100983cc)
                        .add(0x20)
                        .cast::<i32>(),
                    write_unaligned,
                    w
                );
            }
            return mech_trace::rt!("10041b10", 0xc026);
        }
        if i_var3 != 2 {
            return mech_trace::rt!("10041b10", 0);
        }
        mech_trace::rt!("10041b10", 0xc029)
    }
}
