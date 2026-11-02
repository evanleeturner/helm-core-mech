// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b2b10: [u8; 256];
}

#[inline(always)]
unsafe fn rd(a: u32) -> u8 {
    // SAFETY: the address is one the original reads; the caller guarantees it is valid.
    unsafe { mech_trace::rd!("100579a2", (a as *const u8), read) }
}

#[inline(always)]
unsafe fn wr(a: u32, v: u8) {
    // SAFETY: the address is one the original writes; the caller guarantees it is valid.
    unsafe { mech_trace::wr!("100579a2", (a as *mut u8), write, v) }
}

#[inline(always)]
unsafe fn tb(i: u8) -> u8 {
    // SAFETY: the table is the game's own 256-byte array and i is at most 255.
    unsafe {
        mech_trace::rd!(
            "100579a2",
            (&raw const DAT_100b2b10).cast::<u8>().add(i as usize),
            read
        )
    }
}

const FILL: u8 = 0;
const COPY: u8 = 1;
const JOINED: u8 = 2;
const SKIP: u8 = 3;
const AAE: u8 = 4;
const ROWEND: u8 = 5;

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100579a2(
    param_1: *mut i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
) -> u32 {
    mech_trace::enter!("100579a2", param_1, param_2, param_3, param_4);
    // SAFETY: all pointers and addresses are the game's own buffers, accessed
    // exactly where the original accesses them.
    unsafe {
        let base = mech_trace::rd!("100579a2", param_1, read) as u32;
        let mut i_var2 = mech_trace::rd!(
            "100579a2",
            ((base.wrapping_add(4)) as *const i32),
            read_unaligned
        );
        let i_var5 = i_var2.wrapping_add(1);
        if i_var5 != 0 && -2 < i_var2 {
            i_var2 = mech_trace::rd!(
                "100579a2",
                ((param_2 as u32).wrapping_add(0xc) as *const i32),
                read_unaligned
            );
            let a0 = mech_trace::rd!("100579a2", (base as *const i32), read_unaligned) as u32;
            let a1 = mech_trace::rd!(
                "100579a2",
                ((param_2 as u32).wrapping_add(8) as *const i32),
                read_unaligned
            ) as u32;
            let p1_1 = mech_trace::rd!("100579a2", param_1.byte_add(4), read_unaligned) as u32;
            let p1_2 = mech_trace::rd!("100579a2", param_1.byte_add(8), read_unaligned) as u32;
            let mut p12: u32 = a0
                .wrapping_add(a1)
                .wrapping_add(param_3 as u32)
                .wrapping_add(p1_1)
                .wrapping_add(
                    (i_var2 as u32)
                        .wrapping_add(param_4 as u32)
                        .wrapping_add(p1_2)
                        .wrapping_mul(i_var5 as u32),
                );
            let i_var6 = mech_trace::rd!(
                "100579a2",
                ((param_2 as u32).wrapping_add(0x14) as *const i32),
                read_unaligned
            )
            .wrapping_add(1);
            let mut i_var7 = i_var6.wrapping_sub(i_var2);
            if i_var7 != 0 && i_var2 <= i_var6 {
                let mut pb: u32 = (param_2 as u32).wrapping_add(0x18);
                loop {
                    let mut b = rd(pb);
                    pb = pb.wrapping_add(1);
                    let mut b14 = (b & 1) != 0;
                    let mut b4: u8 = b >> 1;
                    let mut v9: u32 = b4 as u32;
                    let mut p13: u32 = p12;
                    let mut state: u8 = if !b14 && b4 != 0 {
                        FILL
                    } else if b4 != 0 {
                        COPY
                    } else {
                        JOINED
                    };
                    while state != ROWEND {
                        match state {
                            JOINED => {
                                state = if b14 { SKIP } else { ROWEND };
                            }
                            SKIP => {
                                let pb11 = pb.wrapping_add(1);
                                p13 = p13.wrapping_add(rd(pb) as u32);
                                pb = pb.wrapping_add(2);
                                let v = rd(pb11);
                                b14 = (v & 1) != 0;
                                b4 = v >> 1;
                                v9 = b4 as u32;
                                if !b14 && b4 != 0 {
                                    state = FILL;
                                } else if b4 == 0 {
                                    state = if b14 { SKIP } else { ROWEND };
                                } else {
                                    state = COPY;
                                }
                            }
                            COPY => {
                                v9 &= 0xff;
                                let mut skip = false;
                                if v9 != 0 {
                                    if 3 < v9 {
                                        loop {
                                            wr(p13, tb(rd(pb)));
                                            wr(p13.wrapping_add(1), tb(rd(pb.wrapping_add(1))));
                                            wr(p13.wrapping_add(2), tb(rd(pb.wrapping_add(2))));
                                            wr(p13.wrapping_add(3), tb(rd(pb.wrapping_add(3))));
                                            pb = pb.wrapping_add(4);
                                            p13 = p13.wrapping_add(4);
                                            v9 = v9.wrapping_sub(4);
                                            if v9 == 0 {
                                                skip = true;
                                                break;
                                            }
                                            if !(3 < (v9 as i32)) {
                                                break;
                                            }
                                        }
                                    }
                                    if !skip {
                                        loop {
                                            wr(p13, tb(rd(pb)));
                                            pb = pb.wrapping_add(1);
                                            p13 = p13.wrapping_add(1);
                                            v9 = v9.wrapping_sub(1);
                                            if v9 == 0 {
                                                break;
                                            }
                                        }
                                    }
                                }
                                state = AAE;
                            }
                            AAE => {
                                b = rd(pb);
                                pb = pb.wrapping_add(1);
                                b14 = (b & 1) != 0;
                                b4 = b >> 1;
                                v9 = b4 as u32;
                                if b14 || b4 == 0 {
                                    state = if b4 == 0 { JOINED } else { COPY };
                                } else {
                                    state = FILL;
                                }
                            }
                            _ => {
                                // FILL
                                loop {
                                    let pb11 = pb.wrapping_add(1);
                                    let u1 = tb(rd(pb));
                                    let mut v8 = v9 & 3;
                                    while v8 != 0 {
                                        wr(p13, u1);
                                        p13 = p13.wrapping_add(1);
                                        v8 = v8.wrapping_sub(1);
                                    }
                                    let u1w = u1 as u32;
                                    let v3: u32 = (u1w << 16) | (u1w << 8) | u1w;
                                    v9 = (v9 & 0xff) >> 2;
                                    while v9 != 0 {
                                        let t: u32 = (v3 << 8) | u1w;
                                        mech_trace::wr!(
                                            "100579a2",
                                            (p13 as *mut u32),
                                            write_unaligned,
                                            t
                                        );
                                        p13 = p13.wrapping_add(4);
                                        v9 = v9.wrapping_sub(1);
                                    }
                                    pb = pb.wrapping_add(2);
                                    let v = rd(pb11);
                                    b14 = (v & 1) != 0;
                                    b4 = v >> 1;
                                    v9 = (v3 << 8) | (b4 as u32);
                                    if !(!b14 && b4 != 0) {
                                        break;
                                    }
                                }
                                if !b14 {
                                    state = ROWEND;
                                } else if b4 != 0 {
                                    state = COPY;
                                } else {
                                    state = SKIP;
                                }
                            }
                        }
                    }
                    p12 = p12.wrapping_add(i_var5 as u32);
                    i_var7 = i_var7.wrapping_sub(1);
                    if i_var7 == 0 {
                        break;
                    }
                }
            }
        }
        mech_trace::rt!("100579a2", 0)
    }
}
