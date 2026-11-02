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
    // SAFETY: param_1 points at the caller's structure; the table is the game's own.
    unsafe {
        let idx = ((param_1 as u32).wrapping_add(0x24) as *const i32).read_unaligned() as u32;
        let idx = idx.wrapping_mul(3) as usize;
        let entry = (&raw mut DAT_100aa4e0)
            .cast::<Pfn>()
            .add(idx)
            .read_unaligned();
        match entry {
            None => 1,
            Some(f) => {
                let idx2 =
                    ((param_1 as u32).wrapping_add(0x24) as *const i32).read_unaligned() as u32;
                let idx2 = idx2.wrapping_mul(3) as usize;
                match (&raw mut DAT_100aa4e0)
                    .cast::<Pfn>()
                    .add(idx2)
                    .read_unaligned()
                {
                    Some(g) => g(param_1, param_2, param_3, param_4),
                    None => f(param_1, param_2, param_3, param_4),
                }
            }
        }
    }
}
