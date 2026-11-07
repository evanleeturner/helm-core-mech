// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf74: *mut c_void;
    static mut DAT_100b4e78: [i8; 1];
    static mut DAT_100b4e7c: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapAlloc(h: *mut c_void, flags: u32, size: u32) -> *mut c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1005d8e0() -> u32;
    fn FUN_1005db1f(param_1: i32) -> i32;
    fn _strchr(s: *mut i8, v: i32) -> *mut i8;
    fn _strtok(s: *mut i8, d: *mut i8) -> *mut i8;
    fn FUN_1008bf20(param_1: *mut u8, param_2: *mut i32, param_3: u32) -> i32;
    fn _atoi(s: *mut i8) -> i32;
    fn FUN_10032c10(param_1: i32) -> i32;
    fn FUN_1000c8ee(param_1: i32) -> u32;
    fn FUN_1000c9e3(param_1: i32) -> *mut u8;
    fn FUN_1001d32b(param_1: i32, param_2: *mut i32, param_3: *mut i32);
    fn FUN_1001d57c(param_1: i32, param_2: i32, param_3: u16);
}

/// Signed divide using the machine's idiv: returns (quotient, remainder).
#[inline(always)]
unsafe fn idiv(a: i32, b: i32) -> (i32, i32) {
    let q: i32;
    let r: i32;
    // SAFETY: raw idiv; a zero divisor or overflow faults like the original.
    unsafe {
        mech_trace::log::debug!(
            f = "100728a3", b = "idiv", a:% = mech_trace::Args([
                mech_trace::__shown!(a),
                mech_trace::__shown!(b),
            ]); "asm.in"
        );
        core::arch::asm!(
            "cdq",
            "idiv ecx",
            inout("eax") a => q,
            in("ecx") b,
            out("edx") r,
            options(nomem, nostack)
        );
        mech_trace::log::debug!(
            f = "100728a3", b = "idiv", r = "eax", v:% = mech_trace::__shown!(q); "asm.out"
        );
        mech_trace::log::debug!(
            f = "100728a3", b = "idiv", r = "edx", v:% = mech_trace::__shown!(r); "asm.out"
        );
    }
    (q, r)
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100728a3(
    param_1: i32,
    param_2: *mut i8,
    param_3: i32,
    param_4: i32,
) -> u32 {
    mech_trace::enter!("100728a3", param_1, param_2, param_3, param_4);
    let mut param_4 = param_4;
    if param_4 == 0 {
        param_4 = 1;
    }
    // SAFETY: the callees and globals are the game's own; the pointers are
    // the caller's and the game's heap structures, accessed as the original does.
    unsafe {
        if param_1 == 0 {
            let local_c = mech_trace::cr!(
                "100728a3",
                "HeapAlloc",
                HeapAlloc(
                    mech_trace::gr!("100728a3", "100adf74", DAT_100adf74),
                    1,
                    0x50
                )
            ) as *mut i32;
            if local_c.is_null() {
                return mech_trace::rt!("100728a3", 0);
            }
            let i_var3 = mech_trace::cr!("100728a3", "1005d8e0", FUN_1005d8e0());
            let p_var1 =
                mech_trace::cr!("100728a3", "1005db1f", FUN_1005db1f(i_var3 as i32)) as *mut u32;
            mech_trace::wr!("100728a3", p_var1, write_unaligned, local_c as u32);
            mech_trace::wr!("100728a3", local_c.byte_add(12), write_unaligned, 0);
            mech_trace::wr!("100728a3", local_c.byte_add(4), write_unaligned, 0);
            mech_trace::wr!("100728a3", local_c.byte_add(8), write_unaligned, -1);
            let mut local_8 =
                mech_trace::cr!("100728a3", "_strchr", _strchr(param_2, 0x3b)) as *mut u8;
            if !local_8.is_null() {
                mech_trace::dw!("100728a3", local_8, 0);
                local_8 = local_8.byte_add(1);
                local_8 = mech_trace::cr!(
                    "100728a3",
                    "_strtok",
                    _strtok(local_8 as *mut i8, &raw mut DAT_100b4e78 as *mut i8)
                ) as *mut u8;
                while !local_8.is_null()
                    && mech_trace::rd!("100728a3", local_c.byte_add(12), read_unaligned) < 0x10
                {
                    let v = mech_trace::cr!(
                        "100728a3",
                        "1008bf20",
                        FUN_1008bf20(local_8, core::ptr::null_mut(), 0)
                    );
                    let n = mech_trace::rd!("100728a3", local_c.byte_add(12), read_unaligned);
                    mech_trace::wr!(
                        "100728a3",
                        local_c.byte_add((n.wrapping_add(4) as u32).wrapping_mul(4) as usize),
                        write_unaligned,
                        ((v as u32).wrapping_shl(4)) as i32
                    );
                    let n = mech_trace::rd!("100728a3", local_c.byte_add(12), read_unaligned);
                    mech_trace::wr!(
                        "100728a3",
                        local_c.byte_add(12),
                        write_unaligned,
                        n.wrapping_add(1)
                    );
                    local_8 = mech_trace::cr!(
                        "100728a3",
                        "_strtok",
                        _strtok(core::ptr::null_mut(), &raw mut DAT_100b4e7c as *mut i8)
                    ) as *mut u8;
                }
            }
            let i_var3 = mech_trace::cr!("100728a3", "_atoi", _atoi(param_2));
            let i_var3 = mech_trace::cr!("100728a3", "10032c10", FUN_10032c10(i_var3));
            let i_var3 = mech_trace::cr!("100728a3", "1000c8ee", FUN_1000c8ee(i_var3)) as i32;
            if i_var3 == -1 {
                return mech_trace::rt!("100728a3", 0);
            }
            let p_var2 = mech_trace::cr!("100728a3", "1000c9e3", FUN_1000c9e3(i_var3));
            mech_trace::wr!("100728a3", local_c, write_unaligned, p_var2 as i32);
        } else if param_1 == 1 {
            let i_var3 = mech_trace::cr!("100728a3", "1005d8e0", FUN_1005d8e0());
            let pp =
                mech_trace::cr!("100728a3", "1005db1f", FUN_1005db1f(i_var3 as i32)) as *mut i32;
            let local_c = mech_trace::rd!("100728a3", pp, read_unaligned) as *mut i32;
            if local_c.is_null() {
                return mech_trace::rt!("100728a3", 0);
            }
            let first = mech_trace::rd!("100728a3", local_c, read_unaligned);
            if first == 0 {
                return mech_trace::rt!("100728a3", 0);
            }
            let inner = mech_trace::rd!("100728a3", (first as *const i32), read_unaligned);
            if inner == 0 {
                return mech_trace::rt!("100728a3", 0);
            }
            mech_trace::wr!("100728a3", local_c.byte_add(4), write_unaligned, inner);
            let mut local_10: i32 = 0;
            if mech_trace::rd!("100728a3", local_c.byte_add(8), read_unaligned) == -1 {
                FUN_1001d32b(
                    mech_trace::rd!("100728a3", local_c.byte_add(4), read_unaligned),
                    &raw mut local_10,
                    local_c.byte_add(8),
                );
            }
            let i_var3 = mech_trace::rd!("100728a3", local_c.byte_add(12), read_unaligned);
            let mut local_18: i32 = 0;
            while local_18 < mech_trace::rd!("100728a3", local_c.byte_add(8), read_unaligned) {
                let (q, _) = idiv(param_3, param_4);
                let (_, r) = idiv(q, i_var3);
                let (_, r2) = idiv(
                    r.wrapping_add(local_18),
                    mech_trace::rd!("100728a3", local_c.byte_add(12), read_unaligned),
                );
                let off = (r2.wrapping_add(4) as u32).wrapping_mul(4) as usize;
                let v = mech_trace::rd!("100728a3", local_c.byte_add(off), read_unaligned);
                FUN_1001d57c(
                    mech_trace::rd!("100728a3", local_c.byte_add(4), read_unaligned),
                    local_18,
                    v as i16 as u16,
                );
                local_18 = local_18.wrapping_add(1);
            }
        }
    }
    mech_trace::rt!("100728a3", 1)
}
