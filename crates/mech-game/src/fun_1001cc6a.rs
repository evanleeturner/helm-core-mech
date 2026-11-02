// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1001cfba(param_1: i32, param_2: i32);
    fn FUN_1001cb1e(param_1: *mut u8, param_2: *mut u32, param_3: *mut u32, param_4: *mut u32);
    fn FUN_1001cd78(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001cc6a(param_1: i32) {
    // SAFETY: param_1 points at the game's structure; all offsets are read
    // and written where the original does, and callees are the game's own.
    unsafe {
        let i_var2 = ((param_1.wrapping_add(0x20)) as *const i32).read_unaligned();
        if i_var2 != 0 {
            let mut local_10: i32 =
                ((i_var2.wrapping_add(6)) as *const i16).read_unaligned() as i32;
            loop {
                let i_var1 = local_10.wrapping_sub(1);
                if local_10 == 0 {
                    break;
                }
                let i_var3 = ((i_var2.wrapping_add(8)) as *const i32)
                    .read_unaligned()
                    .wrapping_add(i_var2)
                    .wrapping_add(i_var1.wrapping_mul(0x24));
                FUN_1001cfba(i_var3, i_var2.wrapping_add(0x18));
                let mut local_14: u32 =
                    ((i_var3.wrapping_add(2)) as *const u16).read_unaligned() as u32;
                loop {
                    local_10 = i_var1;
                    if local_14 == 0 {
                        break;
                    }
                    let idx_addr = ((i_var3.wrapping_add(4)) as *const i32)
                        .read_unaligned()
                        .wrapping_add(local_14.wrapping_sub(1) as i32)
                        .wrapping_add(i_var3);
                    let b = (idx_addr as *const u8).read() as u32;
                    let p_this = b
                        .wrapping_mul(0x44)
                        .wrapping_add(i_var2 as u32)
                        .wrapping_add(0x18) as i32;
                    let p2c = p_this.wrapping_add(0x2c) as *mut u32;
                    let p30 = p_this.wrapping_add(0x30) as *mut u32;
                    let p34 = p_this.wrapping_add(0x34) as *mut u32;
                    p2c.write_unaligned(
                        p2c.read_unaligned().wrapping_add(
                            ((i_var3.wrapping_add(8)) as *const u32).read_unaligned(),
                        ),
                    );
                    p30.write_unaligned(
                        p30.read_unaligned().wrapping_add(
                            ((i_var3.wrapping_add(0xc)) as *const u32).read_unaligned(),
                        ),
                    );
                    p34.write_unaligned(p34.read_unaligned().wrapping_add(
                        ((i_var3.wrapping_add(0x10)) as *const u32).read_unaligned(),
                    ));
                    FUN_1001cb1e(p_this as *mut u8, p2c, p30, p34);
                    local_14 = local_14.wrapping_sub(1);
                }
            }
            FUN_1001cd78(param_1);
        }
    }
}
