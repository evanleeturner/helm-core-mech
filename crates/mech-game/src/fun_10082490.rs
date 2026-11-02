// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b9f60: u32;
    static mut DAT_100eef40: [u32; 0];
    static mut DAT_10215670: u8;
    static mut DAT_10215674: u8;
    static mut DAT_10215678: u8;
    static mut DAT_1021567c: u8;
    static mut DAT_10215680: u8;
    static mut DAT_10215688: u8;
    static mut DAT_1021568c: u8;
    static mut DAT_10215690: u8;
    static mut DAT_10215694: u8;
    static mut DAT_10215698: u8;
    static mut DAT_10215699: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10082490(
    param_1: u32,
    param_2: u32,
    param_3: u32,
    param_4: u32,
) -> i32 {
    let mut local_8: i32 = -1;
    // SAFETY: the globals are the game's own memory; the record table and the
    // pointer table are indexed exactly as the original does.
    unsafe {
        if DAT_100b9f60 < 0x80 && DAT_100b9f60 != 0xFFFF_FFFF {
            let i_var1 = (DAT_100b9f60 as i32).wrapping_mul(0x54) as usize;
            (&raw mut DAT_10215688)
                .byte_add(i_var1)
                .cast::<u32>()
                .write_unaligned(param_2);
            (&raw mut DAT_1021568c)
                .byte_add(i_var1)
                .cast::<u32>()
                .write_unaligned(param_3);
            (&raw mut DAT_10215690)
                .byte_add(i_var1)
                .cast::<u32>()
                .write_unaligned(param_4);
            (&raw mut DAT_10215670)
                .byte_add(i_var1)
                .cast::<u32>()
                .write_unaligned(1);
            (&raw mut DAT_10215694)
                .byte_add(i_var1)
                .cast::<u16>()
                .write_unaligned(0x401);
            (&raw mut DAT_10215678)
                .byte_add(i_var1)
                .cast::<u32>()
                .write_unaligned(param_1 | 0x200);
            let entry = (&raw const DAT_100eef40)
                .cast::<u32>()
                .add(param_1 as usize)
                .read_unaligned();
            let val = (entry.wrapping_add(8) as usize as *const u32).read_unaligned();
            (&raw mut DAT_1021567c)
                .byte_add(i_var1)
                .cast::<u32>()
                .write_unaligned(val);
            (&raw mut DAT_10215680)
                .byte_add(i_var1)
                .cast::<u32>()
                .write_unaligned(3000);
            (&raw mut DAT_10215674)
                .byte_add(i_var1)
                .cast::<u32>()
                .write_unaligned(0);
            (&raw mut DAT_10215698).byte_add(i_var1).write(0x21);
            (&raw mut DAT_10215699).byte_add(i_var1).write(0);
            local_8 = DAT_100b9f60 as i32;
            DAT_100b9f60 = DAT_100b9f60.wrapping_add(1);
        }
    }
    local_8
}
