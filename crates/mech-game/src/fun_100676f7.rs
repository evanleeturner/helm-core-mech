// SPDX-License-Identifier: MIT

use core::arch::asm;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aaf68: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10002570(param_1: u32, param_2: i32) -> u32;
    fn FUN_10067951(
        param_1: i32,
        param_2: *mut i32,
        param_3: u32,
        param_4: i32,
        param_5: *mut i32,
    ) -> *mut i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10006d3e() -> u8;
}

unsafe fn machine_idiv(a: i32, b: i32) -> i32 {
    let q: i32;
    // SAFETY: the machine's own signed divide; a zero divisor or overflow faults as in the original.
    unsafe {
        mech_trace::log::debug!(
            f = "100676f7", b = "idiv", a:% = mech_trace::Args([
                mech_trace::__shown!(b),
                mech_trace::__shown!(a),
            ]); "asm.in"
        );
        asm!("cdq", "idiv {b}", b = in(reg) b, inout("eax") a => q, out("edx") _);
        mech_trace::log::debug!(
            f = "100676f7", b = "idiv", r = "eax", v:% = mech_trace::__shown!(q); "asm.out"
        );
    }
    q
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100676f7(
    param_1: i32,
    param_2: *mut i32,
    param_3: *mut i32,
) -> *mut i32 {
    mech_trace::enter!("100676f7", param_1, param_2, param_3);
    // SAFETY: the pointers and globals belong to the game; reads and calls occur in the original's order.
    unsafe {
        let mut local: [i32; 2] = [0; 2];
        let a = mech_trace::rd!("100676f7", ((param_1 + 0xc) as *const i32), read_unaligned);
        let b = mech_trace::rd!("100676f7", ((param_1 + 4) as *const i32), read_unaligned);
        local[0] = (a.wrapping_sub(b).wrapping_add(1)) >> 1;
        let c = mech_trace::rd!("100676f7", ((param_1 + 0x10) as *const i32), read_unaligned);
        let d = mech_trace::rd!("100676f7", ((param_1 + 8) as *const i32), read_unaligned);
        local[1] = (c.wrapping_sub(d).wrapping_add(1)) >> 1;
        let i_var1 = local[0].wrapping_sub(mech_trace::rd!("100676f7", param_2, read_unaligned));
        let mut local_c: u32 = (-1 < i_var1) as u32;
        let u_var2 =
            local[1].wrapping_sub(mech_trace::rd!("100676f7", param_2.add(1), read_unaligned))
                as u32;
        if -1 < (u_var2 as i32) {
            local_c |= 2;
        }
        if i_var1 == 0
            || 0x7fff < machine_idiv(u_var2 as i32, i_var1)
            || machine_idiv(u_var2 as i32, i_var1) < -0x8000
        {
            local_c |= 4;
        }
        let p =
            mech_trace::gr!("100676f7", "100aaf68", DAT_100aaf68).wrapping_add(0x44) as *const i32;
        mech_trace::cr!(
            "100676f7",
            "10002570",
            FUN_10002570(u_var2, mech_trace::rd!("100676f7", p, read_unaligned))
        );
        let local_8 = mech_trace::cr!("100676f7", "10006d3e", FUN_10006d3e()) as i32;
        mech_trace::cr!(
            "100676f7",
            "10067951",
            FUN_10067951(param_1, local.as_mut_ptr(), local_c, local_8, param_3)
        );
        mech_trace::rt!("100676f7", param_3)
    }
}
