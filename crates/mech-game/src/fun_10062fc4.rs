// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7ba8: u8;
}

#[inline(always)]
unsafe fn rd(addr: i32) -> i32 {
    // SAFETY: the address is a game structure field read where the original reads it.
    unsafe { (addr as usize as *const i32).read_unaligned() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10062fc4(param_1: i32) {
    let mut b_var3 = false;
    // SAFETY: param_1 points at the game's structure; all offsets and the
    // global table are the game's own memory, accessed as the original does.
    unsafe {
        if rd(param_1.wrapping_add(0x3dc)) < 1 {
            return;
        }
        let i_var1 = rd(rd(param_1.wrapping_add(0x3e0))
            .wrapping_mul(0x40)
            .wrapping_add(0xc)
            .wrapping_add(rd(param_1.wrapping_add(0x54))));
        let mut local_20 = i_var1;
        loop {
            local_20 = local_20.wrapping_add(1) % 0xe;
            if local_20 == i_var1 {
                return;
            }
            if b_var3 {
                return;
            }
            let mut local_1c: i32 = 0;
            while local_1c < 0xe {
                let off = local_1c.wrapping_mul(0x40);
                let i_var2 = rd(rd(param_1.wrapping_add(0x54))
                    .wrapping_add(4)
                    .wrapping_add(off));
                if (-1 < i_var2)
                    && (rd(rd(param_1.wrapping_add(0x54)).wrapping_add(off)) != -1)
                    && (rd(rd(param_1.wrapping_add(0x54))
                        .wrapping_add(0xc)
                        .wrapping_add(off))
                        == local_20)
                    && (rd(rd(param_1.wrapping_add(0x54))
                        .wrapping_add(8)
                        .wrapping_add(off))
                        != -1)
                {
                    let b = (&raw mut DAT_100a7ba8)
                        .byte_offset(i_var2.wrapping_mul(0x68) as isize)
                        .read();
                    if (b & 4) != 0 {
                        b_var3 = true;
                        (param_1.wrapping_add(0x3e0) as usize as *mut i32)
                            .write_unaligned(local_1c);
                        break;
                    }
                }
                local_1c = local_1c.wrapping_add(1);
            }
        }
    }
}
