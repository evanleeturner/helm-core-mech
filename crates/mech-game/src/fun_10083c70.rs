// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf38: u32;
    static mut DAT_100eef40: [u32; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10083c3e(param_1: i32) -> u8;
    fn FUN_100832d4(param_1: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10083c70() {
    mech_trace::enter!("10083c70");
    // SAFETY: the globals and callees are the game's own memory and code;
    // iVar1 points at the game's object, read and written where the original does.
    unsafe {
        let mut local_c: u32 = 0;
        let mut local_14: u32 = 0xffff_ffff;
        let mut local_20: i32 = 0x7fff_ffff;
        let i_var1: i32 = mech_trace::rd!(
            "10083c70",
            (&raw mut DAT_100eef40).cast::<u32>().add(mech_trace::gr!(
                "10083c70",
                "100adf38",
                DAT_100adf38
            ) as usize),
            read_unaligned
        ) as i32;
        let u_var2: u32 = mech_trace::rd!(
            "10083c70",
            ((i_var1 as u32).wrapping_add(0xdc) as *const u32),
            read_unaligned
        );
        let p20 = mech_trace::rd!(
            "10083c70",
            ((i_var1 as u32).wrapping_add(0x20) as *const u32),
            read_unaligned
        );
        let b_var5 = mech_trace::rd!(
            "10083c70",
            (p20.wrapping_add(0x3f0) as *const i32),
            read_unaligned
        ) == 1;
        mech_trace::cr!("10083c70", "10083c3e", FUN_10083c3e(0));
        while local_14 != local_c {
            let u_var3 = mech_trace::rd!(
                "10083c70",
                ((i_var1 as u32).wrapping_add(0xdc) as *const u32),
                read_unaligned
            );
            if (u_var3 & 0x1000) != 0 {
                break;
            }
            local_c = u_var3;
            let v = mech_trace::rd!(
                "10083c70",
                ((i_var1 as u32).wrapping_add(0xc4) as *const i32),
                read_unaligned
            );
            if v < local_20 {
                local_c = 0;
                local_20 = v;
                local_14 = u_var3;
            }
            mech_trace::cr!("10083c70", "10083c3e", FUN_10083c3e(1));
        }
        if local_14 == 0xffff_ffff || 0x2ab98 < local_20 {
            mech_trace::wr!(
                "10083c70",
                ((i_var1 as u32).wrapping_add(0xdc) as *mut u32),
                write_unaligned,
                u_var2
            );
            if b_var5 {
                let p4c = mech_trace::rd!(
                    "10083c70",
                    ((i_var1 as u32).wrapping_add(0x4c) as *const u32),
                    read_unaligned
                );
                mech_trace::wr!("10083c70", (p4c.wrapping_add(0x43) as *mut u8), write, 0);
            }
        } else {
            mech_trace::wr!(
                "10083c70",
                ((i_var1 as u32).wrapping_add(0xdc) as *mut u32),
                write_unaligned,
                local_14
            );
            let i_var4 = mech_trace::cr!("10083c70", "100832d4", FUN_100832d4(i_var1));
            if i_var4 == 0 {
                mech_trace::wr!(
                    "10083c70",
                    ((i_var1 as u32).wrapping_add(0xdc) as *mut u32),
                    write_unaligned,
                    u_var2
                );
                if b_var5 {
                    let p4c = mech_trace::rd!(
                        "10083c70",
                        ((i_var1 as u32).wrapping_add(0x4c) as *const u32),
                        read_unaligned
                    );
                    mech_trace::wr!("10083c70", (p4c.wrapping_add(0x43) as *mut u8), write, 0);
                }
            }
        }
    }
}
