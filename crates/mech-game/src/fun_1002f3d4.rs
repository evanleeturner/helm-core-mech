// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002f3d4(param_1: i32, param_2: *mut u32) {
    mech_trace::enter!("1002f3d4", param_1, param_2);
    let mut p_var2 = param_1.wrapping_add(0xc) as *mut u32;
    let mut src = param_2;
    let mut i_var1: i32 = 0xc;
    // SAFETY: param_1 is the address of the caller's object and param_2 points
    // at 12 readable dwords; the copy and flag update follow the original.
    unsafe {
        while i_var1 != 0 {
            mech_trace::wr!(
                "1002f3d4",
                p_var2,
                write_unaligned,
                mech_trace::rd!("1002f3d4", src, read_unaligned)
            );
            src = src.add(1);
            p_var2 = p_var2.add(1);
            i_var1 = i_var1.wrapping_sub(1);
        }
        let flags = param_1.wrapping_add(0x74) as *mut u32;
        mech_trace::wr!(
            "1002f3d4",
            flags,
            write_unaligned,
            mech_trace::rd!("1002f3d4", flags, read_unaligned) | 1
        );
    }
}
