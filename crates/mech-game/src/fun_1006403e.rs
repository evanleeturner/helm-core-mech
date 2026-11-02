// SPDX-License-Identifier: MIT

unsafe extern "C" {}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002f2d6(
        param_1: *mut i32,
        param_2: i32,
        param_3: *mut i32,
        param_4: *mut i32,
        param_5: *mut i32,
    );
    fn FUN_10089cc9(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
        param_7: i32,
    );
    fn FUN_10089118(param_1: *mut i32, param_2: *mut i32, param_3: *mut i32, param_4: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1006403e(
    param_1: i32,
    param_2: *mut i32,
    param_3: *mut i32,
    param_4: *mut i32,
) {
    mech_trace::enter!("1006403e", param_1, param_2, param_3, param_4);
    let mut local_44: [i32; 12] = [0; 12];
    let mut local_14: i32 = 0;
    let mut local_10: i32 = 0;
    let local_c: i32;
    let mut local_8: i32 = 0;

    // SAFETY: the pointers are the caller's outputs and param_1 points at the
    // game's own object; the callees are the game's own functions.
    unsafe {
        mech_trace::wr!("1006403e", param_3, write_unaligned, 0);
        mech_trace::wr!(
            "1006403e",
            param_2,
            write_unaligned,
            mech_trace::rd!("1006403e", param_3, read_unaligned)
        );
        mech_trace::wr!("1006403e", param_4, write_unaligned, 0x10000);
        FUN_1002f2d6(
            param_2,
            mech_trace::rd!(
                "1006403e",
                ((param_1 as u32).wrapping_add(0x44) as *const i32),
                read_unaligned
            ),
            &raw mut local_8,
            &raw mut local_10,
            &raw mut local_14,
        );
        local_c = mech_trace::rd!(
            "1006403e",
            ((param_1 as u32).wrapping_add(0x20) as *const i32),
            read_unaligned
        );
        let flags = mech_trace::rd!(
            "1006403e",
            ((local_c as u32).wrapping_add(0x442) as *const u16),
            read_unaligned
        );
        let base = mech_trace::rd!(
            "1006403e",
            ((local_c as u32).wrapping_add(0x1c) as *const i32),
            read_unaligned
        );
        if (flags & 4) == 0 {
            local_8 = base;
        } else {
            local_8 = local_8.wrapping_add(base);
        }
        FUN_10089cc9(local_44.as_mut_ptr(), local_8, local_10, local_14, 0, 0, 0);
        FUN_10089118(local_44.as_mut_ptr(), param_2, param_3, param_4);
    }
}
