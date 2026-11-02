// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100af17c: i32;
    static mut DAT_101686f0: i32;
    static mut DAT_100b38fc: i32;
    static mut DAT_101ec430: [*mut i32; 0];
    static mut DAT_100af0b4: *mut u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1008668b(param_1: i32, param_2: i32, param_3: *mut i32) -> *mut u16;
    fn FUN_100105ba(param_1: i32);
    fn FUN_1005b6c7(param_1: i32) -> i32;
    fn FUN_10056fc4(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
    ) -> u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RaiseException(code: u32, flags: u32, nargs: u32, args: *const u32);
}

unsafe fn idiv(n: i32, d: i32) -> (i32, i32) {
    let q: i32;
    let r: i32;
    // SAFETY: the machine's own signed divide; faults on zero or overflow as the original.
    unsafe {
        core::arch::asm!(
            "cdq",
            "idiv {d:e}",
            d = in(reg) d,
            inout("eax") n => q,
            out("edx") r,
        );
    }
    (q, r)
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10010683(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32) {
    mech_trace::enter!("10010683", param_1, param_2, param_3, param_4);
    // SAFETY: all globals and callees are the game's own; the table entry and
    // the record it points at are read and written where the original does.
    unsafe {
        if param_2 >= 7 {
            return;
        }
        let piVar1: *mut i32 = mech_trace::rd!(
            "10010683",
            (&raw mut DAT_101ec430 as *mut *mut i32).offset(param_2 as isize),
            read
        );
        if piVar1.is_null() || mech_trace::dr!("10010683", piVar1) == 1 {
            return;
        }
        if mech_trace::dr!("10010683", piVar1.add(6)) == 0 {
            let puVar2 = mech_trace::cr!(
                "10010683",
                "1008668b",
                FUN_1008668b(
                    mech_trace::gr!("10010683", "100af17c", DAT_100af17c),
                    (mech_trace::dr!("10010683", piVar1.add(5)) as u32)
                        .wrapping_add(mech_trace::gr!("10010683", "101686f0", DAT_101686f0) as u32)
                        as i32,
                    mech_trace::gr!("10010683", "100af0b4", DAT_100af0b4) as *mut i32,
                )
            );
            mech_trace::dw!("10010683", piVar1.add(6), puVar2 as i32);
            if mech_trace::dr!("10010683", piVar1.add(6)) == 0 {
                FUN_100105ba(param_2);
                return;
            }
            let iVar3 = mech_trace::cr!(
                "10010683",
                "1005b6c7",
                FUN_1005b6c7(mech_trace::dr!("10010683", piVar1.add(6)))
            );
            mech_trace::dw!("10010683", piVar1.add(1), iVar3);
        }
        if mech_trace::dr!("10010683", piVar1) == 0 {
            mech_trace::dw!("10010683", piVar1, 2);
        }
        let mut local_8: u32;
        if mech_trace::dr!("10010683", piVar1.add(3)) == 0 {
            local_8 = 0;
            mech_trace::dw!(
                "10010683",
                piVar1.add(3),
                mech_trace::gr!("10010683", "100b38fc", DAT_100b38fc)
            );
        } else {
            let num = (mech_trace::gr!("10010683", "100b38fc", DAT_100b38fc) as u32)
                .wrapping_sub(mech_trace::dr!("10010683", piVar1.add(3)) as u32)
                as i32;
            let den = mech_trace::dr!("10010683", piVar1.add(2));
            if den == 0 || (num == i32::MIN && den == -1) {
                RaiseException(0xC0000094, 0, 0, core::ptr::null());
            }
            local_8 = idiv(num, den).0 as u32;
            if (local_8 as i32) < 0 {
                local_8 = 0;
            }
        }
        let flags = mech_trace::dr!("10010683", piVar1.add(4)) as u8;
        let mut local_10: u32 = 0;
        if flags & 1 == 0 {
            let den = mech_trace::dr!("10010683", piVar1.add(1));
            if den == 0 || (local_8 as i32 == i32::MIN && den == -1) {
                RaiseException(0xC0000094, 0, 0, core::ptr::null());
            }
            local_10 = (idiv(local_8 as i32, den).1 as u32) & 0xffff;
        } else if (local_8 as i32) < mech_trace::dr!("10010683", piVar1.add(1)) {
            local_10 = local_8 & 0xffff;
        } else if flags & 2 == 0 {
            mech_trace::dw!("10010683", piVar1, 1);
        } else {
            local_10 =
                (mech_trace::dr!("10010683", piVar1.add(1)) as i16).wrapping_sub(1) as u16 as u32;
            mech_trace::dw!("10010683", piVar1, 3);
        }
        if mech_trace::dr!("10010683", piVar1) == 1 {
            if (mech_trace::dr!("10010683", piVar1.add(4)) as u8) & 8 != 0 {
                FUN_100105ba(param_2);
            }
        } else {
            mech_trace::cr!(
                "10010683",
                "10056fc4",
                FUN_10056fc4(
                    param_1,
                    mech_trace::dr!("10010683", piVar1.add(6)),
                    (local_10 & 0xffff) as i32,
                    param_3,
                    param_4,
                )
            );
        }
    }
}
