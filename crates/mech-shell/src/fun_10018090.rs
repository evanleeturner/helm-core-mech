// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10017df0(param_1: *mut i32, param_2: u32) -> u32;
    fn FUN_100180f0(param_1: *mut i32, param_2: i8) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10018090(param_1: *mut i32, param_2: *mut i32) {
    mech_trace::enter!("10018090", param_1, param_2);
    let mut i_var1: i32 = 0;
    if !param_1.is_null() && !param_2.is_null() {
        // SAFETY: param_1 and param_2 point at the caller's collections; the
        // offsets read and written are those of the original, and the callees
        // are the game's own.
        unsafe {
            if 0 < mech_trace::rd!("10018090", param_2.add(1), read_unaligned) {
                let mut i_var2: i32 = 0;
                loop {
                    i_var2 = i_var2.wrapping_add(4);
                    i_var1 = i_var1.wrapping_add(1);
                    let base = mech_trace::rd!(
                        "10018090",
                        param_2.byte_add(0x11).cast::<i32>(),
                        read_unaligned
                    );
                    let p = (base.wrapping_sub(4).wrapping_add(i_var2)) as *mut u32;
                    mech_trace::cr!(
                        "10018090",
                        "10017df0",
                        FUN_10017df0(param_1, mech_trace::rd!("10018090", p, read_unaligned))
                    );
                    let base = mech_trace::rd!(
                        "10018090",
                        param_2.byte_add(0x11).cast::<i32>(),
                        read_unaligned
                    );
                    let p = (base.wrapping_sub(4).wrapping_add(i_var2)) as *mut u32;
                    mech_trace::wr!("10018090", p, write_unaligned, 0);
                    if i_var1 >= mech_trace::rd!("10018090", param_2.add(1), read_unaligned) {
                        break;
                    }
                }
            }
            mech_trace::cr!("10018090", "100180f0", FUN_100180f0(param_2, 1));
        }
    }
}
