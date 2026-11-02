// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1008b958: i32;
    static mut DAT_100b0328: i32;
    static mut DAT_100b032c: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn __ftol() -> i32;
}

#[inline(always)]
unsafe fn rd(addr: i32) -> i32 {
    // SAFETY: the caller passes an address inside the game's own structure.
    unsafe { (addr as u32 as usize as *const i32).read_unaligned() }
}

#[inline(always)]
unsafe fn wr(addr: i32, v: i32) {
    // SAFETY: the caller passes an address inside the game's own structure.
    unsafe { (addr as u32 as usize as *mut i32).write_unaligned(v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000a910(param_1: i32) {
    // SAFETY: param_1 points at the game's structure; the globals and
    // __ftol are the game's own, accessed in the original's order.
    unsafe {
        if rd(param_1.wrapping_add(0x84f)) == 0 {
            DAT_100b0328 = 0x33333333;
            DAT_100b032c = 0x3ff33333;
        } else {
            DAT_100b0328 = 0x1eb851ec;
            DAT_100b032c = 0x3ff1eb85;
        }
        let pi_var1 = param_1.wrapping_add(0x258);
        let i_var2: i32;
        if rd(param_1.wrapping_add(0x24c)) == 0 {
            i_var2 = rd(param_1.wrapping_add(0x250)).wrapping_shl(4);
        } else {
            i_var2 = __ftol();
        }
        let mut pi_var4 = param_1.wrapping_add(0x76c);
        let mut i_var3: i32 = 8;
        wr(param_1.wrapping_add(0x25c), 0);
        wr(pi_var1, i_var2.wrapping_add(0x31) / 100);
        loop {
            let v = rd(pi_var4.wrapping_sub(4)).wrapping_add(rd(param_1.wrapping_add(0x25c)));
            wr(param_1.wrapping_add(0x25c), v);
            let e = rd(pi_var4);
            if -1 < e {
                wr(param_1.wrapping_add(0x25c), e.wrapping_add(v));
            }
            pi_var4 = pi_var4.wrapping_add(0x12);
            i_var3 = i_var3.wrapping_sub(1);
            if i_var3 == 0 {
                break;
            }
        }
        if rd(pi_var1) < rd(param_1.wrapping_add(0x25c)) {
            loop {
                let a = DAT_1008b958
                    .wrapping_mul(0x12)
                    .wrapping_add(0x768)
                    .wrapping_add(param_1);
                let v = rd(a);
                if 0 < v {
                    wr(a, v.wrapping_sub(1));
                    wr(
                        param_1.wrapping_add(0x25c),
                        rd(param_1.wrapping_add(0x25c)).wrapping_sub(1),
                    );
                }
                if rd(param_1.wrapping_add(0x25c)) <= rd(pi_var1) {
                    return;
                }
                let b = DAT_1008b958
                    .wrapping_mul(0x12)
                    .wrapping_add(0x76c)
                    .wrapping_add(param_1);
                let v2 = rd(b);
                if 0 < v2 {
                    wr(b, v2.wrapping_sub(1));
                    wr(
                        param_1.wrapping_add(0x25c),
                        rd(param_1.wrapping_add(0x25c)).wrapping_sub(1),
                    );
                }
                DAT_1008b958 = DAT_1008b958.wrapping_add(1);
                if DAT_1008b958 > 7 {
                    DAT_1008b958 = 0;
                }
                if !(rd(pi_var1) < rd(param_1.wrapping_add(0x25c))) {
                    break;
                }
            }
        }
    }
}
