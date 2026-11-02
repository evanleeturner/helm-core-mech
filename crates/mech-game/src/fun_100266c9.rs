// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002637d(param_1: i32) -> u8;
    fn FUN_10026423(param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100266c9(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32) {
    // SAFETY: param_1 points at the caller's struct of at least 14 i32 words;
    // the callees are the game's own and take the struct address as an integer.
    unsafe {
        param_1.add(3).write_unaligned(param_2);
        param_1.add(4).write_unaligned(param_3);
        param_1.add(5).write_unaligned(param_4);
        param_1
            .add(6)
            .write_unaligned(param_2.wrapping_sub(param_1.read_unaligned()));
        param_1
            .add(7)
            .write_unaligned(param_3.wrapping_sub(param_1.add(1).read_unaligned()));
        param_1
            .add(8)
            .write_unaligned(param_4.wrapping_sub(param_1.add(2).read_unaligned()));
        let i_var1 = param_1.add(0xd).read_unaligned();
        param_1.add(0xd).write_unaligned(0);
        if i_var1 == 1 {
            FUN_1002637d(param_1 as i32);
        } else if i_var1 == 2 {
            FUN_10026423(param_1 as i32);
        } else {
            param_1.add(0xc).write_unaligned(0);
            let v = param_1.add(0xc).read_unaligned();
            param_1.add(0xb).write_unaligned(v);
            let v = param_1.add(0xb).read_unaligned();
            param_1.add(10).write_unaligned(v);
            let v = param_1.add(10).read_unaligned();
            param_1.add(9).write_unaligned(v);
        }
    }
}
