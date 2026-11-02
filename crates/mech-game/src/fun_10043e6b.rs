// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100447ff(param_1: i32, param_2: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10043e6b(param_1: i32) -> i32 {
    let mut local_c: i32 = 0;
    // SAFETY: param_1 is the address of the game's own object; fields are read
    // where the original reads them and the callee is the game's function.
    unsafe {
        let flags = ((param_1.wrapping_add(0xdc)) as *const u32).read_unaligned();
        if (flags & 0xf00) == 0x200 {
            let f54 = ((param_1.wrapping_add(0x54)) as *const i32).read_unaligned();
            let b_var1 = FUN_100447ff(param_1, f54);
            if b_var1 == 0 {
                let a = ((param_1.wrapping_add(0x54)) as *const i32).read_unaligned();
                let b = ((param_1.wrapping_add(0xcc)) as *const i32).read_unaligned();
                if a >= b.wrapping_sub(800) {
                    let p = ((param_1.wrapping_add(0x20)) as *const i32).read_unaligned();
                    let v = ((p.wrapping_add(0x3f4)) as *const i32).read_unaligned();
                    if v >= 0 {
                        local_c = 1;
                    } else {
                        local_c = 0;
                    }
                } else {
                    local_c = 0;
                }
            }
        }
    }
    local_c
}
