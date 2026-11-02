// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38d8: i32;
    static mut DAT_100eef40: [u32; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10050498(param_1: i32, param_2: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10050975() {
    mech_trace::enter!("10050975");
    let mut b_var1 = false;
    let mut local_c: i32 = 0;
    // SAFETY: the globals and the callee are the game's own; the table entries
    // are pointers to records with a field at offset 0x1c, accessed as the original does.
    unsafe {
        while local_c < mech_trace::gr!("10050975", "100b38d8", DAT_100b38d8) && !b_var1 {
            let base = (&raw mut DAT_100eef40).cast::<u32>();
            let rec = mech_trace::rd!("10050975", base.offset(local_c as isize), read_unaligned);
            if mech_trace::rd!(
                "10050975",
                ((rec.wrapping_add(0x1c)) as *const i32),
                read_unaligned
            ) != -1
            {
                b_var1 = true;
                let rec =
                    mech_trace::rd!("10050975", base.offset(local_c as isize), read_unaligned);
                mech_trace::cr!(
                    "10050975",
                    "10050498",
                    FUN_10050498(
                        local_c,
                        mech_trace::rd!(
                            "10050975",
                            ((rec.wrapping_add(0x1c)) as *const i32),
                            read_unaligned
                        ),
                    )
                );
                let rec =
                    mech_trace::rd!("10050975", base.offset(local_c as isize), read_unaligned);
                mech_trace::wr!(
                    "10050975",
                    ((rec.wrapping_add(0x1c)) as *mut u32),
                    write_unaligned,
                    0xffff_ffff
                );
            }
            local_c = local_c.wrapping_add(1);
        }
    }
}
