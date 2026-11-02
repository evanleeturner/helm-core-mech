// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10065a2b(
    param_1: *mut i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: u8,
) -> u32 {
    // SAFETY: param_1 points at the game's surface-clip record whose first
    // word points at the surface header; the pixel pointer is the game's own
    // buffer, written where the original does.
    unsafe {
        let mut param_2 = param_2;
        let mut param_3 = param_3;
        let mut param_4 = param_4;
        let mut param_5 = param_5;

        let pi_var1 = param_1.read_unaligned() as usize as *const i32;
        let mut i_var9 = pi_var1.add(1).read_unaligned();
        let i_var5 = i_var9.wrapping_add(1);
        if i_var5 == 0 || i_var9 < -1 {
            return 0xffff_ffff;
        }
        let mut i_var11 = pi_var1.add(2).read_unaligned();
        if i_var11 < 0 {
            return 0xffff_ffff;
        }
        let i_var2 = param_1.add(1).read_unaligned();
        let mut i_var6 = i_var2;
        if i_var2 < 1 {
            i_var6 = 0;
        }
        let i_var3 = param_1.add(2).read_unaligned();
        let mut i_var7 = i_var3;
        if i_var3 < 1 {
            i_var7 = 0;
        }
        let mut i_var8 = param_1.add(3).read_unaligned();
        if i_var9 <= param_1.add(3).read_unaligned() {
            i_var8 = i_var9;
        }
        i_var9 = param_1.add(4).read_unaligned();
        if i_var11 <= param_1.add(4).read_unaligned() {
            i_var9 = i_var11;
        }
        if i_var6 <= i_var8 && i_var7 <= i_var9 {
            param_2 = param_2.wrapping_add(i_var2);
            param_4 = param_4.wrapping_add(i_var2);
            param_3 = param_3.wrapping_add(i_var3);
            param_5 = param_5.wrapping_add(i_var3);
            if param_2 <= i_var6 {
                param_2 = i_var6;
            }
            if param_3 <= i_var7 {
                param_3 = i_var7;
            }
            if i_var8 <= param_4 {
                param_4 = i_var8;
            }
            if i_var9 <= param_5 {
                param_5 = i_var9;
            }
            if param_2 <= param_4 {
                let base = pi_var1.read_unaligned();
                let mut puvar10 = param_3
                    .wrapping_mul(i_var5)
                    .wrapping_add(base)
                    .wrapping_add(param_2) as usize as *mut u8;
                let mut u_var12 = param_5.wrapping_sub(param_3) as u32;
                if param_3 <= param_5 {
                    loop {
                        i_var9 = param_4.wrapping_sub(param_2).wrapping_add(1);
                        let mut puvar13 = puvar10;
                        if (u_var12 & 1) != 0 {
                            i_var9 = param_4.wrapping_sub(param_2);
                            puvar13 = puvar10.wrapping_add(1);
                        }
                        loop {
                            puvar13.write(param_6);
                            i_var11 = i_var9.wrapping_sub(2);
                            let b_var4 = 1 < i_var9;
                            i_var9 = i_var11;
                            puvar13 = puvar13.wrapping_add(2);
                            if !(i_var11 != 0 && b_var4) {
                                break;
                            }
                        }
                        u_var12 = u_var12.wrapping_sub(1);
                        if (u_var12 as i32) < 0 {
                            break;
                        }
                        puvar10 = puvar10.wrapping_offset(i_var5 as isize);
                    }
                    return 0;
                }
            }
            return 0xffff_fffc;
        }
        0xffff_fffe
    }
}
