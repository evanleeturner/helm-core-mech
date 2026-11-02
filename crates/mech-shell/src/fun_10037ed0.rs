// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10083508: u32;
    static mut DAT_10094430: [i8; 1];
    static mut DAT_100a6ba8: u32;
}

unsafe extern "C" {
    fn atoi(_Str: *mut i8) -> i32;
    fn rand() -> i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100706c0(param_1: u32);
    fn FUN_10055d70(param_1: *mut i16) -> u32;
    fn FUN_10037d80(param_1: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10037ed0(
    param_1: *mut i16,
    param_2: i8,
    param_3: u32,
    param_4: u32,
    param_5: i32,
    param_6: i8,
) -> u32 {
    mech_trace::enter!(
        "10037ed0", param_1, param_2, param_3, param_4, param_5, param_6
    );
    let mut param_5 = param_5;
    let mut u_var5: u32 = 0;
    // SAFETY: param_1 points at the caller's record of at least 0xf3 bytes;
    // the globals and callees are the game's own memory and code.
    unsafe {
        let mut i = 0usize;
        while i < 0xf3 {
            mech_trace::wr!("10037ed0", param_1.cast::<u8>().add(i), write, 0);
            i += 1;
        }
        if param_5 < 0 {
            let mut local_10 = [0i8; 16];
            local_10[0] = mech_trace::rd!(
                "10037ed0",
                ((mech_trace::gr!("10037ed0", "10083508", DAT_10083508).wrapping_add(0x169))
                    as *const i8),
                read
            );
            local_10[1] = 0;
            param_5 = mech_trace::cr!("10037ed0", "atoi", atoi(local_10.as_mut_ptr()));
        }
        mech_trace::wr!("10037ed0", param_1, write_unaligned, param_5 as i16);
        mech_trace::wr!(
            "10037ed0",
            param_1.byte_add(2).cast::<u32>(),
            write_unaligned,
            param_3
        );
        mech_trace::wr!(
            "10037ed0",
            param_1.byte_add(6).cast::<u32>(),
            write_unaligned,
            param_4
        );
        let v = mech_trace::rd!(
            "10037ed0",
            ((mech_trace::gr!("10037ed0", "10083508", DAT_10083508).wrapping_add(0x122))
                as *const u32),
            read_unaligned
        );
        mech_trace::wr!(
            "10037ed0",
            param_1.byte_add(10).cast::<u32>(),
            write_unaligned,
            v
        );

        let src = &raw mut DAT_10094430 as *mut i8;
        let mut len = 0usize;
        while mech_trace::rd!("10037ed0", src.add(len), read) != 0 {
            len += 1;
        }
        len += 1;
        let dst = param_1.byte_add(14).cast::<i8>();
        let mut j = 0usize;
        while j < len {
            mech_trace::wr!(
                "10037ed0",
                dst.add(j),
                write,
                mech_trace::rd!("10037ed0", src.add(j), read)
            );
            j += 1;
        }

        let mut skip = false;
        if param_2 == 1 {
            let r = mech_trace::gr!("10037ed0", "100a6ba8", DAT_100a6ba8) % 256000000;
            mech_trace::wr!(
                "10037ed0",
                ((mech_trace::gr!("10037ed0", "10083508", DAT_10083508).wrapping_add(0x162))
                    as *mut u32),
                write_unaligned,
                r
            );
            u_var5 = mech_trace::rd!(
                "10037ed0",
                ((mech_trace::gr!("10037ed0", "10083508", DAT_10083508).wrapping_add(0x162))
                    as *const u32),
                read_unaligned
            );
        } else if param_6 != 1 {
            skip = true;
        } else {
            u_var5 = mech_trace::cr!("10037ed0", "rand", rand()) as u32;
        }
        if !skip {
            FUN_100706c0(u_var5);
        }
        mech_trace::wr!(
            "10037ed0",
            param_1.byte_add(0x20).cast::<u32>(),
            write_unaligned,
            u_var5
        );
        let u_var3 = mech_trace::cr!("10037ed0", "10055d70", FUN_10055d70(param_1));
        FUN_10037d80((u_var3 as i16) as i32 as u32);
    }
    mech_trace::rt!("10037ed0", u_var5)
}
