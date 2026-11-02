// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a9cb1: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1006a7b5(param_1: *mut i32, param_2: *mut u32) -> u64 {
    // SAFETY: the global table and the caller's structures are the game's own
    // memory; reads and writes follow the original's order and offsets.
    unsafe {
        let tbl = &raw mut DAT_100a9cb1;
        let mut puvar9 = tbl as *mut u32;
        let mut i: i32 = 0x40;
        while i != 0 {
            puvar9.write_unaligned(0);
            puvar9 = puvar9.add(1);
            i = i.wrapping_sub(1);
        }
        let mut param_2 = param_2;
        let i_var5 = param_1.add(3).read_unaligned();
        let i_var1 = param_1.add(1).read_unaligned();
        let mut i_var7 = param_1
            .add(4)
            .read_unaligned()
            .wrapping_sub(param_1.add(2).read_unaligned());
        let base = param_1.read_unaligned();
        let i_var3 = ((base as u32).wrapping_add(4) as *const i32)
            .read_unaligned()
            .wrapping_add(1);
        let l_var2: i64 = (param_1.add(2).read_unaligned() as i64).wrapping_mul(i_var3 as i64);
        let mut i_var8 = (base as u32 as *const i32)
            .read_unaligned()
            .wrapping_add(l_var2 as i32)
            .wrapping_add(param_1.add(1).read_unaligned());
        let mut local_8: i32 = -1;
        let mut i_var6 = i_var5.wrapping_sub(i_var1);
        loop {
            loop {
                let addr = i_var6.wrapping_add(i_var8) as u32 as *const u8;
                let u_var4 = u32::from(addr.read());
                let mut puvar10 = param_2;
                let p = tbl.byte_add(u_var4 as usize);
                if p.read() == 0 {
                    p.write(p.read() | 1);
                    local_8 = local_8.wrapping_add(1);
                    if !param_2.is_null() {
                        puvar10 = param_2.add(1);
                        param_2.write_unaligned(u_var4);
                    }
                }
                i_var6 = i_var6.wrapping_sub(1);
                param_2 = puvar10;
                if !(-1 < i_var6) {
                    break;
                }
            }
            i_var8 = i_var8.wrapping_add(i_var3);
            i_var7 = i_var7.wrapping_sub(1);
            i_var6 = i_var5.wrapping_sub(i_var1);
            if !(-1 < i_var7) {
                break;
            }
        }
        let hi = (((l_var2 as u64) >> 32) as u32 as i32) as i64 as u64;
        (hi << 32) | (local_8.wrapping_add(1) as i64 as u64)
    }
}
