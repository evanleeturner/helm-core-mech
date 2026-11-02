// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10092000: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100328f0(param_1: i32) -> i32 {
    mech_trace::enter!("100328f0", param_1);
    let mut i_var1: i32 = param_1.wrapping_sub(param_1 % 100);
    let mut i_var4: i32 = 0;
    // SAFETY: the global and the table it points to are the game's own memory;
    // reads and writes happen at the same offsets and in the same order as the original.
    unsafe {
        'outer: loop {
            i_var4 = i_var4.wrapping_add(1);
            let mut local_c: i32 = 8;
            let mut local_10: *const i32 = mech_trace::gr!("100328f0", "10092000", DAT_10092000)
                .wrapping_add(0x368) as *const i32;
            loop {
                let mut i_var3: i32 = 0xc;
                let mut pi_var2: *const i32 = local_10;
                loop {
                    let v = mech_trace::rd!("100328f0", pi_var2, read_unaligned);
                    if v / -100 == i_var1 / -100 && v % 100 == i_var4 {
                        continue 'outer;
                    }
                    pi_var2 = pi_var2.wrapping_add(1);
                    i_var3 = i_var3.wrapping_sub(1);
                    if i_var3 == 0 {
                        break;
                    }
                }
                local_10 = local_10.wrapping_add(0xc);
                local_c = local_c.wrapping_sub(1);
                if local_c == 0 {
                    i_var3 = 0;
                    i_var1 = i_var1.wrapping_add(i_var4);
                    let mut p: *const i32 = mech_trace::gr!("100328f0", "10092000", DAT_10092000)
                        .wrapping_add(0x2f0)
                        as *const i32;
                    loop {
                        if mech_trace::rd!("100328f0", p, read_unaligned) == -1 {
                            let base = mech_trace::gr!("100328f0", "10092000", DAT_10092000);
                            let off = (i_var3 as u32).wrapping_mul(8);
                            mech_trace::wr!(
                                "100328f0",
                                (base.wrapping_add(0x2f0).wrapping_add(off) as *mut i32),
                                write_unaligned,
                                i_var1
                            );
                            let base = mech_trace::gr!("100328f0", "10092000", DAT_10092000);
                            mech_trace::wr!(
                                "100328f0",
                                (base.wrapping_add(0x2f4).wrapping_add(off) as *mut u32),
                                write_unaligned,
                                0xffffffff
                            );
                            return mech_trace::rt!("100328f0", i_var1);
                        }
                        p = p.wrapping_add(2);
                        i_var3 = i_var3.wrapping_add(1);
                        if i_var3 >= 0xe {
                            break;
                        }
                    }
                    return mech_trace::rt!("100328f0", 0);
                }
            }
        }
    }
}
