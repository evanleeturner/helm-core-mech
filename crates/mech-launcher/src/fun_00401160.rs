// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00401110(param_1: *mut i8) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00401160(param_1: *mut i8) {
    mech_trace::enter!("00401160", param_1);
    let mut u_var2: u32 = 0xffff_ffff;
    let mut pc_var3: *mut i8 = param_1;
    loop {
        if u_var2 == 0 {
            break;
        }
        u_var2 = u_var2.wrapping_sub(1);
        // SAFETY: param_1 points at a NUL-terminated string owned by the caller.
        let c_var1 = unsafe { mech_trace::rd!("00401160", pc_var3, read) };
        pc_var3 = pc_var3.wrapping_add(1);
        if c_var1 == 0 {
            break;
        }
    }
    let len = (!u_var2).wrapping_sub(1) as i32;
    let mut i_var4: i32 = 0;
    if 0 < len {
        loop {
            let pc = param_1.wrapping_offset(i_var4 as isize);
            i_var4 = i_var4.wrapping_add(0x50);
            // SAFETY: the callee is the game's own; pc points within the caller's string.
            unsafe {
                mech_trace::cr!("00401160", "00401110", FUN_00401110(pc));
            }
            if i_var4 >= len {
                break;
            }
        }
    }
}
