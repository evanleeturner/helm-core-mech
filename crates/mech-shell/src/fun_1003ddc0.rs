// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100d0970: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn __strcmpi(_Str1: *mut i8, _Str2: *mut i8) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003ddc0(str2: *mut i8, param_2: i32) -> i32 {
    mech_trace::enter!("1003ddc0", str2, param_2);
    let mut i_var2: i32 = 0;
    if 0 < param_2 {
        // SAFETY: the table and the compare routine are the game's own; the
        // table is walked in 0xf-byte entries as the original does.
        unsafe {
            let mut str1: *mut i8 = (&raw mut DAT_100d0970).cast::<i8>();
            loop {
                let i_var1 = mech_trace::cr!("1003ddc0", "__strcmpi", __strcmpi(str1, str2));
                if i_var1 == 0 {
                    return mech_trace::rt!("1003ddc0", i_var2);
                }
                str1 = str1.byte_add(0xf);
                i_var2 = i_var2.wrapping_add(1);
                if i_var2 >= param_2 {
                    break;
                }
            }
        }
    }
    mech_trace::rt!("1003ddc0", -1)
}
