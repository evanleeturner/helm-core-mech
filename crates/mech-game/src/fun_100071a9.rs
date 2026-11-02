// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000712c(param_1: *mut i8) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100071a9(param_1: *mut i8) {
    mech_trace::enter!("100071a9", param_1);
    let mut u_var2: u32 = 0xffffffff;
    let mut pc_var3: *mut i8 = param_1;
    // SAFETY: param_1 is a NUL-terminated string owned by the caller; the
    // callee is the game's own function.
    unsafe {
        loop {
            if u_var2 == 0 {
                break;
            }
            u_var2 = u_var2.wrapping_sub(1);
            let c_var1 = mech_trace::rd!("100071a9", pc_var3, read);
            pc_var3 = pc_var3.wrapping_add(1);
            if c_var1 == 0 {
                break;
            }
        }
        let len = (!u_var2).wrapping_sub(1) as i32;
        let mut local_8: i32 = 0;
        while local_8 < len {
            mech_trace::cr!(
                "100071a9",
                "1000712c",
                FUN_1000712c(param_1.byte_add(local_8 as usize))
            );
            local_8 = local_8.wrapping_add(0x50);
        }
    }
}
