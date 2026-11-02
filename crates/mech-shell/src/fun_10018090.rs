// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10017df0(param_1: *mut i32, param_2: u32) -> u32;
    fn FUN_100180f0(param_1: *mut i32, param_2: i8) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10018090(param_1: *mut i32, param_2: *mut i32) {
    let mut i_var1: i32 = 0;
    if !param_1.is_null() && !param_2.is_null() {
        // SAFETY: param_1 and param_2 point at the caller's collections; the
        // offsets read and written are those of the original, and the callees
        // are the game's own.
        unsafe {
            if 0 < param_2.add(1).read_unaligned() {
                let mut i_var2: i32 = 0;
                loop {
                    i_var2 = i_var2.wrapping_add(4);
                    i_var1 = i_var1.wrapping_add(1);
                    let base = param_2.byte_add(0x11).cast::<i32>().read_unaligned();
                    let p = (base.wrapping_sub(4).wrapping_add(i_var2)) as *mut u32;
                    FUN_10017df0(param_1, p.read_unaligned());
                    let base = param_2.byte_add(0x11).cast::<i32>().read_unaligned();
                    let p = (base.wrapping_sub(4).wrapping_add(i_var2)) as *mut u32;
                    p.write_unaligned(0);
                    if i_var1 >= param_2.add(1).read_unaligned() {
                        break;
                    }
                }
            }
            FUN_100180f0(param_2, 1);
        }
    }
}
