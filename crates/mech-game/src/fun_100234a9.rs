// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf38: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10051f43(param_1: i32) -> i32;
    fn FUN_10003742(param_1: u32, param_2: i32, param_3: u32, param_4: i32, param_5: u16) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100234a9(param_1: i32) {
    // SAFETY: param_1 is a pointer to the game's object; its fields and the
    // object at offset 0x20 are read and written where the original does.
    unsafe {
        let i_var1 = ((param_1 as usize).wrapping_add(0x20) as *const i32).read_unaligned();
        let i_var2 = FUN_10051f43(100);
        let p1b4 = ((param_1 as usize).wrapping_add(0x1b4) as *const i32).read_unaligned();
        if i_var2 < p1b4 {
            let p = (i_var1 as usize).wrapping_add(0x440) as *mut u16;
            p.write_unaligned(p.read_unaligned() | 0x80);
            let w = ((param_1 as usize).wrapping_add(0x17a) as *const u16).read_unaligned();
            if u32::from(w) == (DAT_100adf38 | 0x200) {
                FUN_10003742(0x5c, 100, 0x40, 5, 0x32);
            }
        }
    }
}
