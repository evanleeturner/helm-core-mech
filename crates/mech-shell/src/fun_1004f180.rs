// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1004f130(param_1: *mut i8) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004f180(param_1: *mut i8) {
    mech_trace::enter!("1004f180", param_1);
    let mut u_var2: u32 = 0xffff_ffff;
    let mut pc_var3: *mut i8 = param_1;
    // SAFETY: param_1 points at a NUL-terminated string owned by the caller;
    // the callee is the game's own.
    unsafe {
        loop {
            if u_var2 == 0 {
                break;
            }
            u_var2 = u_var2.wrapping_sub(1);
            let c_var1 = mech_trace::dr!("1004f180", pc_var3);
            pc_var3 = pc_var3.add(1);
            if c_var1 == 0 {
                break;
            }
        }
        let len = (!u_var2).wrapping_sub(1) as i32;
        let mut i_var4: i32 = 0;
        if 0 < len {
            loop {
                let p = param_1.byte_add(i_var4 as usize);
                i_var4 = i_var4.wrapping_add(0x50);
                mech_trace::cr!("1004f180", "1004f130", FUN_1004f130(p));
                if i_var4 >= len {
                    break;
                }
            }
        }
    }
}
