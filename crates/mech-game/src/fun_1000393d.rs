// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a40f8: *mut u32;
    static mut DAT_100b3c4c: u32;
    static mut DAT_100aaf68: u32;
    static mut DAT_100af0a8: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002f322(param_1: i32, param_2: *mut u32, param_3: *mut u32, param_4: *mut u32);
    fn FUN_10083a59(
        param_1: u32,
        param_2: u32,
        param_3: u32,
        param_4: *mut i32,
        param_5: *mut u32,
        param_6: *mut u32,
        param_7: *mut i32,
    );
    fn FUN_10003c3e(param_1: i32);
    fn FUN_1008668b(param_1: i32, param_2: u32, param_3: *mut i32) -> *mut u16;
    fn FUN_1008664f(param_1: u32, param_2: *mut i32);
    fn FUN_10003cf8(param_1: i32) -> i32;
    fn FUN_10003d60(param_1: i32, param_2: i32) -> i32;
    fn FUN_1000381c(param_1: i32) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn _AIL_allocate_file_sample_12(a: u32, b: u32, c: u32) -> u32;
    fn _AIL_set_sample_type_12(a: u32, b: u32, c: u32) -> u32;
    fn _AIL_set_sample_loop_count_8(a: u32, b: u32) -> u32;
    fn _AIL_set_sample_pan_8(a: u32, b: u32) -> u32;
    fn _AIL_set_sample_volume_8(a: u32, b: u32) -> u32;
    fn FUN_1000387d() -> u32;
    fn _AIL_set_sample_playback_rate_8(a: u32, b: u32) -> u32;
    fn _AIL_start_sample_4(a: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000393d(param_1: *mut i32) {
    let mut local_28: u32 = 0;
    let mut local_24: u32 = 0;
    let mut local_20: i32 = 0;
    let mut local_1c: u32 = 0;
    let mut local_14: u32 = 0;
    let mut local_c: u32 = 0;
    let mut local_18: i32 = 0;
    // SAFETY: all globals and callees are the game's own; param_1 points at the
    // caller's sound record and DAT_100a40f8 at the game's sound table.
    unsafe {
        if DAT_100a40f8.is_null() {
            return;
        }
        if param_1.add(6).read() != 0 {
            param_1.add(6).write(0);
            return;
        }
        FUN_1002f322(
            param_1.add(4).read(),
            &raw mut local_14,
            &raw mut local_24,
            &raw mut local_28,
        );
        let b = DAT_100aaf68 as *const i32;
        local_14 = (b.read_unaligned() as u32).wrapping_sub(local_14);
        local_24 = (b.byte_add(4).read_unaligned() as u32).wrapping_sub(local_24);
        local_28 = (b.byte_add(8).read_unaligned() as u32).wrapping_sub(local_28);
        FUN_10083a59(
            local_14,
            local_24,
            local_28,
            &raw mut local_20,
            &raw mut local_1c,
            &raw mut local_c,
            (&raw mut local_c).cast::<i32>(),
        );
        if param_1.read() < local_1c as i32 {
            FUN_10003c3e(param_1 as i32);
            return;
        }
        if param_1.add(1).read() == -1 {
            local_c = 8;
            while (local_c as i32) < 0x10 {
                if DAT_100a40f8
                    .wrapping_add(local_c.wrapping_add(0x12) as usize)
                    .read_unaligned()
                    == 0
                {
                    param_1.add(1).write(local_c as i32);
                    break;
                }
                local_c = local_c.wrapping_add(1);
            }
            if param_1.add(1).read() == -1 {
                return;
            }
            if param_1.add(2).read() == 0 {
                let p = FUN_1008668b(
                    0,
                    (param_1.add(7).read() as i16) as i32 as u32,
                    DAT_100af0a8 as *mut i32,
                );
                param_1.add(2).write(p as i32);
            }
            let mut local_8: u32 = 0;
            let mut fail = param_1.add(2).read() == 0;
            if !fail {
                local_8 = _AIL_allocate_file_sample_12(
                    DAT_100a40f8.read_unaligned(),
                    param_1.add(2).read() as u32,
                    0xffffffff,
                );
                fail = local_8 == 0;
            }
            if fail {
                if (param_1.add(7).read() as i16) != -1 {
                    FUN_1008664f(
                        (param_1.add(7).read() as i16) as i32 as u32,
                        DAT_100af0a8 as *mut i32,
                    );
                }
                param_1.add(1).write(-1);
                param_1.add(2).write(0);
                return;
            }
            let i = param_1.add(1).read();
            DAT_100a40f8
                .wrapping_add(i.wrapping_add(1) as usize)
                .write_unaligned(local_8);
            let i = param_1.add(1).read();
            DAT_100a40f8
                .wrapping_add(i.wrapping_add(0x12a) as usize)
                .write_unaligned((param_1.add(7).read() as i16) as i32 as u32);
            let i = param_1.add(1).read();
            DAT_100a40f8
                .cast::<u8>()
                .wrapping_add(i.wrapping_mul(2).wrapping_add(0x4e8) as usize)
                .cast::<u16>()
                .write_unaligned(0x1000);
            _AIL_set_sample_type_12(local_8, 0, 0);
            _AIL_set_sample_loop_count_8(local_8, 0);
            local_18 = 1;
        }
        let i = param_1.add(1).read();
        let local_8 = DAT_100a40f8
            .wrapping_add(i.wrapping_add(1) as usize)
            .read_unaligned();
        let mut local_10 = FUN_10003cf8(local_1c as i32);
        if local_10 < 1 {
            FUN_10003c3e(param_1 as i32);
            return;
        }
        local_10 = FUN_10003d60(local_10, DAT_100b3c4c as i32);
        local_10 >>= 2;
        let i_var2 = FUN_1000381c(local_20);
        _AIL_set_sample_pan_8(local_8, i_var2 as u32);
        _AIL_set_sample_volume_8(local_8, local_10 as u32);
        let u_var3 = FUN_1000387d();
        _AIL_set_sample_playback_rate_8(local_8, u_var3);
        let i = param_1.add(1).read();
        if local_18 != 0
            || DAT_100a40f8
                .wrapping_add(i.wrapping_add(0x12) as usize)
                .read_unaligned()
                == 0
        {
            _AIL_start_sample_4(local_8);
            let i = param_1.add(1).read();
            DAT_100a40f8
                .wrapping_add(i.wrapping_add(0x12) as usize)
                .write_unaligned(1);
        }
    }
}
