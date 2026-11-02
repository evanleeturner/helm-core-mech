// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1005aae4(param_1: i32) -> u32;
    fn FUN_1005aaf7(param_1: i32, param_2: i32) -> u32;
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
pub unsafe extern "C" fn FUN_1006670a(
    param_1: *mut i32,
    param_2: *mut i8,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
) {
    let mut local_c: i32 = 0;
    // SAFETY: the callees are the game's own; param_2 is a NUL-terminated
    // string supplied by the caller and is only read.
    unsafe {
        let i_var2 = FUN_1005aae4(param_5) as i32;
        let mut local_10: u32 = 0;
        loop {
            let mut u_var4: u32 = 0xffff_ffff;
            let mut pc_var5: *const i8 = param_2;
            loop {
                if u_var4 == 0 {
                    break;
                }
                u_var4 = u_var4.wrapping_sub(1);
                let c_var1 = pc_var5.read();
                pc_var5 = pc_var5.add(1);
                if c_var1 == 0 {
                    break;
                }
            }
            if (!u_var4).wrapping_sub(1) <= local_10 {
                FUN_10056489(
                    param_1,
                    param_3,
                    param_4.wrapping_add(i_var2),
                    param_3.wrapping_add(local_c).wrapping_sub(1),
                    param_4.wrapping_add(i_var2),
                    0,
                    param_6,
                );
                return;
            }
            let i_var3 = FUN_1005aaf7(param_5, param_2.add(local_10 as usize).read() as i32) as i32;
            local_c = local_c.wrapping_add(i_var3);
            local_10 = local_10.wrapping_add(1);
        }
    }
}
