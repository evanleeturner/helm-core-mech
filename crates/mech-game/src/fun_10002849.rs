// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a40f8: u32;
    static mut DAT_100b38fc: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn _AIL_sample_buffer_ready_4(param_1: u32) -> u32;
    fn _AIL_set_sample_pan_8(param_1: u32, param_2: u32) -> u32;
    fn _AIL_set_sample_volume_8(param_1: u32, param_2: u32) -> u32;
    fn _AIL_load_sample_buffer_16(param_1: u32, param_2: u32, param_3: u32, param_4: u32) -> u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000381c(param_1: i32) -> i32;
    fn FUN_100396a4(
        param_1: *mut u8,
        param_2: *mut u8,
        param_3: i32,
        param_4: u32,
        param_5: *mut i32,
    ) -> *mut u8;
}

#[inline(always)]
unsafe fn base() -> u32 {
    // SAFETY: the global is the game's own memory.
    unsafe { DAT_100a40f8 }
}

#[inline(always)]
unsafe fn addr(off: u32, i: u32) -> u32 {
    // SAFETY: reads the game's global base pointer.
    unsafe { base().wrapping_add(off).wrapping_add(i.wrapping_mul(4)) }
}

#[inline(always)]
unsafe fn rd(a: u32) -> u32 {
    // SAFETY: the address lies in the game's sample table.
    unsafe { (a as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn wr(a: u32, v: u32) {
    // SAFETY: the address lies in the game's sample table.
    unsafe { (a as *mut u32).write_unaligned(v) }
}

#[inline(always)]
unsafe fn bufslot(i: u32, ready: u32) -> u32 {
    // SAFETY: reads the game's global base pointer.
    unsafe {
        i.wrapping_mul(8)
            .wrapping_add(ready.wrapping_mul(4))
            .wrapping_add(0x2a8)
            .wrapping_add(base())
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10002849() {
    // SAFETY: all addresses are the game's own tables and functions, accessed
    // exactly as the original does.
    unsafe {
        if DAT_100a40f8 == 0 {
            return;
        }
        let mut i: u32 = 0;
        while i < rd(base().wrapping_add(0x44)) {
            if rd(addr(0x48, i)) != 0
                && rd(addr(0x368, i)) != 0
                && (rd(addr(0x3a8, i)) as i32) <= (DAT_100b38fc as i32)
            {
                let ready = _AIL_sample_buffer_ready_4(rd(addr(4, i)));
                if ready != 0xffff_ffff {
                    if rd(addr(0x428, i)) == 0xffff_ffff {
                        let pan = FUN_1000381c(rd(addr(1000, i)) as i32);
                        _AIL_set_sample_pan_8(rd(addr(4, i)), pan as u32);
                    } else {
                        _AIL_set_sample_pan_8(rd(addr(4, i)), rd(addr(0x428, i)));
                    }
                    _AIL_set_sample_volume_8(rd(addr(4, i)), rd(addr(0x468, i)));
                    if rd(addr(0x168, i)) == 0 {
                        _AIL_load_sample_buffer_16(rd(addr(4, i)), ready, rd(bufslot(i, ready)), 0);
                    } else {
                        let len: u32;
                        if rd(addr(0x228, i)) < rd(addr(0x168, i)) {
                            let p = FUN_100396a4(
                                rd(addr(0x368, i)) as *mut u8,
                                rd(bufslot(i, ready)) as *mut u8,
                                rd(addr(0x228, i)) as i32,
                                rd(addr(0x1a8, i)),
                                i.wrapping_mul(4).wrapping_add(base()).wrapping_add(0x328)
                                    as *mut i32,
                            );
                            wr(addr(0x368, i), p as u32);
                            len = rd(addr(0x268, i));
                            let a = addr(0x168, i);
                            wr(a, rd(a).wrapping_sub(rd(addr(0x228, i))));
                        } else {
                            let p = FUN_100396a4(
                                rd(addr(0x368, i)) as *mut u8,
                                rd(bufslot(i, ready)) as *mut u8,
                                rd(addr(0x168, i)) as i32,
                                rd(addr(0x1a8, i)),
                                i.wrapping_mul(4).wrapping_add(base()).wrapping_add(0x328)
                                    as *mut i32,
                            );
                            wr(addr(0x368, i), p as u32);
                            len = (rd(addr(0x1a8, i)) as i32)
                                .wrapping_mul(rd(addr(0x168, i)) as i32)
                                as u32;
                            wr(addr(0x168, i), 0);
                        }
                        _AIL_load_sample_buffer_16(
                            rd(addr(4, i)),
                            ready,
                            rd(bufslot(i, ready)),
                            len,
                        );
                    }
                }
            }
            i = i.wrapping_add(1);
        }
    }
}
