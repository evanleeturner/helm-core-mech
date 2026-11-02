// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1009c558: u32;
    static mut DAT_100bd950: [i8; 1];
    static mut DAT_100bfd98: [u32; 16];
    static mut s_Default_Config_1009ff50: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10054000();
    fn FUN_100540d0();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10054460() {
    // SAFETY: the globals and callees are the game's own memory and code;
    // the string and destination buffers are read and written as the original does.
    unsafe {
        let u_var2 = DAT_1009c558;
        let mut i_var6: i32 = 0;
        FUN_10054000();
        loop {
            DAT_1009c558 = i_var6 as u32;
            if (&raw const DAT_100bfd98)
                .cast::<u32>()
                .add(i_var6 as usize)
                .read()
                != 0
            {
                FUN_100540d0();
            }
            i_var6 = (DAT_1009c558 as i32).wrapping_add(1);
            if i_var6 >= 0x10 {
                break;
            }
        }
        let mut len: u32 = 0;
        let src_base = (&raw const s_Default_Config_1009ff50).cast::<i8>();
        loop {
            let c = src_base.add(len as usize).read();
            len = len.wrapping_add(1);
            if c == 0 {
                break;
            }
        }
        let mut src = src_base;
        let mut dst = (&raw mut DAT_100bd950).cast::<i8>();
        let mut n = len >> 2;
        while n != 0 {
            dst.cast::<u32>()
                .write_unaligned(src.cast::<u32>().read_unaligned());
            src = src.add(4);
            dst = dst.add(4);
            n -= 1;
        }
        let mut n = len & 3;
        while n != 0 {
            dst.write(src.read());
            src = src.add(1);
            dst = dst.add(1);
            n -= 1;
        }
        DAT_1009c558 = u_var2;
    }
}
