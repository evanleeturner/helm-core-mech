// SPDX-License-Identifier: MIT
use core::mem::MaybeUninit;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ef030: u8;
    static mut DAT_100ef034: u8;
    static mut DAT_101f64a4: u8;
    static mut DAT_101f64ac: u8;
    static mut DAT_101f64b4: u8;
    static mut DAT_101f64b8: u8;
    static mut DAT_101f64bc: u8;
    static mut DAT_101f64c0: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000d2d0(param_1: i32) -> u8;
    fn FUN_1002fc9d(param_1: i32) -> i32;
    fn FUN_1001d671(param_1: *mut u8) -> u8;
    fn FUN_1000d875(param_1: i32, param_2: i32) -> u32;
    fn FUN_1002fc82(param_1: i32, param_2: *mut u32) -> u8;
    fn FUN_1000d301(param_1: i32) -> u8;
    fn FUN_1002fa9c(param_1: *mut i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000d403(param_1: i32) -> i32 {
    mech_trace::enter!("1000d403", param_1);
    // SAFETY: the globals are the game's own tables; offsets index its
    // records exactly as the original does.
    unsafe {
        let mut local_34: MaybeUninit<[u32; 12]> = MaybeUninit::uninit();
        let l34 = local_34.as_mut_ptr().cast::<u32>();
        let mut i_var1 = param_1.wrapping_mul(0x7c);
        let local_38: i32;
        let p_b8 = (&raw mut DAT_101f64b8)
            .byte_add(i_var1 as usize)
            .cast::<i32>();
        if mech_trace::rd!("1000d403", p_b8, read_unaligned) == -1 {
            local_38 = -1;
        } else {
            let mut i_var5 = mech_trace::rd!("1000d403", p_b8, read_unaligned);
            let p_ac = (&raw mut DAT_101f64ac)
                .byte_add(i_var1 as usize)
                .cast::<u32>();
            mech_trace::wr!(
                "1000d403",
                p_ac,
                write_unaligned,
                mech_trace::rd!("1000d403", p_ac, read_unaligned) | 0x200
            );
            mech_trace::wr!(
                "1000d403",
                p_ac,
                write_unaligned,
                mech_trace::rd!("1000d403", p_ac, read_unaligned) | 0x800
            );
            let p30 = (&raw mut DAT_100ef030)
                .byte_add(i_var5.wrapping_mul(0x40) as usize)
                .cast::<u16>();
            mech_trace::wr!(
                "1000d403",
                p30,
                write_unaligned,
                mech_trace::rd!("1000d403", p30, read_unaligned) | 4
            );
            mech_trace::wr!(
                "1000d403",
                (&raw mut DAT_100ef034)
                    .byte_add(i_var5.wrapping_mul(0x40) as usize)
                    .cast::<u32>(),
                write_unaligned,
                0xffffffff
            );
            mech_trace::cr!("1000d403", "1000d2d0", FUN_1000d2d0(param_1));
            let p_bc = (&raw mut DAT_101f64bc)
                .byte_add(i_var1 as usize)
                .cast::<*mut u8>();
            let p_c0 = (&raw mut DAT_101f64c0)
                .byte_add(i_var1 as usize)
                .cast::<i32>();
            let p_var2 = mech_trace::rd!("1000d403", p_bc, read_unaligned);
            if !p_var2.is_null() {
                if mech_trace::rd!("1000d403", p_c0, read_unaligned) != 0 {
                    let mut src = mech_trace::cr!(
                        "1000d403",
                        "1002fc9d",
                        FUN_1002fc9d(mech_trace::rd!("1000d403", p_c0, read_unaligned))
                    ) as usize as *const u32;
                    let mut dst = l34;
                    i_var5 = 0xc;
                    while i_var5 != 0 {
                        mech_trace::wr!(
                            "1000d403",
                            dst,
                            write_unaligned,
                            mech_trace::rd!("1000d403", src, read_unaligned)
                        );
                        src = src.add(1);
                        dst = dst.add(1);
                        i_var5 -= 1;
                    }
                }
                mech_trace::cr!("1000d403", "1001d671", FUN_1001d671(p_var2));
                mech_trace::wr!("1000d403", p_bc, write_unaligned, core::ptr::null_mut());
                mech_trace::wr!("1000d403", p_c0, write_unaligned, 0);
            }
            let mut l38 = mech_trace::rd!(
                "1000d403",
                (&raw mut DAT_101f64b4)
                    .byte_add(i_var1 as usize)
                    .cast::<i32>(),
                read_unaligned
            );
            if l38 != -1 {
                i_var1 = l38.wrapping_mul(0x7c);
                let p_ac = (&raw mut DAT_101f64ac)
                    .byte_add(i_var1 as usize)
                    .cast::<u32>();
                mech_trace::wr!(
                    "1000d403",
                    p_ac,
                    write_unaligned,
                    mech_trace::rd!("1000d403", p_ac, read_unaligned) & 0xfffff7ff
                );
                let a4 = mech_trace::rd!(
                    "1000d403",
                    (&raw mut DAT_101f64a4)
                        .byte_add(i_var1 as usize)
                        .cast::<i32>(),
                    read_unaligned
                );
                let r = mech_trace::cr!("1000d403", "1000d875", FUN_1000d875(l38, a4));
                if r == 0 {
                    mech_trace::wr!(
                        "1000d403",
                        p_ac,
                        write_unaligned,
                        mech_trace::rd!("1000d403", p_ac, read_unaligned) & 0x800
                    );
                    l38 = -1;
                } else {
                    let pi_var3 = mech_trace::rd!(
                        "1000d403",
                        (&raw mut DAT_101f64c0)
                            .byte_add(i_var1 as usize)
                            .cast::<*mut i32>(),
                        read_unaligned
                    );
                    if !pi_var3.is_null() {
                        mech_trace::cr!(
                            "1000d403",
                            "1002fc82",
                            FUN_1002fc82(pi_var3 as usize as i32, l34)
                        );
                    }
                    mech_trace::cr!("1000d403", "1000d301", FUN_1000d301(l38));
                    if !pi_var3.is_null() {
                        mech_trace::cr!("1000d403", "1002fa9c", FUN_1002fa9c(pi_var3));
                    }
                }
            }
            local_38 = l38;
        }
        mech_trace::rt!("1000d403", local_38)
    }
}
