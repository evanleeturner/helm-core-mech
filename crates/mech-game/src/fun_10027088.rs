// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ad378: [unsafe extern "C" fn(i32, i16, i16) -> i16; 8];
    static mut DAT_100ad398: [unsafe extern "C" fn(i32, *mut i16) -> i32; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10027ad4(param_1: i32);
    fn FUN_1002949b(param_1: i16, param_2: i16, param_3: i16, param_4: i16) -> i16;
    fn FUN_1002a4e0(param_1: i32, param_2: i16, param_3: u16, param_4: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10027088(param_1: i32) -> u32 {
    let mut local_18: i16 = 0;
    let mut b_var2: bool = false;
    // SAFETY: param_1 is the game's object pointer; all offsets, tables and
    // callees are the game's own memory and code, accessed as the original does.
    unsafe {
        FUN_10027ad4(param_1);
        loop {
            let list = ((param_1 as u32).wrapping_add(0x144) as *const i32).read_unaligned();
            let off = (local_18 as i32).wrapping_mul(4);
            let entry = (list.wrapping_add(off) as *const i32).read_unaligned();
            if entry == 0 {
                return 0;
            }
            let p_var1 = entry as *mut i16;
            local_18 = local_18.wrapping_add(1);
            let s17a = ((param_1.wrapping_add(0x17a)) as *const i16).read_unaligned();
            let s178 = ((param_1.wrapping_add(0x178)) as *const i16).read_unaligned();
            let mut s_var4 = FUN_1002949b(p_var1.byte_add(2).read_unaligned(), s17a, 0, s178);
            let s_var5 = FUN_1002949b(p_var1.byte_add(4).read_unaligned(), s17a, 0, s178);
            let first = p_var1.read_unaligned();
            let p170 = (param_1.wrapping_add(0x170)) as *mut i16;
            let p172 = (param_1.wrapping_add(0x172)) as *const u16;
            if p170.read_unaligned() == first
                && (p172.read_unaligned() as u32) == (s_var4 as i32 as u32)
            {
                b_var2 = true;
                p170.write_unaligned(0);
            }
            if !b_var2 {
                let b_var3;
                if first < 8 {
                    let f = (&raw const DAT_100ad378)
                        .cast::<unsafe extern "C" fn(i32, i16, i16) -> i16>()
                        .offset(first as isize)
                        .read();
                    s_var4 = f(param_1, s_var4, s_var5);
                    b_var3 = s_var4 != 0;
                } else {
                    b_var3 = false;
                }
                if !b_var3 {
                    continue;
                }
            }
            let idx = p_var1.byte_add(6).cast::<u8>().read() as usize;
            let g = (&raw const DAT_100ad398)
                .cast::<unsafe extern "C" fn(i32, *mut i16) -> i32>()
                .add(idx)
                .read();
            let i_var7 = g(param_1, p_var1);
            if i_var7 != 0 {
                let tmp9 = p_var1.byte_add(9).read_unaligned();
                let s17a = ((param_1.wrapping_add(0x17a)) as *const i16).read_unaligned();
                let s178 = ((param_1.wrapping_add(0x178)) as *const i16).read_unaligned();
                let u_var6 = FUN_1002949b(tmp9, s17a, s_var4, s178) as u16;
                let tmp7 = p_var1.byte_add(7).read_unaligned();
                FUN_1002a4e0(param_1, tmp7, u_var6, 0);
                return 1;
            }
        }
    }
}
