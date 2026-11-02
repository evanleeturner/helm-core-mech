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
            0x74 => DAT_100b0870 = 1,
            0x75 => DAT_100b0870 = 2,
            0x76 => DAT_100b0870 = 3,
            0x77 => DAT_100b0870 = 4,
            0x78 => DAT_100b0870 = 5,
            0x79 => DAT_100b0870 = 6,
            0x7a => DAT_100b0870 = 7,
            0x7b => DAT_100b0870 = 8,
            0x7c => DAT_100b0870 = 9,
            0x7d => DAT_100b0870 = 0,
            0x7e => {
                say!(s_Head_Selected_100b0898);
                DAT_100b0874 = 1;
            }
            0x7f => {
                say!(s_Right_Torso_Selected_100b08a8);
                DAT_100b0874 = 2;
            }
            0x80 => {
                say!(s_Center_Torso_Selected_100b08c0);
                DAT_100b0874 = 3;
            }
            0x81 => {
                say!(s_Left_Torso_Selected_100b08d8);
                DAT_100b0874 = 4;
            }
            0x82 => {
                say!(s_Right_Arm_Selected_100b08ec);
                DAT_100b0874 = 5;
            }
            0x83 => {
                say!(s_Left_Arm_Selected_100b0900);
                DAT_100b0874 = 6;
            }
            0x84 => {
                say!(s_Right_Leg_Selected_100b0914);
                DAT_100b0874 = 7;
            }
            0x85 => {
                say!(s_Left_Leg_Selected_100b0928);
                DAT_100b0874 = 8;
            }
            0x86..=0x8f => {
                DAT_100aca50 = (param_1 & 0xff).wrapping_sub(0x86);
                if DAT_10123230 <= DAT_100aca50 {
                    DAT_100aca50 = 0xffff_ffff;
                }
            }
            0x90 => {
                if DAT_100aca50 == 0xffff_ffff {
                    DAT_100aca50 = 0xffff_fffe;
                } else {
                    DAT_100aca50 = 0xffff_ffff;
                }
            }
            0x91 => DAT_100aca54 = DAT_100aca54.wrapping_sub(1),
            0x92 => DAT_100aca54 = DAT_100aca54.wrapping_add(1),
            0x93 => {
                if DAT_100aca50 != 0xffff_ffff && cell!().read() == 3 {
                    cell!().write(6);
                }
            }
            0x94 => {
                if DAT_100aca50 != 0xffff_ffff && cell!().read() == 3 {
                    cell!().write(5);
                }
            }
            0x95 => FUN_1002cd49(0x100),
            0x96 => FUN_1002cd49(0x200),
            0x97 => FUN_1002cd49(0x400),
            0x98 => FUN_1002cd49(0x800),
            0x9a => {
                DAT_100b3a60 = (DAT_100b3a60 == 0) as u32;
                DAT_100b3a64 = DAT_100b3a60;
            }
            0x9c => {
                if DAT_100aafa8 == 0 {
                    DAT_100aafa8 = 1;
                } else if DAT_100aafa8 == 1 {
                    DAT_100aafa8 = 2;
                } else {
                    DAT_100aafa8 = 0;
                }
            }
            0x9d => {
                DAT_100b3a24 = (DAT_100b3a24 == 0) as u32;
                DAT_100b3a28 = DAT_100b3a24;
            }
            0x9e => {
                let p = (DAT_10114db0 as usize).wrapping_add(6) as *mut i16;
                if p.read_unaligned() == 0 {
                    p.write_unaligned(1);
                } else {
                    p.write_unaligned(0);
                }
            }
            0x9f => {
                DAT_100b3a48 = (DAT_100b3a48 == 0) as u32;
                DAT_100b3a4c = DAT_100b3a48;
            }
            0xa0 => {
                DAT_100b3a54 = (DAT_100b3a54 == 0) as u32;
                DAT_100b3a58 = DAT_100b3a54;
            }
            0xa3 => {
                let p = (DAT_100adf34 as usize).wrapping_add(2) as *mut i8;
                if p.read() == 0 {
                    p.write(1);
                    say!(s_Splash_Damage_ON_100b093c);
                } else {
                    p.write(0);
                    say!(s_Splash_Damage_OFF_100b0950);
                }
            }
            0xa4 => {
                let p = (DAT_100adf34 as usize).wrapping_add(3) as *mut i8;
                if p.read() == 0 {
                    p.write(1);
                    say!(s_Collision_Damage_ON_100b0964);
                } else {
                    p.write(0);
                    say!(s_Collision_Damage_OFF_100b0978);
                }
            }
            0xa5 => {
                let p = (DAT_100adf34 as usize).wrapping_add(4) as *mut i8;
                if p.read() == 0 {
                    p.write(1);
                    say!(s_Heat_Tracking_ON_100b0990);
                } else {
                    p.write(0);
                    say!(s_Heat_Tracking_OFF_100b09a4);
                }
            }
            0xa6 => {
                if DAT_100aefa8 == 2 {
                    DAT_100aefa8 = 1;
                    say!(s_LOD_Quality_HIGH_100b09b8);
                } else {
                    DAT_100aefa8 = 2;
                    say!(s_LOD_Quality_LOW_100b09cc);
                }
            }
            0xa7 => FUN_100864d7(),
            0xa9 => DAT_100a7558 = (DAT_100a7558 == 0) as u32,
            0xaa => DAT_100b3a24 = (DAT_100b3a24 == 0) as u32,
            0xac => DAT_100b3a54 = (DAT_100b3a54 == 0) as u32,
            0xad => DAT_100b3a48 = (DAT_100b3a48 == 0) as u32,
            0xae => {
                if DAT_100aafa4 == 0 {
                    DAT_100aafa4 = 2;
                } else if DAT_100aafa4 == 1 {
                    DAT_100aafa4 = 0;
                } else {
                    DAT_100aafa4 = 1;
                }
            }
            0xaf => DAT_100b3a0c = (DAT_100b3a0c == 0) as u32,
            _ => {}
        }
    }
}
