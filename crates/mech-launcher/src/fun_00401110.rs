// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_000b0f00: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004010d0() -> u8;
    fn FUN_004010f0() -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00401110(param_1: *mut i8) {
    mech_trace::enter!("00401110", param_1);
    // SAFETY: the globals and callees are the game's own; param_1 points at a
    // NUL-terminated string, read as the original does, and the destination
    // buffer is the game's own memory written at every second byte.
    unsafe {
        let mut p_var3: *mut i8 = (&raw mut DAT_000b0f00).cast::<i8>();
        let mut u_var2: u32 = 0xffff_ffff;
        let mut p_var5: *const i8 = param_1;
        loop {
            if u_var2 == 0 {
                break;
            }
            u_var2 = u_var2.wrapping_sub(1);
            let c_var1 = mech_trace::dr!("00401110", p_var5);
            p_var5 = p_var5.add(1);
            if c_var1 == 0 {
                break;
            }
        }
        let mut i_var4: i32 = (!u_var2).wrapping_sub(1) as i32;
        if 0x50 < i_var4 {
            i_var4 = 0x50;
        }
        mech_trace::cr!("00401110", "004010d0", FUN_004010d0());
        mech_trace::cr!("00401110", "004010f0", FUN_004010f0());
        if 0 < i_var4 {
            let mut src: *const i8 = param_1;
            loop {
                let c_var1 = mech_trace::dr!("00401110", src);
                src = src.add(1);
                mech_trace::dw!("00401110", p_var3, c_var1);
                p_var3 = p_var3.add(2);
                i_var4 = i_var4.wrapping_sub(1);
                if i_var4 == 0 {
                    break;
                }
            }
        }
    }
}
