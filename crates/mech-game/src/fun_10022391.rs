// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ab094: u32;
    static mut DAT_100ab2fc: u32;
    static mut DAT_100ab300: u32;
    static mut DAT_100ab304: u32;
    static mut DAT_100ab30c: u32;
    static mut DAT_100ab318: u32;
    static mut DAT_100ab31c: u32;
    static mut DAT_100ab330: u32;
    static mut DAT_100ab334: u32;
    static mut DAT_100b38fc: u32;
    static mut DAT_100aaf68: u32;
    static mut DAT_100ab34c: [i8; 1];
    static mut DAT_100ab364: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1001a5e1(param_1: i32);
    fn FUN_10003742(param_1: u32, param_2: i32, param_3: u32, param_4: i32, param_5: u16);
    fn FUN_10062e97(param_1: *mut i32);
    fn FUN_100632e4(param_1: i32) -> u32;
    fn FUN_100082c4(param_1: i32, param_2: i32);
    fn sprintf(dest: *mut i8, format: *const i8, ...) -> i32;
    fn FUN_1005be24(param_1: *mut i8, param_2: i32, param_3: i32, param_4: i32) -> u32;
    fn FUN_100037f0(param_1: u32, param_2: u32, param_3: u32, param_4: u32, param_5: i32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1006b1d9();
    fn FUN_100635a2();
}

unsafe fn rd32(addr: i32) -> i32 {
    // SAFETY: caller supplies a valid game address.
    unsafe { (addr as *const i32).read_unaligned() }
}

unsafe fn rd8(addr: i32) -> u8 {
    // SAFETY: caller supplies a valid game address.
    unsafe { (addr as *const u8).read() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10022391(param_1: *mut i32) {
    if param_1.is_null() {
        return;
    }
    // SAFETY: param_1 is the game's object; all globals, callees and
    // derived addresses are the game's own memory, accessed as the original does.
    unsafe {
        let local_c = param_1;
        let mut local_3c = [0i8; 40];
        let i_var1 = local_c.add(0xf5).read_unaligned();
        if i_var1 == 2 {
            if DAT_100ab30c != 0 {
                if local_c.add(0xfa).read_unaligned() == 0 {
                    FUN_10003742(0x112, 0x32, 0x40, 5, 0x32);
                    FUN_10062e97(local_c);
                }
                DAT_100ab30c = 0;
            }
            if rd8(rd32(local_c.read_unaligned().wrapping_add(0x4c)).wrapping_add(0x26)) != 0
                && local_c.add(0xfa).read_unaligned() == 0
            {
                FUN_10003742(0x112, 0x32, 0x40, 5, 0x32);
                DAT_100ab30c = 1;
                FUN_10062e97(local_c);
                DAT_100ab30c = 0;
            }
            if rd8(rd32(local_c.read_unaligned().wrapping_add(0x4c)).wrapping_add(0x2b)) != 0
                && rd8(rd32(local_c.read_unaligned().wrapping_add(0x4c)).wrapping_add(0x27)) == 0
            {
                FUN_100635a2();
            }
            if DAT_100ab2fc != 0 {
                DAT_100ab2fc = 0;
                FUN_100632e4(local_c as i32);
            }
            if DAT_100ab300 != 0 {
                DAT_100ab300 = 0;
                if (local_c.add(0x110).cast::<u16>().read_unaligned() & 0x10) == 0 {
                    sprintf(local_3c.as_mut_ptr(), (&raw mut DAT_100ab34c).cast::<i8>());
                    FUN_1005be24(local_3c.as_mut_ptr(), 1, 0x16a, 0x32);
                } else if DAT_100ab304 == 0 {
                    DAT_100ab304 = 1;
                    FUN_10003742(0xdc, 100, 0x40, 5, 0x50);
                    FUN_100082c4(0x1f, 1);
                } else {
                    DAT_100ab304 = 0;
                    FUN_10003742(0xdb, 100, 0x40, 5, 0x50);
                    FUN_100082c4(0x1f, 0);
                }
            }
        } else {
            if i_var1 == 4 {
                if DAT_100ab330 != 0 {
                    return;
                }
                DAT_100ab330 = 1;
                FUN_1006b1d9();
                DAT_100ab318 = 1;
                FUN_1001a5e1(1);
                return;
            }
            if i_var1 == 5 {
                if DAT_100ab334 == 0 {
                    local_c
                        .add(0xef)
                        .write_unaligned(DAT_100b38fc.wrapping_add(0x389) as i32);
                    DAT_100ab334 = DAT_100ab334.wrapping_add(1);
                    return;
                }
                if (DAT_100b38fc as i32) < local_c.add(0xef).read_unaligned() {
                    FUN_1001a5e1(4);
                    return;
                }
                DAT_100ab318 = 1;
                return;
            }
        }
        if DAT_100ab31c == 0xffff_ffff {
            if local_c.add(0xf5).read_unaligned() != 3 {
                FUN_100082c4(0xd, -1);
                let base = local_c.read_unaligned();
                let p = DAT_100aaf68 as i32;
                let local_8 = rd32(base.wrapping_add(0x50)).wrapping_sub(rd32(p)) as u32;
                let local_10 =
                    rd32(base.wrapping_add(0x54)).wrapping_sub(rd32(p.wrapping_add(4))) as u32;
                let local_14 =
                    rd32(base.wrapping_add(0x58)).wrapping_sub(rd32(p.wrapping_add(8))) as u32;
                FUN_100037f0(local_8, local_10, local_14, 0x107, DAT_100ab094 as i32);
                local_c.add(0xf5).write_unaligned(3);
                DAT_100ab31c = 0;
            }
        } else if DAT_100ab31c != 0
            && DAT_100ab31c == 1
            && ((local_c.add(0x110).cast::<u16>().read_unaligned() & 4) == 0
                || (local_c.add(0x110).cast::<u16>().read_unaligned() & 8) != 0)
            && local_c.add(0xf5).read_unaligned() == 3
        {
            sprintf(local_3c.as_mut_ptr(), (&raw mut DAT_100ab364).cast::<i8>());
            FUN_1005be24(local_3c.as_mut_ptr(), 1, 0x16a, 0x32);
            local_c.add(0xf5).write_unaligned(0);
            let q = (local_c.read_unaligned().wrapping_add(0x14)) as *mut u16;
            q.write_unaligned(q.read_unaligned() & 0xffef);
            DAT_100ab31c = 0;
        }
    }
}
