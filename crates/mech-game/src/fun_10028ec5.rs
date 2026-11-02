// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002954c(param_1: u16) -> i32;
    fn FUN_10028bde(
        param_1: i32,
        param_2: u16,
        param_3: i16,
        param_4: *mut u16,
        param_5: *mut u16,
    ) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10028ec5(param_1: i32, param_2: u16, param_3: i16) -> u16 {
    let mut param_3 = param_3;
    let mut local_c: [u16; 2] = [0; 2];
    let mut local_8: [u16; 2] = [0; 2];
    let base = param_1 as usize as *const u8;
    // SAFETY: param_1 is the game's object pointer; reads happen at the
    // offsets the original uses, and the callees are the game's own.
    unsafe {
        if (base.byte_add(0x17e).read() & 3) == 0 {
            if param_3 == -2 {
                param_3 = (base.byte_add(0x19a).cast::<i32>().read_unaligned() / 100) as i16;
            } else if param_3 == -1 {
                param_3 = (base.byte_add(0x196).cast::<i32>().read_unaligned() / 100) as i16;
            } else if param_3 == 0 {
                let i_var1 = FUN_1002954c(param_2);
                param_3 = (i_var1 / 100) as i16;
            }
            FUN_10028bde(
                param_1,
                param_2,
                param_3,
                local_c.as_mut_ptr(),
                local_8.as_mut_ptr(),
            );
            if base.byte_add(0x1a8).cast::<i32>().read_unaligned() != 0 {
                local_8[0] = local_c[0];
            }
        } else {
            local_8[0] = 0;
        }
    }
    local_8[0]
}
