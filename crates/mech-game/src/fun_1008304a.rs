// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100eef40: [i32; 1];
    static mut DAT_100ef030: [u8; 1];
    static mut DAT_100b38dc: i32;
    static mut DAT_100b9f68: i32;
    static mut DAT_100adf38: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1004d60d(param_1: i32) -> i32;
    fn FUN_1000dd4e(param_1: i32) -> i32;
    fn FUN_1000dd98(param_1: i32) -> i32;
    fn FUN_1000dde1(param_1: i32, param_2: *mut i32, param_3: *mut i32, param_4: *mut i32);
    fn FUN_10083a59(
        param_1: u32,
        param_2: i32,
        param_3: u32,
        param_4: *mut i32,
        param_5: *mut i32,
        param_6: *mut u32,
        param_7: *mut i32,
    );
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1008304a(param_1: i32, param_2: i32, param_3: u32) -> u32 {
    let mut local_30: i32 = 0;
    let mut local_28: i32 = 0;
    let mut local_24: i32 = 0;
    let mut local_20: u32 = 0;
    let mut local_14: i32 = 0;
    let mut local_10: i32 = 0;
    let mut local_8: i32 = 0;

    // SAFETY: globals and callees are the game's own; the pointers derived
    // from them address the game's tables and structures, accessed as the
    // original does.
    unsafe {
        if param_2 < 0 {
            return 0xffff_ffff;
        }
        if param_2 >= DAT_100b38dc {
            return 0xffff_fffe;
        }
        if (param_3 & 0x23) != 0 {
            return 0xffff_fff7;
        }
        let local_1c: *mut u8 = (&raw mut DAT_100eef40)
            .cast::<i32>()
            .offset(param_1 as isize)
            .read_unaligned() as u32 as usize as *mut u8;
        let local_18: *mut u8 = (&raw mut DAT_100ef030)
            .cast::<u8>()
            .add((param_2 as u32).wrapping_mul(0x40) as usize);
        let flags = local_18.cast::<u16>().read_unaligned();
        if (flags & 4) != 0 {
            return 0xffff_fffd;
        }
        if (param_3 & 0x20000) != 0 && FUN_1004d60d(param_2) != 0 {
            return 0xffff_fff7;
        }
        if (param_3 & 0x40000) != 0 && FUN_1004d60d(param_2) != 1 {
            return 0xffff_fff7;
        }
        if DAT_100b9f68 == 0 && (flags & 0x1400) == 0 && param_1 == DAT_100adf38 {
            return 0xffff_fffb;
        }
        if (flags & 0x800) != 0 && param_1 == DAT_100adf38 {
            return 0xffff_fffb;
        }
        let val = local_18.byte_add(4).cast::<i32>().read_unaligned();
        if FUN_1000dd4e(val) == 0 {
            return 0xffff_fff8;
        }
        if FUN_1000dd98(val) == 0 {
            return 0xffff_fff8;
        }
        FUN_1000dde1(val, &raw mut local_30, &raw mut local_8, &raw mut local_10);
        let local_2c =
            (local_30 as u32).wrapping_sub(local_1c.byte_add(0x50).cast::<u32>().read_unaligned());
        let local_c =
            (local_10 as u32).wrapping_sub(local_1c.byte_add(0x58).cast::<u32>().read_unaligned());
        FUN_10083a59(
            local_2c,
            (local_8 as u32).wrapping_sub(local_1c.byte_add(0x54).cast::<u32>().read_unaligned())
                as i32,
            local_c,
            &raw mut local_24,
            &raw mut local_28,
            &raw mut local_20,
            &raw mut local_14,
        );
        if (flags & 0x1000) == 0 && local_28 > 0x2ab98 && param_1 == DAT_100adf38 {
            return 0xffff_fff9;
        }
        local_1c
            .byte_add(200)
            .cast::<i32>()
            .write_unaligned(local_30);
        local_1c
            .byte_add(0xcc)
            .cast::<i32>()
            .write_unaligned(local_8);
        local_1c
            .byte_add(0xd0)
            .cast::<i32>()
            .write_unaligned(local_10);
        local_1c
            .byte_add(0xd4)
            .cast::<i32>()
            .write_unaligned(local_24);
        local_1c
            .byte_add(0xc4)
            .cast::<i32>()
            .write_unaligned(local_28);
        local_1c
            .byte_add(0xc0)
            .cast::<u32>()
            .write_unaligned(local_20);
        local_1c
            .byte_add(0xd8)
            .cast::<i32>()
            .write_unaligned(local_14);
        1
    }
}
