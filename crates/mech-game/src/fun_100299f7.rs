// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100eef40: [i32; 1];
    static mut DAT_100adf38: i32;
    static mut DAT_100b38fc: i32;
    static mut DAT_1016ac70: i32;
    static mut DAT_100ad620: u32;
    static mut DAT_100ad624: u8;
    static mut DAT_100ad628: u16;
    static mut DAT_100ad62a: u8;
    static mut DAT_100ad62c: u32;
    static mut DAT_100acbb8: *mut i8;
    static mut DAT_100acae0: [*mut i8; 1];
    static mut DAT_100ad630: [i8; 1];
    static mut DAT_10215678: [u8; 1];
    static mut DAT_10215694: [u8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10051f43(param_1: i32) -> i32;
    fn FUN_1004d47f(param_1: u32) -> i32;
    fn FUN_10082490(param_1: u32, param_2: u32, param_3: u32, param_4: u32) -> u32;
    fn FUN_1002a8fe(param_1: i32);
    fn FUN_1002a6ac(param_1: i32);
    fn FUN_100274a5(param_1: i16, param_2: *mut *mut i8, param_3: i32) -> *mut i8;
    fn _itoa(value: i32, buf: *mut i8, radix: i32) -> *mut i8;
    fn sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
    fn FUN_1005dd5a(param_1: *mut i8);
}

unsafe fn r32(a: i32) -> i32 {
    // SAFETY: the address is a game structure field supplied by the caller.
    unsafe { (a as u32 as usize as *const i32).read_unaligned() }
}

unsafe fn r16(a: i32) -> u16 {
    // SAFETY: the address is a game structure field supplied by the caller.
    unsafe { (a as u32 as usize as *const u16).read_unaligned() }
}

unsafe fn r8(a: i32) -> u8 {
    // SAFETY: the address is a game structure field supplied by the caller.
    unsafe { (a as u32 as usize as *const u8).read() }
}

unsafe fn w32(a: i32, v: u32) {
    // SAFETY: the address is a game structure field supplied by the caller.
    unsafe { (a as u32 as usize as *mut u32).write_unaligned(v) }
}

