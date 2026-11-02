// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10018020(param_1: i32) -> u8;
    fn FUN_100707a0(
        param_1: u32,
        param_2: u32,
        param_3: u32,
        param_4: i32,
        param_5: *mut u8,
    ) -> u32;
    fn FUN_10018060(param_1: i32, param_2: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10017f30(param_1: i32, mut param_2: i32) -> u32 {
    if param_1 == 0 {
        return 0xffffffff;
    }
    // SAFETY: param_1 points at the game's own container structure; fields are
    // read at the original's byte offsets, and callees are the game's own.
    unsafe {
        let base = param_1 as usize as *const u8;
        if (base.byte_add(0xc).read() & 2) == 0 {
            let mut u_var1: u32 = 0;
            if 0 < base.byte_add(4).cast::<i32>().read_unaligned() {
                loop {
                    let mut local_8: i32 = FUN_10018060(param_1, u_var1 as i32) as i32;
                    let mut local_4: i32 = param_2;
                    let cb = base
                        .byte_add(0x19)
                        .cast::<Option<unsafe extern "C" fn(*mut i32, *mut i32) -> i32>>()
                        .read_unaligned();
                    match cb {
                        None => {
                            if local_8 == param_2 {
                                return u_var1;
                            }
                        }
                        Some(f) => {
                            let i_var2 = f(&raw mut local_8, &raw mut local_4);
                            if i_var2 == 0 {
                                return u_var1;
                            }
                        }
                    }
                    u_var1 = u_var1.wrapping_add(1);
                    if !((u_var1 as i32) < base.byte_add(4).cast::<i32>().read_unaligned()) {
                        break;
                    }
                }
            }
        } else {
            FUN_10018020(param_1);
            let u_var1 = FUN_100707a0(
                (&raw mut param_2) as usize as u32,
                base.byte_add(0x11).cast::<u32>().read_unaligned(),
                base.byte_add(4).cast::<u32>().read_unaligned(),
                4,
                base.byte_add(0x19).cast::<*mut u8>().read_unaligned(),
            );
            if u_var1 != 0 {
                return u_var1
                    .wrapping_sub(base.byte_add(0x11).cast::<i32>().read_unaligned() as u32)
                    >> 2;
            }
        }
    }
    0xffffffff
}
