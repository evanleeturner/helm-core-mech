// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10002570(origin: u32, direction: i32) -> i32;
}

#[inline(always)]
unsafe fn idiv(a: i32, d: i32) -> i32 {
    let q: i32;
    // SAFETY: the machine's own idiv, so a zero divisor or MIN / -1 faults as the original does.
    unsafe {
        core::arch::asm!(
            "cdq",
            "idiv {d:e}",
            d = in(reg) d,
            inout("eax") a => q,
            out("edx") _,
        );
    }
    q
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10026815(
    origin: i32,
    direction: i32,
    slab_min: i32,
    slab_max: i32,
    t_near: *mut i32,
    t_far: *mut i32,
) -> u32 {
    mech_trace::enter!(
        "10026815", origin, direction, slab_min, slab_max, t_near, t_far
    );
    let mut degenerate = false;
    let mut local_c: u32 = 0;
    let mut local_18: u32 = 0;
    let local_10: i32;
    let local_14: i32;

    // SAFETY: the divisions are the machine's own; the callee is the game's;
    // t_near and t_far point at the caller's integers.
    unsafe {
        if direction == 0 {
            degenerate = true;
        } else {
            local_c = (slab_min as u32).wrapping_sub(origin as u32);
            local_18 = (slab_max as u32).wrapping_sub(origin as u32);
            if idiv(local_c as i32, direction) < 0x8000 && -0x8001 < idiv(local_c as i32, direction)
            {
                if 0x7fff < idiv(local_18 as i32, direction)
                    || idiv(local_18 as i32, direction) < -0x8000
                {
                    degenerate = true;
                }
            } else {
                degenerate = true;
            }
        }
        if degenerate {
            if origin < slab_min || slab_max < origin {
                return mech_trace::rt!("10026815", 1);
            }
            local_10 = -0x7fffffff;
            local_14 = 0x7fffffff;
        } else {
            local_10 = mech_trace::cr!("10026815", "10002570", FUN_10002570(local_c, direction));
            local_14 = mech_trace::cr!("10026815", "10002570", FUN_10002570(local_18, direction));
        }
        if local_14 < local_10 {
            mech_trace::wr!("10026815", t_near, write_unaligned, local_14);
            mech_trace::wr!("10026815", t_far, write_unaligned, local_10);
        } else {
            mech_trace::wr!("10026815", t_near, write_unaligned, local_10);
            mech_trace::wr!("10026815", t_far, write_unaligned, local_14);
        }
    }
    mech_trace::rt!("10026815", 0)
}
