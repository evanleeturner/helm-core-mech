// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b2ca0: *mut i8;
    static mut DAT_100b2cac: u8;
    static mut DAT_100b2cb0: u8;
    static mut DAT_100b2cb4: u8;
    static mut DAT_100b2cbc: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn strncpy(dest: *mut i8, src: *const i8, n: u32) -> *mut i8;
    fn FUN_10065d78() -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005be24(
    source: *mut i8,
    param_2: i32,
    param_3: i32,
    param_4: i32,
) -> u32 {
    let mut b_var2 = false;
    let mut local_20: u32 = 0;
    let mut local_1c: i32 = -1;
    let mut local_18: i32 = 0;
    let mut local_8: i32 = 0x29a;
    let mut local_14: i32 = 0;
    // SAFETY: the globals are the game's own tables; offsets are the
    // original's 0x24-byte record strides.
    unsafe {
        let base_ptrs = (&raw mut DAT_100b2ca0) as *mut *mut i8;
        let b_ac = &raw mut DAT_100b2cac;
        let b_b0 = &raw mut DAT_100b2cb0;
        let b_b4 = &raw mut DAT_100b2cb4;
        let b_bc = &raw mut DAT_100b2cbc;
        while !b_var2 {
            let off = local_14.wrapping_mul(0x24) as isize;
            if b_ac.offset(off).cast::<i32>().read_unaligned() == 0 {
                local_1c = local_14;
                b_var2 = true;
            } else {
                let v_b0 = b_b0.offset(off).cast::<i32>().read_unaligned();
                if v_b0 <= param_4 {
                    if v_b0 < local_8 {
                        local_1c = local_14;
                        local_8 = b_b0.offset(off).cast::<i32>().read_unaligned();
                        local_18 = b_bc.offset(off).cast::<i32>().read_unaligned();
                    } else if v_b0 == local_8
                        && local_18 < b_bc.offset(off).cast::<i32>().read_unaligned()
                    {
                        local_1c = local_14;
                        local_18 = b_bc.offset(off).cast::<i32>().read_unaligned();
                    }
                }
                local_14 = local_14.wrapping_add(1);
                if local_14 == 2 {
                    b_var2 = true;
                }
            }
        }
        if local_1c != -1 {
            local_20 = 1;
            let i_var1 = local_1c.wrapping_mul(0x24) as isize;
            let slot = base_ptrs.offset(local_1c.wrapping_mul(9) as isize);
            strncpy(slot.read_unaligned(), source, 0xff);
            slot.read_unaligned().add(0xff).write(0);
            b_ac.offset(i_var1).cast::<i32>().write_unaligned(1);
            b_b0.offset(i_var1).cast::<i32>().write_unaligned(param_4);
            let i_var3 = FUN_10065d78();
            b_bc.offset(i_var1)
                .cast::<i32>()
                .write_unaligned((param_3 as u32).wrapping_add(i_var3 as u32) as i32);
            if param_2 < 1 {
                b_b4.offset(i_var1).cast::<i32>().write_unaligned(1);
            } else {
                b_b4.offset(i_var1).cast::<i32>().write_unaligned(param_2);
            }
        }
    }
    local_20
}
