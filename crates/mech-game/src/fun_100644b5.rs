// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf38: u32;
    static mut DAT_100eef40: [u32; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100644b5() {
    mech_trace::enter!("100644b5");
    // SAFETY: the globals and the object they point to are the game's own
    // memory; the fields are read where the original reads them, and the
    // callback is the game's own cdecl function.
    unsafe {
        let idx = mech_trace::gr!("100644b5", "100adf38", DAT_100adf38) as usize;
        let i_var1 = mech_trace::rd!(
            "100644b5",
            (&raw mut DAT_100eef40).cast::<u32>().add(idx),
            read_unaligned
        );
        if i_var1 != 0 {
            let base = i_var1 as usize as *const u8;
            let f = mech_trace::rd!(
                "100644b5",
                base.byte_add(0x34)
                    .cast::<Option<unsafe extern "C" fn(u32)>>(),
                read_unaligned
            );
            if f.is_some()
                && mech_trace::rd!(
                    "100644b5",
                    base.byte_add(0x20).cast::<i32>(),
                    read_unaligned
                ) != 0
            {
                let g = mech_trace::rd!(
                    "100644b5",
                    base.byte_add(0x34)
                        .cast::<Option<unsafe extern "C" fn(u32)>>(),
                    read_unaligned
                );
                let arg = mech_trace::rd!(
                    "100644b5",
                    base.byte_add(0x20).cast::<u32>(),
                    read_unaligned
                );
                if let Some(func) = g {
                    mech_trace::cr!("100644b5", "ind", func(arg));
                }
            }
        }
    }
}
