// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adca0: u32;
    static mut DAT_100adca4: u32;
    static mut DAT_100adf74: *mut core::ffi::c_void;
    static mut DAT_101eb5f4: u8;
    static mut DAT_101eb5f8: u8;
    static mut DAT_101eb5fc: u8;
    static mut DAT_101eb600: u8;
    static mut DAT_101ec400: u32;
    static mut DAT_100af0e0: u32;
    static mut DAT_100af14c: *mut i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1007bbed(
        param_1: *mut i16,
        param_2: *mut i32,
        param_3: *mut i8,
        param_4: u32,
        param_5: *mut u32,
        param_6: *mut i32,
    ) -> *mut u16;
    fn FUN_1008664f(param_1: u32, param_2: *mut i32) -> u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapFree(h: *mut core::ffi::c_void, flags: u32, mem: *mut core::ffi::c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004764c(param_1: *mut i16) -> u32 {
    let mut local_8: u32 = 0;
    // SAFETY: the callees and globals are the game's own; param_1 and the
    // returned buffer are read and the global tables written where the
    // original does.
    unsafe {
        let lp_mem = FUN_1007bbed(
            param_1,
            DAT_100af0e0 as *mut i32,
            DAT_100af14c,
            6,
            &raw mut local_8,
            core::ptr::null_mut(),
        );
        if lp_mem.is_null() {
            return 0;
        }
        let mut local_1c: *mut u16 = lp_mem;
        let mut local_10: i32 = 0;
        while local_10 < 5 {
            let a = local_1c.cast::<i32>().read_unaligned();
            (&raw mut DAT_100adca0)
                .offset((local_10 as isize) * 2)
                .write(a as u32);
            let b = local_1c.add(2).cast::<i32>().read_unaligned();
            (&raw mut DAT_100adca4)
                .offset((local_10 as isize) * 2)
                .write(b as u32);
            local_1c = local_1c.add(4);
            local_10 += 1;
        }
        local_10 = 0;
        while local_10 < 3 {
            let v = local_1c.cast::<i32>().read_unaligned();
            local_1c = local_1c.add(2);
            (&raw mut DAT_101ec400)
                .offset(local_10 as isize)
                .write(v as u32);
            local_10 += 1;
        }
        local_10 = 0;
        while local_10 < 0xf {
            let mut local_28 = local_1c.cast::<i32>().read_unaligned();
            let mut local_2c = local_1c.add(2).cast::<i32>().read_unaligned();
            let mut local_20 = local_1c.add(4).cast::<i32>().read_unaligned();
            let mut local_24 = local_1c.add(6).cast::<i32>().read_unaligned();
            local_1c = local_1c.add(8);
            if local_28 < 0
                || 100 < local_28
                || local_2c < 0
                || 100 < local_2c
                || local_20 < 0
                || 100 < local_20
                || local_24 < 0
                || 100 < local_24
            {
                local_24 = 0;
                local_20 = 0;
                local_2c = 0;
                local_28 = 0;
            }
            let mut local_14: i32 = 0;
            while local_14 < 8 {
                let off = (local_14 as isize) * 0x140 + (local_10 as isize) * 0x14;
                (&raw mut DAT_101eb5f4)
                    .byte_offset(off)
                    .cast::<i32>()
                    .write_unaligned(local_28);
                (&raw mut DAT_101eb5f8)
                    .byte_offset(off)
                    .cast::<i32>()
                    .write_unaligned(local_2c);
                (&raw mut DAT_101eb5fc)
                    .byte_offset(off)
                    .cast::<i32>()
                    .write_unaligned(local_20);
                (&raw mut DAT_101eb600)
                    .byte_offset(off)
                    .cast::<i32>()
                    .write_unaligned(local_24);
                local_14 += 1;
            }
            local_10 += 1;
        }
        if param_1.read() == -1 {
            HeapFree(DAT_100adf74, 1, lp_mem.cast());
        } else {
            FUN_1008664f(param_1.read() as i32 as u32, DAT_100af0e0 as *mut i32);
        }
        1
    }
}
