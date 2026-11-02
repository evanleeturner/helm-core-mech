// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c2be0: *const i8;
    static mut DAT_100c2dec: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn __isctype(param_1: i32, param_2: i32) -> i32;
    fn _tolower(param_1: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001767b(param_1: i32, param_2: *mut i8) {
    mech_trace::enter!("1001767b", param_1, param_2);
    // SAFETY: the globals and callees are the game's own; param_2 is a
    // NUL-terminated string and param_1 an address of a buffer large enough
    // for the copy, as the original assumes.
    unsafe {
        let mut u_var3: u32 = 0xffff_ffff;
        let mut pc_var4: *const i8 = param_2;
        loop {
            if u_var3 == 0 {
                break;
            }
            u_var3 = u_var3.wrapping_sub(1);
            let c_var1 = mech_trace::rd!("1001767b", pc_var4, read);
            pc_var4 = pc_var4.byte_add(1);
            if c_var1 == 0 {
                break;
            }
        }
        let limit = (!u_var3).wrapping_sub(1) as i32;
        let mut local_c: i32 = 0;
        while local_c < limit {
            let ch = mech_trace::rd!("1001767b", param_2.byte_add(local_c as usize), read);
            let local_10: u32;
            if mech_trace::gr!("1001767b", "100c2dec", DAT_100c2dec) < 2 {
                let v = mech_trace::rd!(
                    "1001767b",
                    mech_trace::gr!("1001767b", "100c2be0", DAT_100c2be0)
                        .byte_offset((ch as i32).wrapping_mul(2) as isize)
                        .cast::<u16>(),
                    read_unaligned
                );
                local_10 = (v as u32) & 0x103;
            } else {
                local_10 =
                    mech_trace::cr!("1001767b", "__isctype", __isctype(ch as i32, 0x103)) as u32;
            }
            let dst = param_1.wrapping_add(local_c) as *mut u8;
            if local_10 == 0 {
                mech_trace::wr!("1001767b", dst, write, ch as u8);
            } else {
                let i_var2 = mech_trace::cr!("1001767b", "_tolower", _tolower(ch as i32));
                mech_trace::wr!("1001767b", dst, write, i_var2 as u8);
            }
            local_c = local_c.wrapping_add(1);
        }
        mech_trace::wr!(
            "1001767b",
            (param_1.wrapping_add(local_c) as *mut u8),
            write,
            0
        );
    }
}
