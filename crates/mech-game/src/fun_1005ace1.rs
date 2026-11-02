// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005ace1(
    param_1: *mut i32,
    param_2: i32,
    param_3: *mut u32,
    param_4: *mut u8,
) -> *mut u32 {
    // SAFETY: all pointers are the caller's own structures and buffers; reads
    // and writes happen in the same order and extent as the original.
    unsafe {
        let mut param_3 = param_3 as *mut u8;
        let mut n: u32 = param_4 as usize as u32;
        let pi_var1 = param_1.read_unaligned() as u32 as usize as *mut i32;
        let i_var2 = pi_var1.byte_add(4).read_unaligned();
        if i_var2.wrapping_add(1) == 0 || i_var2 < -1 {
            return 0xffff_ffffusize as *mut u32;
        }
        let i_var3 = pi_var1.byte_add(8).read_unaligned();
        if i_var3 < 0 {
            return 0xffff_ffffusize as *mut u32;
        }
        let mut local_8 = param_1.byte_add(4).read_unaligned();
        let i_var5 = if local_8 < 1 { 0 } else { local_8 };
        let i_var4 = param_1.byte_add(8).read_unaligned();
        let i_var6 = if i_var4 < 1 { 0 } else { i_var4 };
        let mut i_var7 = param_1.byte_add(12).read_unaligned();
        if i_var2 <= param_1.byte_add(12).read_unaligned() {
            i_var7 = i_var2;
        }
        let mut i_var8 = param_1.byte_add(16).read_unaligned();
        if i_var3 <= param_1.byte_add(16).read_unaligned() {
            i_var8 = i_var3;
        }
        if i_var5 <= i_var7 && i_var6 <= i_var8 {
            let i_var4 = param_2.wrapping_add(i_var4);
            let mut p9: i32 = i_var7
                .wrapping_sub(local_8)
                .wrapping_add(1)
                .wrapping_sub(n as i32);
            let mut ok = p9 > -1;
            if !ok {
                let (r, of) = (n as i32).overflowing_add(p9);
                n = r as u32;
                ok = n != 0 && of == ((n as i32) < 0);
            }
            if ok {
                p9 = local_8.wrapping_sub(i_var5);
                if p9 < 0 {
                    let (r, of) = (n as i32).overflowing_add(p9);
                    n = r as u32;
                    if n == 0 || of != ((n as i32) < 0) {
                        return p9 as u32 as usize as *mut u32;
                    }
                    param_3 = param_3.byte_offset(0isize.wrapping_sub(p9 as isize));
                    local_8 = local_8.wrapping_sub(p9);
                }
                p9 = i_var8.wrapping_sub(i_var4);
                if p9 > -1 {
                    p9 = i_var4.wrapping_sub(i_var6);
                    if p9 > -1 {
                        p9 = i_var4
                            .wrapping_mul(i_var2.wrapping_add(1))
                            .wrapping_add(pi_var1.read_unaligned())
                            .wrapping_add(local_8);
                        let mut dst = p9 as u32 as usize as *mut u8;
                        let mut c = n & 3;
                        while c != 0 {
                            dst.write(param_3.read());
                            param_3 = param_3.byte_add(1);
                            dst = dst.byte_add(1);
                            c -= 1;
                        }
                        let mut c = n >> 2;
                        while c != 0 {
                            dst.cast::<u32>()
                                .write_unaligned(param_3.cast::<u32>().read_unaligned());
                            param_3 = param_3.byte_add(4);
                            dst = dst.byte_add(4);
                            c -= 1;
                        }
                    }
                }
            }
            return p9 as u32 as usize as *mut u32;
        }
        0xffff_fffeusize as *mut u32
    }
}
