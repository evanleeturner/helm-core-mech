// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10006d3e() -> i32;
    fn FUN_10056489(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
        param_7: i32,
    ) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10009896(param_1: *mut i32, param_2: i32, param_3: i32) {
    // SAFETY: param_1 and param_2 point at the game's own records; the reads
    // and the calls happen in the original's order.
    unsafe {
        let i_var1 = (param_2.wrapping_add(0x74) as *const i32).read_unaligned();
        let i_var2 = (((param_1.add(3).read_unaligned() as u32)
            .wrapping_sub(param_1.add(1).read_unaligned() as u32))
        .wrapping_add(1) as i32)
            >> 1;
        let i_var3 = (((param_1.add(4).read_unaligned() as u32)
            .wrapping_sub(param_1.add(2).read_unaligned() as u32))
        .wrapping_add(1) as i32)
            >> 1;
        let local_8 = FUN_10006d3e();
        let i_var4 =
            0x5a0000i32.wrapping_sub((param_3 % 0x1680000).wrapping_add(0x1680000) % 0x1680000);
        let cb_ptr = param_2.wrapping_add(0x88) as *const u32;
        if cb_ptr.read_unaligned() != 0 {
            let cb = (param_2.wrapping_add(0x88)
                as *const Option<unsafe extern "C" fn(*mut i32, i32, *mut i32)>)
                .read_unaligned()
                .unwrap_unchecked();
            let mut local_10: [i32; 2] = [0; 2];
            cb(param_1, i_var4.wrapping_sub(local_8), local_10.as_mut_ptr());
            FUN_10056489(
                param_1,
                i_var2,
                i_var3,
                local_10[0],
                local_10[1],
                0,
                ((i_var1.wrapping_add(0x2c)) as *const i32).read_unaligned(),
            );
            let cb = (param_2.wrapping_add(0x88)
                as *const Option<unsafe extern "C" fn(*mut i32, i32, *mut i32)>)
                .read_unaligned()
                .unwrap_unchecked();
            cb(param_1, i_var4.wrapping_add(local_8), local_10.as_mut_ptr());
            FUN_10056489(
                param_1,
                i_var2,
                i_var3,
                local_10[0],
                local_10[1],
                0,
                ((i_var1.wrapping_add(0x2c)) as *const i32).read_unaligned(),
            );
        }
    }
}
