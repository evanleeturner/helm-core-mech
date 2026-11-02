// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b6800: u8;
    static mut DAT_100cf0d0: u8;
    static mut DAT_100cf0d4: u8;
    static mut DAT_100cf0e0: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1007c97d(param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1007c8c6() {
    mech_trace::enter!("1007c8c6");
    let mut local_8: i32 = 0;
    // SAFETY: the globals are the game's own tables; offsets and reads follow
    // the original, and the callback is the game's own function pointer.
    unsafe {
        while local_8 < 0x80 {
            let off = local_8.wrapping_mul(0x14) as isize as usize;
            let a4 = (&raw mut DAT_100cf0d4).cast::<u8>().wrapping_add(off);
            let ae = (&raw mut DAT_100cf0e0).cast::<u8>().wrapping_add(off);
            if mech_trace::rd!("1007c8c6", a4.cast::<i32>(), read_unaligned) != 0
                && mech_trace::rd!("1007c8c6", ae.cast::<i32>(), read_unaligned) != 0
            {
                let f_addr = mech_trace::rd!("1007c8c6", ae.cast::<u32>(), read_unaligned);
                let f: unsafe extern "C" fn(i32) =
                    core::mem::transmute::<u32, unsafe extern "C" fn(i32)>(f_addr);
                mech_trace::cr!(
                    "1007c8c6",
                    "ind",
                    f(mech_trace::rd!(
                        "1007c8c6",
                        a4.cast::<i32>(),
                        read_unaligned
                    ))
                );
            }
            let mut src = (&raw mut DAT_100b6800).cast::<u32>();
            let mut dst = (&raw mut DAT_100cf0d0)
                .cast::<u8>()
                .wrapping_add(off)
                .cast::<u32>();
            let mut n: i32 = 5;
            while n != 0 {
                mech_trace::wr!(
                    "1007c8c6",
                    dst,
                    write_unaligned,
                    mech_trace::rd!("1007c8c6", src, read_unaligned)
                );
                src = src.wrapping_add(1);
                dst = dst.wrapping_add(1);
                n -= 1;
            }
            local_8 = local_8.wrapping_add(1);
        }
        local_8 = 0;
        while local_8 < 0x80 {
            mech_trace::cr!("1007c8c6", "1007c97d", FUN_1007c97d(local_8));
            local_8 = local_8.wrapping_add(1);
        }
    }
}
