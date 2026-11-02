// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ab0a0: i32;
    static mut DAT_100ab0a8: i32;
    static mut DAT_100ab318: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002fc9d(param_1: i32) -> *mut i32;
    fn FUN_10089060(param_1: *mut i32, param_2: *mut i32, param_3: *mut i32, param_4: *mut i32);
    fn FUN_1002f2d6(param_1: i32, param_2: *mut i32, param_3: *mut i32, param_4: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001a8a3(
    param_1: *mut i32,
    param_2: *mut i32,
    param_3: *mut i32,
    param_4: *mut i32,
    param_5: *mut i32,
    param_6: *mut i32,
) {
    let mut local_10: i32 = 0;
    let mut local_c: i32 = 0;
    let mut local_8: i32 = 0;
    // SAFETY: the globals and callees are the game's own; the params point at
    // the caller's ints, and the structure at DAT_100ab0a0 is read at the
    // offsets the original uses.
    unsafe {
        param_6.write_unaligned(0);
        param_5.write_unaligned(param_6.read_unaligned());
        param_4.write_unaligned(param_5.read_unaligned());
        param_3.write_unaligned(param_4.read_unaligned());
        param_2.write_unaligned(param_3.read_unaligned());
        param_1.write_unaligned(param_2.read_unaligned());
        let i_var1 = DAT_100ab0a0;
        if DAT_100ab0a0 != 0 {
            let base = i_var1 as usize as *const u8;
            param_1.write_unaligned(base.byte_add(0x5c).cast::<i32>().read_unaligned());
            param_2.write_unaligned(base.byte_add(0x60).cast::<i32>().read_unaligned());
            param_3.write_unaligned(base.byte_add(0x64).cast::<i32>().read_unaligned());
            if base.byte_add(0x44).cast::<i32>().read_unaligned() == 0 {
                param_4.write_unaligned(base.byte_add(0x50).cast::<i32>().read_unaligned());
                param_5.write_unaligned(base.byte_add(0x54).cast::<i32>().read_unaligned());
                param_6.write_unaligned(base.byte_add(0x58).cast::<i32>().read_unaligned());
            } else {
                if DAT_100ab0a8 != 0 {
                    let add = (DAT_100ab0a8 as usize as *const i32).read_unaligned();
                    param_5.write_unaligned(param_5.read_unaligned().wrapping_add(add));
                }
                let v44 = base.byte_add(0x44).cast::<i32>().read_unaligned();
                let pi_var2 = FUN_1002fc9d(v44);
                FUN_10089060(pi_var2, param_4, param_5, param_6);
                let v44 = base.byte_add(0x44).cast::<i32>().read_unaligned();
                FUN_1002f2d6(v44, &raw mut local_8, &raw mut local_c, &raw mut local_10);
                if DAT_100ab318 == 0 {
                    let a = base.byte_add(0x68).cast::<i32>().read_unaligned();
                    param_1.write_unaligned(param_1.read_unaligned().wrapping_add(a));
                    let b = base.byte_add(0x6c).cast::<i32>().read_unaligned();
                    param_2.write_unaligned(param_2.read_unaligned().wrapping_add(b));
                    param_3.write_unaligned(local_10 >> 1);
                } else {
                    param_1.write_unaligned(local_8);
                    param_2.write_unaligned(local_c);
                    param_3.write_unaligned(local_10);
                }
            }
        }
    }
}
