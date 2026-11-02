// SPDX-License-Identifier: MIT

use core::arch::asm;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100af4f8: u8;
    static mut DAT_100af4fc: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002fc9d(param_1: i32) -> i32;
    fn FUN_10089060(
        param_1: *mut i32,
        param_2: *mut i32,
        param_3: *mut i32,
        param_4: *mut i32,
    ) -> u8;
    fn FUN_100261f0(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
        param_7: i32,
    ) -> u8;
    fn FUN_1000fc30(param_1: i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_10026423(param_1: i32) -> u8;
    fn FUN_1002637d(param_1: i32) -> u8;
}

#[inline(always)]
unsafe fn idiv(a: i32, b: i32) -> i32 {
    let q: i32;
    // SAFETY: the machine's own signed divide, faulting as the original does.
    unsafe {
        asm!("cdq", "idiv {b:e}", b = in(reg) b, inout("eax") a => q, out("edx") _, options(nostack));
    }
    q
}

#[inline(always)]
fn fold(param_3: i32, param_4: i16) -> i32 {
    if param_3 < 0 {
        let v = 0x10u32.wrapping_sub(param_4 as i32 as u32);
        let u2 = ((v as i32) >> 0x1f) as u32;
        let t = (v ^ u2).wrapping_sub(u2) & 0xf;
        ((t ^ u2).wrapping_sub(u2)) as i32
    } else {
        param_4 as i32
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100444ea(
    param_1: i32,
    param_2: *mut i32,
    param_3: i32,
    param_4: i16,
    param_5: i32,
    param_6: i32,
) {
    mech_trace::enter!(
        "100444ea", param_1, param_2, param_3, param_4, param_5, param_6
    );
    // SAFETY: the callees and globals are the game's own; param_1 addresses
    // the game's object and param_2 its segment record, accessed as the original does.
    unsafe {
        let mut local_20: i32 = 0;
        let iv = mech_trace::rd!(
            "100444ea",
            ((param_1.wrapping_add(0x20)) as *const i32),
            read_unaligned
        );
        let local_8 = mech_trace::cr!(
            "100444ea",
            "1002fc9d",
            FUN_1002fc9d(mech_trace::rd!(
                "100444ea",
                ((param_1.wrapping_add(0x40)) as *const i32),
                read_unaligned
            ))
        ) as *mut i32;
        let local_28 = fold(param_3, param_4);
        let mut local_14: i32 = mech_trace::rd!(
            "100444ea",
            (&raw mut DAT_100af4f8)
                .byte_offset(local_28.wrapping_mul(8) as isize)
                .cast::<i32>(),
            read_unaligned
        );
        let local_2c = fold(param_3, param_4);
        let mut local_1c: i32 = mech_trace::rd!(
            "100444ea",
            (&raw mut DAT_100af4fc)
                .byte_offset(local_2c.wrapping_mul(8) as isize)
                .cast::<i32>(),
            read_unaligned
        );
        if local_14 != 0 {
            local_14 = idiv(param_5, local_14);
        }
        if local_1c != 0 {
            local_1c = idiv(param_5, local_1c);
        }
        mech_trace::cr!(
            "100444ea",
            "10089060",
            FUN_10089060(
                local_8,
                &raw mut local_14,
                &raw mut local_20,
                &raw mut local_1c,
            )
        );
        let mut local_c: i32;
        let mut local_10: i32;
        let mut local_18: i32;
        if param_6 == 0 {
            local_10 = mech_trace::rd!(
                "100444ea",
                ((param_1.wrapping_add(0x54)) as *const i32),
                read_unaligned
            );
            local_c = mech_trace::rd!(
                "100444ea",
                ((param_1.wrapping_add(0x50)) as *const i32),
                read_unaligned
            );
            local_18 = mech_trace::rd!(
                "100444ea",
                ((param_1.wrapping_add(0x58)) as *const i32),
                read_unaligned
            );
        } else {
            let w = mech_trace::rd!(
                "100444ea",
                (iv.wrapping_add(0x41c) as *const i32),
                read_unaligned
            );
            if param_3 < 0 {
                local_c = 1i32.wrapping_sub(w);
            } else {
                local_c = w.wrapping_add(-1);
            }
            local_18 = 0;
            mech_trace::cr!(
                "100444ea",
                "10089060",
                FUN_10089060(
                    local_8,
                    &raw mut local_c,
                    &raw mut local_20,
                    &raw mut local_18,
                )
            );
            local_10 = mech_trace::rd!(
                "100444ea",
                ((param_1.wrapping_add(0x54)) as *const i32),
                read_unaligned
            );
        }
        mech_trace::cr!(
            "100444ea",
            "100261f0",
            FUN_100261f0(
                param_2, local_c, local_10, local_18, local_14, local_10, local_1c,
            )
        );
        let r = mech_trace::cr!(
            "100444ea",
            "1000fc30",
            FUN_1000fc30(
                local_14.wrapping_sub(local_c),
                0,
                local_1c.wrapping_sub(local_18),
            )
        );
        if r < 0xbb9 {
            mech_trace::cr!("100444ea", "1002637d", FUN_1002637d(param_2 as i32));
        } else {
            mech_trace::cr!("100444ea", "10026423", FUN_10026423(param_2 as i32));
        }
    }
}
