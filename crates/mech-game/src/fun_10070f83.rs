// SPDX-License-Identifier: MIT

unsafe extern "C" {
    fn __strcmpi(str1: *const i8, str2: *const i8) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10070f83(param_1: i32, str1: *mut i8) -> i32 {
    let mut local_8: i32 = 0;
    // SAFETY: param_1 is the address of the game's own table structure; the
    // count and entries are read where the original reads them, and the
    // string compare is the C runtime's.
    unsafe {
        while local_8 < ((param_1 as u32).wrapping_add(0x4b4) as *const i32).read_unaligned() {
            let entry = (param_1 as u32)
                .wrapping_add(4)
                .wrapping_add((local_8 as u32).wrapping_mul(0xc))
                as *const *mut i8;
            let i_var1 = __strcmpi(str1, entry.read_unaligned());
            if i_var1 == 0 {
                break;
            }
            local_8 = local_8.wrapping_add(1);
        }
        if ((param_1 as u32).wrapping_add(0x4b4) as *const i32).read_unaligned() == local_8 {
            local_8 = -1;
        }
    }
    local_8
}
