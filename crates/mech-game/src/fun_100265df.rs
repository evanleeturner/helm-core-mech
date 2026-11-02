// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10003d60(param_1: i32, param_2: i32) -> i32;
    fn FUN_1002637d(param_1: i32) -> u8;
    fn FUN_10026423(param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100265df(param_1: *mut i32, param_2: i32) {
    // SAFETY: param_1 points at the caller's record of at least 14 ints; the
    // callees are the game's own functions.
    unsafe {
        let i_var1 = param_1.add(0xd).read_unaligned();
        if i_var1 == 0 {
            FUN_1002637d(param_1 as i32);
        }
        if 0 < param_1.add(0xc).read_unaligned() {
            let i_var2 = FUN_10003d60(param_2, param_1.add(9).read_unaligned());
            param_1.write_unaligned(param_1.read_unaligned().wrapping_add(i_var2));
            let i_var2 = FUN_10003d60(param_2, param_1.add(10).read_unaligned());
            param_1
                .add(1)
                .write_unaligned(param_1.add(1).read_unaligned().wrapping_add(i_var2));
            let i_var2 = FUN_10003d60(param_2, param_1.add(0xb).read_unaligned());
            param_1
                .add(2)
                .write_unaligned(param_1.add(2).read_unaligned().wrapping_add(i_var2));
            param_1.add(6).write_unaligned(
                param_1
                    .add(3)
                    .read_unaligned()
                    .wrapping_sub(param_1.read_unaligned()),
            );
            param_1.add(7).write_unaligned(
                param_1
                    .add(4)
                    .read_unaligned()
                    .wrapping_sub(param_1.add(1).read_unaligned()),
            );
            param_1.add(8).write_unaligned(
                param_1
                    .add(5)
                    .read_unaligned()
                    .wrapping_sub(param_1.add(2).read_unaligned()),
            );
            param_1.add(0xd).write_unaligned(0);
            if i_var1 == 2 {
                FUN_10026423(param_1 as i32);
            } else {
                FUN_1002637d(param_1 as i32);
            }
        }
    }
}
