// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7b60: u32;
    static mut DAT_100b36d0: i32;
    static mut DAT_100f3040: [u8; 0];
    static mut DAT_100f3044: [u8; 0];
    static mut DAT_100f3048: [u8; 0];
    static mut DAT_100f304c: [u8; 0];
    static mut DAT_100f3050: [u8; 0];
    static mut DAT_101f52e0: [u8; 0];
    static mut PTR_DAT_100aaf68: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100603ff(param_1: i32) {
    // SAFETY: all globals are the game's own memory; the pointer held in
    // PTR_DAT_100aaf68 and the tables at DAT_100f30xx are valid for the
    // offsets accessed, as in the original.
    unsafe {
        if (-1 < param_1) && (param_1 < 0xb) && (DAT_100b36d0 != param_1) {
            let i_var1 = param_1.wrapping_mul(0x14) as usize;
            let base = PTR_DAT_100aaf68 as usize;
            ((base.wrapping_add(0x2c)) as *mut u32).write_unaligned(0);
            ((base.wrapping_add(0x34)) as *mut u32).write_unaligned(0);
            let a = (&raw mut DAT_100f304c)
                .cast::<u8>()
                .add(i_var1)
                .cast::<i32>()
                .read_unaligned();
            let b = (&raw mut DAT_100f3044)
                .cast::<u8>()
                .add(i_var1)
                .cast::<i32>()
                .read_unaligned();
            ((base.wrapping_add(0x30)) as *mut i32).write_unaligned(a.wrapping_sub(b));
            let c = (&raw mut DAT_100f3050)
                .cast::<u8>()
                .add(i_var1)
                .cast::<i32>()
                .read_unaligned();
            let d = (&raw mut DAT_100f3048)
                .cast::<u8>()
                .add(i_var1)
                .cast::<i32>()
                .read_unaligned();
            ((base.wrapping_add(0x38)) as *mut i32).write_unaligned(c.wrapping_sub(d));
            ((base.wrapping_add(0x4c)) as *mut u32).write_unaligned(0);
            ((base.wrapping_add(0x50)) as *mut u32).write_unaligned(0);
            let mut src = (&raw mut DAT_100f3040)
                .cast::<u8>()
                .add(i_var1)
                .cast::<u32>();
            let mut dst = (&raw mut DAT_101f52e0).cast::<u32>();
            let mut n = 5;
            while n != 0 {
                dst.write_unaligned(src.read_unaligned());
                src = src.add(1);
                dst = dst.add(1);
                n -= 1;
            }
            DAT_100b36d0 = param_1;
            DAT_100a7b60 = 1;
        }
    }
}
