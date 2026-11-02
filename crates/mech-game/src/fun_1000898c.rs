// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7160: u32;
    static mut DAT_100a7310: u32;
    static mut DAT_100a7b30: [u8; 0];
    static mut DAT_100aafc8: u32;
    static mut DAT_100aafd0: u32;
    static mut DAT_100aafd4: u32;
    static mut DAT_100ad6a4: u32;
    static mut DAT_100adf38: u32;
    static mut DAT_100eef40: [u32; 0];
    static mut DAT_100f3040: [u8; 0];
    static mut DAT_10215658: u32;
    static mut DAT_1021565c: u32;
    static mut DAT_10215664: u32;
    static mut PTR_DAT_100aaf68: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002f322(param_1: i32, param_2: *mut u32, param_3: *mut u32, param_4: *mut u32) -> u8;
    fn FUN_10035220(param_1: *mut u32, param_2: i32, param_3: i32, param_4: u32) -> u8;
    fn FUN_1001a5e1(param_1: i32) -> u8;
    fn FUN_10058e74(param_1: *mut i32, param_2: u8) -> u32;
    fn FUN_100353df(param_1: u8) -> u8;
    fn FUN_10009783(param_1: *mut i32, param_2: i32) -> u8;
    fn FUN_10008ed5(param_1: *mut i32) -> u8;
    fn FUN_100099a1(param_1: *mut u32) -> u8;
    fn FUN_10009e05(param_1: *mut u16) -> u32;
    fn FUN_10009edf(param_1: *mut u16, param_2: i32, param_3: u32) -> u32;
    fn FUN_1000a1d2(param_1: i32, param_2: *mut u32, param_3: u32) -> u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1001a620() -> u32;
    fn FUN_10050b20() -> u8;
    fn FUN_1003545c() -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1000898c() {
    mech_trace::enter!("1000898c");
    // SAFETY: all globals and callees are the game's own memory and code;
    // pointers read from them are used where the original uses them.
    unsafe {
        if mech_trace::gr!("1000898c", "10215658", DAT_10215658) == 0 {
            return;
        }
        let local_3c: *mut i32 = mech_trace::rd!(
            "1000898c",
            (&raw mut DAT_100a7b30)
                .cast::<u8>()
                .byte_add(
                    mech_trace::gr!("1000898c", "10215658", DAT_10215658).wrapping_mul(4) as usize
                )
                .cast::<*mut i32>(),
            read_unaligned
        );
        if local_3c.is_null() {
            return;
        }
        let i_var1: i32 = mech_trace::rd!(
            "1000898c",
            (&raw mut DAT_100eef40).cast::<u32>().add(mech_trace::gr!(
                "1000898c",
                "100adf38",
                DAT_100adf38
            ) as usize),
            read_unaligned
        ) as i32;
        let local_38: *mut i32 =
            mech_trace::rd!("1000898c", local_3c, read_unaligned) as usize as *mut i32;
        mech_trace::gw!("1000898c", "100a7310", DAT_100a7310, 0);
        let i_var2: i32 = mech_trace::rd!("1000898c", local_3c.add(2), read_unaligned);
        let mut pi4: *const i32 = local_38;
        let mut pi5: *mut i32 = (&raw mut DAT_100f3040)
            .cast::<u8>()
            .byte_offset((i_var2.wrapping_mul(0x14)) as isize)
            .cast::<i32>();
        let mut i = 5;
        while i != 0 {
            mech_trace::wr!(
                "1000898c",
                pi5,
                write_unaligned,
                mech_trace::rd!("1000898c", pi4, read_unaligned)
            );
            pi4 = pi4.add(1);
            pi5 = pi5.add(1);
            i -= 1;
        }
        let p1 = i_var1 as usize as *const u8;
        let inner = mech_trace::rd!("1000898c", p1.byte_add(0x20).cast::<i32>(), read_unaligned);
        let mut local_10: i32 = mech_trace::rd!(
            "1000898c",
            (inner as usize as *const u8).byte_add(0xc).cast::<i32>(),
            read_unaligned
        );
        let local_c: i32 =
            mech_trace::rd!("1000898c", p1.byte_add(0x60).cast::<i32>(), read_unaligned);
        let n = mech_trace::gr!("1000898c", "10215658", DAT_10215658);
        if n > 0 && n < 3 {
            // keep local_10
        } else if n == 4 {
            local_10 = local_10.wrapping_add(local_c);
        } else {
            local_10 = 0;
        }
        // local_34, local_30, local_2c are consecutive
        let mut vec3: [u32; 3] = [0, 0, 0];
        if mech_trace::gr!("1000898c", "10215664", DAT_10215664) == 0 {
            let mut local_44: u32 = 0;
            let mut local_48: u32 = 0;
            let mut local_4c: u32 = 0;
            let local_14 =
                mech_trace::rd!("1000898c", p1.byte_add(0x44).cast::<i32>(), read_unaligned);
            if local_14 != 0 {
                mech_trace::cr!(
                    "1000898c",
                    "1002f322",
                    FUN_1002f322(
                        local_14,
                        &raw mut local_4c,
                        &raw mut local_48,
                        &raw mut local_44,
                    )
                );
            }
            vec3[0] = local_4c;
            vec3[2] = local_44;
        } else {
            let pp = mech_trace::gr!("1000898c", "PTR_DAT_100aaf68", PTR_DAT_100aaf68) as usize
                as *const u8;
            vec3[0] = mech_trace::rd!("1000898c", pp.cast::<u32>(), read_unaligned);
            vec3[2] = mech_trace::rd!("1000898c", pp.byte_add(8).cast::<u32>(), read_unaligned);
        }
        let local_40: i32 = mech_trace::rd!("1000898c", local_3c.add(6), read_unaligned);
        let local_8: i32 = local_40;
        vec3[1] = local_40 as u32;
        mech_trace::cr!(
            "1000898c",
            "10035220",
            FUN_10035220(vec3.as_mut_ptr(), i_var2, local_40, local_8 as u32)
        );
        if mech_trace::gr!("1000898c", "10215658", DAT_10215658) == 4 {
            mech_trace::gw!(
                "1000898c",
                "100aafc8",
                DAT_100aafc8,
                FUN_10009e05 as usize as u32
            );
            mech_trace::gw!(
                "1000898c",
                "100aafd0",
                DAT_100aafd0,
                FUN_10009edf as usize as u32
            );
            mech_trace::gw!(
                "1000898c",
                "100aafd4",
                DAT_100aafd4,
                FUN_1000a1d2 as usize as u32
            );
            let local_18 = mech_trace::cr!("1000898c", "1001a620", FUN_1001a620()) as i32;
            mech_trace::cr!("1000898c", "1001a5e1", FUN_1001a5e1(6));
            mech_trace::cr!("1000898c", "10050b20", FUN_10050b20());
            mech_trace::cr!("1000898c", "1001a5e1", FUN_1001a5e1(local_18));
            mech_trace::cr!(
                "1000898c",
                "10058e74",
                FUN_10058e74(
                    local_38,
                    mech_trace::gr!("1000898c", "100ad6a4", DAT_100ad6a4) as u8
                )
            );
            mech_trace::cr!("1000898c", "100353df", FUN_100353df(0));
        }
        if mech_trace::gr!("1000898c", "10215658", DAT_10215658) != 4
            || (mech_trace::rd!("1000898c", p1.byte_add(0x14).cast::<u16>(), read_unaligned) & 0x16)
                == 0
        {
            mech_trace::cr!("1000898c", "10009783", FUN_10009783(local_3c, local_10));
        }
        mech_trace::cr!("1000898c", "10008ed5", FUN_10008ed5(local_3c));
        mech_trace::cr!("1000898c", "1003545c", FUN_1003545c());
        let cb = mech_trace::rd!("1000898c", local_3c.add(0x1f), read_unaligned);
        if cb != 0 {
            let f: unsafe extern "C" fn(*mut i32, u32) =
                core::mem::transmute::<usize, unsafe extern "C" fn(*mut i32, u32)>(cb as usize);
            let q = mech_trace::rd!("1000898c", local_3c.add(0x1d), read_unaligned);
            let arg = mech_trace::rd!(
                "1000898c",
                (q as usize as *const u8).byte_add(0x30).cast::<u32>(),
                read_unaligned
            );
            mech_trace::cr!("1000898c", "ind", f(local_38, arg));
        }
        if mech_trace::gr!("1000898c", "1021565c", DAT_1021565c) == 4
            && (mech_trace::gr!("1000898c", "10215658", DAT_10215658) != 4
                || mech_trace::gr!("1000898c", "100a7160", DAT_100a7160) < 2)
        {
            mech_trace::cr!("1000898c", "100099a1", FUN_100099a1(local_3c.cast::<u32>()));
        }
    }
}
