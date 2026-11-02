// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a40a8: i32;
    static mut _DAT_100b67f8: i32;
    static mut DAT_100cded0: i32;
    static mut DAT_100cded4: i32;
    static mut DAT_100cdef0: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1007c97d(param_1: i32);
    fn FUN_1002fcce(param_1: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1007be21(param_1: *mut i32, param_2: u32) -> i32 {
    let mut local_8: i32;
    if param_1.is_null() {
        return -1;
    }
    // SAFETY: the globals and callees are the game's own; the table is
    // indexed with the same element-scaled offsets as the original.
    unsafe {
        local_8 = 0;
        while local_8 < 0x80
            && (&raw mut DAT_100cded4)
                .wrapping_offset(local_8.wrapping_mul(0x24) as isize)
                .cast::<*mut i32>()
                .read_unaligned()
                != param_1
        {
            local_8 = local_8.wrapping_add(1);
        }
        if local_8 == 0x80 {
            local_8 = 0;
            while local_8 < 0x80
                && (&raw mut DAT_100cded4)
                    .wrapping_offset(local_8.wrapping_mul(0x24) as isize)
                    .read_unaligned()
                    != 0
            {
                local_8 = local_8.wrapping_add(1);
            }
        }
        if local_8 == 0x80 {
            local_8 = -1;
        } else {
            FUN_1007c97d(local_8);
            _DAT_100b67f8 = _DAT_100b67f8.wrapping_add(1);
            FUN_1002fcce(param_1);
            let i_var1 = local_8.wrapping_mul(0x24);
            (&raw mut DAT_100cded0)
                .wrapping_offset(i_var1 as isize)
                .cast::<u32>()
                .write_unaligned(param_2);
            (&raw mut DAT_100cded4)
                .wrapping_offset(i_var1 as isize)
                .cast::<*mut i32>()
                .write_unaligned(param_1);
            (&raw mut DAT_100cdef0)
                .wrapping_offset(i_var1 as isize)
                .write_unaligned(DAT_100a40a8.wrapping_neg());
        }
    }
    local_8
}
