// SPDX-License-Identifier: MIT

use core::mem::MaybeUninit;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c6c40: i16;
    static mut DAT_100c6c42: u8;
    static mut DAT_100c6c30: *mut u8;
    static mut DAT_100a14c8: u8;
    static mut PTR_s_Glen_Garry_100a1b58: *const u8;
    static mut DAT_100a1b44: u32;
    static mut PTR_s_Federated_Commonwealth_100a1ae0: u8;
    static mut s_mis_dscn_d_100a2c18: u8;
    static mut s_rmglog_txt_100a2c28: u8;
    static mut DAT_100a2c24: u8;
    static mut s_______Can_t_open_RMG_log_file_100a2c34: u8;
    static mut s_Instant_Action_100a2c54: u8;
    static mut DAT_100a2c64: u8;
    static mut s__________RMG_LOG_FILE_FOR_DATE___100a2c68: u8;
    static mut s_Success_pay___ld__Fail_pay___ld_100a2ca4: u8;
    static mut s____Date_is_less_than_3044_100a2cc8: u8;
    static mut s____Planet_name_is_too_long_100a2ce4: u8;
    static mut s____missionType_value_is_not_betw_100a2d00: u8;
    static mut s____difficulty_level_is_not_betwe_100a2d30: u8;
    static mut DAT_100a2d80: u8;
    static mut s____Planet_name__s_not_a_valid_na_100a2d5c: u8;
    static mut s_ran_dwld1_bwd_100a2db4: u8;
    static mut s____Creating__s_100a2dc4: u8;
    static mut s____s_will__s__s_100a2dd4: u8;
    static mut s___Chose_planet_file__s_for_plane_100a2de8: u8;
    static mut s_en_02dstar_bwd_100a2e10: u8;
    static mut s_rmg__s_bwd_100a2e20: u8;
    static mut s___4sare_d_bwd_100a2e2c: u8;
    static mut DAT_100a2e3c: u8;
    static mut DAT_100a2e50: u8;
    static mut s___4sstp_d_100a2e58: u8;
    static mut s_randnav_d_100a2e64: u8;
    static mut s_Alpha_100a2e78: u8;
    static mut DAT_100a2e88: u8;
    static mut s_Gamma_100a2e98: u8;
    static mut s_randnav_d_bwd_100a2ea0: u8;
    static mut s_Nav__s_100a2eb0: u8;
    static mut s___Creating__s_100a2eb8: u8;
    static mut s_rmg__s_bwd_100a2ec8: u8;
    static mut s_ran_dscn1_bwd_100a2ed4: u8;
    static mut s___copy__s_to__s_100a2ee4: u8;
    static mut s__c_s___Mercenary_Briefing_n_n_tY_100a2ef8: u8;
    static mut s_ran_dbrf1_bwd_100a3188: u8;
    static mut s__lMISSION_FAILED_n_nI_can_t_beli_100a3198: u8;
    static mut s_ran_ddbff_bwd_100a3204: u8;
    static mut s__lMISSION_SUCCESSFUL_n_nThank_yo_100a3214: u8;
    static mut s_ran_ddbfs_bwd_100a3294: u8;
    static mut s_light_100a32a4: u8;
    static mut s_medium_100a32ac: u8;
    static mut s_heavy_100a32b4: u8;
    static mut s__c_l_s_Mercenary_Contract_n_n_t__100a32bc: u8;
    static mut s_rnd_dcam1_bwd_100a34f0: u8;
    static mut s______Couldn_t_create_logo0_file_100a3500: u8;
    static mut s______Couldn_t_create_logo_d_file_100a3524: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _sprintf(dest: *mut u8, fmt: *const u8, ...) -> i32;
    fn FUN_10056be0(a: i32, b: i32) -> i32;
    fn __wfopen(name: *const u8, mode: *const u8) -> *mut u8;
    fn _wprintf(fmt: *const u8, ...) -> i32;
    fn _fseek(f: *mut u8, off: i32, origin: i32) -> i32;
    fn _fwprintf(f: *mut u8, fmt: *const u8, ...) -> i32;
    fn _fclose(f: *mut u8) -> i32;
    fn _fflush(f: *mut u8) -> i32;
    fn FUN_1006fab0(n: u32) -> *mut u8;
    fn FUN_1006e790(p: *mut u8);
    fn FUN_10056c60(a: i16, b: i16, c: *mut *mut u8);
    fn FUN_10056ad0(p: *mut u8, a: *mut i32, b: *mut i32, c: *mut i32, d: *mut i32) -> u32;
    fn FUN_100572d0(a: *const u8, b: *const u8, c: *mut i16, d: *mut u8, e: *mut i16, f: i32);
    fn FUN_1005e170(s: *const u8, t: u16);
    fn FUN_10056c80(a: *const u8, b: *const u8) -> u32;
    fn FUN_1005e200(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32, h: i32, i: i32);
    fn FUN_1005e4f0(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32);
    fn FUN_1005e6f0(s: *const u8) -> u32;
    fn FUN_1005e400(
        a: u32,
        b: u32,
        c: u32,
        d: u32,
        e: u32,
        f: u16,
        g: u16,
        h: u16,
        i: u16,
        j: u16,
        k: u16,
        s: *const u8,
    );
    fn FUN_10056dd0(a: i32, b: *mut u8, c: u32, d: *mut i16, e: i16) -> u32;
    fn FUN_10057440(a: u32, b: *mut u8) -> i32;
    fn FUN_1005e7f0(s: *const u8);
    fn FUN_1005e870(a: *const u8, b: *const u8, c: i32);
    fn FUN_1005e910(s: *const u8);
    fn FUN_10057530(a: i16, b: i16) -> u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1005e670();
    fn FUN_1005e5c0();
}

