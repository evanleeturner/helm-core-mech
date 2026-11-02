// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ab374: u32;
    static mut DAT_100ab378: u32;
    static mut DAT_100b38fc: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10024008(param_1: u32, param_2: u32, param_3: u32) -> *mut u32;
    fn FUN_1000fc30(param_1: i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_1002f783(param_1: i32) -> u8;
    fn FUN_1002f37a(param_1: i32, param_2: u32, param_3: u32, param_4: u32) -> u8;
    fn FUN_1002fbc0(param_1: *mut i32) -> u8;
    fn FUN_10024df7(param_1: i32) -> bool;
    fn FUN_10024eaa(param_1: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10025061(
    param_1: u32,
    param_2: u32,
    param_3: u32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
) {
    // SAFETY: globals and callees are the game's own; the object returned by
    // FUN_10024008 is a game record whose fields are accessed at the
    // original's byte offsets.
    unsafe {
        if DAT_100ab378.wrapping_add(1) < DAT_100ab374 {
            let p = FUN_10024008(
                FUN_10024df7 as usize as u32,
                FUN_10024eaa as usize as u32,
                0,
            );
            if !p.is_null() {
                p.byte_add(16).cast::<u16>().write_unaligned(1);
                let sub = p.byte_add(0x1d2).cast::<i32>().read_unaligned();
                let i_var1 = (sub as usize + 0x6c) as *const i32;
                let i_var1 = i_var1.read_unaligned() as usize;
                let i_var3 = FUN_1000fc30(
                    param_4.wrapping_mul(100),
                    param_5.wrapping_mul(100),
                    param_6.wrapping_mul(100),
                );
                ((i_var1 + 0x40) as *mut i32).write_unaligned(i_var3);
                ((i_var1 + 0x34) as *mut u32).write_unaligned(param_1);
                ((i_var1 + 0x38) as *mut u32).write_unaligned(param_2);
                ((i_var1 + 0x3c) as *mut u32).write_unaligned(param_3);
                ((i_var1 + 0x28) as *mut u32).write_unaligned(0);
                ((i_var1 + 0x2c) as *mut u32).write_unaligned(0);
                ((i_var1 + 0x30) as *mut u32).write_unaligned(0);
                p.byte_add(0x1ea)
                    .cast::<i32>()
                    .write_unaligned(param_4 >> 4);
                p.byte_add(0x1ee)
                    .cast::<i32>()
                    .write_unaligned(param_5 >> 4);
                p.byte_add(0x1f2)
                    .cast::<i32>()
                    .write_unaligned(param_6 >> 4);
                FUN_1002f783(p.byte_add(0x1d2).cast::<i32>().read_unaligned());
                FUN_1002f37a(
                    p.byte_add(0x1d2).cast::<i32>().read_unaligned(),
                    param_1,
                    param_2,
                    param_3,
                );
                FUN_1002fbc0(p.byte_add(0x1d2).cast::<i32>().read_unaligned() as *mut i32);
                DAT_100ab378 = DAT_100ab378.wrapping_add(1);
                p.byte_add(0x1de)
                    .cast::<u32>()
                    .write_unaligned(DAT_100b38fc);
                let mut local_1c = param_6 / 10;
                let mut local_18 = param_5 / 10;
                let mut local_14 = param_4 / 10;
                let mut local_20: i32 = 0;
                while local_20 < 10 {
                    let off = local_20.wrapping_mul(0xe) as usize;
                    p.byte_add(off + 0x1e).cast::<u16>().write_unaligned(1);
                    local_14 = local_14.wrapping_add(param_4 / 10);
                    local_18 = local_18.wrapping_add(param_5 / 10);
                    local_1c = local_1c.wrapping_add(param_6 / 10);
                    p.byte_add(off + 0x12)
                        .cast::<i32>()
                        .write_unaligned(local_14);
                    p.byte_add(off + 0x16)
                        .cast::<i32>()
                        .write_unaligned(local_18);
                    p.byte_add(off + 0x1a)
                        .cast::<i32>()
                        .write_unaligned(local_1c);
                    local_20 += 1;
                }
                p.add(3).write_unaligned(3);
            }
        }
    }
}
