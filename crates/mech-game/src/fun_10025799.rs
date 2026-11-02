// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf74: *mut core::ffi::c_void;
    static mut DAT_100aca28: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1006a38c(param_1: *mut i8);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapAlloc(h: *mut core::ffi::c_void, flags: u32, size: u32) -> *mut u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10025799(
    param_1: u32,
    param_2: u32,
    param_3: u32,
    param_4: u32,
    param_5: u32,
    param_6: u32,
    param_7: u32,
) -> *mut u32 {
    // SAFETY: the globals and callees are the game's own; the heap block holds
    // param_7 * 4 + 0x2c bytes, and every write stays inside it.
    unsafe {
        let p_var1 = HeapAlloc(DAT_100adf74, 1, param_7.wrapping_mul(4).wrapping_add(0x2c));
        if p_var1.is_null() {
            FUN_1006a38c((&raw mut DAT_100aca28).cast::<i8>());
        } else {
            p_var1.write_unaligned(param_1);
            p_var1.add(1).write_unaligned(param_2);
            p_var1.add(2).write_unaligned(param_3);
            p_var1.add(3).write_unaligned(param_4);
            p_var1.add(4).write_unaligned(param_5);
            p_var1.add(5).write_unaligned(param_6);
            p_var1.add(6).write_unaligned(param_7);
            let mut local_10: i32 = 4;
            while local_10 != 0 {
                p_var1.offset((local_10 + 6) as isize).write_unaligned(0);
                local_10 -= 1;
            }
            if 0 < (param_7 as i32) {
                let mut p_var4 = p_var1.add(0xb);
                let mut u_var2 = param_7 & 0x3fffffff;
                while u_var2 != 0 {
                    p_var4.write_unaligned(0);
                    p_var4 = p_var4.add(1);
                    u_var2 -= 1;
                }
            }
        }
        p_var1
    }
}
