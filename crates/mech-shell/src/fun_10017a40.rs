// SPDX-License-Identifier: MIT

unsafe extern "C" {
    #[link_name = "_strcmpi"]
    fn __strcmpi(_str1: *const i8, _str2: *const i8) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10017a40(param_1: i32, str1: *mut i8) -> i32 {
    let mut i_var2: i32 = 0;
    // SAFETY: param_1 points at the game's table structure; entries are read
    // as the original does, and the string compare is the CRT's own.
    unsafe {
        let count_ptr = (param_1 as usize).wrapping_add(0x4b4) as *const i32;
        let mut b_var4 = count_ptr.read_unaligned() == 0;
        if 0 < count_ptr.read_unaligned() {
            let mut pu_var3 = (param_1 as usize).wrapping_add(4) as *const u32;
            loop {
                let s2 = pu_var3.read_unaligned() as *const i8;
                let i_var1 = __strcmpi(str1, s2);
                if i_var1 == 0 {
                    break;
                }
                pu_var3 = pu_var3.add(3);
                i_var2 = i_var2.wrapping_add(1);
                if !(i_var2 < count_ptr.read_unaligned()) {
                    break;
                }
            }
            b_var4 = count_ptr.read_unaligned() == i_var2;
        }
        let mut i_var1 = -1;
        if !b_var4 {
            i_var1 = i_var2;
        }
        i_var1
    }
}
