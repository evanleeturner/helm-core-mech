// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10092000: u32;
    static mut DAT_1009368c: i8;
    static mut DAT_1009366c: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1005b1b0(param_1: *mut i8) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10032ce0() {
    let mut local_8: i32 = 0;
    let mut local_c: i32 = 0;
    let mut local_10: i32 = 0;
    // SAFETY: the globals and the callee are the game's own; DAT_10092000
    // holds the address of a game structure whose table at +0x368 is read
    // and written where the original does.
    unsafe {
        loop {
            let mut local_14: i32 = 0;
            loop {
                let base = DAT_10092000.wrapping_add(0x368);
                let i2 = ((base as usize)
                    .wrapping_add((local_10.wrapping_add(local_14) as usize).wrapping_mul(4))
                    as *const i32)
                    .read_unaligned();
                if i2 / 100 == 0x50 {
                    let mut i5: i32 = 0;
                    let mut i4: i32 = 0;
                    loop {
                        let mut i3: i32 = 0;
                        loop {
                            if local_c != i5 || i3 != local_14 {
                                let p = (base as usize)
                                    .wrapping_add((i3.wrapping_add(i4) as usize).wrapping_mul(4))
                                    as *mut i32;
                                let v = p.read_unaligned();
                                if v / 100 == 0x50 && v % 100 == i2 % 100 {
                                    p.write_unaligned(0);
                                    local_8 = local_8.wrapping_add(1);
                                }
                            }
                            i3 = i3.wrapping_add(1);
                            if i3 >= 0xc {
                                break;
                            }
                        }
                        i4 = i4.wrapping_add(0xc);
                        i5 = i5.wrapping_add(1);
                        if i4 >= 0x60 {
                            break;
                        }
                    }
                }
                local_14 = local_14.wrapping_add(1);
                if local_14 >= 0xc {
                    break;
                }
            }
            local_10 = local_10.wrapping_add(0xc);
            local_c = local_c.wrapping_add(1);
            if local_10 >= 0x60 {
                break;
            }
        }
        if local_8 == 0 {
            FUN_1005b1b0(&raw mut DAT_1009366c);
            return;
        }
        FUN_1005b1b0(&raw mut DAT_1009368c);
        ((DAT_10092000.wrapping_add(0x228)) as *mut u32).write_unaligned(0);
    }
}
