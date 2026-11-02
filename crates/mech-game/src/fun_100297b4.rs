// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002a132(param_1: i32) -> i32;
    fn FUN_10003d60(param_1: i32, param_2: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100297b4(param_1: i32) -> i32 {
    // SAFETY: the callees are the game's own; param_1 is the caller's object
    // whose field at +0x4c points at a struct with a writable i32 at +0xc.
    unsafe {
        let i_var1 = FUN_1002a132(param_1);
        let i_var2 = (i_var1 >> 0x10) % 0x168;
        if i_var2 < 0x2e {
            if i_var2 < -0x2d {
                let p = ((param_1.wrapping_add(0x4c)) as *const i32).read_unaligned();
                ((p.wrapping_add(0xc)) as *mut i32).write_unaligned(0xfccccccd_u32 as i32);
            } else {
                let r = FUN_10003d60(0x3333333, (i_var2.wrapping_shl(0x10)) / 0x2d);
                let p = ((param_1.wrapping_add(0x4c)) as *const i32).read_unaligned();
                ((p.wrapping_add(0xc)) as *mut i32).write_unaligned(r);
            }
        } else {
            let p = ((param_1.wrapping_add(0x4c)) as *const i32).read_unaligned();
            ((p.wrapping_add(0xc)) as *mut i32).write_unaligned(0x3333333);
        }
        i_var1
    }
}