unsafe fn r8(p: *const u8, o: usize) -> u8 {
    // SAFETY: caller passes a pointer to readable game memory.
    unsafe { mech_trace::rd!("10055d70", p.byte_add(o), read) }
}

unsafe fn r16(p: *const u8, o: usize) -> i16 {
    // SAFETY: caller passes a pointer to readable game memory.
    unsafe { mech_trace::rd!("10055d70", p.byte_add(o).cast::<i16>(), read_unaligned) }
}

unsafe fn r32(p: *const u8, o: usize) -> i32 {
    // SAFETY: caller passes a pointer to readable game memory.
    unsafe { mech_trace::rd!("10055d70", p.byte_add(o).cast::<i32>(), read_unaligned) }
}

unsafe fn w16(p: *mut u8, o: usize, v: i16) {
    // SAFETY: caller passes a pointer to writable game memory.
    unsafe { mech_trace::wr!("10055d70", p.byte_add(o).cast::<i16>(), write_unaligned, v) }
}

unsafe fn w32(p: *mut u8, o: usize, v: i32) {
    // SAFETY: caller passes a pointer to writable game memory.
    unsafe { mech_trace::wr!("10055d70", p.byte_add(o).cast::<i32>(), write_unaligned, v) }
}

unsafe fn cstrlen(p: *const u8) -> u32 {
    let mut n: u32 = 0;
    // SAFETY: p points at a NUL-terminated string.
    unsafe {
        while mech_trace::rd!("10055d70", p.add(n as usize), read) != 0 {
            n = n.wrapping_add(1);
        }
    }
    n
}

unsafe fn pstr(i: i16) -> *const u8 {
    // SAFETY: the table is the game's own pointer array.
    unsafe {
        mech_trace::rd!(
            "10055d70",
            (&raw const DAT_100a1b44)
                .byte_offset((i as isize).wrapping_mul(4))
                .cast::<*const u8>(),
            read_unaligned
        )
    }
}

