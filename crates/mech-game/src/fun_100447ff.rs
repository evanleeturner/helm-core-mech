// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100261f0(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
        param_7: i32,
    );
    fn FUN_1002637d(param_1: i32);
    fn FUN_10014768(param_1: *mut i32, param_2: *mut i32, param_3: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100447ff(param_1: i32, param_2: i32) -> bool {
    let mut local_48: [i32; 14] = [0; 14];
    let mut local_10: [i32; 3] = [0; 3];
    local_10[0] = 0;
    // SAFETY: the callees are the game's own; param_1 points at the caller's
    // object and local_10[0] points at a record returned by the game.
    unsafe {
        let p1 = param_1 as usize;
        FUN_100261f0(
            local_48.as_mut_ptr(),
            ((p1 + 0x50) as *const i32).read_unaligned(),
            param_2,
            ((p1 + 0x58) as *const i32).read_unaligned(),
            ((p1 + 200) as *const i32).read_unaligned(),
            ((p1 + 0xcc) as *const i32).read_unaligned(),
            ((p1 + 0xd0) as *const i32).read_unaligned(),
        );
        FUN_1002637d(local_48.as_mut_ptr() as i32);
        let i_var2 = FUN_10014768(
            local_48.as_mut_ptr(),
            local_10.as_mut_ptr(),
            ((p1 + 4) as *const u32).read_unaligned(),
        );
        if i_var2 == 0 || local_10[0] == 0 {
            return true;
        }
        let rec = local_10[0] as usize;
        let u_var1 = ((p1 + 0xdc) as *const u32).read_unaligned();
        let flags = ((rec + 2) as *const u16).read_unaligned() as u32;
        let val = ((rec + 0x14) as *const u16).read_unaligned() as u32;
        if (flags & 0x100) == 0 || (u_var1 & 0x200) == 0 {
            if (flags & 0x200) == 0 || (u_var1 & 0x400) == 0 {
                false
            } else {
                val == (u_var1 & 0xff)
            }
        } else {
            val == (u_var1 & 0xff)
        }
    }
}
