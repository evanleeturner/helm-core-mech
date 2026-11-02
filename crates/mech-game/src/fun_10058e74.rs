// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10058e74(param_1: *mut i32, param_2: u8) -> u32 {
    // SAFETY: param_1 points at the caller's descriptor array and its first
    // element points at the surface header and pixel buffer, accessed where
    // the original does.
    unsafe {
        let pi_var1 = *param_1 as u32 as *mut i32;
        let mut i_var7 = pi_var1.add(1).read_unaligned();
        let i_var3 = (i_var7 as u32).wrapping_add(1) as i32;
        if i_var3 == 0 || i_var7 < -1 {
            return 0xffff_ffff;
        }
        let i_var2 = pi_var1.add(2).read_unaligned();
        if i_var2 < 0 {
            return 0xffff_ffff;
        }
        let mut i_var4 = param_1.add(1).read_unaligned();
        if i_var4 < 1 {
            i_var4 = 0;
        }
        let mut i_var5 = param_1.add(2).read_unaligned();
        if i_var5 < 1 {
            i_var5 = 0;
        }
        let mut i_var6 = param_1.add(3).read_unaligned();
        if i_var7 <= param_1.add(3).read_unaligned() {
            i_var6 = i_var7;
        }
        i_var7 = param_1.add(4).read_unaligned();
        if i_var2 <= param_1.add(4).read_unaligned() {
            i_var7 = i_var2;
        }
        if i_var4 <= i_var6 && i_var5 <= i_var7 {
            let addr = (i_var5 as u32)
                .wrapping_mul(i_var3 as u32)
                .wrapping_add(pi_var1.read_unaligned() as u32)
                .wrapping_add(i_var4 as u32);
            let mut p = addr as usize as *mut u8;
            let u_var10 = (i_var6 as u32).wrapping_add(1).wrapping_sub(i_var4 as u32);
            let fill = (param_2 as u32).wrapping_mul(0x0101_0101);
            while i_var5 <= i_var7 {
                let mut u_var9 = u_var10 & 3;
                while u_var9 != 0 {
                    p.write(param_2);
                    p = p.wrapping_add(1);
                    u_var9 = u_var9.wrapping_sub(1);
                }
                u_var9 = u_var10 >> 2;
                while u_var9 != 0 {
                    p.cast::<u32>().write_unaligned(fill);
                    p = p.wrapping_add(4);
                    u_var9 = u_var9.wrapping_sub(1);
                }
                p = p.wrapping_add((i_var3 as u32).wrapping_sub(u_var10) as usize);
                i_var5 = (i_var5 as u32).wrapping_add(1) as i32;
            }
            return 0;
        }
        0xffff_fffe
    }
}
