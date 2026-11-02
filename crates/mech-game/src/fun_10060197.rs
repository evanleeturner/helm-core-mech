// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa540: u32;
    static mut DAT_100aa544: u32;
    static mut DAT_100aa548: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100150df(param_1: i32, param_2: i32, param_3: i32, param_4: i32) -> u32;
    fn FUN_1001b82d(
        param_1: i32,
        param_2: i32,
        param_3: i32,
        param_4: u32,
        param_5: i32,
        param_6: i32,
    ) -> u32;
}

#[inline(always)]
unsafe fn rd_i32(addr: i32) -> i32 {
    // SAFETY: the caller guarantees addr is a readable game address.
    unsafe { (addr as usize as *const i32).read_unaligned() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10060197(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
) -> u32 {
    // SAFETY: all addresses are the game's own structures reachable from
    // param_1, read in the same order as the original.
    unsafe {
        let mut local_8: i32 = 0;
        let mut b_var1 = false;
        let mut i_var2: i32 = rd_i32(param_1.wrapping_add(0x20)).wrapping_add(0x18);
        let i_var3: i32;
        loop {
            if b_var1 {
                return 0;
            }
            let p = rd_i32(param_1.wrapping_add(0x20));
            let c = rd_i32(p.wrapping_add(8))
                .wrapping_add(p)
                .wrapping_add(local_8.wrapping_mul(0x24));
            if rd_i32(c.wrapping_add(0x18)) < 0 {
                let r = FUN_100150df(c, i_var2, param_2, param_4);
                if r != 0 {
                    i_var3 = c;
                    break;
                }
            }
            local_8 = local_8.wrapping_add(1);
            let p2 = rd_i32(param_1.wrapping_add(0x20));
            let s = ((p2.wrapping_add(6)) as usize as *const i16).read_unaligned();
            if (s as i32) == local_8 {
                b_var1 = true;
            }
        }
        let idx =
            ((rd_i32(i_var3.wrapping_add(4)).wrapping_add(i_var3)) as usize as *const u8).read();
        i_var2 = (idx as i32).wrapping_mul(0x44).wrapping_add(i_var2);
        let i_var4 = rd_i32(i_var2.wrapping_add(0x10));
        let r = FUN_1001b82d(
            rd_i32(i_var3.wrapping_add(0x14)),
            rd_i32(i_var3.wrapping_add(0x18)),
            rd_i32(i_var3.wrapping_add(0x1c)),
            0,
            param_2.wrapping_sub(rd_i32(i_var2.wrapping_add(0xc))),
            param_4.wrapping_sub(rd_i32(i_var2.wrapping_add(0x14))),
        ) as i32;
        DAT_100aa540 = (rd_i32(i_var3.wrapping_add(0x14)) >> 0xd) as u32;
        DAT_100aa544 = (rd_i32(i_var3.wrapping_add(0x18)) >> 0xd) as u32;
        DAT_100aa548 = (rd_i32(i_var3.wrapping_add(0x1c)) >> 0xd) as u32;
        if i_var4.wrapping_sub(r) < param_3 {
            return 1;
        }
        0
    }
}
