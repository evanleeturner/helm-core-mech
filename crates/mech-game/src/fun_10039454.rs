// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ae744: u32;
    static mut DAT_100ae748: u32;
    static mut DAT_100ae74c: u32;
    static mut DAT_100ae750: u32;
    static mut DAT_100ae754: u32;
    static mut DAT_100ae758: *mut i32;
    static mut DAT_100c6ce0: i32;
    static mut DAT_100c6ce4: u32;
    static mut DAT_100c6ce8: i32;
    static mut DAT_100c6cf0: u32;
    static mut DAT_100c6cf4: u32;
    static mut DAT_100c6cf8: u32;
    static mut DAT_100c6cfc: u32;
    static mut DAT_100aaf68: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002f783(param_1: i32);
    fn FUN_1002f37a(param_1: i32, param_2: u32, param_3: u32, param_4: u32);
    fn FUN_1002fbc0(param_1: *mut i32);
}

#[inline(always)]
unsafe fn idiv(a: i32, b: i32) -> i32 {
    let q: i32;
    // SAFETY: the machine's own signed divide; faults exactly as the original does.
    unsafe {
        mech_trace::log::debug!(
            f = "10039454", b = "idiv", a:% = mech_trace::Args([
                mech_trace::__shown!(b),
                mech_trace::__shown!(a),
            ]); "asm.in"
        );
        core::arch::asm!(
            "cdq",
            "idiv {d:e}",
            d = in(reg) b,
            inout("eax") a => q,
            out("edx") _,
        );
        mech_trace::log::debug!(
            f = "10039454", b = "idiv", r = "eax", v:% = mech_trace::__shown!(q); "asm.out"
        );
    }
    q
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10039454() {
    mech_trace::enter!("10039454");
    // SAFETY: the globals and callees are the game's own; the pointer held in
    // DAT_100aaf68 points at the game's position data.
    unsafe {
        if mech_trace::gr!("10039454", "100ae750", DAT_100ae750) != 0
            && mech_trace::gr!("10039454", "100ae748", DAT_100ae748) != 0
        {
            let base = mech_trace::gr!("10039454", "100aaf68", DAT_100aaf68) as *const u8;
            let mut local_20: i32 = mech_trace::rd!("10039454", base.cast::<i32>(), read_unaligned);
            let mut local_1c: i32 =
                mech_trace::rd!("10039454", base.byte_add(8).cast::<i32>(), read_unaligned);
            if mech_trace::gr!("10039454", "100ae754", DAT_100ae754) == 0 {
                local_20 = idiv(
                    local_20,
                    mech_trace::gr!("10039454", "100ae744", DAT_100ae744) as i32,
                );
                local_1c = idiv(
                    local_1c,
                    mech_trace::gr!("10039454", "100ae744", DAT_100ae744) as i32,
                );
            } else {
                let d0 = local_20
                    .wrapping_sub(mech_trace::gr!("10039454", "100c6cf0", DAT_100c6cf0) as i32);
                let d1 = local_1c
                    .wrapping_sub(mech_trace::gr!("10039454", "100c6cf8", DAT_100c6cf8) as i32);
                let u_var1: u32 = (d0 >> 0x1f) as u32;
                let u_var2: u32 = (d1 >> 0x1f) as u32;
                let a0 = ((d0 as u32) ^ u_var1).wrapping_sub(u_var1) as i32;
                let a1 = ((d1 as u32) ^ u_var2).wrapping_sub(u_var2) as i32;
                if (mech_trace::gr!("10039454", "100c6cfc", DAT_100c6cfc) as i32) < a0
                    || (mech_trace::gr!("10039454", "100c6cfc", DAT_100c6cfc) as i32) < a1
                {
                    let mut local_10: i32 =
                        (mech_trace::gr!("10039454", "100ae744", DAT_100ae744) as i32) >> 1;
                    let mut local_14: i32 = local_10;
                    if local_20 < 0 {
                        local_14 = (mech_trace::gr!("10039454", "100ae744", DAT_100ae744) as i32)
                            .wrapping_neg()
                            >> 1;
                    }
                    if local_1c < 0 {
                        local_10 = (mech_trace::gr!("10039454", "100ae744", DAT_100ae744) as i32)
                            .wrapping_neg()
                            >> 1;
                    }
                    local_20 = idiv(
                        local_14.wrapping_add(local_20),
                        mech_trace::gr!("10039454", "100ae744", DAT_100ae744) as i32,
                    );
                    local_1c = idiv(
                        local_10.wrapping_add(local_1c),
                        mech_trace::gr!("10039454", "100ae744", DAT_100ae744) as i32,
                    );
                } else {
                    local_20 = mech_trace::gr!("10039454", "100c6ce0", DAT_100c6ce0);
                    local_1c = mech_trace::gr!("10039454", "100c6ce8", DAT_100c6ce8);
                }
            }
            if mech_trace::gr!("10039454", "100ae74c", DAT_100ae74c) == 0 {
                FUN_1002f783(mech_trace::gr!("10039454", "100ae758", DAT_100ae758) as i32);
                mech_trace::gw!("10039454", "100ae74c", DAT_100ae74c, 1);
            }
            if mech_trace::gr!("10039454", "100c6ce0", DAT_100c6ce0) != local_20
                || mech_trace::gr!("10039454", "100c6ce8", DAT_100c6ce8) != local_1c
            {
                mech_trace::gw!(
                    "10039454",
                    "100c6cf0",
                    DAT_100c6cf0,
                    local_20
                        .wrapping_mul(mech_trace::gr!("10039454", "100ae744", DAT_100ae744) as i32)
                        as u32
                );
                mech_trace::gw!("10039454", "100c6cf4", DAT_100c6cf4, 0);
                mech_trace::gw!(
                    "10039454",
                    "100c6cf8",
                    DAT_100c6cf8,
                    local_1c
                        .wrapping_mul(mech_trace::gr!("10039454", "100ae744", DAT_100ae744) as i32)
                        as u32
                );
                if !mech_trace::gr!("10039454", "100ae758", DAT_100ae758).is_null() {
                    FUN_1002f783(mech_trace::gr!("10039454", "100ae758", DAT_100ae758) as i32);
                    FUN_1002f37a(
                        mech_trace::gr!("10039454", "100ae758", DAT_100ae758) as i32,
                        mech_trace::gr!("10039454", "100c6cf0", DAT_100c6cf0),
                        mech_trace::gr!("10039454", "100c6cf4", DAT_100c6cf4),
                        mech_trace::gr!("10039454", "100c6cf8", DAT_100c6cf8),
                    );
                    FUN_1002fbc0(mech_trace::gr!("10039454", "100ae758", DAT_100ae758));
                }
            }
            mech_trace::gw!("10039454", "100c6ce0", DAT_100c6ce0, local_20);
            mech_trace::gw!("10039454", "100c6ce4", DAT_100c6ce4, 0);
            mech_trace::gw!("10039454", "100c6ce8", DAT_100c6ce8, local_1c);
        }
    }
}
