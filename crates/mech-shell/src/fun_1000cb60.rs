// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a67b4: *mut i32;
    static mut DAT_100a67bc: u32;
    static mut DAT_100a67c0: u32;
    static mut DAT_100a67c4: u32;
    static mut DAT_100a67c8: u32;
    static mut DAT_100a67cc: *mut i32;
    static mut DAT_100a67d4: u32;
    static mut DAT_100a67d8: u32;
    static mut DAT_100a67dc: u32;
    static mut DAT_100a67e0: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10018060(param_1: i32, param_2: i32) -> u32;
    fn FUN_10017df0(param_1: *mut i32, param_2: u32) -> u32;
}

macro_rules! count {
    ($src:expr) => {
        (($src).wrapping_add(4) as *const i32).read_unaligned()
    };
}

macro_rules! copy_loop {
    ($src:ident, $dst:ident) => {{
        let mut i_var4: i32 = 0;
        if 0 < count!($src) {
            loop {
                let i_var3 = i_var4.wrapping_add(1);
                let u_var1 = FUN_10018060($src as i32, i_var4);
                FUN_10017df0($dst, u_var1);
                i_var4 = i_var3;
                if !(i_var3 < count!($src)) {
                    break;
                }
            }
        }
    }};
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1000cb60() {
    mech_trace::enter!("1000cb60");
    // SAFETY: the globals and callees are the game's own; the collection
    // pointers held in the globals point at live collection objects whose
    // count lives at offset 4, read where the original reads it.
    unsafe {
        copy_loop!(DAT_100a67bc, DAT_100a67b4);
        copy_loop!(DAT_100a67c0, DAT_100a67b4);
        copy_loop!(DAT_100a67c4, DAT_100a67b4);

        let mut i_var4: i32 = 0;
        if 0 < count!(DAT_100a67c8) {
            loop {
                let pi_var2 = mech_trace::cr!(
                    "1000cb60",
                    "10018060",
                    FUN_10018060(
                        mech_trace::gr!("1000cb60", "100a67c8", DAT_100a67c8) as i32,
                        i_var4
                    )
                ) as *const i32;
                if mech_trace::rd!("1000cb60", pi_var2, read_unaligned) != 9 {
                    mech_trace::cr!(
                        "1000cb60",
                        "10017df0",
                        FUN_10017df0(
                            mech_trace::gr!("1000cb60", "100a67b4", DAT_100a67b4),
                            pi_var2 as u32
                        )
                    );
                }
                i_var4 = i_var4.wrapping_add(1);
                if !(i_var4 < count!(DAT_100a67c8)) {
                    break;
                }
            }
        }

        copy_loop!(DAT_100a67d4, DAT_100a67cc);
        copy_loop!(DAT_100a67d8, DAT_100a67cc);
        copy_loop!(DAT_100a67dc, DAT_100a67cc);
        copy_loop!(DAT_100a67e0, DAT_100a67cc);
    }
}
