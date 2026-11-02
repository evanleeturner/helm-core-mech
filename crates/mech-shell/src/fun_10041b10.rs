// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10083508: *mut i8;
    static mut DAT_10098224: *mut core::ffi::c_void;
    static mut DAT_100983cc: *mut i8;
    static mut DAT_100983d0: i32;
    static mut DAT_100a6798: i32;
    static mut DAT_100a67a4: i32;
    static mut DAT_100ba1d8: [i8; 1];
    static mut DAT_100ba1f8: [i8; 1];
    static mut DAT_10098494: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000be10() -> i32;
    fn FUN_10017df0(param_1: i32, param_2: *mut i8);
    fn FUN_10018060(param_1: i32, param_2: i32) -> i32;
    fn FUN_1005bbb0(param_1: *mut i8);
    fn FUN_1000c2b0();
    fn FUN_1000c250();
    fn FUN_1000c410();
}

unsafe extern "stdcall" {
    fn HeapAlloc(
        h_heap: *mut core::ffi::c_void,
        dw_flags: u32,
        dw_bytes: u32,
    ) -> *mut core::ffi::c_void;
}

/// Copies a NUL-terminated string including the terminator.
unsafe fn copy_str(src: *const i8, dst: *mut i8) {
    // SAFETY: caller guarantees src is NUL-terminated and dst has room.
    unsafe {
        let mut n: usize = 0;
        while *src.add(n) != 0 {
            n += 1;
        }
        n += 1;
        let mut i = 0;
        while i < n {
            *dst.add(i) = *src.add(i);
            i += 1;
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10041b10() -> u32 {
    // SAFETY: all globals and callees are the game's own memory; pointers
    // are read and written at the offsets the original uses.
    unsafe {
        *DAT_10083508.add(1) = 1;
        *DAT_10083508 = 1;
        DAT_100983cc = HeapAlloc(DAT_10098224, 1, 0x2066) as *mut i8;
        FUN_1005bbb0(&raw mut DAT_100ba1d8 as *mut i8);

        copy_str(&raw const DAT_100ba1d8 as *const i8, DAT_100983cc);

        let p = DAT_100983cc;
        *p.add(0x20) = -1;
        *p.add(0x21) = -1;
        *p.add(0x22) = -1;
        *p.add(0x23) = -1;
        let u_var2 = FUN_1000be10();
        DAT_100983cc.add(0x24).cast::<i32>().write_unaligned(u_var2);

        let mut i_var3: i32 = 0;
        loop {
            let i_var4 = i_var3.wrapping_add(2);
            let q = DAT_100983cc.offset(i_var3 as isize).add(0x28);
            *q = -1;
            *q.add(1) = -1;
            i_var3 = i_var4;
            if i_var4 >= 0x10 {
                break;
            }
        }

        let p = DAT_100983cc;
        *p.add(0x38) = 0;
        *p.add(0x39) = 0;
        *p.add(0x3a) = 0;
        *p.add(0x3b) = 0;
        let p = DAT_100983cc;
        *p.add(0x3c) = 0;
        *p.add(0x3d) = 0;
        *p.add(0x3e) = 0;
        *p.add(0x3f) = 0;
        let p = DAT_100983cc;
        *p.add(0x40) = 0;
        *p.add(0x41) = 0;
        *p.add(0x42) = 0;
        *p.add(0x43) = 0;
        *DAT_100983cc.add(0x2064) = 1;

        copy_str(&raw const DAT_10098494 as *const i8, DAT_100983cc.add(100));

        FUN_10017df0(DAT_100a67a4, DAT_100983cc);

        copy_str(
            &raw const DAT_100ba1f8 as *const i8,
            DAT_10083508.add(0x102),
        );

        let i_var3 = DAT_10083508.add(0x23bb).cast::<i32>().read_unaligned();
        if i_var3 == 0 {
            FUN_1000c2b0();
            FUN_1000c250();
            FUN_1000c410();
            return 0xc026;
        }
        if i_var3 == 1 {
            DAT_100983d0 = FUN_10018060(DAT_100a6798, 0);
            if DAT_100983d0 != 0 {
                let d = DAT_100983d0 as usize as *mut u8;
                let v = DAT_100983cc.add(0x24).cast::<i32>().read_unaligned();
                d.add(0x80c).cast::<i32>().write_unaligned(v);
                let w = d.add(0x810).cast::<i32>().read_unaligned();
                DAT_100983cc.add(0x20).cast::<i32>().write_unaligned(w);
            }
            return 0xc026;
        }
        if i_var3 != 2 {
            return 0;
        }
        0xc029
    }
}
