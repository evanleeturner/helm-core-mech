// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c6aa0: u8;
    static mut DAT_100c6aa4: u16;
    static mut DAT_100c6aa6: u8;
    static mut DAT_100c6aca: u8;
    static mut DAT_100c6ace: u8;
    static mut DAT_100c6bca: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10059ea0(param_1: u32) -> u8;
    fn __strnset(_dest: *mut i8, _val: i32, _count: u32) -> *mut i8;
    fn __close(_file_handle: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005a180(param_1: i32) {
    mech_trace::enter!("1005a180", param_1);
    // SAFETY: the globals and callees are the game's own; all addresses are
    // computed as the original does, as byte offsets from the game's tables.
    unsafe {
        let mut u_var4: u32 = 0;
        let i_var2: i32 = param_1.wrapping_mul(0x14a);
        let p_aa6 = (&raw mut DAT_100c6aa6) as *mut u8;
        let p_aca = (&raw mut DAT_100c6aca) as *mut u8;
        let p_ace = (&raw mut DAT_100c6ace) as *mut u8;

        mech_trace::cr!(
            "1005a180",
            "10059ea0",
            FUN_10059ea0(mech_trace::rd!(
                "1005a180",
                p_aa6.byte_offset(i_var2 as isize).cast::<u32>(),
                read_unaligned
            ),)
        );
        mech_trace::wr!(
            "1005a180",
            p_aa6.byte_offset(i_var2 as isize).cast::<u32>(),
            write_unaligned,
            0
        );
        loop {
            let i_var1: i32 = i_var2.wrapping_add(u_var4.wrapping_mul(8) as i32);
            let a = p_aca.byte_offset(i_var1 as isize).cast::<i32>();
            if mech_trace::rd!("1005a180", a, read_unaligned) != 0 {
                mech_trace::cr!(
                    "1005a180",
                    "10059ea0",
                    FUN_10059ea0(mech_trace::rd!("1005a180", a, read_unaligned) as u32)
                );
                mech_trace::wr!("1005a180", a, write_unaligned, 0);
            }
            let b = p_ace.byte_offset(i_var1 as isize).cast::<i32>();
            if mech_trace::rd!("1005a180", b, read_unaligned) != 0 {
                mech_trace::cr!(
                    "1005a180",
                    "10059ea0",
                    FUN_10059ea0(mech_trace::rd!("1005a180", b, read_unaligned) as u32)
                );
                mech_trace::wr!("1005a180", b, write_unaligned, 0);
            }
            let u_var3: u16 = (u_var4 as u16).wrapping_add(1);
            u_var4 = u_var3 as u32;
            if u_var3 >= 0x20 {
                break;
            }
        }
        mech_trace::wr!(
            "1005a180",
            ((&raw mut DAT_100c6aa4) as *mut u16)
                .byte_offset((param_1.wrapping_mul(0xa5) as isize).wrapping_mul(2)),
            write_unaligned,
            0
        );
        mech_trace::cr!(
            "1005a180",
            "__strnset",
            __strnset(
                ((&raw mut DAT_100c6bca) as *mut u8)
                    .byte_offset(i_var2 as isize)
                    .cast::<i8>(),
                0,
                0x20,
            )
        );
        mech_trace::cr!(
            "1005a180",
            "__close",
            __close(mech_trace::rd!(
                "1005a180",
                ((&raw mut DAT_100c6aa0) as *mut u8)
                    .byte_offset(i_var2 as isize)
                    .cast::<i32>(),
                read_unaligned
            ),)
        );
    }
}
