// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000a800(param_1: i32) -> u8;
    fn FUN_1000a890(param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10032a90(param_1: i32) -> i32 {
    // SAFETY: param_1 is the address of the game's own structure; fields are
    // read and written at the same offsets as the original, and the callees
    // are the game's own functions.
    unsafe {
        let base = param_1 as usize;
        let limit = ((base + 0x280) as *const i32).read_unaligned();
        let cur = ((base + 0x288) as *const i32).read_unaligned();
        if limit <= cur {
            return 0;
        }
        let cur = ((base + 0x288) as *const i32).read_unaligned();
        ((base + 0x288) as *mut i32).write_unaligned(cur.wrapping_add(1));
        let mut i_var3: i32 = 0;
        'outer: loop {
            let i_var4 = i_var3;
            i_var3 = i_var4.wrapping_add(1);
            let mut local_8: i32 = 0;
            let mut local_c: usize = base + 0x368;
            loop {
                let mut i_var1: i32 = 0;
                let mut p = local_c;
                loop {
                    let v = (p as *const i32).read_unaligned();
                    if (v / 100)
                        .wrapping_mul(4)
                        .wrapping_sub(0x154)
                        .wrapping_mul(0x19)
                        == 0
                        && v % 100 == i_var3
                    {
                        continue 'outer;
                    }
                    p = p.wrapping_add(4);
                    i_var1 = i_var1.wrapping_add(1);
                    if i_var1 >= 0xc {
                        break;
                    }
                }
                local_c = local_c.wrapping_add(0x30);
                local_8 = local_8.wrapping_add(1);
                if 7 < local_8 {
                    FUN_1000a800(param_1);
                    FUN_1000a890(param_1);
                    return i_var4.wrapping_add(0x2135);
                }
            }
        }
    }
}
