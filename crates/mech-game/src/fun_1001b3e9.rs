// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10160c00: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1003cc75();
    fn FUN_10009bd5() -> i32;
    fn FUN_10006d3e() -> i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002f322(param_1: i32, param_2: *mut i32, param_3: *mut i32, param_4: *mut i32);
    fn FUN_1002f668(param_1: i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
    fn FUN_1002fbc0(param_1: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1001b3e9() {
    let mut local_28: i32;
    let mut local_24: i32;
    let mut local_20: i32 = 0;
    let mut local_18: i32 = 0;
    let mut local_14: i32 = 0;
    let mut local_10: i32 = 0;
    let mut local_c: i32;
    let mut local_8: i32;

    // SAFETY: the callees and globals are the game's own; the list nodes are
    // game memory addressed and read exactly where the original does.
    unsafe {
        FUN_1003cc75();
        local_c = DAT_10160c00;
        while local_c != 0 {
            local_20 = local_20.wrapping_add(1);
            local_24 = ((local_c.wrapping_add(8)) as *const i32).read_unaligned();
            while local_24 != 0 {
                let b = ((local_24.wrapping_add(2)) as *const u8).read() & 0xf0;
                if b == 0x10 || b == 0x60 {
                    let p1 =
                        ((local_24.wrapping_add(0x18)) as *const i32).read_unaligned() as *mut i32;
                    if !p1.is_null() {
                        FUN_1002f322(
                            p1 as i32,
                            &raw mut local_10,
                            &raw mut local_14,
                            &raw mut local_18,
                        );
                        if FUN_10009bd5() == 0 {
                            local_8 = FUN_10006d3e();
                            local_28 = 0;
                        } else {
                            local_8 = 0xb40000;
                            local_28 = -0x2d0000;
                        }
                        FUN_1002f668(p1 as i32, local_28, local_8, 0, 0);
                        FUN_1002fbc0(p1);
                    }
                }
                local_24 = ((local_24.wrapping_add(8)) as *const i32).read_unaligned();
            }
            local_c = (&raw mut DAT_10160c00)
                .wrapping_offset(local_20.wrapping_mul(4) as isize)
                .read_unaligned();
        }
    }
    let _ = (local_10, local_14, local_18);
}
