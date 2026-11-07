// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aae58: u32;
    static mut DAT_100aae5c: u32;
    static mut DAT_100aae64: u32;
    static mut DAT_100aae68: i32;
    static mut DAT_100aafbc: u32;
    static mut DAT_100b0f18: u32;
    static mut DAT_100b0f1c: u32;
    static mut DAT_100c96bc: u32;
    static mut DAT_10115100: [u32; 0];
    static mut DAT_1011515c: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10035f13();
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10017bc1(param_1: i32, param_2: u32, param_3: i32, param_4: i32) -> u32;
    fn FUN_1001793d(param_1: i32, param_2: *mut i32, param_3: i32, param_4: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10017f0b(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
) {
    mech_trace::enter!("10017f0b", param_1, param_2, param_3, param_4, param_5);
    let mut buf: [i32; 61] = [0; 61];
    // SAFETY: the callee and the globals are the game's own memory; the
    // local buffer mirrors the original's contiguous stack frame and the
    // table entries are pointers to game structures.
    unsafe {
        FUN_10035f13();
        let u_var2 = mech_trace::gr!("10017f0b", "100aafbc", DAT_100aafbc);
        mech_trace::gw!("10017f0b", "100b0f18", DAT_100b0f18, 0);
        mech_trace::gw!("10017f0b", "100b0f1c", DAT_100b0f1c, 0);
        mech_trace::gw!("10017f0b", "1011515c", DAT_1011515c, 1);
        let mut local_c: i32 = (mech_trace::gr!("10017f0b", "100aae64", DAT_100aae64)
            .wrapping_add(mech_trace::gr!("10017f0b", "100c96bc", DAT_100c96bc))
            >> 3) as i32;
        if local_c < 1 {
            local_c = 0;
        } else if 0xf < local_c {
            local_c = 0xf;
        }
        let local_554 =
            (mech_trace::gr!("10017f0b", "100aae5c", DAT_100aae5c) as i32).wrapping_mul(param_4);
        let local_54c =
            (mech_trace::gr!("10017f0b", "100aae5c", DAT_100aae5c) as i32).wrapping_mul(param_5);
        let local_510 =
            (mech_trace::gr!("10017f0b", "100aae5c", DAT_100aae5c) as i32).wrapping_add(local_554);
        let local_4c4 =
            (mech_trace::gr!("10017f0b", "100aae5c", DAT_100aae5c) as i32).wrapping_add(local_54c);
        let base = mech_trace::gr!("10017f0b", "100aae58", DAT_100aae58) as i32;
        buf[3] = local_554;
        buf[4] = 0;
        buf[5] = local_54c;
        buf[6] = 0;
        buf[7] = 0;
        buf[9] = 0;
        buf[20] = local_510;
        buf[21] = 0;
        buf[22] = local_54c;
        buf[23] = base;
        buf[24] = 0;
        buf[26] = 0;
        buf[37] = local_510;
        buf[38] = 0;
        buf[39] = local_4c4;
        buf[40] = base;
        buf[41] = base;
        buf[43] = 0;
        buf[54] = local_554;
        buf[55] = 0;
        buf[56] = local_4c4;
        buf[57] = 0;
        buf[58] = base;
        buf[60] = 0;
        mech_trace::gw!("10017f0b", "100aafbc", DAT_100aafbc, 0);

        let table = (&raw mut DAT_10115100) as *mut u32;
        let buf_addr = buf.as_mut_ptr() as i32;
        let local_10: u32 = if mech_trace::gr!("10017f0b", "100aae68", DAT_100aae68) == param_2 {
            mech_trace::cr!(
                "10017f0b",
                "10017bc1",
                FUN_10017bc1(buf_addr, 4, param_1, param_2.wrapping_add(200))
            )
        } else {
            mech_trace::cr!(
                "10017f0b",
                "10017bc1",
                FUN_10017bc1(buf_addr, 4, param_1, param_2)
            )
        };
        if local_10 != 0 {
            if mech_trace::gr!("10017f0b", "100aae68", DAT_100aae68) == param_2 {
                let mut i: u32 = 0;
                while i < local_10 {
                    let p = mech_trace::rd!("10017f0b", table.add(i as usize), read_unaligned);
                    mech_trace::wr!(
                        "10017f0b",
                        ((p as usize + 0x2c) as *mut i32),
                        write_unaligned,
                        local_c
                    );
                    i = i.wrapping_add(1);
                }
            } else {
                let mut i: u32 = 0;
                while i < local_10 {
                    let i_var1 =
                        mech_trace::rd!("10017f0b", table.add(i as usize), read_unaligned) as usize;
                    let p8 =
                        mech_trace::rd!("10017f0b", ((i_var1 + 8) as *const i32), read_unaligned);
                    let local_8 = p8.wrapping_add(param_1.wrapping_mul(-4));
                    let p2c = (i_var1 + 0x2c) as *mut i32;
                    if local_8 < 0 {
                        mech_trace::wr!("10017f0b", p2c, write_unaligned, 0xf);
                    } else {
                        let p24 = param_2.wrapping_mul(4);
                        let num = (p24.wrapping_sub(local_8)).wrapping_mul(local_c);
                        let den = p24.wrapping_add(param_1.wrapping_mul(-4));
                        let q: i32;
                        mech_trace::log::debug!(
                            f = "10017f0b", b = "idiv", a:% = mech_trace::Args([
                                mech_trace::__shown!(den),
                                mech_trace::__shown!(num),
                            ]); "asm.in"
                        );
                        core::arch::asm!(
                            "cdq",
                            "idiv {d:e}",
                            d = in(reg) den,
                            inout("eax") num => q,
                            out("edx") _,
                        );
                        mech_trace::log::debug!(
                            f = "10017f0b", b = "idiv", r = "eax",
                            v:% = mech_trace::__shown!(q); "asm.out"
                        );
                        mech_trace::wr!("10017f0b", p2c, write_unaligned, q);
                    }
                    let v = mech_trace::rd!("10017f0b", p2c, read_unaligned);
                    if v < 1 {
                        mech_trace::wr!("10017f0b", p2c, write_unaligned, 0);
                    } else if 0xf < mech_trace::rd!("10017f0b", p2c, read_unaligned) {
                        mech_trace::wr!("10017f0b", p2c, write_unaligned, 0xf);
                    }
                    i = i.wrapping_add(1);
                }
            }
            mech_trace::cr!(
                "10017f0b",
                "1001793d",
                FUN_1001793d(param_3, table as *mut i32, local_10 as i32, 0)
            );
        }
        mech_trace::gw!("10017f0b", "100aafbc", DAT_100aafbc, u_var2);
    }
}
