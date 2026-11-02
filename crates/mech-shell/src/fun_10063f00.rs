// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10063ee0(param_1: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10063f00(param_1: *mut i32, param_2: i32) {
    let mut p = param_1;
    // SAFETY: the callee is the game's own; param_1 points at an array of
    // 0xe-int entries terminated by a -1 first field, read and written as the original does.
    unsafe {
        FUN_10063ee0(p);
        let mut i_var2 = p.read_unaligned();
        while i_var2 != -1 {
            if p.add(9).read_unaligned() == param_2 {
                p.add(0xb).write_unaligned(1);
            }
            p = p.add(0xe);
            i_var2 = p.read_unaligned();
        }
    }
}
