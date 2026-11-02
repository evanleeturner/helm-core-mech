// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3904: u32;
    static mut DAT_100eef40: [u32; 0];
    static mut DAT_100ef030: u8;
    static mut DAT_100ef034: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000dde1(param_1: i32, param_2: *mut u32, param_3: *mut u32, param_4: *mut u32) -> u8;
    fn FUN_1000fc30(param_1: i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_10002570(param_1: u32, param_2: i32) -> u32;
    fn FUN_1000fef4(param_1: *mut u32, param_2: *mut u32, param_3: *mut u32) -> u8;
    fn FUN_10003d60(param_1: i32, param_2: i32) -> i32;
    fn FUN_10006bbb(param_1: u32) -> i32;
    fn FUN_1002f668(param_1: i32, param_2: i32, param_3: i32, param_4: i32, param_5: u32) -> u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10006d3e() -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004a6cc(param_1: i32, param_2: i32, param_3: i32, param_4: i32) {
    let p = param_1 as *mut u8;
    let mut local_34: i32;
    let mut local_8: i32;
    let mut local_c: i32;
    // SAFETY: param_1 points at the game's object; the globals, tables and
    // callees are the game's own memory and code, accessed as the original does.
    unsafe {
        if p.byte_add(0x34).cast::<i32>().read_unaligned() < 0 {
            return;
        }
        if p.byte_add(0x38).cast::<i32>().read_unaligned() == 0x200 {
            let idx = p.byte_add(0x34).cast::<i32>().read_unaligned();
            let local_10 = (&raw const DAT_100eef40)
                .cast::<u32>()
                .offset(idx as isize)
                .read_unaligned() as i32;
            let q = local_10 as *mut u8;
            let f = q.byte_add(0x14).cast::<u16>().read_unaligned();
            if (f & 2) != 0 || (f & 4) != 0 {
                p.byte_add(0x20).cast::<i32>().write_unaligned(0);
                p.byte_add(0x1c).cast::<i32>().write_unaligned(0);
                p.byte_add(0x18).cast::<i32>().write_unaligned(0);
                p.byte_add(0x38).cast::<i32>().write_unaligned(0);
                return;
            }
            local_34 = q.byte_add(0x50).cast::<i32>().read_unaligned();
            local_8 = q.byte_add(0x54).cast::<i32>().read_unaligned();
            local_c = q.byte_add(0x58).cast::<i32>().read_unaligned();
        } else {
            if p.byte_add(0x38).cast::<i32>().read_unaligned() != 0x400 {
                return;
            }
            let off = p
                .byte_add(0x34)
                .cast::<i32>()
                .read_unaligned()
                .wrapping_mul(0x40) as isize;
            let l20 = (&raw mut DAT_100ef030).offset(off).cast::<u16>();
            if (l20.read_unaligned() & 4) != 0 {
                p.byte_add(0x20).cast::<i32>().write_unaligned(0);
                p.byte_add(0x1c).cast::<i32>().write_unaligned(0);
                p.byte_add(0x18).cast::<i32>().write_unaligned(0);
                p.byte_add(0x38).cast::<i32>().write_unaligned(0);
                return;
            }
            let arg = (&raw mut DAT_100ef034)
                .offset(off)
                .cast::<i32>()
                .read_unaligned();
            local_34 = 0;
            local_8 = 0;
            local_c = 0;
            FUN_1000dde1(
                arg,
                &raw mut local_34 as *mut u32,
                &raw mut local_8 as *mut u32,
                &raw mut local_c as *mut u32,
            );
        }
        let mut local_18: u32 = local_34.wrapping_sub(param_2) as u32;
        let mut local_14: u32 = local_8.wrapping_sub(param_3) as u32;
        let mut local_1c: u32 = local_c.wrapping_sub(param_4) as u32;
        let local_24 = FUN_1000fc30(local_18 as i32, local_14 as i32, local_1c as i32);
        if local_24 < 0x65 {
            let v = p.byte_add(0x3c).cast::<u32>().read_unaligned();
            p.byte_add(0x3c).cast::<u32>().write_unaligned(v | 0x8000);
        }
        local_18 = FUN_10002570(local_18, local_24);
        local_14 = FUN_10002570(local_14, local_24);
        local_1c = FUN_10002570(local_1c, local_24);
        let mut local_28: u32 = p.byte_add(0xc).cast::<u32>().read_unaligned();
        let mut local_2c: u32 = p.byte_add(0x10).cast::<u32>().read_unaligned();
        let mut local_30: u32 = p.byte_add(0x14).cast::<u32>().read_unaligned();
        FUN_1000fef4(&raw mut local_28, &raw mut local_2c, &raw mut local_30);
        if (p.byte_add(4).read() & 0x10) == 0 {
            p.byte_add(0x18)
                .cast::<u32>()
                .write_unaligned(local_18.wrapping_sub(local_28));
            p.byte_add(0x1c)
                .cast::<u32>()
                .write_unaligned(local_14.wrapping_sub(local_2c));
            p.byte_add(0x20)
                .cast::<u32>()
                .write_unaligned(local_1c.wrapping_sub(local_30));
        } else {
            let s = FUN_10002570(DAT_100b3904, 0x2d4) as i32;
            let c18 = p.byte_add(0x18).cast::<i32>().read_unaligned();
            let a = FUN_10003d60(
                local_18.wrapping_sub(local_28).wrapping_sub(c18 as u32) as i32,
                s,
            );
            let c1c = p.byte_add(0x1c).cast::<i32>().read_unaligned();
            let b = FUN_10003d60(
                local_14.wrapping_sub(local_2c).wrapping_sub(c1c as u32) as i32,
                s,
            );
            let c20 = p.byte_add(0x20).cast::<i32>().read_unaligned();
            let c = FUN_10003d60(
                local_1c.wrapping_sub(local_30).wrapping_sub(c20 as u32) as i32,
                s,
            );
            let v18 = p.byte_add(0x18).cast::<i32>().read_unaligned();
            p.byte_add(0x18)
                .cast::<i32>()
                .write_unaligned(v18.wrapping_add(a));
            let v1c = p.byte_add(0x1c).cast::<i32>().read_unaligned();
            p.byte_add(0x1c)
                .cast::<i32>()
                .write_unaligned(v1c.wrapping_add(b));
            let v20 = p.byte_add(0x20).cast::<i32>().read_unaligned();
            p.byte_add(0x20)
                .cast::<i32>()
                .write_unaligned(v20.wrapping_add(c));
        }
        local_8 = FUN_10006d3e();
        local_34 = FUN_10006bbb(local_2c.wrapping_shl(0xd));
        local_34 = local_34.wrapping_neg();
        FUN_1002f668(
            p.byte_add(0x30).cast::<i32>().read_unaligned(),
            local_34,
            local_8,
            0,
            0,
        );
    }
}
