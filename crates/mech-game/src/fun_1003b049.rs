// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38fc: u32;
    static mut DAT_100aaf68: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000fc30(param_1: i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_10039980(param_1: i32) -> i32;
    fn FUN_10024008(param_1: u32, param_2: u32, param_3: u32) -> *mut u32;
    fn FUN_100399ec(param_1: i32, param_2: i32);
    fn FUN_10051f43(param_1: i32) -> i32;
    fn FUN_1002f783(param_1: i32);
    fn FUN_1002f37a(param_1: i32, param_2: u32, param_3: u32, param_4: u32);
    fn FUN_1002fbc0(param_1: *mut i32);
    fn FUN_1003ad1e(param_1: i32) -> bool;
    fn FUN_1003ae65(param_1: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003b049(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
    param_7: i32,
) {
    // SAFETY: all globals and callees are the game's own; pointers are the
    // game's structures, read and written at the original's offsets.
    unsafe {
        let base = DAT_100aaf68 as usize as *const u8;
        let i0 = base.cast::<i32>().read_unaligned();
        let i1 = base.byte_add(4).cast::<i32>().read_unaligned();
        let i2 = base.byte_add(8).cast::<i32>().read_unaligned();
        let mut i_var1 = FUN_1000fc30(
            param_1.wrapping_sub(i0),
            param_2.wrapping_sub(i1),
            param_3.wrapping_sub(i2),
        );
        let limit = (DAT_100aaf68 as usize as *const u8)
            .byte_add(0x40)
            .cast::<i32>()
            .read_unaligned();
        if i_var1 <= limit {
            let pi_var2 = FUN_10039980(param_7) as usize as *mut i32;
            if !pi_var2.is_null() {
                let pu = FUN_10024008(
                    FUN_1003ad1e as usize as u32,
                    FUN_1003ae65 as usize as u32,
                    pi_var2 as usize as u32,
                );
                if pu.is_null() {
                    FUN_100399ec(param_7, pi_var2.read_unaligned());
                } else {
                    let p = pu.cast::<u8>();
                    let v = pu.byte_add(8).read_unaligned();
                    pi_var2.byte_add(4).write_unaligned(v as i32);
                    i_var1 = FUN_10051f43(500);
                    pi_var2
                        .byte_add(20)
                        .write_unaligned(i_var1.wrapping_sub(0xfa));
                    pi_var2.byte_add(24).write_unaligned(0);
                    i_var1 = FUN_10051f43(500);
                    pi_var2
                        .byte_add(28)
                        .write_unaligned(i_var1.wrapping_sub(0xfa));
                    p.byte_add(0x10).cast::<u16>().write_unaligned(1);
                    let obj = p.byte_add(0x1d2).cast::<i32>().read_unaligned();
                    let i_var1 = (obj as usize as *const u8)
                        .byte_add(0x6c)
                        .cast::<i32>()
                        .read_unaligned() as usize as *mut u8;
                    let i_var4 = FUN_1000fc30(param_4, param_5, param_6);
                    i_var1.byte_add(0x40).cast::<i32>().write_unaligned(i_var4);
                    i_var1.byte_add(0x34).cast::<i32>().write_unaligned(param_1);
                    i_var1.byte_add(0x38).cast::<i32>().write_unaligned(param_2);
                    i_var1.byte_add(0x3c).cast::<i32>().write_unaligned(param_3);
                    i_var1.byte_add(0x28).cast::<u32>().write_unaligned(0);
                    i_var1.byte_add(0x2c).cast::<u32>().write_unaligned(0);
                    i_var1.byte_add(0x30).cast::<u32>().write_unaligned(0);
                    p.byte_add(0x1ea).cast::<i32>().write_unaligned(param_4);
                    p.byte_add(0x1ee).cast::<i32>().write_unaligned(param_5);
                    p.byte_add(0x1f2).cast::<i32>().write_unaligned(param_6);
                    FUN_1002f783(p.byte_add(0x1d2).cast::<i32>().read_unaligned());
                    FUN_1002f37a(
                        p.byte_add(0x1d2).cast::<i32>().read_unaligned(),
                        param_1 as u32,
                        param_2 as u32,
                        param_3 as u32,
                    );
                    FUN_1002fbc0(
                        p.byte_add(0x1d2).cast::<i32>().read_unaligned() as usize as *mut i32
                    );
                    p.byte_add(0x1de)
                        .cast::<u32>()
                        .write_unaligned(DAT_100b38fc);
                    let mut local_20 = param_6 >> 4;
                    let mut local_1c = param_5 >> 4;
                    let mut local_18 = param_4 >> 4;
                    let mut local_24: i32 = 0;
                    while local_24 < 0x20 {
                        let off = (local_24 as usize).wrapping_mul(0xe);
                        p.byte_add(off)
                            .byte_add(0x1e)
                            .cast::<u16>()
                            .write_unaligned(1);
                        local_18 = local_18.wrapping_add(param_4 >> 4);
                        local_1c = local_1c.wrapping_add(param_5 >> 4);
                        local_20 = local_20.wrapping_add(param_6 >> 4);
                        let r = FUN_10051f43(10);
                        p.byte_add(off)
                            .byte_add(0x12)
                            .cast::<i32>()
                            .write_unaligned(local_18.wrapping_add(r).wrapping_sub(5));
                        let r = FUN_10051f43(0x32);
                        p.byte_add(off)
                            .byte_add(0x16)
                            .cast::<i32>()
                            .write_unaligned(local_1c.wrapping_add(r).wrapping_sub(0x19));
                        let r = FUN_10051f43(10);
                        p.byte_add(off)
                            .byte_add(0x1a)
                            .cast::<i32>()
                            .write_unaligned(local_20.wrapping_add(r).wrapping_sub(5));
                        local_24 += 1;
                    }
                }
            }
        }
    }
}
