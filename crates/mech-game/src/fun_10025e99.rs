// SPDX-License-Identifier: MIT

unsafe extern "C" {
    fn FUN_100267fd(dst: *mut core::ffi::c_void, src: *mut core::ffi::c_void);
    fn FUN_10025c83(param_1: *mut i32, param_2: i32, param_3: *mut i32) -> i32;
    fn FUN_1002634e(param_1: *mut i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10025e99(param_1: i32, param_2: i32, param_3: *mut i32) -> i32 {
    mech_trace::enter!("10025e99", param_1, param_2, param_3);
    let mut local_8: i32 = 0x7fffffff;
    let mut local_40: [i32; 14] = [0; 14];
    let mut local_78: [i32; 14] = [0; 14];
    let mut local_7c: i32 = 0;
    // SAFETY: the callees are the game's own; param_1 points at the caller's
    // structure holding four pointers at offset 0x1c, and param_3 at the
    // caller's buffer, copied where the original does.
    unsafe {
        FUN_100267fd(local_40.as_mut_ptr().cast(), param_3.cast());
        while local_7c < 4 {
            let p = ((param_1 as u32)
                .wrapping_add(0x1c)
                .wrapping_add((local_7c as u32).wrapping_mul(4)))
                as *const *mut i32;
            let i_var2 = mech_trace::cr!(
                "10025e99",
                "10025c83",
                FUN_10025c83(
                    mech_trace::rd!("10025e99", p, read_unaligned),
                    param_2,
                    local_40.as_mut_ptr()
                )
            );
            if i_var2 != 0 {
                let i_var2 =
                    mech_trace::cr!("10025e99", "1002634e", FUN_1002634e(local_40.as_mut_ptr()));
                if i_var2 < local_8 {
                    local_8 = i_var2;
                    FUN_100267fd(local_78.as_mut_ptr().cast(), local_40.as_mut_ptr().cast());
                }
                FUN_100267fd(local_40.as_mut_ptr().cast(), param_3.cast());
            }
            local_7c = local_7c.wrapping_add(1);
        }
        let b_var1 = (local_8 < 0x7fffffff) as i32;
        if b_var1 != 0 {
            FUN_100267fd(param_3.cast(), local_78.as_mut_ptr().cast());
        }
        mech_trace::rt!("10025e99", b_var1)
    }
}
