// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_000b0f00: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1004f0f0();
    fn FUN_1004f110();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004f130(param_1: *mut i8) {
    mech_trace::enter!("1004f130", param_1);
    // SAFETY: the globals and callees are the game's own; param_1 points at
    // a NUL-terminated string, read as the original does, and the destination
    // buffer is the game's own memory, written at every second byte.
    unsafe {
        let mut pc_var3: *mut i8 = (&raw mut DAT_000b0f00).cast::<i8>();
        let mut u_var2: u32 = 0xffff_ffff;
        let mut pc_var5: *const i8 = param_1;
        loop {
            if u_var2 == 0 {
                break;
            }
            u_var2 = u_var2.wrapping_sub(1);
            let c_var1 = mech_trace::rd!("1004f130", pc_var5, read);
            pc_var5 = pc_var5.add(1);
            if c_var1 == 0 {
                break;
            }
        }
        let mut i_var4: i32 = (!u_var2).wrapping_sub(1) as i32;
        if 0x50 < i_var4 {
            i_var4 = 0x50;
        }
        FUN_1004f0f0();
        FUN_1004f110();
        if 0 < i_var4 {
            let mut src: *const i8 = param_1;
            loop {
                let c_var1 = mech_trace::rd!("1004f130", src, read);
                src = src.add(1);
                mech_trace::wr!("1004f130", pc_var3, write, c_var1);
                pc_var3 = pc_var3.add(2);
                i_var4 = i_var4.wrapping_sub(1);
                if i_var4 == 0 {
                    break;
                }
            }
        }
    }
}
