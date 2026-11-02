// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7558: u32;
    static mut DAT_100aafa4: u32;
    static mut DAT_100aafa8: u32;
    static mut DAT_100aca50: u32;
    static mut DAT_100aca54: u32;
    static mut DAT_100adf34: u32;
    static mut DAT_100aefa8: u32;
    static mut DAT_100b0870: u32;
    static mut DAT_100b0874: u32;
    static mut DAT_100b3a0c: u32;
    static mut DAT_100b3a24: u32;
    static mut DAT_100b3a28: u32;
    static mut DAT_100b3a48: u32;
    static mut DAT_100b3a4c: u32;
    static mut DAT_100b3a54: u32;
    static mut DAT_100b3a58: u32;
    static mut DAT_100b3a60: u32;
    static mut DAT_100b3a64: u32;
    static mut DAT_10114db0: u32;
    static mut DAT_10123230: u32;
    static mut DAT_1012327a: [u8; 0];
    static mut s_Center_Torso_Selected_100b08c0: [i8; 0];
    static mut s_Collision_Damage_OFF_100b0978: [i8; 0];
    static mut s_Collision_Damage_ON_100b0964: [i8; 0];
    static mut s_Head_Selected_100b0898: [i8; 0];
    static mut s_Heat_Tracking_OFF_100b09a4: [i8; 0];
    static mut s_Heat_Tracking_ON_100b0990: [i8; 0];
    static mut s_LOD_Quality_HIGH_100b09b8: [i8; 0];
    static mut s_LOD_Quality_LOW_100b09cc: [i8; 0];
    static mut s_Left_Arm_Selected_100b0900: [i8; 0];
    static mut s_Left_Leg_Selected_100b0928: [i8; 0];
    static mut s_Left_Torso_Selected_100b08d8: [i8; 0];
    static mut s_Right_Arm_Selected_100b08ec: [i8; 0];
    static mut s_Right_Leg_Selected_100b0914: [i8; 0];
    static mut s_Right_Torso_Selected_100b08a8: [i8; 0];
    static mut s_Splash_Damage_OFF_100b0950: [i8; 0];
    static mut s_Splash_Damage_ON_100b093c: [i8; 0];

    fn sprintf(dest: *mut i8, format: *const i8, ...) -> i32;
    fn FUN_1001a5e1(param_1: i32);
    fn FUN_1001a7d4(param_1: i32, param_2: i32);
    fn FUN_1005be24(param_1: *mut i8, param_2: i32, param_3: i32, param_4: i32) -> u32;
    fn FUN_1002cd49(param_1: u32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1002c530();
    fn FUN_100864d7();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005124a(param_1: u32) {
    mech_trace::enter!("1005124a", param_1);
    let mut local_2c = [0i8; 40];
    let buf = local_2c.as_mut_ptr();

    macro_rules! say {
        ($s:ident) => {{
            sprintf(buf, (&raw const $s).cast::<i8>());
            FUN_1005be24(buf, 1, 0x16a, 0x32);
        }};
    }

    macro_rules! cell {
        () => {
            (&raw mut DAT_1012327a).cast::<u8>().offset(
                (DAT_100aca50 as i32)
                    .wrapping_mul(0x3c0a)
                    .wrapping_add((DAT_100aca54 as i32).wrapping_mul(0x13f)) as isize,
            )
        };
    }

    // SAFETY: globals and callees are the game's own; the cell pointer and the
    // DAT_10114db0 / DAT_100adf34 struct pointers are read and written where the
    // original does.
    unsafe {
        match param_1 & 0xffff {
            0x15 => FUN_1001a5e1(2),
            0x17 => FUN_1001a7d4(1, 0),
            0x18 => FUN_1001a7d4(0, 0),
            0x26 => FUN_1002c530(),
            0x74 => mech_trace::gw!("1005124a", "100b0870", DAT_100b0870, 1),
            0x75 => mech_trace::gw!("1005124a", "100b0870", DAT_100b0870, 2),
            0x76 => mech_trace::gw!("1005124a", "100b0870", DAT_100b0870, 3),
            0x77 => mech_trace::gw!("1005124a", "100b0870", DAT_100b0870, 4),
            0x78 => mech_trace::gw!("1005124a", "100b0870", DAT_100b0870, 5),
            0x79 => mech_trace::gw!("1005124a", "100b0870", DAT_100b0870, 6),
            0x7a => mech_trace::gw!("1005124a", "100b0870", DAT_100b0870, 7),
            0x7b => mech_trace::gw!("1005124a", "100b0870", DAT_100b0870, 8),
            0x7c => mech_trace::gw!("1005124a", "100b0870", DAT_100b0870, 9),
            0x7d => mech_trace::gw!("1005124a", "100b0870", DAT_100b0870, 0),
            0x7e => {
                say!(s_Head_Selected_100b0898);
                mech_trace::gw!("1005124a", "100b0874", DAT_100b0874, 1);
            }
            0x7f => {
                say!(s_Right_Torso_Selected_100b08a8);
                mech_trace::gw!("1005124a", "100b0874", DAT_100b0874, 2);
            }
            0x80 => {
                say!(s_Center_Torso_Selected_100b08c0);
                mech_trace::gw!("1005124a", "100b0874", DAT_100b0874, 3);
            }
            0x81 => {
                say!(s_Left_Torso_Selected_100b08d8);
                mech_trace::gw!("1005124a", "100b0874", DAT_100b0874, 4);
            }
            0x82 => {
                say!(s_Right_Arm_Selected_100b08ec);
                mech_trace::gw!("1005124a", "100b0874", DAT_100b0874, 5);
            }
            0x83 => {
                say!(s_Left_Arm_Selected_100b0900);
                mech_trace::gw!("1005124a", "100b0874", DAT_100b0874, 6);
            }
            0x84 => {
                say!(s_Right_Leg_Selected_100b0914);
                mech_trace::gw!("1005124a", "100b0874", DAT_100b0874, 7);
            }
            0x85 => {
                say!(s_Left_Leg_Selected_100b0928);
                mech_trace::gw!("1005124a", "100b0874", DAT_100b0874, 8);
            }
            0x86..=0x8f => {
                mech_trace::gw!(
                    "1005124a",
                    "100aca50",
                    DAT_100aca50,
                    (param_1 & 0xff).wrapping_sub(0x86)
                );
                if mech_trace::gr!("1005124a", "10123230", DAT_10123230)
                    <= mech_trace::gr!("1005124a", "100aca50", DAT_100aca50)
                {
                    mech_trace::gw!("1005124a", "100aca50", DAT_100aca50, 0xffff_ffff);
                }
            }
            0x90 => {
                if mech_trace::gr!("1005124a", "100aca50", DAT_100aca50) == 0xffff_ffff {
                    mech_trace::gw!("1005124a", "100aca50", DAT_100aca50, 0xffff_fffe);
                } else {
                    mech_trace::gw!("1005124a", "100aca50", DAT_100aca50, 0xffff_ffff);
                }
            }
            0x91 => mech_trace::gw!(
                "1005124a",
                "100aca54",
                DAT_100aca54,
                mech_trace::gr!("1005124a", "100aca54", DAT_100aca54).wrapping_sub(1)
            ),
            0x92 => mech_trace::gw!(
                "1005124a",
                "100aca54",
                DAT_100aca54,
                mech_trace::gr!("1005124a", "100aca54", DAT_100aca54).wrapping_add(1)
            ),
            0x93 => {
                if mech_trace::gr!("1005124a", "100aca50", DAT_100aca50) != 0xffff_ffff
                    && mech_trace::rd!("1005124a", cell!(), read) == 3
                {
                    mech_trace::wr!("1005124a", cell!(), write, 6);
                }
            }
            0x94 => {
                if mech_trace::gr!("1005124a", "100aca50", DAT_100aca50) != 0xffff_ffff
                    && mech_trace::rd!("1005124a", cell!(), read) == 3
                {
                    mech_trace::wr!("1005124a", cell!(), write, 5);
                }
            }
            0x95 => FUN_1002cd49(0x100),
            0x96 => FUN_1002cd49(0x200),
            0x97 => FUN_1002cd49(0x400),
            0x98 => FUN_1002cd49(0x800),
            0x9a => {
                mech_trace::gw!(
                    "1005124a",
                    "100b3a60",
                    DAT_100b3a60,
                    (mech_trace::gr!("1005124a", "100b3a60", DAT_100b3a60) == 0) as u32
                );
                mech_trace::gw!(
                    "1005124a",
                    "100b3a64",
                    DAT_100b3a64,
                    mech_trace::gr!("1005124a", "100b3a60", DAT_100b3a60)
                );
            }
            0x9c => {
                if mech_trace::gr!("1005124a", "100aafa8", DAT_100aafa8) == 0 {
                    mech_trace::gw!("1005124a", "100aafa8", DAT_100aafa8, 1);
                } else if mech_trace::gr!("1005124a", "100aafa8", DAT_100aafa8) == 1 {
                    mech_trace::gw!("1005124a", "100aafa8", DAT_100aafa8, 2);
                } else {
                    mech_trace::gw!("1005124a", "100aafa8", DAT_100aafa8, 0);
                }
            }
            0x9d => {
                mech_trace::gw!(
                    "1005124a",
                    "100b3a24",
                    DAT_100b3a24,
                    (mech_trace::gr!("1005124a", "100b3a24", DAT_100b3a24) == 0) as u32
                );
                mech_trace::gw!(
                    "1005124a",
                    "100b3a28",
                    DAT_100b3a28,
                    mech_trace::gr!("1005124a", "100b3a24", DAT_100b3a24)
                );
            }
            0x9e => {
                let p = (mech_trace::gr!("1005124a", "10114db0", DAT_10114db0) as usize)
                    .wrapping_add(6) as *mut i16;
                if mech_trace::rd!("1005124a", p, read_unaligned) == 0 {
                    mech_trace::wr!("1005124a", p, write_unaligned, 1);
                } else {
                    mech_trace::wr!("1005124a", p, write_unaligned, 0);
                }
            }
            0x9f => {
                mech_trace::gw!(
                    "1005124a",
                    "100b3a48",
                    DAT_100b3a48,
                    (mech_trace::gr!("1005124a", "100b3a48", DAT_100b3a48) == 0) as u32
                );
                mech_trace::gw!(
                    "1005124a",
                    "100b3a4c",
                    DAT_100b3a4c,
                    mech_trace::gr!("1005124a", "100b3a48", DAT_100b3a48)
                );
            }
            0xa0 => {
                mech_trace::gw!(
                    "1005124a",
                    "100b3a54",
                    DAT_100b3a54,
                    (mech_trace::gr!("1005124a", "100b3a54", DAT_100b3a54) == 0) as u32
                );
                mech_trace::gw!(
                    "1005124a",
                    "100b3a58",
                    DAT_100b3a58,
                    mech_trace::gr!("1005124a", "100b3a54", DAT_100b3a54)
                );
            }
            0xa3 => {
                let p = (mech_trace::gr!("1005124a", "100adf34", DAT_100adf34) as usize)
                    .wrapping_add(2) as *mut i8;
                if mech_trace::rd!("1005124a", p, read) == 0 {
                    mech_trace::wr!("1005124a", p, write, 1);
                    say!(s_Splash_Damage_ON_100b093c);
                } else {
                    mech_trace::wr!("1005124a", p, write, 0);
                    say!(s_Splash_Damage_OFF_100b0950);
                }
            }
            0xa4 => {
                let p = (mech_trace::gr!("1005124a", "100adf34", DAT_100adf34) as usize)
                    .wrapping_add(3) as *mut i8;
                if mech_trace::rd!("1005124a", p, read) == 0 {
                    mech_trace::wr!("1005124a", p, write, 1);
                    say!(s_Collision_Damage_ON_100b0964);
                } else {
                    mech_trace::wr!("1005124a", p, write, 0);
                    say!(s_Collision_Damage_OFF_100b0978);
                }
            }
            0xa5 => {
                let p = (mech_trace::gr!("1005124a", "100adf34", DAT_100adf34) as usize)
                    .wrapping_add(4) as *mut i8;
                if mech_trace::rd!("1005124a", p, read) == 0 {
                    mech_trace::wr!("1005124a", p, write, 1);
                    say!(s_Heat_Tracking_ON_100b0990);
                } else {
                    mech_trace::wr!("1005124a", p, write, 0);
                    say!(s_Heat_Tracking_OFF_100b09a4);
                }
            }
            0xa6 => {
                if mech_trace::gr!("1005124a", "100aefa8", DAT_100aefa8) == 2 {
                    mech_trace::gw!("1005124a", "100aefa8", DAT_100aefa8, 1);
                    say!(s_LOD_Quality_HIGH_100b09b8);
                } else {
                    mech_trace::gw!("1005124a", "100aefa8", DAT_100aefa8, 2);
                    say!(s_LOD_Quality_LOW_100b09cc);
                }
            }
            0xa7 => FUN_100864d7(),
            0xa9 => mech_trace::gw!(
                "1005124a",
                "100a7558",
                DAT_100a7558,
                (mech_trace::gr!("1005124a", "100a7558", DAT_100a7558) == 0) as u32
            ),
            0xaa => mech_trace::gw!(
                "1005124a",
                "100b3a24",
                DAT_100b3a24,
                (mech_trace::gr!("1005124a", "100b3a24", DAT_100b3a24) == 0) as u32
            ),
            0xac => mech_trace::gw!(
                "1005124a",
                "100b3a54",
                DAT_100b3a54,
                (mech_trace::gr!("1005124a", "100b3a54", DAT_100b3a54) == 0) as u32
            ),
            0xad => mech_trace::gw!(
                "1005124a",
                "100b3a48",
                DAT_100b3a48,
                (mech_trace::gr!("1005124a", "100b3a48", DAT_100b3a48) == 0) as u32
            ),
            0xae => {
                if mech_trace::gr!("1005124a", "100aafa4", DAT_100aafa4) == 0 {
                    mech_trace::gw!("1005124a", "100aafa4", DAT_100aafa4, 2);
                } else if mech_trace::gr!("1005124a", "100aafa4", DAT_100aafa4) == 1 {
                    mech_trace::gw!("1005124a", "100aafa4", DAT_100aafa4, 0);
                } else {
                    mech_trace::gw!("1005124a", "100aafa4", DAT_100aafa4, 1);
                }
            }
            0xaf => mech_trace::gw!(
                "1005124a",
                "100b3a0c",
                DAT_100b3a0c,
                (mech_trace::gr!("1005124a", "100b3a0c", DAT_100b3a0c) == 0) as u32
            ),
            _ => {}
        }
    }
}
