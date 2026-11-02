// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aaf68: *mut u8;
    static mut DAT_100ab07c: i32;
    static mut DAT_100ab318: i32;
    static mut DAT_100ab060: i32;
    static mut DAT_100ab074: i32;
    static mut DAT_100ab078: i32;
    static mut DAT_100b38fc: i32;
    static mut DAT_101eb460: [i32; 3];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10006b20(param_1: i32) -> i32;
    fn FUN_10006b9a(param_1: i32) -> i32;
    fn FUN_10003d60(param_1: i32, param_2: i32) -> i32;
    fn FUN_1001a635(param_1: i32);
    fn FUN_10067af1(param_1: *mut i32) -> i32;
    fn FUN_1001a58f(param_1: *mut i32) -> i32;
}

#[inline(always)]
unsafe fn fld(off: usize) -> *mut i32 {
    // SAFETY: the game's object pointer is valid for the offsets used here.
    unsafe { DAT_100aaf68.byte_add(off).cast::<i32>() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001b179(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
) {
    // SAFETY: the globals and callees are the game's own; the object pointer
    // read from DAT_100aaf68 points at the game's live structure.
    unsafe {
        let mut i_var1 = FUN_10006b20(fld(0xc).read_unaligned());
        let i_var2 = FUN_10006b9a(fld(0xc).read_unaligned());
        FUN_10006b20(fld(0x10).read_unaligned());
        let i_var3 = FUN_10006b9a(fld(0x10).read_unaligned());
        let arr = (&raw mut DAT_101eb460).cast::<i32>();
        if DAT_100ab07c != 2 {
            if DAT_100ab318 != 0 {
                let i_var4 = FUN_10003d60(DAT_100ab060, i_var1);
                fld(0).write_unaligned(fld(0).read_unaligned().wrapping_sub(i_var4 >> 0xd));
                let i_var4 = FUN_10003d60(DAT_100ab060, i_var2);
                fld(8).write_unaligned(fld(8).read_unaligned().wrapping_sub(i_var4 >> 0xd));
            }
            fld(0x14).write_unaligned(0);
            arr.add(2).write_unaligned(0);
            arr.write_unaligned(DAT_100b38fc);
            DAT_100ab078 = DAT_100ab074;
            FUN_1001a635(0);
        }
        let i_var4 = FUN_10003d60(param_3, i_var2);
        fld(0).write_unaligned(fld(0).read_unaligned().wrapping_add(i_var4 >> 0xb));
        let i_var4 = FUN_10003d60(param_3, i_var1);
        fld(8).write_unaligned(fld(8).read_unaligned().wrapping_sub(i_var4 >> 0xb));
        fld(0x10).write_unaligned(fld(0x10).read_unaligned().wrapping_sub(param_5));
        if fld(0x10).read_unaligned() < 0x5a0001 {
            if fld(0x10).read_unaligned() < -0x5a0000 {
                fld(0x10).write_unaligned(0xffa60000u32 as i32);
            }
        } else {
            fld(0x10).write_unaligned(0x5a0000);
        }
        fld(4).write_unaligned(
            fld(4)
                .read_unaligned()
                .wrapping_add(param_1.wrapping_mul(4)),
        );
        arr.add(1).write_unaligned(param_2.wrapping_shl(4));
        let mut local_18 = FUN_10067af1(arr);
        if local_18 < 0x20 && -0x20 < local_18 {
            local_18 = 0;
        }
        fld(0xc).write_unaligned(fld(0xc).read_unaligned().wrapping_add(param_4));
        let i_var4 = i_var3;
        i_var1 = FUN_10003d60(local_18, i_var1);
        i_var1 = FUN_10003d60(i_var1 >> 0xd, i_var4);
        fld(0).write_unaligned(fld(0).read_unaligned().wrapping_sub(i_var1 >> 0xd));
        i_var1 = FUN_10003d60(local_18, i_var2);
        i_var1 = FUN_10003d60(i_var1 >> 0xd, i_var3);
        fld(8).write_unaligned(fld(8).read_unaligned().wrapping_sub(i_var1 >> 0xd));
        i_var1 = FUN_1001a58f(DAT_100aaf68.cast::<i32>());
        if fld(4).read_unaligned() < i_var1 {
            let mut i_var2 = fld(4).read_unaligned();
            if fld(4).read_unaligned() <= i_var1 {
                i_var2 = i_var1;
            }
            fld(4).write_unaligned(i_var2);
        }
    }
}
