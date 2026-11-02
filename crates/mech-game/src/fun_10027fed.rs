// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1004d47f(param_1: i32) -> u32;
    fn FUN_1002b6fe(param_1: i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_1004d0e5(param_1: i32, param_2: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10027fed(param_1: i32) -> i32 {
    let mut local_2c: [i32; 8] = [0; 8];
    // SAFETY: the callees are the game's own; local_2c is a local buffer the
    // callee fills, and the entries read are pointers to the game's records.
    unsafe {
        let local_8 = FUN_1004d47f(param_1) as i32;
        let mut local_34: i32 = -1;
        let i_var2 = FUN_1002b6fe(param_1, local_2c.as_mut_ptr() as i32, 1);
        if i_var2 != 0 {
            let mut local_c: i32 = 0;
            while local_c < i_var2 {
                let i_var1 = *local_2c.as_ptr().offset(local_c as isize);
                let p = i_var1 as usize as *const u8;
                if p.byte_add(4).cast::<i32>().read_unaligned() != local_8
                    && (p.byte_add(0x14).cast::<u16>().read_unaligned() & 6) == 0
                {
                    local_34 = p.byte_add(4).cast::<i32>().read_unaligned();
                    break;
                }
                local_c = local_c.wrapping_add(1);
            }
            FUN_1004d0e5(param_1, local_34);
        }
        local_34
    }
}
