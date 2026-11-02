// SPDX-License-Identifier: MIT

unsafe extern "C" {
    #[link_name = "_strcmpi"]
    fn __strcmpi(_Str1: *const i8, _Str2: *const i8) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002feb2(param_1: i32, str2: *mut i8) -> i32 {
    // SAFETY: param_1 and the linked nodes are the game's own structures;
    // fields are read at the same offsets and order as the original.
    unsafe {
        let name = ((param_1 as u32).wrapping_add(0x70) as *const i32).read_unaligned();
        if name == 0 || __strcmpi(name as *const i8, str2) != 0 {
            let mut local_8 = ((param_1 as u32).wrapping_add(4) as *const i32).read_unaligned();
            while local_8 != 0 {
                let i_var1 = FUN_1002feb2(local_8, str2);
                if i_var1 != 0 {
                    return i_var1;
                }
                local_8 = ((local_8 as u32).wrapping_add(8) as *const i32).read_unaligned();
            }
            return 0;
        }
        param_1
    }
}
