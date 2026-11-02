// SPDX-License-Identifier: MIT

unsafe extern "C" {
    fn FUN_100895ca(param_1: *mut i32, param_2: *mut i32, param_3: *mut u32);
    fn FUN_1001b72b(param_1: *mut u16, param_2: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002fa9c(param_1: *mut i32) {
    mech_trace::enter!("1002fa9c", param_1);
    // SAFETY: param_1 points at the game's node structure; all offsets are
    // read and written where the original does, and callees are the game's own.
    unsafe {
        mech_trace::wr!(
            "1002fa9c",
            param_1.add(0x1d),
            write_unaligned,
            mech_trace::rd!("1002fa9c", param_1.add(0x1d), read_unaligned) & 0xfffffffeu32 as i32
        );
        if mech_trace::rd!("1002fa9c", param_1, read_unaligned) == 0 {
            let mut pivar3 = param_1.add(3);
            let mut pivar4 = param_1.add(0xf);
            let mut ivar2: i32 = 0xc;
            while ivar2 != 0 {
                mech_trace::wr!(
                    "1002fa9c",
                    pivar4,
                    write_unaligned,
                    mech_trace::rd!("1002fa9c", pivar3, read_unaligned)
                );
                pivar3 = pivar3.add(1);
                pivar4 = pivar4.add(1);
                ivar2 = ivar2.wrapping_sub(1);
            }
        } else {
            FUN_100895ca(
                (mech_trace::rd!("1002fa9c", param_1, read_unaligned).wrapping_add(0x3c))
                    as *mut i32,
                param_1.add(3),
                param_1.add(0xf).cast::<u32>(),
            );
        }
        let puvar1 = mech_trace::rd!("1002fa9c", param_1.add(0x1b), read_unaligned) as *mut u16;
        if !puvar1.is_null() {
            FUN_1001b72b(puvar1, param_1.add(0xf));
            mech_trace::wr!(
                "1002fa9c",
                puvar1.add(0xc).cast::<*mut i32>(),
                write_unaligned,
                param_1
            );
        }
        let mut local_8 = mech_trace::rd!("1002fa9c", param_1.add(1), read_unaligned) as *mut i32;
        while !local_8.is_null() {
            FUN_1002fa9c(local_8);
            local_8 = mech_trace::rd!("1002fa9c", local_8.add(2), read_unaligned) as *mut i32;
        }
    }
}
