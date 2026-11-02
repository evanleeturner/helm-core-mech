// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b17f9: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1005ace1(
        param_1: *mut i32,
        param_2: i32,
        param_3: *mut i32,
        param_4: *mut core::ffi::c_void,
    );
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005b057(param_1: *mut i32, param_2: i32) -> i32 {
    // SAFETY: param_2 is the address of the caller's record, read where the
    // original reads; the global buffer and callee are the game's own.
    unsafe {
        let s_var3 = ((param_2.wrapping_add(10)) as *const i16).read_unaligned();
        let s_var4 = ((param_2.wrapping_add(6)) as *const i16).read_unaligned();
        let mut i_var9: i32 = 0;
        let u_var5 = ((param_2.wrapping_add(0x42)) as *const u16).read_unaligned();
        let mut pu_var10: *const u8 = param_2.wrapping_add(0x80) as *const u8;
        let base: *mut u8 = &raw mut DAT_100b17f9;
        loop {
            let mut pu_var11: *mut u8 = base;
            loop {
                let pu_var1 = pu_var10.add(1);
                let b = pu_var10.read() as u16;
                let u_var6: u16 = ((b << 8) | b) & 0xC0FF;
                let u_var7: u16 = u_var6 ^ 0xC000;
                let pu_var12: *mut u8;
                if ((u_var7 >> 8) as u8) == 0 {
                    pu_var10 = pu_var10.add(2);
                    let u_var2 = pu_var1.read();
                    let mut p = pu_var11;
                    let mut u_var8: u32 = (u_var7 & 0x3F) as u32;
                    while u_var8 != 0 {
                        p.write(u_var2);
                        p = p.add(1);
                        u_var8 = u_var8.wrapping_sub(1);
                    }
                    pu_var12 = p;
                } else {
                    pu_var12 = pu_var11.add(1);
                    pu_var11.write(u_var6 as u8);
                    pu_var10 = pu_var1;
                }
                pu_var11 = pu_var12;
                let end = (base as usize as i32).wrapping_add(u_var5 as i32);
                if !((pu_var12 as usize as i32) < end) {
                    break;
                }
            }
            FUN_1005ace1(
                param_1,
                i_var9,
                base as *mut i32,
                u_var5 as usize as *mut core::ffi::c_void,
            );
            i_var9 = i_var9.wrapping_add(1);
            if !(i_var9 <= (s_var3.wrapping_sub(s_var4) as u16 as i32)) {
                break;
            }
        }
        0
    }
}
