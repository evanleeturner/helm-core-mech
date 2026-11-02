// SPDX-License-Identifier: MIT

type Pfn = Option<unsafe extern "C" fn(i32, u32, u32, u32) -> u32>;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa4e0: [Pfn; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10014718(
    param_1: i32,
    param_2: u32,
    param_3: u32,
    param_4: u32,
) -> u32 {
    mech_trace::enter!("10014718", param_1, param_2, param_3, param_4);
    // SAFETY: param_1 points at the caller's structure; the table is the game's own.
    unsafe {
        let idx = mech_trace::rd!(
            "10014718",
            ((param_1 as u32).wrapping_add(0x24) as *const i32),
            read_unaligned
        ) as u32;
        let idx = idx.wrapping_mul(3) as usize;
        let entry = mech_trace::rd!(
            "10014718",
            (&raw mut DAT_100aa4e0).cast::<Pfn>().add(idx),
            read_unaligned
        );
        mech_trace::rt!(
            "10014718",
            match entry {
                None => 1,
                Some(f) => {
                    let idx2 = mech_trace::rd!(
                        "10014718",
                        ((param_1 as u32).wrapping_add(0x24) as *const i32),
                        read_unaligned
                    ) as u32;
                    let idx2 = idx2.wrapping_mul(3) as usize;
                    match mech_trace::rd!(
                        "10014718",
                        (&raw mut DAT_100aa4e0).cast::<Pfn>().add(idx2),
                        read_unaligned
                    ) {
                        Some(g) => mech_trace::cr!(
                            "10014718",
                            "ind",
                            g(param_1, param_2, param_3, param_4)
                        ),
                        None => mech_trace::cr!(
                            "10014718",
                            "ind",
                            f(param_1, param_2, param_3, param_4)
                        ),
                    }
                }
            }
        )
    }
}
