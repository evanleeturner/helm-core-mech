// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a502c: [u8; 0];
    static mut DAT_100a5030: u32;
    static mut DAT_100a5034: u32;
    static mut DAT_100a5038: u32;
    static mut DAT_100a503c: u32;
    static mut PTR_DAT_100a4fa8: [u32; 0];
    static mut PTR_DAT_100a5028: [u32; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10007440(param_1: *mut i32, param_2: *mut i32, param_3: i32) -> bool;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10007f49(param_1: i32, param_2: i32) {
    // SAFETY: all globals and the callee are the game's own memory; the
    // table reads and pointer values mirror the original's accesses.
    unsafe {
        let mut local_8: *mut i32 = core::ptr::null_mut();
        let mut local_c: *mut i32 = core::ptr::null_mut();
        if -1 < param_1 && (param_1 < 3 || param_1 == 4) && -1 < param_2 && param_2 < 8 {
            let tbl = (&raw const PTR_DAT_100a4fa8).cast::<u32>();
            if param_1 == 0 {
                if -1 < (DAT_100a5030 as i32) {
                    let base = tbl.offset(DAT_100a5030 as i32 as isize).read_unaligned();
                    if param_2 == 0 {
                        local_8 =
                            base.wrapping_add(DAT_100a5034.wrapping_mul(0xc)) as usize as *mut i32;
                        DAT_100a5034 = (DAT_100a5034 == 0) as u32;
                    } else {
                        local_8 = base.wrapping_add((param_2 as u32).wrapping_mul(0xc)) as usize
                            as *mut i32;
                    }
                }
                if -1 < (DAT_100a5038 as i32) {
                    let base = tbl.offset(DAT_100a5038 as i32 as isize).read_unaligned();
                    if param_2 == 0 {
                        local_c =
                            base.wrapping_add(DAT_100a503c.wrapping_mul(0xc)) as usize as *mut i32;
                        DAT_100a503c = (DAT_100a503c == 0) as u32;
                    } else {
                        local_c = base.wrapping_add((param_2 as u32).wrapping_mul(0xc)) as usize
                            as *mut i32;
                    }
                }
            } else {
                let v = (&raw const PTR_DAT_100a5028)
                    .cast::<u32>()
                    .offset(param_1.wrapping_mul(2) as isize)
                    .read_unaligned();
                if -1 < (v as i32) {
                    let p = (&raw mut DAT_100a502c)
                        .cast::<u8>()
                        .byte_offset(param_1.wrapping_mul(8) as isize)
                        .cast::<u32>();
                    let base = tbl.offset(v as i32 as isize).read_unaligned();
                    if param_2 == 0 {
                        let cur = p.read_unaligned();
                        local_8 = base.wrapping_add(cur.wrapping_mul(0xc)) as usize as *mut i32;
                        if p.read_unaligned() == 0 {
                            p.write_unaligned(1);
                        } else {
                            p.write_unaligned(0);
                        }
                    } else {
                        local_8 = base.wrapping_add((param_2 as u32).wrapping_mul(0xc)) as usize
                            as *mut i32;
                    }
                }
                local_c = core::ptr::null_mut();
            }
            if !local_8.is_null() || !local_c.is_null() {
                FUN_10007440(local_8, local_c, -1);
            }
        }
    }
}
