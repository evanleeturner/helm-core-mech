// SPDX-License-Identifier: MIT
use core::mem::MaybeUninit;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aafd4: unsafe extern "C" fn(i32, *mut u32, u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10019d88(
    param_1: u32,
    param_2: u32,
    param_3: u32,
    param_4: u32,
    param_5: u32,
    param_6: u32,
    param_7: u32,
) {
    let mut local_4c: [MaybeUninit<u32>; 17] = [MaybeUninit::uninit(); 17];
    let p = local_4c.as_mut_ptr().cast::<u32>();
    // SAFETY: p points at a 17-slot local buffer; indices are in range, and
    // the callback is the game's own function pointer.
    unsafe {
        p.add(3).write(param_1);
        p.add(0).write(param_1);
        p.add(4).write(param_2);
        p.add(1).write(param_2);
        p.add(9).write(param_3);
        p.add(6).write(param_3);
        p.add(10).write(param_4);
        p.add(7).write(param_4);
        p.add(15).write(param_5);
        p.add(12).write(param_5);
        p.add(16).write(param_6);
        p.add(13).write(param_6);
        (DAT_100aafd4)(3, p, param_7);
    }
}
