// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aefc0: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003e369(param_1: i32) {
    mech_trace::enter!("1003e369", param_1);
    let p = param_1 as usize as *mut u8;
    // SAFETY: param_1 points at the caller's node; linked nodes and the
    // global are the game's own memory, accessed in the original's order.
    unsafe {
        let next = mech_trace::rd!("1003e369", p.byte_add(8).cast::<i32>(), read_unaligned);
        if next != 0 {
            let prev = mech_trace::rd!("1003e369", p.byte_add(4).cast::<u32>(), read_unaligned);
            mech_trace::wr!(
                "1003e369",
                (next as usize as *mut u8).byte_add(4).cast::<u32>(),
                write_unaligned,
                prev
            );
        }
        let prev = mech_trace::rd!("1003e369", p.byte_add(4).cast::<i32>(), read_unaligned);
        if prev != 0 {
            let next = mech_trace::rd!("1003e369", p.byte_add(8).cast::<u32>(), read_unaligned);
            mech_trace::wr!(
                "1003e369",
                (prev as usize as *mut u8).byte_add(8).cast::<u32>(),
                write_unaligned,
                next
            );
        }
        mech_trace::wr!("1003e369", p.byte_add(4).cast::<u32>(), write_unaligned, 0);
        let v = mech_trace::rd!("1003e369", p.byte_add(4).cast::<u32>(), read_unaligned);
        mech_trace::wr!("1003e369", p.byte_add(8).cast::<u32>(), write_unaligned, v);
        if mech_trace::gr!("1003e369", "100aefc0", DAT_100aefc0) != 0 {
            mech_trace::wr!(
                "1003e369",
                p.byte_add(0x44).cast::<u32>(),
                write_unaligned,
                9999
            );
            mech_trace::wr!(
                "1003e369",
                p.byte_add(0x48).cast::<u32>(),
                write_unaligned,
                9999
            );
        }
    }
}
