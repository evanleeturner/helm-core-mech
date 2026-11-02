// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10082be8: i32;
    static mut DAT_10092000: i32;
    static mut DAT_10098224: i32;
    static mut DAT_10092010: i32;
    static mut DAT_100a67b8: i32;
    static mut DAT_10082bf0: i32;
    static mut DAT_10082c38: i32;
    static mut DAT_10082c2c: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000d230(param_1: i32) -> i32;
    fn FUN_1000d260(param_1: i32, param_2: i32) -> i32;
    fn FUN_10031890(param_1: i32) -> i32;
    fn FUN_10031a00(param_1: i32, param_2: i32);
    fn FUN_10031910(param_1: i32);
    fn FUN_1000a8c0(param_1: i32);
    fn FUN_1000aa80(param_1: i32);
    fn FUN_1000a890(param_1: i32);
    fn HeapAlloc(param_1: i32, param_2: u32, param_3: u32) -> *mut i32;
    fn FUN_10050390(param_1: *mut i32);
    fn FUN_1000c770(param_1: *mut i32, param_2: i32);
    fn FUN_10017df0(param_1: i32, param_2: *mut i32);
    fn FUN_100318e0(param_1: i32);
    fn FUN_100503d0(param_1: *mut i32);
    fn FUN_10031980(param_1: i32);
    fn FUN_10031950(param_1: i32);
    fn FUN_100319b0(param_1: i32);
    fn FUN_10050400(param_1: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100315b0(param_1: i32) {
    if 0 < param_1 {
        // SAFETY: all callees and globals are the game's own; the allocated
        // record is written within its allocated size, as the original does.
        unsafe {
            let mut c_var1 = FUN_1000d230(param_1);
            let mut i_var2 = FUN_1000d260(c_var1, param_1);
            if c_var1 == 2 {
                let v = (&raw mut DAT_10082be8)
                    .byte_offset(i_var2.wrapping_mul(0xc) as isize)
                    .read_unaligned();
                if v == 9000 {
                    FUN_10031a00(DAT_10092000, 9000);
                    return;
                }
                if v == 0x251c {
                    FUN_10031a00(DAT_10092000, 0x251c);
                    return;
                }
            }
            i_var2 = FUN_10031890(param_1);
            if (-1 < param_1) && (i_var2 != 0) {
                c_var1 = FUN_1000d230(param_1);
                i_var2 = c_var1;
                let i_var3 = FUN_1000d260(i_var2, param_1);
                if i_var2 == 0 {
                    FUN_10031910(param_1);
                    FUN_1000a8c0(DAT_10092000);
                    FUN_1000aa80(DAT_10092000);
                    FUN_1000a890(DAT_10092000);
                    let p = HeapAlloc(DAT_10098224, 1, 0x1a);
                    p.write_unaligned(i_var3);
                    p.byte_add(8).cast::<i8>().write(0);
                    p.byte_add(10).cast::<i32>().write_unaligned(-1);
                    p.byte_add(0xe).cast::<i32>().write_unaligned(-1);
                    p.byte_add(0x12).cast::<i32>().write_unaligned(100);
                    FUN_10050390(p);
                    FUN_1000c770(p, 0);
                    FUN_10017df0(DAT_10092010, p);
                    return;
                }
                if i_var2 == 1 {
                    FUN_100318e0(param_1);
                    FUN_1000a8c0(DAT_10092000);
                    FUN_1000aa80(DAT_10092000);
                    FUN_1000a890(DAT_10092000);
                    let p = HeapAlloc(DAT_10098224, 1, 0x1a);
                    p.write_unaligned(i_var3);
                    p.byte_add(8).cast::<i8>().write(1);
                    p.byte_add(0xe).cast::<i32>().write_unaligned(1);
                    p.byte_add(0x12).cast::<i32>().write_unaligned(-1);
                    p.byte_add(0x16).cast::<i32>().write_unaligned(100);
                    FUN_100503d0(p);
                    FUN_10017df0(DAT_100a67b8, p);
                    FUN_10017df0(DAT_10092010, p);
                    return;
                }
                if i_var2 != 2 {
                    return;
                }
                i_var2 = (&raw mut DAT_10082be8)
                    .byte_offset(i_var3.wrapping_mul(0xc) as isize)
                    .read_unaligned();
                let p = HeapAlloc(DAT_10098224, 1, 0x16);
                if i_var2 < 0x1f41 {
                    if i_var2 == 8000 {
                        let v = ((DAT_10092000.wrapping_add(0x228)) as *const i32).read_unaligned();
                        p.write_unaligned(1);
                        if v != 0 {
                            p.write_unaligned(2);
                        }
                        let idx = p.read_unaligned();
                        p.byte_add(4).write_unaligned(
                            (&raw mut DAT_10082bf0)
                                .byte_offset(idx.wrapping_mul(0xc) as isize)
                                .read_unaligned(),
                        );
                        FUN_10031980(DAT_10092000);
                    } else {
                        if i_var2 != 7000 {
                            return;
                        }
                        p.write_unaligned(10);
                    }
                } else if i_var2 == 0x2134 {
                    p.write_unaligned(6);
                    p.byte_add(4).write_unaligned(DAT_10082c38);
                    FUN_10031950(DAT_10092000);
                } else {
                    if i_var2 != 0x254e {
                        return;
                    }
                    p.write_unaligned(5);
                    p.byte_add(4).write_unaligned(DAT_10082c2c);
                    FUN_100319b0(DAT_10092000);
                }
                p.byte_add(8).cast::<i8>().write(2);
                p.byte_add(4).write_unaligned(0);
                p.byte_add(10).cast::<i32>().write_unaligned(-1);
                p.byte_add(0xe).cast::<i32>().write_unaligned(-1);
                p.byte_add(0x12).cast::<i32>().write_unaligned(-1);
                FUN_1000c770(p, 0);
                FUN_10050400(p);
                FUN_10017df0(DAT_10092010, p);
            }
        }
    }
}
