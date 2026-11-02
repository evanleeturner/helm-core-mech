// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ab2d4: *mut i32;
    static mut DAT_101e84f0: [u8; 1];
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10086110() -> u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10070f52(param_1: *mut i32) -> u8;
    fn FUN_10070ed9(param_1: *mut i8) -> *mut i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001df10(param_1: *mut i8) {
    mech_trace::enter!("1001df10", param_1);
    // SAFETY: the callees and globals are the game's own; param_1 points at a
    // NUL-terminated string, copied into the game's buffer at DAT_101e84f0.
    unsafe {
        mech_trace::cr!("1001df10", "10086110", FUN_10086110());
        if !mech_trace::gr!("1001df10", "100ab2d4", DAT_100ab2d4).is_null() {
            mech_trace::cr!(
                "1001df10",
                "10070f52",
                FUN_10070f52(mech_trace::gr!("1001df10", "100ab2d4", DAT_100ab2d4))
            );
        }
        let mut u_var2: u32 = 0xffff_ffff;
        let mut pc_var4: *const i8 = param_1;
        let mut pc_var5: *const i8;
        loop {
            pc_var5 = pc_var4;
            if u_var2 == 0 {
                break;
            }
            u_var2 = u_var2.wrapping_sub(1);
            pc_var5 = pc_var4.wrapping_add(1);
            let c_var1 = mech_trace::rd!("1001df10", pc_var4, read);
            pc_var4 = pc_var5;
            if c_var1 == 0 {
                break;
            }
        }
        u_var2 = !u_var2;
        pc_var4 = pc_var5.wrapping_sub(u_var2 as usize);
        let mut dst: *mut u8 = (&raw mut DAT_101e84f0).cast::<u8>();
        let mut src: *const u8 = pc_var4.cast::<u8>();
        let mut u_var3 = u_var2 >> 2;
        while u_var3 != 0 {
            mech_trace::wr!(
                "1001df10",
                dst.cast::<u32>(),
                write_unaligned,
                mech_trace::rd!("1001df10", src.cast::<u32>(), read_unaligned)
            );
            src = src.wrapping_add(4);
            dst = dst.wrapping_add(4);
            u_var3 = u_var3.wrapping_sub(1);
        }
        u_var2 &= 3;
        while u_var2 != 0 {
            mech_trace::wr!(
                "1001df10",
                dst,
                write,
                mech_trace::rd!("1001df10", src, read)
            );
            src = src.wrapping_add(1);
            dst = dst.wrapping_add(1);
            u_var2 = u_var2.wrapping_sub(1);
        }
        mech_trace::gw!(
            "1001df10",
            "100ab2d4",
            DAT_100ab2d4,
            mech_trace::cr!("1001df10", "10070ed9", FUN_10070ed9(param_1))
        );
    }
}
