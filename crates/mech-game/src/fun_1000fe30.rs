// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000ffd0(value: u32) -> u32;
    fn FUN_10003d60(value: u32, param_2: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000fe30(value: u32) -> u32 {
    mech_trace::enter!("1000fe30", value);
    // SAFETY: the callees are the game's own functions; they take and
    // return plain integers.
    unsafe {
        let mut root = mech_trace::cr!("1000fe30", "1000ffd0", FUN_1000ffd0(value));
        let mut lower = root >> 1;
        let mut upper = root.wrapping_mul(2);
        let mut tries_left: i32 = 100;
        loop {
            if tries_left == 0 {
                return mech_trace::rt!("1000fe30", root);
            }
            let u_var1 = mech_trace::cr!("1000fe30", "10003d60", FUN_10003d60(root, root));
            if u_var1 == value {
                break;
            }
            if value < u_var1 {
                upper = root;
            } else {
                lower = root;
            }
            let next = upper.wrapping_add(lower).wrapping_add(1) >> 1;
            tries_left = tries_left.wrapping_sub(1);
            if root == next {
                return mech_trace::rt!("1000fe30", next);
            }
            root = next;
        }
        mech_trace::rt!("1000fe30", root)
    }
}
