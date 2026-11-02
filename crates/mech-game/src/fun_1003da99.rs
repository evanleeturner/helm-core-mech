// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aefc0: u32;
    static mut DAT_100aefc4: u32;
    static mut DAT_100af098: *mut u8;
    static mut DAT_100c6d00: [u8; 1];
    static mut DAT_100c6d10: *mut u16;
    static mut DAT_100c6d30: [u8; 1];
    static mut DAT_100c6d48: [u8; 1];
    static mut DAT_100c6d58: [u8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1003d742(param_1: *mut u16) -> *mut u8;
    fn FUN_1003e3e3(param_1: i32, param_2: i32) -> u8;
    fn FUN_1002f322(param_1: i32, param_2: *mut u32, param_3: *mut u32, param_4: *mut u32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003da99(param_1: *mut u16) {
    // SAFETY: globals and callees are the game's own; param_1 points at the
    // caller's record, accessed at the same offsets as the original.
    unsafe {
        if DAT_100af098 != (&raw mut DAT_100c6d00).cast::<u8>() || param_1.is_null() {
            return;
        }
        param_1.add(0x22).write_unaligned(9999);
        param_1.add(0x23).write_unaligned(0);
        param_1.add(0x24).write_unaligned(9999);
        param_1.add(0x25).write_unaligned(0);
        let local_8: *mut u8;
        if DAT_100aefc0 == 0 {
            if (param_1.read_unaligned() & 0x1000) == 0 {
                local_8 = (&raw mut DAT_100c6d00).cast::<u8>();
            } else {
                local_8 = (&raw mut DAT_100c6d30).cast::<u8>();
            }
        } else {
            local_8 = FUN_1003d742(param_1);
        }
        FUN_1003e3e3(param_1 as i32, local_8 as i32);
        if param_1.add(0x12).cast::<i32>().read_unaligned() == 4 {
            param_1.write_unaligned(param_1.read_unaligned() | 0x800);
        }
        if (param_1.read_unaligned() & 0x800) == 0 {
            if DAT_100aefc0 == 0 {
                if !DAT_100c6d10.is_null() {
                    DAT_100c6d10
                        .add(6)
                        .cast::<*mut u16>()
                        .write_unaligned(param_1);
                }
                param_1
                    .add(8)
                    .cast::<*mut u16>()
                    .write_unaligned(DAT_100c6d10);
                DAT_100c6d10 = param_1;
                param_1
                    .add(6)
                    .cast::<*mut u8>()
                    .write_unaligned((&raw mut DAT_100c6d00).cast::<u8>());
            } else {
                let mut local_18: i32 = 0;
                let mut local_1c: u32 = 0;
                let mut local_14: [i32; 3] = [0; 3];
                FUN_1002f322(
                    param_1.add(0xc).cast::<i32>().read_unaligned(),
                    (&raw mut local_18).cast::<u32>(),
                    &raw mut local_1c,
                    local_14.as_mut_ptr().cast::<u32>(),
                );
                let i_var1 = local_18.wrapping_add(150000) / 25000;
                let i_var2 = local_14[0].wrapping_add(150000) / 25000;
                if i_var1 < 0 || 0xb < i_var1 || i_var2 < 0 || 0xb < i_var2 {
                    let base = DAT_100aefc4;
                    let head = (base.wrapping_add(0x10) as *const u32).read_unaligned();
                    if head != 0 {
                        (head.wrapping_add(0xc) as *mut u32).write_unaligned(param_1 as u32);
                    }
                    param_1.add(8).cast::<u32>().write_unaligned(
                        (DAT_100aefc4.wrapping_add(0x10) as *const u32).read_unaligned(),
                    );
                    (DAT_100aefc4.wrapping_add(0x10) as *mut u32).write_unaligned(param_1 as u32);
                    param_1.add(6).cast::<u32>().write_unaligned(DAT_100aefc4);
                } else {
                    let off = i_var2
                        .wrapping_mul(0x18)
                        .wrapping_add(i_var1.wrapping_mul(0x120));
                    let slot = (&raw mut DAT_100c6d58)
                        .cast::<u8>()
                        .wrapping_offset(off as isize)
                        .cast::<u32>();
                    let head = slot.read_unaligned();
                    if head != 0 {
                        (head.wrapping_add(0xc) as *mut u32).write_unaligned(param_1 as u32);
                    }
                    param_1
                        .add(8)
                        .cast::<u32>()
                        .write_unaligned(slot.read_unaligned());
                    slot.write_unaligned(param_1 as u32);
                    let owner = (&raw mut DAT_100c6d48)
                        .cast::<u8>()
                        .wrapping_offset(i_var1.wrapping_mul(0x120) as isize)
                        .wrapping_offset(i_var2.wrapping_mul(0x18) as isize);
                    param_1.add(6).cast::<*mut u8>().write_unaligned(owner);
                }
            }
        }
    }
}
