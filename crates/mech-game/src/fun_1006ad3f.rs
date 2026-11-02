// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf38: u32;
    static mut DAT_100b38d8: i32;
    static mut DAT_100b38fc: u32;
    static mut DAT_100eee60: u32;
    static mut DAT_100eef40: [u32; 1];
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn _AIL_serve_0() -> u32;
    #[link_name = "FUN_10002849"]
    fn RefillStreamingSampleBuffers() -> u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10003742(param_1: u32, param_2: i32, param_3: u32, param_4: i32, param_5: u16) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1006ad3f() {
    // SAFETY: the callees and globals are the game's own; the table entries
    // are pointers to the game's records, read at the original's offsets.
    unsafe {
        _AIL_serve_0();
        RefillStreamingSampleBuffers();
        if DAT_100eee60 <= DAT_100b38fc {
            DAT_100eee60 = DAT_100b38fc.wrapping_add(0x389);
            let table = (&raw mut DAT_100eef40).cast::<u32>();
            let cur = table.add(DAT_100adf38 as usize).read_unaligned();
            let flags = (cur as usize as *const u8)
                .byte_add(0x14)
                .cast::<u16>()
                .read_unaligned();
            if (flags & 0x16) == 0 {
                let mut local_c: i32 = 0;
                while local_c < DAT_100b38d8 {
                    let i_var1 = table.add(local_c as usize).read_unaligned() as usize as *const u8;
                    if i_var1.byte_add(4).cast::<u32>().read_unaligned() != DAT_100adf38
                        && (u32::from(i_var1.byte_add(0x17a).cast::<u16>().read_unaligned()) & 0xff)
                            == DAT_100adf38
                    {
                        let s = i_var1.byte_add(0x176).cast::<i16>().read_unaligned();
                        if (s == 2 || s == 3)
                            && i_var1.byte_add(0xc4).cast::<i32>().read_unaligned() < 0x249f1
                        {
                            FUN_10003742(0x116, 100, 0x40, 5, 0x32);
                            return;
                        }
                    }
                    local_c = local_c.wrapping_add(1);
                }
            }
        }
    }
}
