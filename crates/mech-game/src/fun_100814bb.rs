// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b98a4: i32;
    static mut DAT_100c8280: i32;
    static mut DAT_100c8284: i32;
    static mut DAT_100c8288: i32;
    static mut DAT_100c828c: i32;
    static mut DAT_100c8240: i32;
    static mut DAT_100c8244: i32;
    static mut DAT_100c8248: i32;
    static mut DAT_100b98a8: [i8; 0];
    static mut DAT_100b98c4: [i8; 0];
    static mut DAT_100b98ac: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100108a0(param_1: i32, param_2: *mut i8);
    fn FUN_1005dd5a(param_1: *mut i8);
    fn FUN_1001ddf0(param_1: i32) -> *mut i8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100814bb(param_1: i32, param_2: i32) -> i32 {
    // SAFETY: all globals and callees are the game's own memory and code;
    // table offsets are computed exactly as the original does.
    unsafe {
        let mut local_10: i32 = 0;
        let mut local_8: i32 = 0;
        while local_8 < DAT_100b98a4
            && (&raw mut DAT_100c8280)
                .byte_offset(local_8.wrapping_mul(0x14) as isize)
                .read_unaligned()
                != param_2
        {
            local_8 = local_8.wrapping_add(1);
        }
        if DAT_100b98a4 == local_8 {
            FUN_100108a0(0x46, core::ptr::null_mut());
            FUN_1005dd5a((&raw mut DAT_100b98a8).cast::<i8>());
            let pc = FUN_1001ddf0(param_2);
            FUN_1005dd5a(pc);
            local_10 = 0;
        } else {
            let off = local_8.wrapping_mul(0x14) as isize;
            FUN_1005dd5a((&raw mut DAT_100b98ac).cast::<i8>());
            let pc = FUN_1001ddf0(param_2);
            FUN_1005dd5a(pc);
            let a = (&raw mut DAT_100c8284).byte_offset(off).read_unaligned();
            let b = (&raw mut DAT_100c8288).byte_offset(off).read_unaligned();
            if a.wrapping_sub(b) < param_1 {
                FUN_100108a0(0x47, core::ptr::null_mut());
                FUN_1005dd5a((&raw mut DAT_100b98c4).cast::<i8>());
                let pc = FUN_1001ddf0(param_2);
                FUN_1005dd5a(pc);
            } else {
                let i_var1 = (&raw mut DAT_100c828c).byte_offset(off).read_unaligned();
                let o = i_var1.wrapping_mul(12) as isize;
                let p44 = (&raw mut DAT_100c8244).byte_offset(o);
                let p48 = (&raw mut DAT_100c8248).byte_offset(o);
                let p40 = (&raw mut DAT_100c8240).byte_offset(o);
                local_10 = p44
                    .read_unaligned()
                    .wrapping_sub(p48.read_unaligned())
                    .wrapping_add(p40.read_unaligned());
                p48.write_unaligned(p48.read_unaligned().wrapping_sub(param_1));
                let p88 = (&raw mut DAT_100c8288).byte_offset(off);
                p88.write_unaligned(p88.read_unaligned().wrapping_add(param_1));
            }
        }
        local_10
    }
}
