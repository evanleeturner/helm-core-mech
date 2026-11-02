// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b24f9: [u32; 256];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005bc15(param_1: *mut i32, param_2: *mut u32) -> u64 {
    mech_trace::enter!("1005bc15", param_1, param_2);
    // SAFETY: the global table and the caller's structures are the game's own
    // memory; reads and writes happen in the original's order.
    unsafe {
        let tbl = (&raw mut DAT_100b24f9) as *mut u32;
        let mut param_2 = param_2;
        for i in 0..0x40usize {
            mech_trace::wr!("1005bc15", tbl.add(i), write_unaligned, 0);
        }
        let p0 = mech_trace::rd!("1005bc15", param_1, read_unaligned);
        let p1 = mech_trace::rd!("1005bc15", param_1.byte_add(4), read_unaligned);
        let p2 = mech_trace::rd!("1005bc15", param_1.byte_add(8), read_unaligned);
        let p3 = mech_trace::rd!("1005bc15", param_1.byte_add(12), read_unaligned);
        let p4 = mech_trace::rd!("1005bc15", param_1.byte_add(16), read_unaligned);

        let i_var5 = p3;
        let i_var1 = p1;
        let mut i_var7 = p4.wrapping_sub(p2);
        let i_var3 = mech_trace::rd!(
            "1005bc15",
            (p0 as usize as *const i32).byte_add(4),
            read_unaligned
        )
        .wrapping_add(1);
        let l_var2: i64 = (p2 as i64).wrapping_mul(i_var3 as i64);
        let base = mech_trace::rd!("1005bc15", (p0 as usize as *const i32), read_unaligned);
        let mut i_var8 = base.wrapping_add(l_var2 as i32).wrapping_add(p1);
        let mut local_8: i32 = -1;
        let mut i_var6 = i_var5.wrapping_sub(i_var1);
        loop {
            loop {
                let addr = i_var6.wrapping_add(i_var8) as u32 as usize as *const u8;
                let u_var4 = mech_trace::rd!("1005bc15", addr, read) as u32;
                let mut puvar10 = param_2;
                let slot = tbl.add(u_var4 as usize);
                if mech_trace::rd!("1005bc15", slot, read_unaligned) == 0 {
                    mech_trace::wr!(
                        "1005bc15",
                        slot,
                        write_unaligned,
                        mech_trace::rd!("1005bc15", slot, read_unaligned) | 1
                    );
                    local_8 = local_8.wrapping_add(1);
                    if !param_2.is_null() {
                        puvar10 = param_2.add(1);
                        mech_trace::wr!("1005bc15", param_2, write_unaligned, u_var4);
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
        let hi = ((l_var2 as u64) >> 32) as u32;
        mech_trace::rt!(
            "1005bc15",
            ((hi as u64) << 32) | (local_8.wrapping_add(1) as u32 as u64)
        )
    }
}