unsafe fn fed(i: i16, o: usize) -> u32 {
    // SAFETY: the table is the game's own 14-byte record array.
    unsafe {
        mech_trace::rd!(
            "10055d70",
            (&raw const PTR_s_Federated_Commonwealth_100a1ae0)
                .byte_offset((i as isize).wrapping_mul(14))
                .byte_add(o)
                .cast::<u32>(),
            read_unaligned
        )
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case, unused_assignments, unused_mut)]
pub unsafe extern "C" fn FUN_10055d70(param_1: *mut u8) -> u32 {
    mech_trace::enter!("10055d70", param_1);
    let mut c: [i32; 4] = [0x1e848, -0x1e848, -0x1e848, 0x1e848];
    let mut bx: [i32; 6] = [0; 6];
    let mut e4 = MaybeUninit::<[*mut u8; 57]>::uninit();
    let e4p = e4.as_mut_ptr().cast::<*mut u8>();
    let mut l19c = [0u8; 32];
    let mut l188 = [0u8; 32];
    let mut l178 = [0u8; 64];
    let mut l14c = [0u8; 32];
    let mut l138 = [0u8; 32];
    let mut l128 = [0u8; 32];
    let mut l118 = [0u8; 32];
    let mut l104 = [0u8; 32];
    let mut lf4 = [0u8; 32];
    let mut l1c8 = [0u8; 32];
    let mut cn: i16 = 0;
    let mut l1ec: i16 = 0;
    let mut s1ea: i16 = 0;
    let mut l1e8: i16 = 0;
    let mut l1e6: i16 = 0;
    // SAFETY: every global, callee and pointer here is the game's own; param_1
    // points at the caller's record, read and written where the original does.
    unsafe {
        mech_trace::gw!("10055d70", "100c6c40", DAT_100c6c40, 0x1e);
        cn = 0;
        loop {
            let idx = cn as i32;
            let off = idx.wrapping_mul(0x532) as usize;
            let rec = (&raw mut DAT_100c6c42).byte_add(off);
            mech_trace::cp!(
                "10055d70",
                core::ptr::copy_nonoverlapping,
                &raw const DAT_100a14c8,
                rec,
                0x532
            );
            let quot = cn / 3;
            let rem = cn % 3;
            mech_trace::cr!(
                "10055d70",
                "_sprintf",
                _sprintf(
                    rec,
                    &raw const s_mis_dscn_d_100a2c18,
                    quot as i32,
                    (rem as i32).wrapping_add(1),
                )
            );
            let di = rem.wrapping_add(1);
            w16(rec, 9, di);
            w16(rec, 0xd, di);
            let e = if quot > 6 {
                (quot as i32).wrapping_add(1)
            } else {
                (quot as i32).wrapping_add(2)
            };
            cn = cn.wrapping_add(1);
            w16(rec, 0xb, (e % 3) as i16);
            if cn >= 0x1e {
                break;
            }
        }
        let fl: i32;
        if r32(param_1, 0xa) == -1 {
            fl = 1;
            let v = mech_trace::cr!("10055d70", "10056be0", FUN_10056be0(0xbe4, 0xbee));
            w32(param_1, 0xa, v);
        } else {
            fl = mech_trace::rd!("10055d70", e4p.cast::<i32>(), read);
        }
        mech_trace::gw!(
            "10055d70",
            "100c6c30",
            DAT_100c6c30,
            mech_trace::cr!(
                "10055d70",
                "__wfopen",
                __wfopen(&raw const s_rmglog_txt_100a2c28, &raw const DAT_100a2c24)
            )
        );
        if mech_trace::gr!("10055d70", "100c6c30", DAT_100c6c30).is_null() {
            let r = mech_trace::cr!(
                "10055d70",
                "_wprintf",
                _wprintf(&raw const s_______Can_t_open_RMG_log_file_100a2c34)
            );
            return mech_trace::rt!("10055d70", (r as u32 & 0xffff0000) | 1);
        }
        mech_trace::cr!(
            "10055d70",
            "_fseek",
            _fseek(mech_trace::gr!("10055d70", "100c6c30", DAT_100c6c30), 0, 0)
        );
        let who: *const u8 = if fl == 0 {
            &raw const DAT_100a2c64
        } else {
            &raw const s_Instant_Action_100a2c54
        };
        mech_trace::cr!(
            "10055d70",
            "_fwprintf",
            _fwprintf(
                mech_trace::gr!("10055d70", "100c6c30", DAT_100c6c30),
                &raw const s__________RMG_LOG_FILE_FOR_DATE___100a2c68,
                r32(param_1, 0xa),
                who,
                r32(param_1, 0x20),
            )
        );
        mech_trace::cr!(
            "10055d70",
            "_fwprintf",
            _fwprintf(
                mech_trace::gr!("10055d70", "100c6c30", DAT_100c6c30),
                &raw const s_Success_pay___ld__Fail_pay___ld_100a2ca4,
                r32(param_1, 2),
                r32(param_1, 6),
            )
        );
        let mut f: bool = r32(param_1, 0xa) < 0xbe4;
        if f {
            mech_trace::cr!(
                "10055d70",
                "_fwprintf",
                _fwprintf(
                    mech_trace::gr!("10055d70", "100c6c30", DAT_100c6c30),
                    &raw const s____Date_is_less_than_3044_100a2cc8,
                )
            );
        } else {
            let len = cstrlen(param_1.byte_add(0xe));
            f = len > 0x10;
            if f {
                mech_trace::cr!(
                    "10055d70",
                    "_fwprintf",
                    _fwprintf(
                        mech_trace::gr!("10055d70", "100c6c30", DAT_100c6c30),
                        &raw const s____Planet_name_is_too_long_100a2ce4,
                    )
                );
            } else {
                let m = r16(param_1, 0x1e);
                f = m < 0 || m > 3;
                if f {
                    mech_trace::cr!(
                        "10055d70",
                        "_fwprintf",
                        _fwprintf(
                            mech_trace::gr!("10055d70", "100c6c30", DAT_100c6c30),
                            &raw const s____missionType_value_is_not_betw_100a2d00,
                        )
                    );
                } else {
                    f = r8(param_1, 0xe1) > 3;
                    if f {
                        mech_trace::cr!(
                            "10055d70",
                            "_fwprintf",
                            _fwprintf(
                                mech_trace::gr!("10055d70", "100c6c30", DAT_100c6c30),
                                &raw const s____difficulty_level_is_not_betwe_100a2d30,
                            )
                        );
                    }
                }
            }
        }
        let name: *mut u8 = param_1.byte_add(0xe);
        let nlen = cstrlen(name);
        if nlen != 0 {
            cn = 0;
            loop {
                let mut p: *const u8 = mech_trace::rd!(
                    "10055d70",
                    (&raw const PTR_s_Glen_Garry_100a1b58).byte_add((cn as usize).wrapping_mul(8)),
                    read
                );
                let mut q: *const u8 = name;
                let eq = loop {
                    let x = mech_trace::rd!("10055d70", p, read);
                    let y = mech_trace::rd!("10055d70", q, read);
                    if x != y {
                        break false;
                    }
                    if x == 0 {
                        break true;
                    }
                    p = p.add(1);
                    q = q.add(1);
                };
                if eq {
                    break;
                }
                cn = cn.wrapping_add(1);
                if cn >= 0xf {
                    break;
                }
            }
        }
        if !f && cn != 0xf {
            mech_trace::cr!(
                "10055d70",
                "_fwprintf",
                _fwprintf(
                    mech_trace::gr!("10055d70", "100c6c30", DAT_100c6c30),
                    &raw const DAT_100a2d80,
                    r16(param_1, 0x1e) as i32,
                    r8(param_1, 0xe1) as u32,
                )
            );
            if r16(param_1, 0x1e) == 0 {
                let v = mech_trace::cr!("10055d70", "10056be0", FUN_10056be0(1, 3));
                w16(param_1, 0x1e, v as i16);
            }
            if r8(param_1, 0xe1) == 0 {
                mech_trace::wr!("10055d70", param_1.byte_add(0xe1), write, 2);
            }
            let mut esi: *mut u8;
            loop {
                let r = mech_trace::cr!(
                    "10055d70",
                    "10056be0",
                    FUN_10056be0(
                        0,
                        (mech_trace::gr!("10055d70", "100c6c40", DAT_100c6c40) as i32)
                            .wrapping_sub(1)
                    )
                );
                let off = r.wrapping_mul(0x532) as usize;
                esi = (&raw mut DAT_100c6c42).byte_add(off);
                if r16(esi, 0xd) > 2 {
                    continue;
                }
                let edx = r16(esi, 0xb) as i32;
                let eax = r16(param_1, 0x1e) as i32;
                if eax.wrapping_sub(edx) != 1 {
                    continue;
                }
                break;
            }
            let n = r16(esi, 0xf) as i32;
            let alloc = mech_trace::cr!(
                "10055d70",
                "1006fab0",
                FUN_1006fab0(n.wrapping_shl(2) as u32)
            )
            .cast::<u32>();
            let mut zc: u32 = (n.wrapping_shl(2) as u32) >> 2;
            let mut zp = alloc;
            while zc != 0 {
                mech_trace::wr!("10055d70", zp, write_unaligned, 0);
                zp = zp.add(1);
                zc = zc.wrapping_sub(1);
            }
            l1e6 = 0;
            loop {
                let r = mech_trace::cr!(
                    "10055d70",
                    "10056be0",
                    FUN_10056be0(0, (r16(esi, 0x11) as i32).wrapping_sub(1))
                );
                let edx = r as i16 as i32;
                let slot = alloc.offset(edx as isize);
                if mech_trace::rd!("10055d70", slot, read_unaligned) == 0 {
                    let k = l1e6 as i32;
                    l1e6 = l1e6.wrapping_add(1);
                    let p = esi.offset(edx.wrapping_mul(0x17).wrapping_add(0x13) as isize);
                    mech_trace::dw!("10055d70", e4p.offset(k as isize), p);
                    w32(p, 0xf, 1);
                    mech_trace::wr!("10055d70", slot, write_unaligned, 1);
                }
                if r16(esi, 0xd) == l1e6 {
                    break;
                }
            }
            loop {
                let r = mech_trace::cr!(
                    "10055d70",
                    "10056be0",
                    FUN_10056be0(
                        r16(esi, 0x11) as i32,
                        (r16(esi, 0xf) as i32).wrapping_sub(1),
                    )
                );
                let eax = r as i16 as i32;
                let slot = alloc.offset(eax as isize);
                if mech_trace::rd!("10055d70", slot, read_unaligned) == 0 {
                    let k = l1e6 as i32;
                    l1e6 = l1e6.wrapping_add(1);
                    let p = esi.offset(eax.wrapping_mul(0x17).wrapping_add(0x13) as isize);
                    mech_trace::dw!("10055d70", e4p.offset(k as isize), p);
                    mech_trace::wr!("10055d70", slot, write_unaligned, 1);
                }
                let ecx = (r16(esi, 0xf) as i32).wrapping_sub(r16(esi, 0x11) as i32);
                if !(ecx >= l1e6 as i32) {
                    break;
                }
            }
            FUN_1006e790(alloc.cast::<u8>());
            loop {
                let mut bxi: i16 = 0;
                if r16(esi, 0xd) > bxi {
                    loop {
                        let r = mech_trace::cr!(
                            "10055d70",
                            "10056be0",
                            FUN_10056be0(0, (l1e6 as i32).wrapping_sub(1))
                        );
                        if bxi != r as i16 {
                            FUN_10056c60(bxi, r as i16, e4p);
                        }
                        bxi = bxi.wrapping_add(1);
                        if !(r16(esi, 0xd) > bxi) {
                            break;
                        }
                    }
                }
                bxi = 0;
                let mut di: i16 = 0;
                bx[5] = c[1];
                bx[2] = c[1];
                bx[3] = c[1];
                bx[4] = c[2];
                let mut retry = false;
                if l1e6 > bxi {
                    loop {
                        let ptr = mech_trace::dr!("10055d70", e4p.offset(bxi as isize));
                        let ok = mech_trace::cr!(
                            "10055d70",
                            "10056ad0",
                            FUN_10056ad0(
                                ptr,
                                c.as_mut_ptr(),
                                bx.as_mut_ptr().add(2),
                                bx.as_mut_ptr().add(1),
                                bx.as_mut_ptr(),
                            )
                        );
                        if ok == 0 {
                            if r16(esi, 0xd) > di {
                                retry = true;
                                break;
                            }
                        } else if r32(ptr, 0xf) != 0 {
                            di = di.wrapping_add(1);
                        }
                        bxi = bxi.wrapping_add(1);
                        if !(l1e6 > bxi) {
                            break;
                        }
                    }
                }
                if !retry {
                    break;
                }
            }
            mech_trace::cr!(
                "10055d70",
                "_sprintf",
                _sprintf(
                    l128.as_mut_ptr(),
                    &raw const s_ran_dwld1_bwd_100a2db4,
                    r16(param_1, 0) as i32,
                )
            );
            mech_trace::cr!(
                "10055d70",
                "_fwprintf",
                _fwprintf(
                    mech_trace::gr!("10055d70", "100c6c30", DAT_100c6c30),
                    &raw const s____Creating__s_100a2dc4,
                    l128.as_ptr(),
                )
            );
            FUN_1005e670();
            FUN_100572d0(
                pstr(r16(param_1, 0x1e)),
                name,
                &raw mut l1ec,
                l138.as_mut_ptr(),
                &raw mut s1ea,
                r32(param_1, 0xa),
            );
            let l1dc = fed(s1ea, 0);
            let l1e0 = fed(l1ec, 0);
            mech_trace::cr!(
                "10055d70",
                "_fwprintf",
                _fwprintf(
                    mech_trace::gr!("10055d70", "100c6c30", DAT_100c6c30),
                    &raw const s____s_will__s__s_100a2dd4,
                    l1e0,
                    pstr(r16(param_1, 0x1e)),
                    l1dc,
                )
            );
            FUN_1005e170(l138.as_ptr(), 0xffff);
            mech_trace::cr!(
                "10055d70",
                "_fwprintf",
                _fwprintf(
                    mech_trace::gr!("10055d70", "100c6c30", DAT_100c6c30),
                    &raw const s___Chose_planet_file__s_for_plane_100a2de8,
                    l138.as_ptr(),
                    name,
                )
            );
            let mut d: i16 = 0;
            if r16(esi, 9) > d {
                loop {
                    let i = d as i32;
                    d = d.wrapping_add(1);
                    mech_trace::cr!(
                        "10055d70",
                        "_sprintf",
                        _sprintf(
                            l104.as_mut_ptr(),
                            &raw const s_en_02dstar_bwd_100a2e10,
                            i.wrapping_add(1),
                        )
                    );
                    FUN_1005e170(l104.as_ptr(), 0xfffe);
                    if !(r16(esi, 9) > d) {
                        break;
                    }
                }
            }
            d = 0;
            bx[5] = c[1];
            bx[2] = c[1];
            cn = 1;
            bx[3] = c[1];
            bx[4] = c[2];
            if l1e6 > d {
                loop {
                    let pp = e4p.offset(d as isize);
                    let ptr = mech_trace::dr!("10055d70", pp);
                    if r32(ptr, 0xf) != 0 {
                        mech_trace::cr!(
                            "10055d70",
                            "_sprintf",
                            _sprintf(l118.as_mut_ptr(), &raw const s_rmg__s_bwd_100a2e20, ptr)
                        );
                        let ebp = cn as i32;
                        mech_trace::cr!(
                            "10055d70",
                            "_sprintf",
                            _sprintf(
                                l19c.as_mut_ptr(),
                                &raw const s___4sare_d_bwd_100a2e2c,
                                esi as *const u8,
                                ebp,
                            )
                        );
                        mech_trace::cr!(
                            "10055d70",
                            "_fwprintf",
                            _fwprintf(
                                mech_trace::gr!("10055d70", "100c6c30", DAT_100c6c30),
                                &raw const DAT_100a2e3c,
                                l118.as_ptr(),
                                l19c.as_ptr(),
                            )
                        );
                        let r = mech_trace::cr!(
                            "10055d70",
                            "10056c80",
                            FUN_10056c80(l118.as_ptr(), l19c.as_ptr())
                        );
                        if r as u16 != 0 {
                            return mech_trace::rt!("10055d70", r);
                        }
                        mech_trace::cr!(
                            "10055d70",
                            "_sprintf",
                            _sprintf(
                                mech_trace::dr!("10055d70", pp),
                                &raw const DAT_100a2e50,
                                l19c.as_ptr()
                            )
                        );
                        mech_trace::cr!(
                            "10055d70",
                            "_sprintf",
                            _sprintf(
                                lf4.as_mut_ptr(),
                                &raw const s___4sstp_d_100a2e58,
                                esi as *const u8,
                                ebp,
                            )
                        );
                        mech_trace::cr!(
                            "10055d70",
                            "_sprintf",
                            _sprintf(l188.as_mut_ptr(), &raw const s_randnav_d_100a2e64, ebp)
                        );
                        cn = cn.wrapping_add(1);
                    }
                    let ok = mech_trace::cr!(
                        "10055d70",
                        "10056ad0",
                        FUN_10056ad0(
                            mech_trace::dr!("10055d70", pp),
                            c.as_mut_ptr(),
                            bx.as_mut_ptr().add(2),
                            bx.as_mut_ptr().add(1),
                            bx.as_mut_ptr(),
                        )
                    );
                    if ok == 0 {
                        break;
                    }
                    FUN_1005e200(1, 1, 1, 0, 0, 0, bx[1], 0, bx[0]);
                    FUN_1005e4f0(0, 0, 0, 1, 1, 1);
                    let q = mech_trace::dr!("10055d70", pp);
                    let s: *const u8;
                    let tag: u16;
                    if r32(q, 0xf) != 0 {
                        FUN_1005e170(lf4.as_ptr(), 0xffff);
                        FUN_1005e170(mech_trace::dr!("10055d70", pp), 0xfffe);
                        s = l188.as_ptr();
                        tag = 0xfffe;
                    } else {
                        s = q;
                        tag = 0xffff;
                    }
                    d = d.wrapping_add(1);
                    FUN_1005e170(s, tag);
                    FUN_1005e5c0();
                    if !(l1e6 > d) {
                        break;
                    }
                }
            }
            let r = mech_trace::cr!("10055d70", "1005e6f0", FUN_1005e6f0(l128.as_ptr()));
            if r as u16 != 0 {
                return mech_trace::rt!("10055d70", r);
            }
            let mut b: i16 = 0;
            let mut di: i16 = 1;
            let mut names: [*const u8; 4] = [core::ptr::null(); 4];
            if l1e6 > b {
                loop {
                    let ptr = mech_trace::dr!("10055d70", e4p.offset(b as isize));
                    if r32(ptr, 0xf) != 0 {
                        names[1] = &raw const s_Alpha_100a2e78;
                        names[2] = &raw const DAT_100a2e88;
                        names[3] = &raw const s_Gamma_100a2e98;
                        let ebp = di as i32;
                        mech_trace::cr!(
                            "10055d70",
                            "_sprintf",
                            _sprintf(l188.as_mut_ptr(), &raw const s_randnav_d_bwd_100a2ea0, ebp)
                        );
                        mech_trace::cr!(
                            "10055d70",
                            "_sprintf",
                            _sprintf(
                                l1c8.as_mut_ptr(),
                                &raw const s_Nav__s_100a2eb0,
                                mech_trace::dr!("10055d70", names.as_ptr().offset(ebp as isize)),
                            )
                        );
                        mech_trace::cr!(
                            "10055d70",
                            "_fwprintf",
                            _fwprintf(
                                mech_trace::gr!("10055d70", "100c6c30", DAT_100c6c30),
                                &raw const s___Creating__s_100a2eb8,
                                l188.as_ptr(),
                            )
                        );
                        FUN_1005e670();
                        FUN_1005e400(0, 0, 0, 0, 0, 1, 0, 0, 0, 100, 0x100, l1c8.as_ptr());
                        let r =
                            mech_trace::cr!("10055d70", "1005e6f0", FUN_1005e6f0(l188.as_ptr()));
                        if r as u16 != 0 {
                            return mech_trace::rt!("10055d70", r);
                        }
                        di = di.wrapping_add(1);
                    }
                    b = b.wrapping_add(1);
                    if !(l1e6 > b) {
                        break;
                    }
                }
            }
            let r = mech_trace::cr!(
                "10055d70",
                "10056dd0",
                FUN_10056dd0(
                    r16(esi, 9) as i32,
                    param_1,
                    fed(s1ea, 4),
                    &raw mut l1e8,
                    r16(param_1, 0x1e).wrapping_sub(1),
                )
            );
            if r as u16 != 0 {
                return mech_trace::rt!("10055d70", r);
            }
            mech_trace::cr!(
                "10055d70",
                "_sprintf",
                _sprintf(
                    l19c.as_mut_ptr(),
                    &raw const s_rmg__s_bwd_100a2ec8,
                    esi as *const u8,
                )
            );
            mech_trace::cr!(
                "10055d70",
                "_sprintf",
                _sprintf(
                    l14c.as_mut_ptr(),
                    &raw const s_ran_dscn1_bwd_100a2ed4,
                    r16(param_1, 0) as i32,
                )
            );
            mech_trace::cr!(
                "10055d70",
                "_fwprintf",
                _fwprintf(
                    mech_trace::gr!("10055d70", "100c6c30", DAT_100c6c30),
                    &raw const s___copy__s_to__s_100a2ee4,
                    l19c.as_ptr(),
                    l14c.as_ptr(),
                )
            );
            let r = mech_trace::cr!(
                "10055d70",
                "10056c80",
                FUN_10056c80(l19c.as_ptr(), l14c.as_ptr())
            );
            if r as u16 != 0 {
                return mech_trace::rt!("10055d70", r);
            }
            let buf = mech_trace::cr!("10055d70", "1006fab0", FUN_1006fab0(2000));
            if buf.is_null() {
                return mech_trace::rt!("10055d70", 4);
            }
            FUN_1005e670();
            let ds = pstr(r16(param_1, 0x1e));
            let iv = mech_trace::cr!(
                "10055d70",
                "10057440",
                FUN_10057440(r32(param_1, 2) as u32, l178.as_mut_ptr())
            );
            mech_trace::cr!(
                "10055d70",
                "_sprintf",
                _sprintf(
                    buf,
                    &raw const s__c_s___Mercenary_Briefing_n_n_tY_100a2ef8,
                    l1e0,
                    ds,
                    l1dc,
                    name,
                    name,
                    iv,
                    l1e8 as i32,
                    ds,
                    ds,
                )
            );
            FUN_1005e7f0(buf);
            mech_trace::cr!(
                "10055d70",
                "_sprintf",
                _sprintf(
                    l178.as_mut_ptr(),
                    &raw const s_ran_dbrf1_bwd_100a3188,
                    r16(param_1, 0) as i32,
                )
            );
            let r = mech_trace::cr!("10055d70", "1005e6f0", FUN_1005e6f0(l178.as_ptr()));
            if r as u16 != 0 {
                return mech_trace::rt!("10055d70", r);
            }
            FUN_1005e670();
            mech_trace::cr!(
                "10055d70",
                "_sprintf",
                _sprintf(buf, &raw const s__lMISSION_FAILED_n_nI_can_t_beli_100a3198)
            );
            FUN_1005e7f0(buf);
            mech_trace::cr!(
                "10055d70",
                "_sprintf",
                _sprintf(
                    l178.as_mut_ptr(),
                    &raw const s_ran_ddbff_bwd_100a3204,
                    r16(param_1, 0) as i32,
                )
            );
            let r = mech_trace::cr!("10055d70", "1005e6f0", FUN_1005e6f0(l178.as_ptr()));
            if r as u16 != 0 {
                return mech_trace::rt!("10055d70", r);
            }
            FUN_1005e670();
            let iv = mech_trace::cr!(
                "10055d70",
                "10057440",
                FUN_10057440(r32(param_1, 2) as u32, l178.as_mut_ptr())
            );
            mech_trace::cr!(
                "10055d70",
                "_sprintf",
                _sprintf(
                    buf,
                    &raw const s__lMISSION_SUCCESSFUL_n_nThank_yo_100a3214,
                    iv,
                )
            );
            FUN_1005e7f0(buf);
            mech_trace::cr!(
                "10055d70",
                "_sprintf",
                _sprintf(
                    l178.as_mut_ptr(),
                    &raw const s_ran_ddbfs_bwd_100a3294,
                    r16(param_1, 0) as i32,
                )
            );
            let r = mech_trace::cr!("10055d70", "1005e6f0", FUN_1005e6f0(l178.as_ptr()));
            if r as u16 != 0 {
                return mech_trace::rt!("10055d70", r);
            }
            FUN_1005e670();
            let sz: *const u8 = if l1e8 < 4 {
                &raw const s_light_100a32a4
            } else if l1e8 < 6 {
                &raw const s_medium_100a32ac
            } else {
                &raw const s_heavy_100a32b4
            };
            let ds = pstr(r16(param_1, 0x1e));
            let iv = mech_trace::cr!(
                "10055d70",
                "10057440",
                FUN_10057440(r32(param_1, 2) as u32, l178.as_mut_ptr())
            );
            mech_trace::cr!(
                "10055d70",
                "_sprintf",
                _sprintf(
                    buf,
                    &raw const s__c_l_s_Mercenary_Contract_n_n_t__100a32bc,
                    l1e0,
                    l1e0,
                    ds,
                    name,
                    l1dc,
                    sz,
                    l1e0,
                    name,
                    ds,
                    iv,
                    l1e8 as i32,
                )
            );
            let blen = cstrlen(buf);
            let size = blen.wrapping_add(1).wrapping_add(0xb) & 0xfffffffc;
            FUN_1005e870(
                l14c.as_ptr(),
                fed(l1ec, 10) as usize as *const u8,
                size as i32,
            );
            FUN_1005e910(buf);
            mech_trace::cr!(
                "10055d70",
                "_sprintf",
                _sprintf(
                    l178.as_mut_ptr(),
                    &raw const s_rnd_dcam1_bwd_100a34f0,
                    r16(param_1, 0) as i32,
                )
            );
            let r = mech_trace::cr!("10055d70", "1005e6f0", FUN_1005e6f0(l178.as_ptr()));
            if r as u16 != 0 {
                return mech_trace::rt!("10055d70", r);
            }
            let r = mech_trace::cr!("10055d70", "10057530", FUN_10057530(l1ec, 0));
            let dd = r as u16;
            if dd != 0 {
                mech_trace::cr!(
                    "10055d70",
                    "_fwprintf",
                    _fwprintf(
                        mech_trace::gr!("10055d70", "100c6c30", DAT_100c6c30),
                        &raw const s______Couldn_t_create_logo0_file_100a3500,
                    )
                );
                let r2 = mech_trace::cr!(
                    "10055d70",
                    "_fflush",
                    _fflush(mech_trace::gr!("10055d70", "100c6c30", DAT_100c6c30))
                );
                return mech_trace::rt!("10055d70", (r2 as u32 & 0xffff0000) | dd as u32);
            }
            let mut bb: i16 = 1;
            if r16(esi, 9) >= bb {
                loop {
                    let r = mech_trace::cr!("10055d70", "10057530", FUN_10057530(s1ea, bb));
                    let dd = r as u16;
                    if dd != 0 {
                        mech_trace::cr!(
                            "10055d70",
                            "_fwprintf",
                            _fwprintf(
                                mech_trace::gr!("10055d70", "100c6c30", DAT_100c6c30),
                                &raw const s______Couldn_t_create_logo_d_file_100a3524,
                                bb as i32,
                            )
                        );
                        let r2 = mech_trace::cr!(
                            "10055d70",
                            "_fflush",
                            _fflush(mech_trace::gr!("10055d70", "100c6c30", DAT_100c6c30))
                        );
                        return mech_trace::rt!("10055d70", (r2 as u32 & 0xffff0000) | dd as u32);
                    }
                    bb = bb.wrapping_add(1);
                    if !(r16(esi, 9) >= bb) {
                        break;
                    }
                }
            }
            let r = mech_trace::cr!(
                "10055d70",
                "_fclose",
                _fclose(mech_trace::gr!("10055d70", "100c6c30", DAT_100c6c30))
            );
            return mech_trace::rt!("10055d70", r as u32 & 0xffff0000);
        }
        if !f {
            mech_trace::cr!(
                "10055d70",
                "_fwprintf",
                _fwprintf(
                    mech_trace::gr!("10055d70", "100c6c30", DAT_100c6c30),
                    &raw const s____Planet_name__s_not_a_valid_na_100a2d5c,
                    name,
                )
            );
        }
        let r = mech_trace::cr!(
            "10055d70",
            "_fclose",
            _fclose(mech_trace::gr!("10055d70", "100c6c30", DAT_100c6c30))
        );
        mech_trace::rt!("10055d70", (r as u32 & 0xffff0000) | 2)
    }
}
