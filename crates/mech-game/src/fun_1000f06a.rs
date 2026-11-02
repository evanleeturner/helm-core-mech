// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aaf68: *mut u8;
    static mut DAT_100aafc8: unsafe extern "C" fn(*mut u16) -> i32;
    static mut DAT_101f52e0: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1003ba89(param_1: *mut i32, param_2: *mut i32, param_3: *mut i32) -> i32;
    fn FUN_1000f1c0(param_1: i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_10003d60(param_1: i32, param_2: i32) -> i32;
    fn FUN_100594ec(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
    );
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000f06a(param_1: i32) {
    let base = param_1 as usize as *const u8;
    // SAFETY: param_1 is a caller-provided object pointer; fields are read
    // where the original reads them, and the globals and callees are the game's own.
    unsafe {
        if param_1 != 0
            && base.byte_add(8).cast::<i32>().read_unaligned() != 0
            && base.byte_add(4).cast::<i32>().read_unaligned()
                != base.byte_add(8).cast::<i32>().read_unaligned()
        {
            let mut local_c: *mut u16 = base.byte_add(8).cast::<*mut u16>().read_unaligned();
            while !local_c.is_null() {
                let w0 = local_c.read_unaligned();
                let w1 = local_c.add(1).read_unaligned();
                if (((w1 & 0x100) != 0 && (w0 & 0x800) == 0) || ((w1 as u8) & 0xf0) == 0x50)
                    && (DAT_100aafc8)(local_c) == 0
                {
                    let cb = local_c as *const u8;
                    let mut local_10: i32 = cb.byte_add(0x1a).cast::<i32>().read_unaligned();
                    let mut local_14: i32 = cb.byte_add(0x1c).cast::<i32>().read_unaligned();
                    let mut local_18: i32 = cb.byte_add(0x1e).cast::<i32>().read_unaligned();
                    let mut i_var1: i32 = cb.byte_add(0x20).cast::<i32>().read_unaligned();
                    let local_8: i32 = 0xf;
                    let i_var2 =
                        FUN_1003ba89(&raw mut local_10, &raw mut local_14, &raw mut local_18);
                    if i_var2 != 0 {
                        let g = DAT_100aaf68 as *const u8;
                        i_var1 = FUN_1000f1c0(
                            g.byte_add(0x94).cast::<i32>().read_unaligned(),
                            i_var1,
                            local_18,
                        );
                        let i_var2 =
                            FUN_10003d60(i_var1, g.byte_add(0x44).cast::<i32>().read_unaligned());
                        FUN_100594ec(
                            &raw mut DAT_101f52e0,
                            local_10,
                            local_14,
                            i_var1,
                            i_var2,
                            local_8,
                        );
                    }
                }
                local_c = (local_c as *const u8)
                    .byte_add(8)
                    .cast::<*mut u16>()
                    .read_unaligned();
            }
        }
    }
}
