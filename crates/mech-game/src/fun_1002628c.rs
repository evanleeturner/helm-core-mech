// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10003d60(segment: i32, param_2: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002628c(
    segment: *mut i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
    param_7: i32,
    length: i32,
) {
    // SAFETY: segment points at the caller's array of at least 14 ints; the
    // callee is the game's own function.
    unsafe {
        segment.write_unaligned(param_2);
        segment.add(1).write_unaligned(param_3);
        segment.add(2).write_unaligned(param_4);
        segment.add(9).write_unaligned(param_5);
        segment.add(10).write_unaligned(param_6);
        segment.add(11).write_unaligned(param_7);
        let i_var1 = FUN_10003d60(length, param_5);
        segment.add(6).write_unaligned(i_var1);
        let i_var1 = FUN_10003d60(length, param_6);
        segment.add(7).write_unaligned(i_var1);
        let i_var1 = FUN_10003d60(length, param_7);
        segment.add(8).write_unaligned(i_var1);
        segment
            .add(3)
            .write_unaligned(segment.add(6).read_unaligned().wrapping_add(param_2));
        segment
            .add(4)
            .write_unaligned(segment.add(7).read_unaligned().wrapping_add(param_3));
        segment
            .add(5)
            .write_unaligned(segment.add(8).read_unaligned().wrapping_add(param_4));
        segment.add(0xc).write_unaligned(length);
        segment.add(0xd).write_unaligned(1);
    }
}
