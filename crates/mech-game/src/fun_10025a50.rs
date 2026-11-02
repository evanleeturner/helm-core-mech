// SPDX-License-Identifier: MIT

unsafe extern "C" {
    fn FUN_100150df(param_1: i32, param_2: i32, param_3: i32, param_4: i32) -> i32;
    fn FUN_100141fb(
        param_1: i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: *mut i32,
    ) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10025a50(
    param_1: *mut i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
) -> i32 {
    let mut local_1c: i32 = 0;
    let mut local_8: i32;

    if param_1.is_null() {
        return 0;
    }
    // SAFETY: param_1 points at the caller's node of i32 fields; child
    // pointers and item pointers are the game's own and are used as the
    // original does.
    unsafe {
        let rd = |i: usize| param_1.add(i).read_unaligned();
        if rd(0) > param_3 || rd(1) < param_3 || param_5 < rd(4) || rd(5) < param_5 {
            local_8 = 0;
        } else if rd(3) < param_4 {
            local_8 = 2;
        } else if param_4 < rd(2) {
            local_8 = 3;
        } else if rd(6) == 0 {
            local_8 = FUN_10025a50(rd(7) as *mut i32, param_2, param_3, param_4, param_5);
            if local_8 == 0 {
                local_8 = FUN_10025a50(rd(8) as *mut i32, param_2, param_3, param_4, param_5);
                if local_8 == 0 {
                    local_8 = FUN_10025a50(rd(9) as *mut i32, param_2, param_3, param_4, param_5);
                    if local_8 == 0 {
                        local_8 =
                            FUN_10025a50(rd(10) as *mut i32, param_2, param_3, param_4, param_5);
                        if local_8 == 0 {
                            local_8 = 2;
                        }
                    }
                }
            }
        } else {
            let local_c = (param_2 as u32).wrapping_add(0x18) as i32;
            let mut local_10 = param_1.add(0xb);
            let mut local_14: i32 = 0;
            while local_14 < rd(6) {
                let local_18 = local_10.read_unaligned();
                local_10 = local_10.add(1);
                let cnt = ((local_18 as u32).wrapping_add(0x18) as *const i32).read_unaligned();
                if 0 < cnt {
                    let iVar2 = FUN_100150df(local_18, local_c, param_3, param_5);
                    if iVar2 != 0 {
                        let iVar2 = FUN_100141fb(
                            local_18,
                            local_c,
                            param_3,
                            param_4,
                            param_5,
                            &raw mut local_1c,
                        );
                        if iVar2 != 0 {
                            return 3;
                        }
                        return 2;
                    }
                }
                local_14 = local_14.wrapping_add(1);
            }
            local_8 = 2;
        }
    }
    local_8
}
