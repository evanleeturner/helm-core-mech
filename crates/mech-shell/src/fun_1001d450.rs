// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut LAB_1001d2b0: u8;
    static mut LAB_1001d3a0: u8;
    static mut LAB_1001d400: u8;
    static mut DAT_10098224: *mut u8;
    static mut DAT_100a6744: u32;
    static mut DAT_100a674c: *mut u8;
    static mut DAT_100c49d0: u8;
    static mut DAT_10090244: i8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapAlloc(h: *mut u8, flags: u32, size: u32) -> *mut i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000c4c0(param_1: *mut u8, param_2: i32, param_3: *const i8) -> i32;
    fn FUN_1001d130(param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001d450(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
) -> *mut i32 {
    // SAFETY: globals and callees are the game's own; param_3 points at a
    // structure whose count lives at +4, and the heap block is sized for
    // (count + 1) entries of 0x70 bytes, written where the original does.
    unsafe {
        let count = ((param_3 as u32).wrapping_add(4) as *const i32).read_unaligned();
        let p4 = HeapAlloc(
            DAT_10098224,
            1,
            (count.wrapping_add(1) as u32).wrapping_mul(0x70),
        );
        let w = |idx: i32, off: isize, v: i32| {
            p4.offset(idx.wrapping_mul(14) as isize)
                .offset(off)
                .write_unaligned(v);
        };
        let rd = |addr: u32| (addr as *const i32).read_unaligned();
        let mut i_var5 = FUN_1000c4c0(DAT_100a674c, 0xd2, &raw const DAT_10090244);
        p4.write_unaligned(i_var5);
        p4.add(1).write_unaligned(param_2);
        p4.add(3)
            .write_unaligned(rd((DAT_100a674c as u32).wrapping_add(0x40c)).wrapping_add(5));
        p4.add(4).write_unaligned(0);
        p4.add(5)
            .write_unaligned((&raw mut DAT_100c49d0) as usize as i32);
        p4.add(6).write_unaligned(0);
        p4.add(7)
            .write_unaligned((&raw mut LAB_1001d400) as usize as i32);
        let mut i_var6: i32 = 1;
        p4.add(8).write_unaligned(0);
        p4.add(9).write_unaligned(-1);
        p4.add(10).write_unaligned(-1);
        p4.add(0xb).write_unaligned(-1);
        let mut local_4: i32 = 0;
        i_var5 = 0;
        if 0 < count {
            loop {
                let i_var2 = i_var5.wrapping_add(i_var6);
                local_4 = local_4.wrapping_add(1);
                i_var6 = i_var6.wrapping_add(1);
                w(i_var2, 0, param_1);
                let i_var1 = i_var5.wrapping_add(1);
                let v = rd(DAT_100a6744.wrapping_add(0x40c))
                    .wrapping_mul(i_var1)
                    .wrapping_add(rd((DAT_100a674c as u32).wrapping_add(0x40c)))
                    .wrapping_add(param_2);
                w(i_var2, 1, v);
                w(i_var2, 2, 0);
                w(
                    i_var2,
                    3,
                    rd(DAT_100a6744.wrapping_add(0x40c)).wrapping_add(5),
                );
                w(i_var2, 4, 0);
                w(i_var2, 5, (&raw mut DAT_100c49d0) as usize as i32);
                w(i_var2, 6, 0);
                w(i_var2, 7, (&raw mut LAB_1001d3a0) as usize as i32);
                w(i_var2, 8, 0);
                w(i_var2, 9, i_var5);
                w(i_var2, 10, -1);
                w(i_var2, 0xb, -1);
                let i3 = i_var5.wrapping_add(i_var6);
                w(i3, 0, 0x96);
                let v = rd(DAT_100a6744.wrapping_add(0x40c))
                    .wrapping_mul(i_var1)
                    .wrapping_add(rd((DAT_100a674c as u32).wrapping_add(0x40c)))
                    .wrapping_add(param_2);
                w(i3, 1, v);
                w(i3, 2, 0xe6);
                w(i3, 3, rd(DAT_100a6744.wrapping_add(0x40c)).wrapping_add(5));
                w(i3, 4, 0);
                w(i3, 5, (&raw mut DAT_100c49d0) as usize as i32);
                w(i3, 6, 0);
                w(i3, 7, (&raw mut LAB_1001d2b0) as usize as i32);
                w(i3, 8, FUN_1001d130 as usize as i32);
                w(i3, 9, i_var5);
                w(i3, 10, param_4);
                w(i3, 0xb, -1);
                i_var5 = i_var1;
                if !(i_var1 < count) {
                    break;
                }
            }
        }
        w(local_4.wrapping_add(i_var6), 0, -1);
        p4
    }
}
