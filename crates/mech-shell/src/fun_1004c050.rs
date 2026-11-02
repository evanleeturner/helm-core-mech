// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1009b6e4: u32;
    static mut DAT_1009b6ec: u32;
    static mut DAT_100a675c: u32;
    static mut DAT_100c49d0: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1005bbb0(param_1: *mut i8);
    fn FUN_10055610(
        param_1: *mut i32,
        param_2: u32,
        param_3: u32,
        param_4: u32,
        param_5: i32,
        param_6: *mut i8,
        param_7: u32,
        param_8: *mut i8,
        param_9: *mut u32,
    );
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004c050(
    param_1: i32,
    param_2: *mut i8,
    param_3: u32,
    param_4: u32,
    param_5: u32,
    param_6: i32,
    param_7: *mut i32,
    param_8: i32,
    param_9: *mut i8,
) {
    mech_trace::enter!(
        "1004c050", param_1, param_2, param_3, param_4, param_5, param_6, param_7, param_8, param_9
    );
    // SAFETY: all pointers are the caller's buffers and the game's own memory,
    // accessed where the original accesses them.
    unsafe {
        let src = param_8 as *const i8;
        let mut i_var5: i32 = 0;
        loop {
            let c = mech_trace::dr!("1004c050", src.offset(i_var5 as isize));
            if c == 0 || c == b'.' as i8 {
                break;
            }
            mech_trace::dw!("1004c050", param_2.offset(i_var5 as isize), c);
            i_var5 = i_var5.wrapping_add(1);
            if i_var5 >= 4 {
                break;
            }
        }

        if i_var5 < 4 {
            let mut u_var3: u32 = (4 - i_var5) as u32;
            let mut u_var2: u32 = u_var3 >> 2;
            let mut pc = param_2.offset(i_var5 as isize);
            while u_var2 != 0 {
                u_var2 -= 1;
                mech_trace::wr!("1004c050", pc.cast::<u32>(), write_unaligned, 0x5f5f5f5f);
                pc = pc.add(4);
            }
            i_var5 = i_var5.wrapping_add(u_var3 as i32);
            u_var3 &= 3;
            while u_var3 != 0 {
                mech_trace::dw!("1004c050", pc, b'_' as i8);
                pc = pc.add(1);
                u_var3 -= 1;
            }
        }
        mech_trace::dw!("1004c050", param_2.offset(i_var5 as isize), 0);

        let kind = (param_1 as usize).wrapping_add(0x10) as *const i32;
        let puvar4: *mut u32 = if mech_trace::rd!("1004c050", kind, read_unaligned) == 2 {
            &raw mut DAT_1009b6e4
        } else {
            &raw mut DAT_1009b6ec
        };

        let mut end = param_2;
        while mech_trace::dr!("1004c050", end) != 0 {
            end = end.add(1);
        }

        mech_trace::wr!(
            "1004c050",
            end.cast::<u32>(),
            write_unaligned,
            mech_trace::dr!("1004c050", puvar4)
        );
        mech_trace::dw!(
            "1004c050",
            end.add(3),
            mech_trace::dr!("1004c050", (puvar4.add(1) as *const i8))
        );

        FUN_1005bbb0(param_2);
        FUN_10055610(
            param_7,
            param_3,
            param_4,
            param_5,
            param_6,
            param_2,
            mech_trace::gr!("1004c050", "100a675c", DAT_100a675c),
            param_9,
            &raw mut DAT_100c49d0,
        );
    }
}
