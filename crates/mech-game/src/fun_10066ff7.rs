// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aaf68: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10006b9a(param_1: i32) -> u8;
    fn FUN_10006b20(param_1: i32) -> i32;
    fn FUN_10003d60(param_1: i32, param_2: i32) -> i32;
    fn FUN_10066ec0(param_1: i32, param_2: u32, param_3: *mut u32) -> i32;
    fn FUN_10067126(param_1: *mut i32, param_2: u32, param_3: i32, param_4: *mut i32) -> *mut i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10066ff7(param_1: i32, param_2: i32, param_3: *mut i32) -> *mut i32 {
    mech_trace::enter!("10066ff7", param_1, param_2, param_3);
    let mut local_c: u32 = 0;
    let mut local_8: u32;
    let mut i_var3: i32 = (param_2 % 0x1680000).wrapping_add(0x1680000) % 0x1680000;
    // SAFETY: param_1 points at the caller's rect of i32 fields; globals and
    // callees are the game's own; local_c and the local array outlive the calls.
    unsafe {
        let a = mech_trace::rd!(
            "10066ff7",
            (param_1.wrapping_add(0xc) as *const i32),
            read_unaligned
        );
        let b = mech_trace::rd!(
            "10066ff7",
            (param_1.wrapping_add(4) as *const i32),
            read_unaligned
        );
        let local_14 = (a.wrapping_sub(b).wrapping_add(1)) >> 1;
        let c = mech_trace::rd!(
            "10066ff7",
            (param_1.wrapping_add(0x10) as *const i32),
            read_unaligned
        );
        let d = mech_trace::rd!(
            "10066ff7",
            (param_1.wrapping_add(8) as *const i32),
            read_unaligned
        );
        let local_10 = (c.wrapping_sub(d).wrapping_add(1)) >> 1;
        let mut arr: [i32; 2] = [local_14, local_10];
        if i_var3 < 0x5a0000 || 0x10e0000 < i_var3 {
            local_8 = 0;
        } else {
            local_8 = 1;
        }
        if i_var3 < 0xb40001 {
            local_8 |= 2;
        }
        let i_var1 = mech_trace::cr!("10066ff7", "10006b9a", FUN_10006b9a(i_var3)) as i32;
        i_var3 = mech_trace::cr!("10066ff7", "10006b20", FUN_10006b20(i_var3));
        let ptr = (mech_trace::gr!("10066ff7", "100aaf68", DAT_100aaf68) as usize)
            .wrapping_add(0x44) as *const i32;
        let u_var2 = mech_trace::cr!(
            "10066ff7",
            "10003d60",
            FUN_10003d60(
                i_var3 >> 0xd,
                mech_trace::rd!("10066ff7", ptr, read_unaligned)
            )
        ) as u32;
        let u_var2 = mech_trace::cr!(
            "10066ff7",
            "10066ec0",
            FUN_10066ec0(i_var1 >> 0xd, u_var2, &raw mut local_c)
        ) as u32;
        local_8 = ((u_var2 & 1) << 2) | (local_8 & 0xfffffffb);
        let neg = (local_c as i32).wrapping_neg();
        mech_trace::cr!(
            "10066ff7",
            "10067126",
            FUN_10067126(arr.as_mut_ptr(), local_8, neg, param_3)
        );
    }
    mech_trace::rt!("10066ff7", param_3)
}
