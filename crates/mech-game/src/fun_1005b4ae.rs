// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005b4ae(param_1: i32, param_2: *mut u8) {
    mech_trace::enter!("1005b4ae", param_1, param_2);
    // SAFETY: param_1 is the address of the caller's GIF header buffer and
    // param_2 points at the palette output; both are read and written at the
    // same offsets as the original.
    unsafe {
        let mut pb_var2: *const u8 = (param_1 as usize).wrapping_add(0xd) as *const u8;
        let mut out: *mut u8 = param_2;
        let flags = mech_trace::dr!(
            "1005b4ae",
            ((param_1 as usize).wrapping_add(10) as *const u8)
        );
        if (flags & 0x80) != 0 {
            let mut i_var1: i32 =
                (1u32.wrapping_shl(((flags & 7) as u32).wrapping_add(1)) as i32).wrapping_mul(3);
            loop {
                let v = mech_trace::dr!("1005b4ae", pb_var2);
                pb_var2 = pb_var2.add(1);
                mech_trace::dw!("1005b4ae", out, v >> 2);
                i_var1 = i_var1.wrapping_sub(1);
                out = out.add(1);
                if i_var1 == 0 {
                    break;
                }
            }
        }
        let flags2 = mech_trace::dr!("1005b4ae", pb_var2.add(9));
        if (flags2 & 0x80) != 0 {
            let mut i_var1: i32 =
                (1u32.wrapping_shl(((flags2 & 7) as u32).wrapping_add(1)) as i32).wrapping_mul(3);
            pb_var2 = pb_var2.add(10);
            loop {
                mech_trace::dw!("1005b4ae", out, mech_trace::dr!("1005b4ae", pb_var2) >> 2);
                i_var1 = i_var1.wrapping_sub(1);
                pb_var2 = pb_var2.add(1);
                out = out.add(1);
                if i_var1 == 0 {
                    break;
                }
            }
        }
    }
}
