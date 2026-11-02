// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38d8: u32;
    static mut DAT_100eef40: u32;
    static mut DAT_101600e0: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1004d34a(param_1: u32, param_2: *mut i32, param_3: *mut i32, param_4: *mut u32) -> u32;
    fn FUN_1002f37a(param_1: i32, param_2: u32, param_3: u32, param_4: u32);
    fn FUN_1002f668(param_1: i32, param_2: i32, param_3: i32, param_4: i32, param_5: u32);
    fn FUN_1002fbc0(param_1: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004d20b(
    param_1: i32,
    param_2: u32,
    param_3: u32,
    param_4: u32,
    param_5: u32,
) -> u32 {
    let mut local_1c: i32 = 0;
    let mut local_c: i32 = 0;
    let mut local_18: u32 = 0;
    // SAFETY: the globals and callees are the game's own memory; the table
    // entries are pointers to records the game keeps live, accessed at the
    // same offsets as the original.
    unsafe {
        if param_1 < 0x10 {
            let local_10 = (&raw mut DAT_101600e0)
                .byte_offset((param_1.wrapping_mul(0x3c)) as isize)
                .cast::<u32>()
                .read_unaligned();
            if local_10 < DAT_100b38d8 {
                let table = &raw mut DAT_100eef40;
                let i_var1 = table.add(local_10 as usize).read_unaligned() as i32;
                let rec = i_var1 as u32 as *mut u8;
                rec.byte_add(0x50).cast::<u32>().write_unaligned(param_2);
                rec.byte_add(0x54).cast::<u32>().write_unaligned(param_3);
                rec.byte_add(0x58).cast::<u32>().write_unaligned(param_4);
                rec.byte_add(0x60).cast::<u32>().write_unaligned(param_5);
                let mut local_20: u32 = 0;
                while local_20 < DAT_100b38d8 {
                    let local_14 = table.add(local_20 as usize).read_unaligned() as i32;
                    let e = local_14 as u32 as *mut u8;
                    if e.byte_add(8).cast::<i32>().read_unaligned() == param_1 {
                        FUN_1004d34a(
                            local_20,
                            &raw mut local_1c,
                            &raw mut local_c,
                            &raw mut local_18,
                        );
                        let h = e.byte_add(0x40).cast::<i32>().read_unaligned();
                        FUN_1002f37a(h, local_1c as u32, param_3, local_c as u32);
                        let h = e.byte_add(0x40).cast::<i32>().read_unaligned();
                        FUN_1002f668(h, 0, local_18 as i32, 0, 0);
                        let h = e.byte_add(0x40).cast::<u32>().read_unaligned();
                        FUN_1002fbc0(h as *mut i32);
                        e.byte_add(0x50).cast::<i32>().write_unaligned(local_1c);
                        e.byte_add(0x54).cast::<u32>().write_unaligned(param_3);
                        e.byte_add(0x58).cast::<i32>().write_unaligned(local_c);
                        e.byte_add(0x60).cast::<u32>().write_unaligned(local_18);
                    }
                    local_20 = local_20.wrapping_add(1);
                }
                return 1;
            }
        }
    }
    0
}
