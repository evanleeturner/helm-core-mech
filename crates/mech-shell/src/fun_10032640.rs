// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10092000: u32;
}

#[inline(always)]
unsafe fn slot(off: i32) -> *mut i32 {
    // SAFETY: DAT_10092000 is the game's own pointer to its state block.
    unsafe {
        mech_trace::gr!("10032640", "10092000", DAT_10092000)
            .wrapping_add(0x368)
            .wrapping_add(off as u32) as usize as *mut i32
    }
}

#[inline(always)]
unsafe fn fill(base: i32, param_3: &mut i32, param_2: i32) {
    let mut i_var1: i32 = 0;
    if mech_trace::dr!("10032640", param_3) != 0 {
        let mut i_var3 = base;
        loop {
            if i_var1 > 0xb {
                break;
            }
            // SAFETY: slot addresses lie within the game's state block.
            unsafe {
                if mech_trace::rd!("10032640", slot(i_var3), read_unaligned) == 0 {
                    mech_trace::dw!("10032640", param_3, param_3.wrapping_sub(1));
                    mech_trace::wr!("10032640", slot(i_var3), write_unaligned, param_2);
                }
            }
            i_var3 = i_var3.wrapping_add(4);
            i_var1 += 1;
            if mech_trace::dr!("10032640", param_3) == 0 {
                break;
            }
        }
    }
}

#[inline(always)]
unsafe fn clear_two(a: i32, b: i32, param_2: i32) {
    let mut i_var1 = b;
    let mut i_var3 = a;
    let mut i_var2: i32 = 0xc;
    loop {
        // SAFETY: slot addresses lie within the game's state block.
        unsafe {
            if mech_trace::rd!("10032640", slot(i_var3), read_unaligned) == param_2 {
                mech_trace::wr!("10032640", slot(i_var3), write_unaligned, 0);
            }
            if mech_trace::rd!("10032640", slot(i_var1), read_unaligned) == param_2 {
                mech_trace::wr!("10032640", slot(i_var1), write_unaligned, 0);
            }
        }
        i_var1 = i_var1.wrapping_add(4);
        i_var3 = i_var3.wrapping_add(4);
        i_var2 -= 1;
        if i_var2 == 0 {
            break;
        }
    }
}

#[inline(always)]
fn pick(param_1: i32) -> i32 {
    match param_1 {
        1 => 4,
        3 => 5,
        4 => 1,
        5 => 3,
        _ => 0,
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10032640(param_1: i32, param_2: i32, mut param_3: i32) -> u32 {
    mech_trace::enter!("10032640", param_1, param_2, param_3);
    if param_2 < 1 {
        return mech_trace::rt!("10032640", 0);
    }
    // SAFETY: all accesses are to the game's own state block via DAT_10092000.
    unsafe {
        fill(param_1.wrapping_mul(0x30), &mut param_3, param_2);
        let i_var1 = param_2.wrapping_sub(param_2 % 100);
        if i_var1 == 0xd48 && param_3 != 0 {
            let i_stack_4 = pick(param_1);
            fill(i_stack_4.wrapping_mul(0x30), &mut param_3, param_2);
            if param_3 != 0 {
                clear_two(
                    param_1.wrapping_mul(0x30),
                    i_stack_4.wrapping_mul(0x30),
                    param_2,
                );
                return mech_trace::rt!("10032640", 0);
            }
        } else if i_var1 == 0x1964 {
            if param_3 != 0 {
                let i_stack_4 = pick(param_1);
                fill(i_stack_4.wrapping_mul(0x30), &mut param_3, param_2);
                if param_3 != 0 {
                    clear_two(
                        param_1.wrapping_mul(0x30),
                        i_stack_4.wrapping_mul(0x30),
                        param_2,
                    );
                    return mech_trace::rt!("10032640", 0);
                }
            }
        } else if param_3 != 0 {
            let mut i_var1 = param_1.wrapping_mul(0x30);
            let mut i_var3: i32 = 0xc;
            loop {
                if mech_trace::rd!("10032640", slot(i_var1), read_unaligned) == param_2 {
                    mech_trace::wr!("10032640", slot(i_var1), write_unaligned, 0);
                }
                i_var1 = i_var1.wrapping_add(4);
                i_var3 -= 1;
                if i_var3 == 0 {
                    break;
                }
            }
            return mech_trace::rt!("10032640", 0);
        }
    }
    mech_trace::rt!("10032640", 1)
}
