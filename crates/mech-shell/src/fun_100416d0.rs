// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100983b0: i32;
    static mut DAT_100983b4: i32;
    static mut DAT_100983b8: i8;
    static mut DAT_100983bc: i8;
    static mut _DAT_100983c0: i32;
    static mut DAT_100983c4: i32;
    static mut DAT_1009845c: [i8; 0];
    static mut DAT_100ba1d8: [i8; 0];
    static mut DAT_10098460: [i8; 0];
    static mut DAT_100ba1f8: [i8; 0];
    static mut DAT_100983c8: i32;
    static mut DAT_100983cc: i32;
    static mut DAT_100983d0: i32;
    static mut DAT_100ba1d0: i32;
    static mut _DAT_100983d4: i32;
    static mut DAT_100983d8: i32;
    static mut _DAT_100983e4: i32;
}

/// Copies the NUL-terminated string at `src` (terminator included) to `dst`.
#[inline(always)]
unsafe fn copy_cstr(src: *const i8, dst: *mut i8) {
    // SAFETY: src is a NUL-terminated game string and dst is a game buffer
    // large enough to hold it, as in the original.
    unsafe {
        let mut u_var2: u32 = 0xffff_ffff;
        let mut pc4 = src;
        loop {
            if u_var2 == 0 {
                break;
            }
            u_var2 = u_var2.wrapping_sub(1);
            let c = pc4.read();
            pc4 = pc4.add(1);
            if c == 0 {
                break;
            }
        }
        let len = 0u32.wrapping_sub(u_var2);
        let mut s = pc4.byte_sub(len as usize);
        let mut d = dst;
        let mut n = len >> 2;
        while n != 0 {
            d.cast::<u32>()
                .write_unaligned(s.cast::<u32>().read_unaligned());
            s = s.byte_add(4);
            d = d.byte_add(4);
            n = n.wrapping_sub(1);
        }
        let mut r = len & 3;
        while r != 0 {
            d.write(s.read());
            s = s.add(1);
            d = d.add(1);
            r = r.wrapping_sub(1);
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100416d0() {
    // SAFETY: all globals and buffers are the game's own memory; strings are
    // copied exactly as the original does.
    unsafe {
        DAT_100983b0 = 0;
        DAT_100983b4 = 0;
        DAT_100983b8 = 0;
        DAT_100983bc = 0;
        _DAT_100983c0 = 0;
        DAT_100983c4 = 0;
        copy_cstr(
            (&raw mut DAT_1009845c).cast::<i8>(),
            (&raw mut DAT_100ba1d8).cast::<i8>(),
        );
        copy_cstr(
            (&raw mut DAT_10098460).cast::<i8>(),
            (&raw mut DAT_100ba1f8).cast::<i8>(),
        );
        DAT_100983c8 = 0;
        DAT_100983cc = 0;
        DAT_100983d0 = 0;
        DAT_100ba1d0 = 0;
        _DAT_100983d4 = 0;
        DAT_100983d8 = 0;
        _DAT_100983e4 = 0;
    }
}
