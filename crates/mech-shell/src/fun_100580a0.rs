// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10098224: *mut u8;
    static mut DAT_100a6744: u32;
    static mut DAT_100c49d0: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn LAB_10058290();
    fn LAB_10058340();
    fn LAB_100583e0();
    fn LAB_100583f0();
    fn FUN_10018060(param_1: i32, param_2: i32) -> u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapAlloc(h: *mut u8, flags: u32, size: u32) -> *mut i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100580a0(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
) -> *mut i32 {
    if param_3 == 0 {
        return core::ptr::null_mut();
    }
    // SAFETY: globals and callees are the game's own; param_3 points at the
    // caller's collection and the allocated block is sized for all entries.
    unsafe {
        let mut i_var5: i32 = 1;
        let count = ((param_3 as u32).wrapping_add(4) as *const i32).read_unaligned();
        let pi2 = HeapAlloc(
            DAT_10098224,
            1,
            (count.wrapping_mul(0x70).wrapping_add(0x118)) as u32,
        );
        let w = |idx: i32, off: i32, v: i32| {
            pi2.offset(idx.wrapping_mul(14).wrapping_add(off) as isize)
                .write_unaligned(v);
        };
        let dat_addr = (&raw mut DAT_100c49d0) as usize as i32;
        let rowh = |_: ()| ((DAT_100a6744.wrapping_add(0x40c)) as *const i32).read_unaligned();
        w(0, 0, param_1);
        w(0, 1, param_2);
        w(0, 2, -1);
        w(0, 3, rowh(()));
        w(0, 4, 0);
        w(0, 5, dat_addr);
        w(0, 6, 0);
        let mut i_var4: i32 = 0;
        let mut local_4: i32 = 1;
        w(0, 7, LAB_100583f0 as usize as i32);
        w(0, 8, 0);
        w(0, 9, -1);
        w(0, 10, -1);
        w(0, 11, -1);
        let count = ((param_3 as u32).wrapping_add(4) as *const i32).read_unaligned();
        if 0 < count {
            loop {
                let i_var3 = FUN_10018060(param_3, i_var4);
                if i_var3 != 0 && (i_var3.wrapping_add(0x2064) as *const i8).read() == 0 {
                    let i_var6 = i_var5.wrapping_add(1);
                    let a = i_var5;
                    w(a, 0, param_1);
                    w(a, 1, rowh(()).wrapping_mul(local_4).wrapping_add(param_2));
                    w(a, 2, 0x96);
                    w(a, 3, rowh(()));
                    w(a, 4, 0);
                    w(a, 5, dat_addr);
                    w(a, 6, 0);
                    w(a, 7, LAB_10058340 as usize as i32);
                    w(a, 8, LAB_100583e0 as usize as i32);
                    w(a, 9, i_var4);
                    w(a, 10, param_4);
                    w(a, 11, 0);
                    i_var5 = i_var5.wrapping_add(2);
                    let b = i_var6;
                    w(b, 0, param_1.wrapping_add(0x96));
                    w(b, 1, rowh(()).wrapping_mul(local_4).wrapping_add(param_2));
                    w(b, 2, -1);
                    w(b, 3, rowh(()));
                    w(b, 4, 0);
                    w(b, 5, dat_addr);
                    w(b, 6, 0);
                    w(b, 7, LAB_10058290 as usize as i32);
                    w(b, 8, 0);
                    w(
                        b,
                        9,
                        ((i_var3.wrapping_add(0x3c)) as *const i32).read_unaligned(),
                    );
                    w(b, 10, -1);
                    w(b, 11, 0);
                    local_4 = local_4.wrapping_add(1);
                }
                i_var4 = i_var4.wrapping_add(1);
                let count = ((param_3 as u32).wrapping_add(4) as *const i32).read_unaligned();
                if !(i_var4 < count) {
                    break;
                }
            }
        }
        w(i_var5, 0, -1);
        pi2
    }
}
