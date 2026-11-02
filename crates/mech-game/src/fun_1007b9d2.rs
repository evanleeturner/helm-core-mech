// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ab2d4: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1007b890(param_1: u32, param_2: *mut i8) -> i32;
    fn FUN_1007bac8(param_1: u32) -> *mut u8;
    fn FUN_10070f83(param_1: i32, param_2: *mut i8) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1007b9d2(param_1: u32, param_2: *mut i8) -> i32 {
    mech_trace::enter!("1007b9d2", param_1, param_2);
    let mut local_84: [i8; 128] = [0; 128];
    // SAFETY: the callees and the global are the game's own; param_2 is a
    // NUL-terminated string from the caller, and the copies write into
    // local_84 exactly where the original does.
    unsafe {
        let mut i_var2 = mech_trace::cr!("1007b9d2", "1007b890", FUN_1007b890(param_1, param_2));
        if i_var2 == -1 {
            if mech_trace::gr!("1007b9d2", "100ab2d4", DAT_100ab2d4) == 0 {
                i_var2 = -1;
            } else {
                // strcpy(local_84, param_2)
                let mut len: usize = 0;
                while mech_trace::dr!("1007b9d2", param_2.add(len)) != 0 {
                    len += 1;
                }
                len += 1;
                let dst = local_84.as_mut_ptr();
                let mut k = 0usize;
                while k < len {
                    mech_trace::dw!(
                        "1007b9d2",
                        dst.add(k),
                        mech_trace::dr!("1007b9d2", param_2.add(k))
                    );
                    k += 1;
                }
                let src = mech_trace::cr!("1007b9d2", "1007bac8", FUN_1007bac8(param_1));
                let mut slen: usize = 0;
                while mech_trace::dr!("1007b9d2", src.add(slen)) != 0 {
                    slen += 1;
                }
                slen += 1;
                let mut end: usize = 0;
                while mech_trace::dr!("1007b9d2", dst.add(end)) != 0 {
                    end += 1;
                }
                let mut k = 0usize;
                while k < slen {
                    mech_trace::dw!(
                        "1007b9d2",
                        dst.add(end + k),
                        mech_trace::dr!("1007b9d2", src.add(k)) as i8
                    );
                    k += 1;
                }
                i_var2 = mech_trace::cr!(
                    "1007b9d2",
                    "10070f83",
                    FUN_10070f83(
                        mech_trace::gr!("1007b9d2", "100ab2d4", DAT_100ab2d4) as i32,
                        dst
                    )
                );
                if i_var2 == -1 {
                    i_var2 = -1;
                } else {
                    i_var2 = i_var2.wrapping_add(0xf000);
                }
            }
        }
        mech_trace::rt!("1007b9d2", i_var2)
    }
}
