// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn _AIL_set_sample_loop_count_8(param_1: u32, param_2: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "fastcall" fn FUN_10010d60(sample: i32) {
    mech_trace::enter!("10010d60", sample);
    // SAFETY: sample points at the game's sample object; the field reads and
    // writes are done where the original does, and the callee is the game's own.
    unsafe {
        let p = sample as usize as *mut u8;
        let handle = mech_trace::rd!("10010d60", p.byte_add(0x29).cast::<i32>(), read_unaligned);
        if handle != 0 {
            mech_trace::wr!(
                "10010d60",
                p.byte_add(0x10).cast::<u32>(),
                write_unaligned,
                0
            );
            let h = mech_trace::rd!("10010d60", p.byte_add(0x29).cast::<i32>(), read_unaligned);
            mech_trace::cr!(
                "10010d60",
                "_AIL_set_sample_loop_count_8",
                _AIL_set_sample_loop_count_8(h as u32, 0)
            );
        }
    }
}
