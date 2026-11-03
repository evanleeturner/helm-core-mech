// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10098224: *mut c_void;
}

#[allow(non_snake_case)]
unsafe extern "thiscall" {
    fn FUN_100108b0(this: *mut c_void, sample: i32);
}

#[allow(non_snake_case)]
unsafe extern "fastcall" {
    fn FUN_10010de0(sample: *mut i32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapFree(h: *mut c_void, flags: u32, mem: *mut c_void) -> i32;
    fn _AIL_release_sample_handle_4(handle: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "fastcall" fn FUN_10010c80(sample: *mut i32) {
    mech_trace::enter!("10010c80", sample);
    // SAFETY: sample points at the game's sample structure; the callees and
    // globals are the game's own, used in the original order.
    unsafe {
        FUN_100108b0(
            mech_trace::rd!("10010c80", sample, read_unaligned) as usize as *mut c_void,
            sample as usize as i32,
        );
        if mech_trace::rd!(
            "10010c80",
            sample.byte_add(0x29).cast::<i32>(),
            read_unaligned
        ) != 0
        {
            FUN_10010de0(sample);
            if mech_trace::rd!("10010c80", sample.add(1), read_unaligned) != 0 {
                mech_trace::cr!(
                    "10010c80",
                    "HeapFree",
                    HeapFree(
                        mech_trace::gr!("10010c80", "10098224", DAT_10098224),
                        1,
                        mech_trace::rd!("10010c80", sample.add(1), read_unaligned) as usize
                            as *mut c_void,
                    )
                );
                mech_trace::wr!("10010c80", sample.add(1), write_unaligned, 0);
            }
            mech_trace::cr!(
                "10010c80",
                "_AIL_release_sample_handle_4",
                _AIL_release_sample_handle_4(mech_trace::rd!(
                    "10010c80",
                    sample.byte_add(0x29).cast::<u32>(),
                    read_unaligned
                ))
            );
        }
    }
}
