// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b2ce8: u32;
    static mut DAT_10114db0: [u32; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1005cfc4() {
    mech_trace::enter!("1005cfc4");
    // SAFETY: the globals are the game's own memory; each table entry is an
    // address of a record whose field at offset 4 is a 16-bit value.
    unsafe {
        mech_trace::gw!(
            "1005cfc4",
            "100b2ce8",
            DAT_100b2ce8,
            (mech_trace::gr!("1005cfc4", "100b2ce8", DAT_100b2ce8) == 0) as u32
        );
        let mut local_8: i32 = 3;
        while local_8 < 0x11 {
            let base = (&raw mut DAT_10114db0).cast::<u32>();
            let p = mech_trace::rd!("1005cfc4", base.add(local_8 as usize), read_unaligned);
            mech_trace::wr!(
                "1005cfc4",
                ((p.wrapping_add(4)) as *mut i16),
                write_unaligned,
                mech_trace::gr!("1005cfc4", "100b2ce8", DAT_100b2ce8) as i16
            );
            local_8 = local_8.wrapping_add(1);
        }
    }
}
