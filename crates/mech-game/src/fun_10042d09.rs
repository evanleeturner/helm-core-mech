// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100eef40: [u32; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002a5d4(param_1: i32) -> u8;
    fn FUN_1004403d(param_1: i32, param_2: u8) -> u8;
    fn FUN_1002950f(param_1: i32, param_2: i16) -> bool;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10042d09(param_1: i32) {
    mech_trace::enter!("10042d09", param_1);
    let base = param_1 as usize as *mut u8;
    // SAFETY: param_1 points at the game's object; all offsets read and
    // written are the ones the original touches, and the callees and the
    // global table are the game's own.
    unsafe {
        let sel = mech_trace::rd!(
            "10042d09",
            base.byte_add(0x1c4).cast::<i16>(),
            read_unaligned
        );
        match sel {
            1 => {
                mech_trace::cr!("10042d09", "1002a5d4", FUN_1002a5d4(param_1));
                let w = mech_trace::rd!(
                    "10042d09",
                    base.byte_add(0x17a).cast::<u16>(),
                    read_unaligned
                );
                if (w & 0x200) != 0 {
                    let tbl = (&raw mut DAT_100eef40).cast::<u32>();
                    let entry =
                        mech_trace::rd!("10042d09", tbl.add((w & 0xff) as usize), read_unaligned);
                    let d = mech_trace::rd!(
                        "10042d09",
                        base.byte_add(0x1d4).cast::<i32>(),
                        read_unaligned
                    );
                    let addr = (entry as i32).wrapping_add(0x1f6).wrapping_add(d / 2);
                    let p = addr as usize as *mut i8;
                    mech_trace::wr!(
                        "10042d09",
                        p,
                        write,
                        mech_trace::rd!("10042d09", p, read).wrapping_sub(1)
                    );
                }
            }
            2 | 3 | 6 => {}
            4 => {
                mech_trace::cr!("10042d09", "1004403d", FUN_1004403d(param_1, 0));
            }
            5 => {
                mech_trace::cr!("10042d09", "1004403d", FUN_1004403d(param_1, 0));
                let p = mech_trace::rd!(
                    "10042d09",
                    base.byte_add(0x4c).cast::<i32>(),
                    read_unaligned
                ) as usize as *mut u8;
                mech_trace::wr!("10042d09", p.byte_add(0x20), write, 0);
                let p = mech_trace::rd!(
                    "10042d09",
                    base.byte_add(0x4c).cast::<i32>(),
                    read_unaligned
                ) as usize as *mut u8;
                mech_trace::wr!("10042d09", p.byte_add(0x21), write, 0);
            }
            7 => {
                mech_trace::cr!("10042d09", "1002a5d4", FUN_1002a5d4(param_1));
            }
            8 | 10 => {
                let p = mech_trace::rd!(
                    "10042d09",
                    base.byte_add(0x4c).cast::<i32>(),
                    read_unaligned
                ) as usize as *mut u8;
                mech_trace::wr!("10042d09", p.byte_add(0x30), write, 0);
            }
            9 => {
                mech_trace::cr!("10042d09", "1004403d", FUN_1004403d(param_1, 0));
            }
            0xb => {
                let p = mech_trace::rd!(
                    "10042d09",
                    base.byte_add(0x4c).cast::<i32>(),
                    read_unaligned
                ) as usize as *mut u8;
                mech_trace::wr!("10042d09", p.byte_add(0x20), write, 0);
                let p = mech_trace::rd!(
                    "10042d09",
                    base.byte_add(0x4c).cast::<i32>(),
                    read_unaligned
                ) as usize as *mut u8;
                mech_trace::wr!("10042d09", p.byte_add(0x21), write, 0);
                let p = mech_trace::rd!(
                    "10042d09",
                    base.byte_add(0x4c).cast::<i32>(),
                    read_unaligned
                ) as usize as *mut u8;
                mech_trace::wr!("10042d09", p.byte_add(0x1e), write, 0);
                let p = mech_trace::rd!(
                    "10042d09",
                    base.byte_add(0x4c).cast::<i32>(),
                    read_unaligned
                ) as usize as *mut u8;
                mech_trace::wr!("10042d09", p.byte_add(0x1f), write, 0);
                mech_trace::cr!("10042d09", "1004403d", FUN_1004403d(param_1, 0));
            }
            0xc | 0xd => {
                mech_trace::cr!("10042d09", "1002a5d4", FUN_1002a5d4(param_1));
            }
            _ => {}
        }
        mech_trace::cr!("10042d09", "1004403d", FUN_1004403d(param_1, 0));
        let v = mech_trace::rd!(
            "10042d09",
            base.byte_add(0x1c4).cast::<u16>(),
            read_unaligned
        );
        mech_trace::wr!(
            "10042d09",
            base.byte_add(0x1c6).cast::<u16>(),
            write_unaligned,
            v
        );
        mech_trace::wr!(
            "10042d09",
            base.byte_add(0x1c4).cast::<u16>(),
            write_unaligned,
            0xffff
        );
        mech_trace::wr!(
            "10042d09",
            base.byte_add(0x1d0).cast::<u32>(),
            write_unaligned,
            0
        );
        let v = mech_trace::rd!(
            "10042d09",
            base.byte_add(0x1d0).cast::<u32>(),
            read_unaligned
        );
        mech_trace::wr!(
            "10042d09",
            base.byte_add(0x1cc).cast::<u32>(),
            write_unaligned,
            v
        );
        let v = mech_trace::rd!(
            "10042d09",
            base.byte_add(0x17a).cast::<u16>(),
            read_unaligned
        );
        mech_trace::wr!(
            "10042d09",
            base.byte_add(0x178).cast::<u16>(),
            write_unaligned,
            v
        );
        mech_trace::wr!(
            "10042d09",
            base.byte_add(0x1d8).cast::<u32>(),
            write_unaligned,
            0
        );
        let v = mech_trace::rd!(
            "10042d09",
            base.byte_add(0x1d8).cast::<u32>(),
            read_unaligned
        );
        mech_trace::wr!(
            "10042d09",
            base.byte_add(0x1d4).cast::<u32>(),
            write_unaligned,
            v
        );
        let s = mech_trace::rd!(
            "10042d09",
            base.byte_add(0x17a).cast::<i16>(),
            read_unaligned
        );
        mech_trace::cr!("10042d09", "1002950f", FUN_1002950f(param_1, s));
    }
}
