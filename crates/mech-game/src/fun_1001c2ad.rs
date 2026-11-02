// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1001c299(param_1: i32, param_2: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001c2ad(param_1: i32) {
    // SAFETY: param_1 is an address of the game's own object; fields are read
    // where the original reads them, and the callee is the game's function.
    unsafe {
        let p20 = ((param_1.wrapping_add(0x20)) as *const i32).read_unaligned();
        if p20 == 0 {
            let v = ((param_1.wrapping_add(0x1c)) as *const i32).read_unaligned();
            FUN_1001c299(param_1, v);
            return;
        }
        let p20 = ((param_1.wrapping_add(0x20)) as *const i32).read_unaligned();
        let c = (p20.wrapping_add(0xc) as *const i32).read_unaligned();
        if c == 0 {
            let v = ((param_1.wrapping_add(0x1c)) as *const i32).read_unaligned();
            FUN_1001c299(param_1, v);
        } else {
            let p20 = ((param_1.wrapping_add(0x20)) as *const i32).read_unaligned();
            let c = (p20.wrapping_add(0xc) as *const i32).read_unaligned();
            FUN_1001c299(param_1, c);
        }
    }
}