unsafe fn w16(a: i32, v: u16) {
    // SAFETY: the address is a game structure field supplied by the caller.
    unsafe { (a as u32 as usize as *mut u16).write_unaligned(v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100299f7(param_1: i32, param_2: i16) {
    let mut local_80: [u8; 8] = [0; 8];
    let mut local_74: [u8; 4] = [0; 4];
    let mut local_70: [i8; 20] = [0; 20];
    let mut local_5c: [i8; 80] = [0; 80];
    let mut local_c: [u8; 4] = [0; 4];
    let mut local_88: *mut u8;
    let mut local_84: *mut u8;

    // SAFETY: the globals and callees are the game's own; param_1 points at
    // the game's structure, and all reads and writes follow the original.
    unsafe {
        if r16(param_1.wrapping_add(0x14)) & 0x10 != 0 {
            w16(
                param_1.wrapping_add(0x14),
                r16(param_1.wrapping_add(0x14)) & 0xffef,
            );
            w32(r32(param_1.wrapping_add(0x20)).wrapping_add(0x3d4), 0);
        }
        if param_2 != 3 {
            let idx = DAT_100adf38 as isize;
            let e = (&raw mut DAT_100eef40).cast::<i32>().offset(idx).read();
            if r32(e.wrapping_add(8)) == r32(param_1.wrapping_add(8)) {
                let t = (r32(param_1.wrapping_add(0x17e)) as u32) & 3;
                w32(param_1.wrapping_add(0x17e), t);
            }
        }
        match param_2 {
            1 | 6 | 9 => {}
            2 => {
                let iv = FUN_10051f43(10);
                let t = (iv as u32)
                    .wrapping_mul(0x16)
                    .wrapping_add(DAT_100b38fc as u32);
                w32(param_1.wrapping_add(0x18e), t);
                let iv3 = FUN_1004d47f((r32(param_1.wrapping_add(8)) != DAT_100adf38) as u32);
                if iv3 != 0
                    && (r16(param_1.wrapping_add(0x176)) as i16) == 7
                    && (r8(param_1.wrapping_add(0x1a3)) & 0x10) != 0
                {
                    let local_8 = FUN_10082490(
                        r32(param_1.wrapping_add(4)) as u32,
                        r32(param_1.wrapping_add(0x50)) as u32,
                        r32(param_1.wrapping_add(0x54)) as u32,
                        r32(param_1.wrapping_add(0x58)) as u32,
                    );
                    if local_8 != 0xffffffff {
                        let off = local_8.wrapping_mul(0x54) as usize;
                        let p = (&raw mut DAT_10215694).cast::<u8>().add(off).cast::<u16>();
                        p.write_unaligned(p.read_unaligned() | 1);
                        let q = (&raw mut DAT_10215678).cast::<u8>().add(off).cast::<u32>();
                        q.write_unaligned((r32(param_1.wrapping_add(4)) as u32) | 0x200);
                        w32(param_1.wrapping_add(0x1a2), local_8 | 0x100);
                    }
                }
            }
            3 => {}
            4 => {
                w32(param_1.wrapping_add(0x17e), 1);
            }
            5 => {
                FUN_1002a8fe(param_1);
            }
            7 => {
                FUN_1002a6ac(param_1);
            }
            8 => {}
            0xb | 10 | 0 => {
                if param_2 == 0xb {
                    w32(param_1.wrapping_add(0x17e), 1);
                }
                if param_2 == 0xb || param_2 == 10 {
                    w16(
                        param_1.wrapping_add(0x14),
                        r16(param_1.wrapping_add(0x14)) | 0x10,
                    );
                    let a = r32(param_1.wrapping_add(0x20)).wrapping_add(0x3d4);
                    w32(a, (r32(a) as u32) | 3);
                }
                w32(r32(param_1.wrapping_add(0x4c)).wrapping_add(8), 0);
                w32(param_1.wrapping_add(0xc0), 0);
            }
            _ => {}
        }
        DAT_1016ac70 = DAT_100b38fc;
        w16(param_1.wrapping_add(0x176), param_2 as u16);

        local_80[..4].copy_from_slice(&DAT_100ad620.to_le_bytes());
        local_80[4] = DAT_100ad624;
        local_74[..2].copy_from_slice(&DAT_100ad628.to_le_bytes());
        local_74[2] = DAT_100ad62a;
        local_c.copy_from_slice(&DAT_100ad62c.to_le_bytes());

        let local_78 = r32(param_1.wrapping_add(0x20));

        if r16(r32(local_78).wrapping_add(0x178)) & 0x200 == 0 {
            local_88 = local_74.as_mut_ptr();
        } else {
            local_88 = local_80.as_mut_ptr();
        }
        if r16(r32(local_78).wrapping_add(0x178)) & 0x100 != 0 {
            local_88 = local_c.as_mut_ptr();
        }
        if r16(r32(local_78).wrapping_add(0x17a)) & 0x200 == 0 {
            local_84 = local_74.as_mut_ptr();
        } else {
            local_84 = local_80.as_mut_ptr();
        }
        if r16(r32(local_78).wrapping_add(0x17a)) & 0x100 != 0 {
            local_84 = local_c.as_mut_ptr();
        }
        if r16(r32(local_78).wrapping_add(0x17a)) & 0x6000 == 0 {
            _itoa(
                (r16(r32(local_78).wrapping_add(0x17a)) & 0xff) as i32,
                local_70.as_mut_ptr(),
                10,
            );
        } else {
            let src = FUN_100274a5(
                r16(r32(local_78).wrapping_add(0x17a)) as i16,
                &raw mut DAT_100acbb8,
                7,
            );
            let mut n: usize = 0;
            while *src.add(n) != 0 {
                n += 1;
            }
            n += 1;
            let mut i: usize = 0;
            while i < n {
                *local_70.as_mut_ptr().add(i) = *src.add(i);
                i += 1;
            }
            let mut l: usize = 0;
            while *local_84.add(l) != 0 {
                l += 1;
            }
            local_84 = local_84.add(l);
        }
        let base = r32(local_78);
        let name = (&raw mut DAT_100acae0)
            .cast::<*mut i8>()
            .offset((r16(base.wrapping_add(0x176)) as i16 as isize) * 2)
            .read();
        sprintf(
            local_5c.as_mut_ptr(),
            (&raw const DAT_100ad630).cast::<i8>(),
            DAT_100b38fc,
            r32(base.wrapping_add(8)),
            r32(base.wrapping_add(4)),
            name,
            local_88,
            (r16(base.wrapping_add(0x178)) & 0xff) as i32,
            local_84,
            local_70.as_mut_ptr(),
        );
        FUN_1005dd5a(local_5c.as_mut_ptr());
    }
}
