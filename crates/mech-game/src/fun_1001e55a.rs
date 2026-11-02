// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_101e8546: u8;
}

unsafe extern "C" {
    #[link_name = "strncmp"]
    fn _strncmp(str1: *const i8, str2: *const i8, max_count: usize) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001e55a(param_1: i32, str2: *const i8) -> u16 {
    mech_trace::enter!("1001e55a", param_1, str2);
    let mut found = false;
    // SAFETY: the global table and the record it points to belong to the game;
    // str2 is the caller's string. Reads happen in the original's order.
    unsafe {
        let slot = (&raw mut DAT_101e8546)
            .wrapping_byte_offset(param_1.wrapping_mul(0x14a) as isize)
            .cast::<i32>();
        let u_var1 = mech_trace::rd!(
            "1001e55a",
            ((mech_trace::rd!("1001e55a", slot, read_unaligned) as u32).wrapping_add(0xc)
                as *const u16),
            read_unaligned
        );
        let i_var2 = mech_trace::rd!("1001e55a", slot, read_unaligned);
        let mut u_var4: u16 = 0;
        while !found && u_var4 < u_var1 {
            let p = (u32::from(u_var4).wrapping_mul(0x18))
                .wrapping_add(i_var2 as u32)
                .wrapping_add(0xe) as *const i8;
            let i_var5 = mech_trace::cr!("1001e55a", "_strncmp", _strncmp(p, str2, 4));
            if i_var5 == 0 {
                found = true;
            } else {
                u_var4 = u_var4.wrapping_add(1);
            }
        }
        if !found {
            u_var4 = 0xffff;
        }
        mech_trace::rt!("1001e55a", u_var4)
    }
}
