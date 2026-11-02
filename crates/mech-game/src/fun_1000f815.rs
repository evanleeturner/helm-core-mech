// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf38: i32;
    static mut DAT_100adf34: *mut i8;
    static mut DAT_100eef40: [i32; 1];
    static mut DAT_10165863: [u8; 4];
    static mut DAT_1016587a: [u8; 4];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10051f43(param_1: i32) -> i32;
    fn FUN_100082c4(param_1: i32, param_2: i32);
    fn FUN_1000fa08(param_1: i32);
}

unsafe fn rd32(addr: i32) -> i32 {
    // SAFETY: the caller supplies an address valid in the game's memory.
    unsafe { mech_trace::rd!("1000f815", (addr as usize as *const i32), read_unaligned) }
}

unsafe fn wr32(addr: i32, v: i32) {
    // SAFETY: the caller supplies an address valid in the game's memory.
    unsafe { mech_trace::wr!("1000f815", (addr as usize as *mut i32), write_unaligned, v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000f815(param_1: i32, param_2: u32, param_3: i32) -> i32 {
    mech_trace::enter!("1000f815", param_1, param_2, param_3);
    let _ = param_2;
    let mut local_10: i32 = 2;
    let mut local_8: i32 = 0;
    // SAFETY: all addresses are the game's own structures and globals,
    // accessed where the original accesses them.
    unsafe {
        if param_1 == 0
            || rd32(param_1.wrapping_add(0x48)) == 0
            || rd32(param_1.wrapping_add(0x20)) == 0
        {
            local_10 = 0;
        } else {
            if rd32(param_3.wrapping_add(0x20))
                < rd32(rd32(param_1.wrapping_add(0x20)).wrapping_add(0x60))
            {
                local_8 = (rd32(param_3.wrapping_add(0x20)) as u32)
                    .wrapping_mul(0x3a)
                    .wrapping_add(rd32(param_1.wrapping_add(0x20)) as u32)
                    .wrapping_add(100) as i32;
            }
            let p8 = local_8;
            let mut v2: i32 = 0;
            if local_8 != 0 {
                v2 = rd32(p8.wrapping_add(2));
            }
            if local_8 == 0 || v2 == 0 {
                local_10 = 0;
            } else {
                if rd32(param_1.wrapping_add(4))
                    == mech_trace::gr!("1000f815", "100adf38", DAT_100adf38)
                {
                    let p = (&raw mut DAT_10165863).cast::<i32>();
                    let v = mech_trace::rd!("1000f815", p, read_unaligned);
                    mech_trace::wr!("1000f815", p, write_unaligned, v.wrapping_add(1));
                }
                let tbl = (&raw mut DAT_100eef40).cast::<i32>();
                let a0 = mech_trace::rd!(
                    "1000f815",
                    tbl.offset(rd32(param_1.wrapping_add(4)) as isize),
                    read
                );
                let a1 = mech_trace::rd!(
                    "1000f815",
                    tbl.offset(mech_trace::gr!("1000f815", "100adf38", DAT_100adf38) as isize),
                    read
                );
                if rd32(a0.wrapping_add(8)) == rd32(a1.wrapping_add(8)) {
                    let p = (&raw mut DAT_1016587a).cast::<i32>();
                    let v = mech_trace::rd!("1000f815", p, read_unaligned);
                    mech_trace::wr!("1000f815", p, write_unaligned, v.wrapping_add(1));
                }
                v2 = rd32(p8.wrapping_add(2));
                if v2 != -1
                    && (mech_trace::dr!(
                        "1000f815",
                        mech_trace::gr!("1000f815", "100adf34", DAT_100adf34)
                    ) == 0
                        || rd32(param_1.wrapping_add(4))
                            != mech_trace::gr!("1000f815", "100adf38", DAT_100adf38))
                {
                    local_10 = mech_trace::cr!("1000f815", "10051f43", FUN_10051f43(5));
                    local_10 = local_10.wrapping_add(1);
                    if rd32(param_3.wrapping_add(0x24)).wrapping_sub(local_10) < 0 {
                        local_10 = rd32(param_3.wrapping_add(0x24));
                    }
                    v2 = rd32(p8.wrapping_add(2));
                    v2 = v2.wrapping_sub(local_10);
                    wr32(p8.wrapping_add(2), v2);
                    if v2 < 0 {
                        wr32(p8.wrapping_add(2), 0);
                    }
                    let ptr = rd32(p8.wrapping_add(0x36));
                    let sp = (ptr.wrapping_add(2) as usize) as *mut i16;
                    let s = mech_trace::rd!("1000f815", sp, read_unaligned);
                    if s == 0 {
                        let mut a = rd32(p8.wrapping_add(10));
                        a = a.wrapping_add(1);
                        wr32(p8.wrapping_add(10), a);
                        let b = rd32(p8.wrapping_add(6));
                        if b == a {
                            a = a.wrapping_sub(1);
                            wr32(p8.wrapping_add(10), a);
                        } else {
                            let q = rd32(p8.wrapping_add(0x36));
                            wr32(p8.wrapping_add(0x36), q.wrapping_add(0x1e));
                        }
                    } else {
                        mech_trace::wr!("1000f815", sp, write_unaligned, s.wrapping_sub(1));
                    }
                    v2 = rd32(p8.wrapping_add(2));
                    if v2 == 0 {
                        wr32(param_3.wrapping_add(8), -1);
                        local_10 = 0;
                        FUN_100082c4(10, 0);
                    }
                }
                FUN_1000fa08(param_1);
            }
        }
    }
    mech_trace::rt!("1000f815", local_10)
}
