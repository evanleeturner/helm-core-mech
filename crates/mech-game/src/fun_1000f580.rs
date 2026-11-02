// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf38: u32;
    static mut DAT_100eef40: [u32; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100082c4(param_1: i32, param_2: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1000f580() {
    let mut local_14: i32 = -1;
    // SAFETY: the globals and the pointers derived from them are the game's
    // own memory; reads and writes happen where the original does.
    unsafe {
        let base = (&raw mut DAT_100eef40).cast::<u32>();
        let entry = base.add(DAT_100adf38 as usize).read_unaligned();
        let i_var1 = ((entry.wrapping_add(0x20)) as *const i32).read_unaligned();
        let mut local_10: i32 = 0;
        while local_10 < 0xe {
            let i_var2 = ((i_var1.wrapping_add(0x54)) as *const i32)
                .read_unaligned()
                .wrapping_add(local_10.wrapping_mul(0x40));
            let p4 = i_var2.wrapping_add(4) as *const i32;
            let p8 = i_var2.wrapping_add(8) as *mut i32;
            if p4.read_unaligned() > -1 && p4.read_unaligned() == 0x1d {
                if p8.read_unaligned() == 4 {
                    p8.write_unaligned(1);
                    local_14 = 1;
                } else if p8.read_unaligned() == 1 {
                    p8.write_unaligned(4);
                    local_14 = 2;
                }
            }
            local_10 = local_10.wrapping_add(1);
        }
        if local_14 != -1 {
            FUN_100082c4(10, local_14);
        }
    }
}
