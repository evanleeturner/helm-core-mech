// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b99d0: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100363b9(param_1: i32, param_2: i32) {
    mech_trace::enter!("100363b9", param_1, param_2);
    if param_1 != 0 && param_2 != 0 {
        // SAFETY: param_2 is the caller's object address; its field at +8 is
        // read, and written through only when non-null. The global is the
        // game's own.
        unsafe {
            let p = mech_trace::rd!(
                "100363b9",
                ((param_2 as usize).wrapping_add(8) as *const *mut u32),
                read_unaligned
            );
            if !p.is_null() {
                mech_trace::wr!(
                    "100363b9",
                    p,
                    write_unaligned,
                    mech_trace::gr!("100363b9", "100b99d0", DAT_100b99d0)
                );
            }
        }
    }
}
