// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b0424: [u8; 0];
    static mut DAT_100b043c: [u16; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10009f90(param_1: u32, param_2: i32, param_3: *mut i32) -> u32 {
    mech_trace::enter!("10009f90", param_1, param_2, param_3);
    let mut i_var1: i32 = 0;
    // SAFETY: the globals are the game's own tables; reads happen at the
    // same offsets and in the same order as the original.
    unsafe {
        let cnt_ptr = (&raw mut DAT_100b043c)
            .cast::<u8>()
            .byte_offset(param_2.wrapping_mul(0x28) as isize)
            .cast::<u16>();
        if 0 < (mech_trace::rd!("10009f90", cnt_ptr, read_unaligned) as i16) {
            let mut pu_var2 = (&raw mut DAT_100b0424)
                .cast::<u8>()
                .byte_offset(param_2.wrapping_mul(0x14) as isize)
                .cast::<u16>();
            loop {
                if u32::from(mech_trace::rd!("10009f90", pu_var2, read_unaligned)) == param_1 {
                    if !param_3.is_null() {
                        mech_trace::wr!("10009f90", param_3, write_unaligned, i_var1);
                    }
                    return mech_trace::rt!("10009f90", 1);
                }
                pu_var2 = pu_var2.byte_add(2);
                i_var1 = i_var1.wrapping_add(1);
                if !(i_var1 < (mech_trace::rd!("10009f90", cnt_ptr, read_unaligned) as i16 as i32))
                {
                    break;
                }
            }
        }
    }
    mech_trace::rt!("10009f90", 0)
}
