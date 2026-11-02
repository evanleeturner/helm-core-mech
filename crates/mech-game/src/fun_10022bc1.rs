// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38fc: i32;
    static mut DAT_100adf38: i32;
    static mut DAT_100eef40: [i32; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10023525(param_1: i32, param_2: *mut i32, param_3: *mut i32, param_4: *mut u32);
    fn FUN_10051f43(param_1: i32) -> i32;
    fn FUN_10023258(param_1: i32, param_2: *mut i32) -> i32;
    fn FUN_1002306a(param_1: i32, param_2: *mut i32) -> i32;
    fn FUN_1004c350(param_1: i32, param_2: i32) -> u8;
    fn FUN_100234a9(param_1: i32);
    fn FUN_1002974a(param_1: i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_1002977f(param_1: i32, param_2: i32, param_3: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10022bc1(param_1: i32, param_2: i32) -> i32 {
    let mut param_2 = param_2;
    let p = param_1 as usize as *mut u8;
    let mut local_24: i32 = 0;
    let mut local_18: i32 = 0;
    let mut local_1c: i32 = 0;
    let mut local_14: [u32; 3] = [0; 3];
    // SAFETY: param_1 points at the game's object; all offsets are read and
    // written where the original does, and callees and globals are the game's.
    unsafe {
        let mut local_20: i32 = p.byte_add(0xd8).cast::<i32>().read_unaligned();
        FUN_10023525(param_1, &mut param_2, &mut local_20, local_14.as_mut_ptr());
        let mut i_var2: i32 = (param_2 as u32)
            .wrapping_sub(p.byte_add(0x6c).cast::<i32>().read_unaligned() as u32)
            as i32;
        let inner = p.byte_add(0x20).cast::<i32>().read_unaligned() as usize as *const u8;
        let sum = (inner.byte_add(0x1c).cast::<i32>().read_unaligned() as u32)
            .wrapping_add(p.byte_add(0x5c).cast::<i32>().read_unaligned() as u32)
            .wrapping_add(local_20 as u32);
        let local_8: i32 = (sum % 0x1680000) as i32;
        let mut tmp18e: i32 = p.byte_add(0x18e).cast::<i32>().read_unaligned();
        'skip: {
            if tmp18e <= DAT_100b38fc {
                local_24 = FUN_10051f43(p.byte_add(0x186).cast::<i8>().read() as i32);
                tmp18e = (local_24 as u32)
                    .wrapping_mul(0x16)
                    .wrapping_add(DAT_100b38fc as u32) as i32;
                p.byte_add(0x18e).cast::<i32>().write_unaligned(tmp18e);
                if i_var2 < 0x10000 && -0x10000 < i_var2 && local_8 < 0xa0000 && -0xa0000 < local_8
                {
                    let us = p.byte_add(0x17a).cast::<u16>().read_unaligned();
                    if (us & 0xff) as i32 != DAT_100adf38 {
                        i_var2 = FUN_10051f43(3);
                        if i_var2 != 0 {
                            break 'skip;
                        }
                    }
                    if local_24 == 0 {
                        let tmp192 = p.byte_add(0x192).cast::<i32>().read_unaligned();
                        let local_2c = if tmp192 == 0 {
                            FUN_1002306a(param_1, &mut local_18)
                        } else {
                            FUN_10023258(param_1, &mut local_18)
                        };
                        if local_2c != 0 {
                            let b_var1 = FUN_1004c350(param_1, 1);
                            if b_var1 as i32 != 0 {
                                let tmp192 = p.byte_add(0x192).cast::<i32>().read_unaligned();
                                let q = p.byte_add(0x4c).cast::<i32>().read_unaligned() as usize
                                    as *mut u8;
                                if tmp192 == 0 {
                                    q.byte_add(0x25).write(1);
                                } else {
                                    q.byte_add(0x27).write(1);
                                }
                                local_1c = 1;
                            }
                        }
                    }
                }
            }
        }
        if local_18 != 0 && local_1c != 0 {
            FUN_100234a9(param_1);
        }
        let us17a = p.byte_add(0x17a).cast::<u16>().read_unaligned();
        if (((us17a >> 8) as u8) & 0xf) == 2 {
            let idx = (p.byte_add(0x17a).cast::<u16>().read_unaligned() & 0xff) as usize;
            let e = (&raw mut DAT_100eef40)
                .cast::<i32>()
                .add(idx)
                .read_unaligned();
            let e4c = (e as usize as *const u8)
                .byte_add(0x4c)
                .cast::<i32>()
                .read_unaligned();
            let v = (e4c as usize as *const u8)
                .byte_add(8)
                .cast::<i32>()
                .read_unaligned();
            if v != 0 {
                local_24 = 0;
            }
        }
        if local_1c == 0 {
            local_24 = 0;
        }
        i_var2 = FUN_1002974a(param_1, param_2, local_24);
        let q = p.byte_add(0x4c).cast::<i32>().read_unaligned() as usize as *mut u8;
        q.byte_add(4)
            .cast::<i32>()
            .write_unaligned((i_var2 as u32).wrapping_mul(0x2d00) as i32);
        i_var2 = FUN_1002977f(param_1, local_20, local_24);
        let q = p.byte_add(0x4c).cast::<i32>().read_unaligned() as usize as *mut i32;
        q.write_unaligned((i_var2 as u32).wrapping_mul(0xfffff100) as i32);
    }
    local_1c
}
