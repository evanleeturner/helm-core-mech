// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100261f0(
    param_1: *mut i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
    param_7: i32,
) {
    // SAFETY: param_1 points at the caller's buffer of at least 14 ints,
    // written in the same order as the original.
    unsafe {
        param_1.write_unaligned(param_2);
        param_1.add(1).write_unaligned(param_3);
        param_1.add(2).write_unaligned(param_4);
        param_1.add(3).write_unaligned(param_5);
        param_1.add(4).write_unaligned(param_6);
        param_1.add(5).write_unaligned(param_7);
        param_1
            .add(6)
            .write_unaligned((param_5 as u32).wrapping_sub(param_2 as u32) as i32);
        param_1
            .add(7)
            .write_unaligned((param_6 as u32).wrapping_sub(param_3 as u32) as i32);
        param_1
            .add(8)
            .write_unaligned((param_7 as u32).wrapping_sub(param_4 as u32) as i32);
        param_1.add(0xc).write_unaligned(0);
        let v = param_1.add(0xc).read_unaligned();
        param_1.add(0xb).write_unaligned(v);
        let v = param_1.add(0xb).read_unaligned();
        param_1.add(10).write_unaligned(v);
        let v = param_1.add(10).read_unaligned();
        param_1.add(9).write_unaligned(v);
        param_1.add(0xd).write_unaligned(0);
    }
}
