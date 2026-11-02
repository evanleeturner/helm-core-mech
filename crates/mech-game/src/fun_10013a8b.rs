// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa1b0: *mut u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10013a8b(
    param_1: *mut u32,
    param_2: *mut u32,
    param_3: u32,
    param_4: u8,
) -> u32 {
    mech_trace::enter!("10013a8b", param_1, param_2, param_3, param_4);
    let _ = param_3;
    if (param_4 & 1) == 0 {
        // SAFETY: the pointers are the caller's and the game's own; reads and
        // writes happen in the same order as the original.
        unsafe {
            let mut pbvar2: *const u8 = param_1.add(9).cast::<u8>();
            let mut pbvar4: *const u8 = mech_trace::gr!("10013a8b", "100aa1b0", DAT_100aa1b0);
            let mut equal = true;
            loop {
                let b1 = mech_trace::rd!("10013a8b", pbvar2, read);
                if b1 != mech_trace::rd!("10013a8b", pbvar4, read) {
                    equal = false;
                    break;
                }
                if b1 == 0 {
                    break;
                }
                let b2 = mech_trace::rd!("10013a8b", pbvar2.add(1), read);
                if b2 != mech_trace::rd!("10013a8b", pbvar4.add(1), read) {
                    equal = false;
                    break;
                }
                pbvar2 = pbvar2.add(2);
                pbvar4 = pbvar4.add(2);
                if b2 == 0 {
                    break;
                }
            }
            if equal {
                let mut src = param_1;
                let mut dst = param_2;
                let mut n: i32 = 0x1f;
                while n != 0 {
                    mech_trace::wr!(
                        "10013a8b",
                        dst,
                        write_unaligned,
                        mech_trace::rd!("10013a8b", src, read_unaligned)
                    );
                    src = src.add(1);
                    dst = dst.add(1);
                    n -= 1;
                }
            }
        }
    }
    mech_trace::rt!("10013a8b", 0)
}
