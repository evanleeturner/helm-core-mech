// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a676c: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn _AIL_waveOutOpen_16(param_1: u32, param_2: u32, param_3: u32, param_4: u32) -> u32;
}

#[repr(C, packed)]
struct WaveFormat {
    format_tag: u16,
    channels: u16,
    samples_per_sec: u32,
    avg_bytes_per_sec: u32,
    block_align: u16,
    bits_per_sample: u16,
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "fastcall" fn FUN_100107a0(param_1: *mut u8) -> i32 {
    mech_trace::enter!("100107a0", param_1);
    // SAFETY: param_1 points at the game's sound driver record; the global and
    // the callee are the game's own, and the format struct is a live local.
    unsafe {
        let slot = param_1.byte_add(5).cast::<i32>();
        let mut result = mech_trace::rd!("100107a0", slot, read_unaligned);
        if result == 0 {
            if mech_trace::rd!("100107a0", param_1, read) != 0
                && mech_trace::gr!("100107a0", "100a676c", DAT_100a676c) != 0
            {
                let fmt = WaveFormat {
                    format_tag: 1,
                    channels: 1,
                    samples_per_sec: 0x2b11,
                    avg_bytes_per_sec: 0x2b11,
                    block_align: 1,
                    bits_per_sample: 8,
                };
                result = mech_trace::cr!(
                    "100107a0",
                    "_AIL_waveOutOpen_16",
                    _AIL_waveOutOpen_16(
                        slot as u32,
                        param_1.byte_add(9) as u32,
                        0,
                        &raw const fmt as u32,
                    )
                ) as i32;
                if result != 0 {
                    mech_trace::wr!("100107a0", slot, write_unaligned, 0);
                }
                return mech_trace::rt!(
                    "100107a0",
                    mech_trace::rd!("100107a0", slot, read_unaligned)
                );
            }
            result = 0;
        }
        mech_trace::rt!("100107a0", result)
    }
}
