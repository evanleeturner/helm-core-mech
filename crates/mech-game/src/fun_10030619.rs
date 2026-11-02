// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ada5c: i32;
    static mut DAT_100ada58: i32;
    static mut DAT_100ada64: i32;
    static mut DAT_100ada60: i32;
    static mut DAT_100ada54: i32;
    static mut DAT_100ada50: i32;
    static mut DAT_100ada4c: i32;
    static mut DAT_10168800: u8;
    static mut DAT_10168900: u8;
    static mut DAT_10169600: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1001c5d1(param_1: i32, param_2: i32, param_3: i32, param_4: *mut i32) -> *mut u16;
    fn FUN_1001c72d(
        param_1: i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
    );
    fn FUN_1001c7df(param_1: i32, param_2: u32, param_3: i32) -> *mut u16;
    fn FUN_1001c85e(param_1: i32, param_2: i32, param_3: i32);
    fn FUN_1002f010(param_1: i32, param_2: i32) -> *mut i32;
    fn FUN_1002f272(param_1: i32, param_2: *mut u16);
    fn FUN_1001d5e1(param_1: i32, param_2: *mut i32);
    fn FUN_1001ca37(param_1: i32, param_2: i32);
    fn FUN_1001cc6a(param_1: i32);
    fn FUN_1002f37a(param_1: i32, param_2: i32, param_3: i32, param_4: i32);
    fn FUN_1002fb55(param_1: *mut i32);
    fn FUN_1003da99(param_1: *mut u16);
    fn FUN_1002f727(param_1: i32);
    fn FUN_1002f7f7(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10030619() {
    // SAFETY: the globals and callees are the game's own; the table
    // addresses are the game's arrays, written where the original does.
    unsafe {
        let mut local_c: i32 = 0;
        let mut local_8: i32 = 0;
        DAT_100ada5c = 0;
        DAT_100ada58 = 0;
        DAT_100ada64 = 0;
        DAT_100ada60 = 0;
        DAT_100ada54 = 0;
        let mut u_var1: u16 = 0;
        while u_var1 < 0x40 && local_8 == 0 {
            let pu2 = FUN_1001c5d1(4, 1, 0x10, &raw mut local_c);
            if pu2.is_null() {
                local_8 = 1;
            } else {
                DAT_100ada50 = DAT_100ada50.wrapping_add(1);
                let p2 = pu2 as usize as i32;
                FUN_1001c72d(p2, -1000, 0, 1000, 0, 0);
                FUN_1001c72d(p2, -1000, 0, -1000, 0, 0);
                FUN_1001c72d(p2, 1000, 0, -1000, 0, 0);
                FUN_1001c72d(p2, 1000, 0, 1000, 0, 0);
                let pu3 = FUN_1001c7df(p2, (u_var1 as u32) | 0x9000, local_c);
                let p3 = pu3 as usize as i32;
                FUN_1001c85e(p2, p3, 0);
                FUN_1001c85e(p2, p3, 1);
                FUN_1001c85e(p2, p3, 2);
                FUN_1001c85e(p2, p3, 3);
                let pi4 = FUN_1002f010(0, 0xc);
                let p4 = pi4 as usize as i32;
                FUN_1002f272(p4, pu2);
                FUN_1001d5e1(p2, pi4);
                FUN_1001ca37(p2, 0);
                FUN_1001cc6a(p2);
                FUN_1002f37a(p4, 0, 0, 0);
                FUN_1002fb55(pi4);
                FUN_1003da99(pu2);
                FUN_1002f727(p4);
                FUN_1002f7f7(p4);
                let idx = u_var1 as usize;
                let b8800 = (&raw mut DAT_10168800) as *mut u8;
                let b8900 = (&raw mut DAT_10168900) as *mut u8;
                b8800
                    .byte_add(idx.wrapping_mul(4))
                    .cast::<u32>()
                    .write_unaligned(u_var1 as u32);
                let rec = b8900.byte_add(idx.wrapping_mul(0x34));
                rec.cast::<u32>().write_unaligned(u_var1 as u32);
                rec.byte_add(0x30)
                    .cast::<u32>()
                    .write_unaligned(pi4 as usize as u32);
                rec.byte_add(0xc).cast::<u32>().write_unaligned(0);
                rec.byte_add(0x10).cast::<u32>().write_unaligned(0);
                rec.byte_add(0x8).cast::<u32>().write_unaligned(0);
                rec.byte_add(0x4).cast::<u32>().write_unaligned(0);
            }
            u_var1 = u_var1.wrapping_add(1);
        }
        DAT_100ada4c = DAT_100ada50.wrapping_sub(1);
        let tbl = (&raw mut DAT_10169600) as *mut u8;
        u_var1 = 0;
        while u_var1 < 0x10 {
            let mut local_18: u32 = 0;
            while local_18 < 0x100 {
                let off = (u_var1 as usize) * 0x100 + local_18 as usize;
                let v: u8 = if local_18 < 0x30 || u_var1 == 0 {
                    local_18 as u8
                } else if (local_18 & 0xf).wrapping_add(u_var1 as u32) < 0x10 {
                    (local_18 as u8).wrapping_add(u_var1 as u8)
                } else {
                    (local_18 as u8) | 0xf
                };
                tbl.add(off).write(v);
                local_18 = (local_18 as u16).wrapping_add(1) as u32;
            }
            u_var1 = u_var1.wrapping_add(1);
        }
    }
}
