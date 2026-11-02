// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100258ea(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
    param_7: *mut i32,
    param_8: *mut i32,
) -> u32 {
    let mut local_34: i32 = 0x7fffffff;
    let mut local_30: i32 = 0x7fffffff;
    let mut local_14: i32 = 0x7fffffff;
    let mut local_10: i32 = -0x7fffffff;
    let mut local_c: i32 = -0x7fffffff;
    let mut local_38: i32 = -0x7fffffff;
    let mut local_28: i32 = 0;
    // SAFETY: param_1 and param_2 point at the game's own structures; reads
    // happen at the same offsets as the original, and param_7/param_8 are
    // caller-provided output slots.
    unsafe {
        loop {
            let count = ((param_1.wrapping_add(2)) as *const u16).read_unaligned() as i32;
            if local_28 >= count {
                break;
            }
            let base = ((param_1.wrapping_add(4)) as *const u32).read_unaligned();
            let addr = base
                .wrapping_add(local_28 as u32)
                .wrapping_add(param_1 as u32);
            let b = (addr as usize as *const u8).read() as u32;
            let i_var3 = b
                .wrapping_mul(0x44)
                .wrapping_add(param_2 as u32)
                .wrapping_add(0x18);
            let i_var1 = (i_var3.wrapping_add(0xc) as usize as *const i32).read_unaligned();
            let i_var2 = (i_var3.wrapping_add(0x10) as usize as *const i32).read_unaligned();
            let i_var3 = (i_var3.wrapping_add(0x14) as usize as *const i32).read_unaligned();
            if i_var1 < local_14 {
                local_14 = i_var1;
            }
            if i_var2 < local_30 {
                local_30 = i_var2;
            }
            if i_var3 < local_34 {
                local_34 = i_var3;
            }
            if local_38 < i_var1 {
                local_38 = i_var1;
            }
            if local_c < i_var2 {
                local_c = i_var2;
            }
            if local_10 < i_var3 {
                local_10 = i_var3;
            }
            local_28 = local_28.wrapping_add(1);
        }
        if param_4 < local_14 || local_38 < param_3 || param_6 < local_34 || local_10 < param_5 {
            0
        } else {
            param_7.write_unaligned(local_30);
            param_8.write_unaligned(local_c);
            1
        }
    }
}
