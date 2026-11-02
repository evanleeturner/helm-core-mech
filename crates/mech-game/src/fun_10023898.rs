// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10023898(param_1: i32, param_2: i32) -> u32 {
    // SAFETY: both params are addresses of game structures; reads happen
    // at the same offsets and in the same order as the original.
    unsafe {
        let p2_20 = ((param_2.wrapping_add(0x20)) as *const i32).read_unaligned();
        let p1_60 = ((param_1.wrapping_add(0x60)) as *const i32).read_unaligned();
        if p2_20 < p1_60 {
            let idx = ((param_2.wrapping_add(0x20)) as *const i32).read_unaligned();
            let addr = param_1
                .wrapping_add(0x66)
                .wrapping_add(idx.wrapping_mul(0x3a));
            if (addr as *const i32).read_unaligned() != 0 {
                return 1;
            }
        }
        let a = ((param_2.wrapping_add(4)) as *const i32).read_unaligned();
        if 0x13 < a {
            let b = ((param_2.wrapping_add(4)) as *const i32).read_unaligned();
            if b < 0x1c {
                return 1;
            }
        }
        let c = ((param_2.wrapping_add(4)) as *const i32).read_unaligned();
        if 0x32 < c {
            let d = ((param_2.wrapping_add(4)) as *const i32).read_unaligned();
            if d < 0x3d {
                return 1;
            }
        }
        0
    }
}
