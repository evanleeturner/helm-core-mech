// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b0864: u32;
    static mut DAT_100b0868: u32;
    static mut DAT_101155c0: u8;
    static mut DAT_101155c4: u8;
    static mut DAT_101155dc: u8;
    static mut DAT_101155e0: u8;
    static mut DAT_101155e4: u8;
    static mut DAT_101155e8: u8;
    static mut DAT_101155ec: u8;
    static mut DAT_101155f0: u8;
    static mut DAT_101155f4: u8;
    static mut DAT_101155f8: u8;
    static mut DAT_10115606: u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10050227() -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005008d(
    param_1: u32,
    param_2: u32,
    param_3: u32,
    param_4: u32,
    param_5: i32,
    param_6: i32,
    param_7: i32,
    param_8: u16,
    param_9: u32,
) -> i32 {
    let mut param_5 = param_5;
    let mut param_6 = param_6;
    let local_8: i32;
    let local_c: *mut u32;
    // SAFETY: the globals and the callee are the game's own memory; the table
    // at 0x101155c0 holds 0x30c records of 0x48 bytes, written where the
    // original writes.
    unsafe {
        if DAT_100b0868 == 0 {
            FUN_10050227();
        }
        if DAT_100b0864 < 0x30c {
            if param_5 == -1 {
                param_5 = -2;
            }
            if param_6 == 0 {
                let i_var1 = (DAT_100b0864 as i32).wrapping_mul(0x48);
                let o = i_var1 as isize;
                local_c = (&raw mut DAT_101155c0).byte_offset(o).cast::<u32>();
                local_c.write_unaligned(0xfffffffe);
                (&raw mut DAT_101155c4)
                    .byte_offset(o)
                    .cast::<u32>()
                    .write_unaligned(0xffffffff);
                (&raw mut DAT_101155dc)
                    .byte_offset(o)
                    .cast::<i32>()
                    .write_unaligned(param_5);
                (&raw mut DAT_101155f0)
                    .byte_offset(o)
                    .cast::<u32>()
                    .write_unaligned(param_2);
                (&raw mut DAT_101155f4)
                    .byte_offset(o)
                    .cast::<u32>()
                    .write_unaligned(param_3);
                (&raw mut DAT_101155f8)
                    .byte_offset(o)
                    .cast::<u32>()
                    .write_unaligned(param_4);
                ((0x101155c8usize as *mut u8).byte_offset(o))
                    .cast::<u32>()
                    .write_unaligned(param_1);
                (&raw mut DAT_101155e4)
                    .byte_offset(o)
                    .cast::<u32>()
                    .write_unaligned(0);
                (&raw mut DAT_101155e8)
                    .byte_offset(o)
                    .cast::<u32>()
                    .write_unaligned(0);
                (&raw mut DAT_101155e0)
                    .byte_offset(o)
                    .cast::<u32>()
                    .write_unaligned(0);
                (&raw mut DAT_10115606)
                    .byte_offset(o)
                    .cast::<u16>()
                    .write_unaligned(0);
                (&raw mut DAT_101155ec)
                    .byte_offset(o)
                    .cast::<u32>()
                    .write_unaligned(param_9);
                local_8 = DAT_100b0864 as i32;
                DAT_100b0864 = DAT_100b0864.wrapping_add(1);
            } else {
                local_8 = param_7;
                if param_7 < 0 {
                    return -1;
                }
                local_c = (&raw mut DAT_101155c0)
                    .byte_offset(param_7.wrapping_mul(0x48) as isize)
                    .cast::<u32>();
            }
            local_c
                .offset(param_6.wrapping_add(2) as isize)
                .write_unaligned(param_1);
            local_c
                .byte_offset((param_6.wrapping_mul(2).wrapping_add(0x3c)) as isize)
                .cast::<u16>()
                .write_unaligned(param_8);
            if param_6 < 5 {
                loop {
                    let i_var1 = param_6;
                    let local_10 = i_var1.wrapping_add(1);
                    if !(local_10 < 5) {
                        break;
                    }
                    param_6 = local_10;
                    let p = local_c.offset(i_var1.wrapping_add(3) as isize);
                    if p.read_unaligned() == 0xffffffff {
                        p.write_unaligned(param_1);
                        local_c
                            .byte_offset((local_10.wrapping_mul(2).wrapping_add(0x3c)) as isize)
                            .cast::<u16>()
                            .write_unaligned(param_8);
                        param_6 = local_10;
                    }
                }
            }
        } else {
            local_8 = -1;
        }
    }
    local_8
}
