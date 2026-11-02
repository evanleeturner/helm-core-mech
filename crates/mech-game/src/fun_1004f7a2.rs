// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10123120: i32;
    static mut DAT_100b38fc: i32;
    static mut DAT_10123230: i32;
    static mut DAT_10123130: [i32; 0];
    static mut DAT_100b07d8: i32;
    static mut DAT_100b0710: i32;
    static mut DAT_100b0a08: i32;
    static mut DAT_100a5534: i32;
    static mut DAT_100b0a1c: i32;
    static mut DAT_100b0a04: i32;
    static mut DAT_100b09f0: i32;
    static mut DAT_10123240: [u8; 0];
    static mut DAT_10123244: [u8; 0];
    static mut DAT_10123248: [u8; 0];
    static mut DAT_1012324c: [u8; 0];
    static mut DAT_10123278: [u8; 0];
    static mut DAT_1012327c: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1004e800(param_1: i32, param_2: i32);
    fn FUN_1004e457(param_1: i32, param_2: u32);
    fn FUN_1004e6d2(param_1: i32, param_2: i32) -> i32;
    fn FUN_1002b107(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1004f7a2() {
    // SAFETY: all globals and callees are the game's own memory and code;
    // the offsets index the game's per-slot tables exactly as the original.
    unsafe {
        DAT_10123120 = DAT_100b38fc / 0xb5;
        let mut local_8: i32 = 0;
        loop {
            if DAT_10123230 <= local_8 {
                return;
            }
            let i_var5: i32 = (local_8 as u32).wrapping_mul(0x3c0a) as i32;
            let off = i_var5 as isize;
            let p4: *mut u8 = (&raw mut DAT_10123240).cast::<u8>().offset(off);
            let b278: *mut u8 = (&raw mut DAT_10123278).cast::<u8>().offset(off);
            let b248: *mut u8 = (&raw mut DAT_10123248).cast::<u8>().offset(off);

            let mut local_c: i32 = 0;
            while local_c < p4.cast::<i32>().read_unaligned() {
                FUN_1004e800(local_8, local_c);
                local_c = local_c.wrapping_add(1);
            }
            if b278.read() == 0 {
                let mut b2: i32 = 1;
                let mut b3: i32 = 0;
                local_c = 0;
                while local_c < p4.cast::<i32>().read_unaligned() {
                    let o = (local_c as u32).wrapping_mul(0x13f) as usize;
                    let tmp = p4.byte_add(o).byte_add(0xaf).cast::<i32>().read_unaligned();
                    if tmp != 0 {
                        let c_var1 = p4.byte_add(o).byte_add(0x3a).read() as i8;
                        if c_var1 != 5 {
                            if c_var1 == 6 || c_var1 == 8 {
                                b2 = 0;
                                b3 = 1;
                            } else {
                                b2 = 0;
                            }
                        }
                    }
                    local_c = local_c.wrapping_add(1);
                }
                if b2 != 0 || DAT_100b07d8 != 0 {
                    b278.write(2);
                    FUN_1004e457(local_8, b278.read() as u32);
                    b248.cast::<i32>().write_unaligned(DAT_10123120);
                } else if b3 != 0 {
                    b278.write(3);
                    FUN_1004e457(local_8, b278.read() as u32);
                    b248.cast::<i32>().write_unaligned(DAT_10123120);
                } else {
                    let p4c = (&raw mut DAT_1012324c).cast::<u8>().offset(off);
                    let tmp = p4c.cast::<i32>().read_unaligned();
                    if 0 < tmp {
                        let tmp = p4c.cast::<i32>().read_unaligned();
                        let tmp2 = (&raw mut DAT_10123244)
                            .cast::<u8>()
                            .offset(off)
                            .cast::<i32>()
                            .read_unaligned();
                        if tmp <= (DAT_10123120 as u32).wrapping_sub(tmp2 as u32) as i32 {
                            b278.write(4);
                            FUN_1004e457(local_8, b278.read() as u32);
                            b248.cast::<i32>().write_unaligned(DAT_10123120);
                        }
                    }
                }
            } else if local_8 == DAT_100b0710
                && DAT_100b0a08 == 0
                && DAT_100a5534 == 0
                && DAT_100b0a1c == 0
            {
                DAT_100b0a1c = 1;
                DAT_100b0a04 = 0;
                DAT_100b09f0 = 0;
            }
            let mut local_10: i32 = -1;
            local_c = 0;
            while local_c < p4.cast::<i32>().read_unaligned() {
                let r = FUN_1004e6d2(local_8, local_c);
                if r != 0 {
                    let o = (local_8 as u32)
                        .wrapping_mul(0x3c0a)
                        .wrapping_add((local_c as u32).wrapping_mul(0x13f))
                        as usize;
                    let utmp = (&raw mut DAT_1012327c)
                        .cast::<u8>()
                        .byte_add(o)
                        .cast::<u32>()
                        .read_unaligned();
                    if utmp & 0xffff0000 == 0 {
                        local_10 = local_c;
                        break;
                    }
                }
                local_c = local_c.wrapping_add(1);
            }
            let slot = (&raw mut DAT_10123130)
                .cast::<i32>()
                .offset(local_8 as isize);
            if slot.read_unaligned() != local_10 {
                slot.write_unaligned(local_10);
                FUN_1002b107(local_8);
            }
            local_8 = local_8.wrapping_add(1);
        }
    }
}
