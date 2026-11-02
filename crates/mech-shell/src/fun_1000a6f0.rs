// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000a180(record: i32) -> *mut u32;
    fn FUN_1000a3f0(record: i32, param_2: u32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000a6f0(record: i32) {
    mech_trace::enter!("1000a6f0", record);
    let base = record as usize;
    let p230 = base.wrapping_add(0x230) as *mut i32;
    let p22c = base.wrapping_add(0x22c) as *const i32;
    let p210 = base.wrapping_add(0x210) as *const i32;
    let p228 = base.wrapping_add(0x228) as *const u32;
    // SAFETY: record points at the game's record; the fields are read and
    // written in the original's order, and the callees are the game's own.
    unsafe {
        let mut running_count = mech_trace::rd!("1000a6f0", p230, read_unaligned);
        let mut new_count = (mech_trace::rd!("1000a6f0", p22c, read_unaligned) / 100)
            .wrapping_sub(mech_trace::rd!("1000a6f0", p210, read_unaligned) / 0x19)
            .wrapping_add(10);
        mech_trace::wr!("1000a6f0", p230, write_unaligned, new_count);
        if new_count < 0 {
            mech_trace::wr!("1000a6f0", p230, write_unaligned, 0);
        }
        if -1 < running_count && running_count != mech_trace::rd!("1000a6f0", p230, read_unaligned)
        {
            if mech_trace::rd!("1000a6f0", p230, read_unaligned) < running_count {
                loop {
                    new_count = running_count.wrapping_add(8000);
                    running_count = running_count.wrapping_sub(1);
                    mech_trace::cr!("1000a6f0", "1000a180", FUN_1000a180(new_count));
                    if !(mech_trace::rd!("1000a6f0", p230, read_unaligned) < running_count) {
                        break;
                    }
                }
            }
            if running_count < mech_trace::rd!("1000a6f0", p230, read_unaligned) {
                loop {
                    new_count = running_count.wrapping_add(0x1f41);
                    running_count = running_count.wrapping_add(1);
                    mech_trace::cr!(
                        "1000a6f0",
                        "1000a3f0",
                        FUN_1000a3f0(new_count, mech_trace::rd!("1000a6f0", p228, read_unaligned))
                    );
                    if !(running_count < mech_trace::rd!("1000a6f0", p230, read_unaligned)) {
                        break;
                    }
                }
            }
        }
    }
}
