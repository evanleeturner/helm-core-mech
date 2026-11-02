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
pub unsafe extern "C" fn FUN_10009783(param_1: *mut i32, param_2: i32) {
    // SAFETY: param_1 points at the game's record; the rect, the object and
    // the callback are read from it exactly where the original reads them.
    unsafe {
        let local_c = param_1.read_unaligned() as *mut i32;
        let i_var1 = param_1.add(0x1d).read_unaligned();
        let r3 = local_c.add(3).read_unaligned();
        let r1 = local_c.add(1).read_unaligned();
        let i_var2 = (r3.wrapping_sub(r1).wrapping_add(1)) >> 1;
        let r4 = local_c.add(4).read_unaligned();
        let r2 = local_c.add(2).read_unaligned();
        let i_var3 = (r4.wrapping_sub(r2).wrapping_add(1)) >> 1;
        let local_8 = FUN_10006d3e();
        let m = (param_2 % 0x1680000).wrapping_add(0x1680000) % 0x1680000;
        let i_var4 = 0x5a0000i32.wrapping_sub(m);
        let mut local_14: [i32; 2] = [0; 2];
        if param_1.add(0x22).read_unaligned() != 0 {
            let f: unsafe extern "C" fn(*mut i32, i32, *mut i32) =
                core::mem::transmute::<i32, unsafe extern "C" fn(*mut i32, i32, *mut i32)>(
                    param_1.add(0x22).read_unaligned(),
                );
            f(local_c, i_var4.wrapping_sub(local_8), local_14.as_mut_ptr());
            FUN_10056489(
                local_c,
                i_var2,
                i_var3,
                local_14[0],
                local_14[1],
                0,
                ((i_var1 as usize).wrapping_add(0x2c) as *const i32).read_unaligned(),
            );
            let f: unsafe extern "C" fn(*mut i32, i32, *mut i32) =
                core::mem::transmute::<i32, unsafe extern "C" fn(*mut i32, i32, *mut i32)>(
                    param_1.add(0x22).read_unaligned(),
                );
            f(local_c, local_8.wrapping_add(i_var4), local_14.as_mut_ptr());
            FUN_10056489(
                local_c,
                i_var2,
                i_var3,
                local_14[0],
                local_14[1],
                0,
                ((i_var1 as usize).wrapping_add(0x2c) as *const i32).read_unaligned(),
            );
        }
    }
}
