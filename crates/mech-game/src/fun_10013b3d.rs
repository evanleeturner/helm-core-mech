// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa208: *mut u32;
    static mut DAT_100aa1b0: *mut u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10013a8b(param_1: *mut u32, param_2: *mut u32, param_3: u32, param_4: u8) -> u32;
}

type CallFn = unsafe extern "stdcall" fn(
    *mut u32,
    *mut u32,
    i32,
    unsafe extern "stdcall" fn(*mut u32, *mut u32, u32, u8) -> u32,
    *mut u32,
    i32,
) -> i32;

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10013b3d(param_1: *mut u32) -> i32 {
    mech_trace::enter!("10013b3d", param_1);
    let mut local_80: [u32; 31] = [0; 31];
    // SAFETY: param_1 points at at least 31 words; the globals and the
    // callee are the game's own; the vtable slot holds a stdcall function.
    unsafe {
        let mut p5 = param_1;
        let mut p6 = local_80.as_mut_ptr();
        let mut n: i32 = 0x1f;
        while n != 0 {
            mech_trace::wr!(
                "10013b3d",
                p6,
                write_unaligned,
                mech_trace::rd!("10013b3d", p5, read_unaligned)
            );
            p5 = p5.add(1);
            p6 = p6.add(1);
            n = n.wrapping_sub(1);
        }
        let obj = mech_trace::gr!("10013b3d", "100aa208", DAT_100aa208);
        let vt = mech_trace::rd!("10013b3d", obj, read_unaligned);
        let slot = vt as usize as *const u32;
        let fp = mech_trace::rd!("10013b3d", slot.byte_add(0x38), read_unaligned);
        let f: CallFn = core::mem::transmute::<u32, CallFn>(fp);
        let mut r = mech_trace::cr!(
            "10013b3d",
            "ind",
            f(obj, local_80.as_mut_ptr(), 500, FUN_10013a8b, param_1, 1)
        );
        mech_trace::rt!(
            "10013b3d",
            if r == 0 {
                let mut p2 = param_1.add(9) as *const u8;
                let mut p4 = mech_trace::gr!("10013b3d", "100aa1b0", DAT_100aa1b0) as *const u8;
                r = 'cmp: {
                    loop {
                        let mut b = mech_trace::rd!("10013b3d", p2, read);
                        let mut c = mech_trace::rd!("10013b3d", p4, read);
                        if b != c {
                            let lt = (b < c) as i32;
                            break 'cmp 1i32.wrapping_sub(lt).wrapping_sub((lt != 0) as i32);
                        }
                        if b == 0 {
                            break;
                        }
                        b = mech_trace::rd!("10013b3d", p2.add(1), read);
                        c = mech_trace::rd!("10013b3d", p4.add(1), read);
                        if b != c {
                            let lt = (b < c) as i32;
                            break 'cmp 1i32.wrapping_sub(lt).wrapping_sub((lt != 0) as i32);
                        }
                        p2 = p2.add(2);
                        p4 = p4.add(2);
                        if b == 0 {
                            break;
                        }
                    }
                    0
                };
                if r == 0 {
                    r = 1;
                }
                r
            } else {
                0
            }
        )
    }
}
