// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b0864: i32;
    static mut DAT_101155c0: [u8; 0];
    static mut DAT_101155dc: [u8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005031a(param_1: i32) {
    mech_trace::enter!("1005031a", param_1);
    let mut local_c: i32 = 0;
    // SAFETY: the globals are the game's own arrays; param_1 points at the
    // caller's structure; reads and writes happen where the original does.
    unsafe {
        while local_c < mech_trace::gr!("1005031a", "100b0864", DAT_100b0864) {
            let i_var1 = local_c.wrapping_mul(0x48);
            let p1 = (&raw mut DAT_101155c0)
                .cast::<u8>()
                .byte_add(i_var1 as usize)
                .cast::<i32>();
            if mech_trace::rd!("1005031a", p1, read_unaligned) == -2 {
                mech_trace::wr!(
                    "1005031a",
                    p1,
                    write_unaligned,
                    mech_trace::rd!(
                        "1005031a",
                        ((param_1.wrapping_add(4)) as *const i32),
                        read_unaligned
                    )
                );
                let p2 = (&raw mut DAT_101155dc)
                    .cast::<u8>()
                    .byte_add(i_var1 as usize)
                    .cast::<i32>();
                if mech_trace::rd!("1005031a", p2, read_unaligned) == -1 {
                    mech_trace::wr!("1005031a", p2, write_unaligned, -2);
                }
            }
            local_c = local_c.wrapping_add(1);
        }
    }
}
