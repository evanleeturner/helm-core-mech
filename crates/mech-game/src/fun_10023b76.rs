// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ab374: u32;
    static mut DAT_100ab378: u32;
    static mut DAT_100ab37c: u32;
    static mut DAT_100ab380: u32;
    static mut DAT_1016aca0: u8;
    static mut DAT_1016aca1: u8;
    static mut DAT_1016aca2: u8;
    static mut DAT_1016aca3: u8;
    static mut DAT_1016acb4: u32;
    static mut DAT_1016acb8: u8;
    static mut DAT_1016bc60: u8;
    static mut DAT_1016cc60: u8;
    static mut DAT_1016cc64: u8;
    static mut DAT_1016cc68: u8;
    static mut DAT_1016cc6c: u8;
    static mut DAT_1016cc72: u8;
    static mut DAT_1016cc76: u8;
    static mut DAT_1016cc7a: u8;
    static mut DAT_1016cc7e: u8;
    static mut DAT_1016ce32: u8;
    static mut DAT_1016ce36: u8;
    static mut DAT_1016ce3a: u8;
    static mut DAT_1016ce56: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1001c5d1(param_1: i32, param_2: i32, param_3: i32, param_4: *mut u32) -> *mut u16;
    fn FUN_1001c72d(
        param_1: i32,
        param_2: u32,
        param_3: u32,
        param_4: u32,
        param_5: u32,
        param_6: u32,
    ) -> u8;
    fn FUN_1001c7df(param_1: i32, param_2: u16, param_3: i32) -> *mut u16;
    fn FUN_1001c85e(param_1: i32, param_2: i32, param_3: u8) -> u8;
    fn FUN_1002f010(param_1: i32, param_2: u8) -> *mut i32;
    fn FUN_1002f272(param_1: i32, param_2: u32) -> u8;
    fn FUN_1001d5e1(param_1: i32, param_2: u32) -> u8;
    fn FUN_1001ca37(param_1: i32, param_2: u16) -> u8;
    fn FUN_1001cc6a(param_1: i32) -> u8;
    fn FUN_1002f37a(param_1: i32, param_2: u32, param_3: u32, param_4: u32) -> u8;
    fn FUN_1002fb55(param_1: *mut i32) -> u8;
    fn FUN_1003da99(param_1: *mut u16) -> u8;
    fn FUN_1002f727(param_1: i32) -> u8;
    fn FUN_1002f7f7(param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10023b76() {
    // SAFETY: all globals and callees are the game's own memory and code;
    // the table writes land at the offsets the original uses.
    unsafe {
        let mut local_c: u32 = 0;
        let mut local_8: i32 = 0;
        DAT_100ab380 = 0;
        DAT_100ab37c = 0;
        DAT_100ab378 = 0;
        let mut u_var1: u16 = 0;
        while u_var1 < 1000 && local_8 == 0 {
            let p4 = FUN_1001c5d1(3, 1, 0xc, &raw mut local_c);
            if p4.is_null() {
                local_8 = 1;
            } else {
                DAT_100ab374 = DAT_100ab374.wrapping_add(1);
                let p4i = p4 as usize as i32;
                FUN_1001c72d(p4i, 0, 0, 0, 0, 0);
                FUN_1001c72d(p4i, 0, 100, 0, 100, 0);
                FUN_1001c72d(p4i, 100, 0, 0, 0, 100);
                let p5 = FUN_1001c7df(p4i, u_var1 | 0x8000, local_c as i32);
                let p5i = p5 as usize as i32;
                FUN_1001c85e(p4i, p5i, 0);
                FUN_1001c85e(p4i, p5i, 1);
                FUN_1001c85e(p4i, p5i, 2);
                let p6 = FUN_1002f010(0, 0xc);
                let p6i = p6 as usize as i32;
                FUN_1002f272(p6i, p4 as usize as u32);
                FUN_1001d5e1(p4i, p6 as usize as u32);
                FUN_1001ca37(p4i, 0x60);
                FUN_1001cc6a(p4i);
                FUN_1002f37a(p6i, 0, 0, 0);
                FUN_1002fb55(p6);
                FUN_1003da99(p4);
                FUN_1002f727(p6i);
                FUN_1002f7f7(p6i);

                let idx = u_var1 as u32;
                let o4 = idx.wrapping_mul(4) as usize;
                let o = idx.wrapping_mul(0x1fa) as usize;
                (&raw mut DAT_1016bc60)
                    .cast::<u8>()
                    .byte_add(o4)
                    .cast::<u32>()
                    .write_unaligned(idx);
                (&raw mut DAT_1016cc68)
                    .cast::<u8>()
                    .byte_add(o)
                    .cast::<u32>()
                    .write_unaligned(idx);
                (&raw mut DAT_1016cc6c)
                    .cast::<u8>()
                    .byte_add(o)
                    .cast::<u32>()
                    .write_unaligned(0);
                (&raw mut DAT_1016ce32)
                    .cast::<u8>()
                    .byte_add(o)
                    .cast::<u32>()
                    .write_unaligned(p6 as usize as u32);
                (&raw mut DAT_1016ce36)
                    .cast::<u8>()
                    .byte_add(o)
                    .cast::<u32>()
                    .write_unaligned(0);
                (&raw mut DAT_1016ce3a)
                    .cast::<u8>()
                    .byte_add(o)
                    .cast::<u32>()
                    .write_unaligned(0);
                (&raw mut DAT_1016cc60)
                    .cast::<u8>()
                    .byte_add(o)
                    .cast::<u32>()
                    .write_unaligned(0);
                (&raw mut DAT_1016cc64)
                    .cast::<u8>()
                    .byte_add(o)
                    .cast::<u32>()
                    .write_unaligned(0);
                (&raw mut DAT_1016ce56)
                    .cast::<u8>()
                    .byte_add(o)
                    .cast::<u32>()
                    .write_unaligned(0);
                let mut u_var2: u16 = 0;
                while u_var2 < 0x20 {
                    let oo = o.wrapping_add((u_var2 as u32).wrapping_mul(0xe) as usize);
                    (&raw mut DAT_1016cc7a)
                        .cast::<u8>()
                        .byte_add(oo)
                        .cast::<u32>()
                        .write_unaligned(0);
                    let v = (&raw mut DAT_1016cc7a)
                        .cast::<u8>()
                        .byte_add(oo)
                        .cast::<u32>()
                        .read_unaligned();
                    (&raw mut DAT_1016cc76)
                        .cast::<u8>()
                        .byte_add(oo)
                        .cast::<u32>()
                        .write_unaligned(v);
                    let v = (&raw mut DAT_1016cc76)
                        .cast::<u8>()
                        .byte_add(oo)
                        .cast::<u32>()
                        .read_unaligned();
                    (&raw mut DAT_1016cc72)
                        .cast::<u8>()
                        .byte_add(oo)
                        .cast::<u32>()
                        .write_unaligned(v);
                    (&raw mut DAT_1016cc7e)
                        .cast::<u8>()
                        .byte_add(oo)
                        .cast::<u16>()
                        .write_unaligned(0);
                    u_var2 = u_var2.wrapping_add(1);
                }
            }
            u_var1 = u_var1.wrapping_add(1);
        }
        DAT_1016acb4 = DAT_100ab374.wrapping_sub(1);
        DAT_1016aca0 = 0x15;
        DAT_1016aca1 = 0x15;
        u_var1 = 0;
        while u_var1 < 8 {
            let c_var3 = u_var1 as i8;
            let o2 = (u_var1 as u32).wrapping_mul(2) as usize;
            (&raw mut DAT_1016aca2)
                .byte_add(o2)
                .write(0x3fi8.wrapping_sub(c_var3) as u8);
            (&raw mut DAT_1016aca3)
                .byte_add(o2)
                .write(0x3fi8.wrapping_sub(c_var3) as u8);
            (&raw mut DAT_1016acb8)
                .byte_add(u_var1 as usize)
                .write((-0x22i8).wrapping_sub(c_var3) as u8);
            u_var1 = u_var1.wrapping_add(1);
        }
    }
}
