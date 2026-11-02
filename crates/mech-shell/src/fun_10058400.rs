// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut LAB_100583f0: u8;
    static mut LAB_100585e0: u8;
    static mut LAB_10058680: u8;
    static mut LAB_10058730: u8;
    static mut DAT_10098224: *mut u8;
    static mut DAT_100a6744: u32;
    static mut DAT_100c49d0: u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapAlloc(h: *mut u8, flags: u32, size: u32) -> *mut u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10018060(param_1: i32, param_2: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10058400(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
) -> *mut i32 {
    if param_3 == 0 {
        return core::ptr::null_mut();
    }
    // SAFETY: globals, the allocator and the callee are the game's own; param_3
    // points at the caller's collection and the allocated block is sized for
    // every slot written.
    unsafe {
        let mut i_var4: i32 = 1;
        let cnt = ((param_3 as u32).wrapping_add(4) as *const i32).read_unaligned();
        let p = HeapAlloc(
            DAT_10098224,
            1,
            (cnt.wrapping_mul(0x70).wrapping_add(0x118)) as u32,
        ) as *mut i32;
        let set = |i: i32, v: i32| {
            // SAFETY: index lies within the block allocated above.
            unsafe { p.offset(i as isize).write_unaligned(v) }
        };
        let row = || -> i32 {
            // SAFETY: DAT_100a6744 points at the game's structure.
            unsafe { ((DAT_100a6744.wrapping_add(0x40c)) as *const i32).read_unaligned() }
        };
        let c49d0 = (&raw mut DAT_100c49d0) as i32;
        set(0, param_1);
        set(1, param_2);
        set(2, 100);
        set(3, row());
        set(4, 0);
        set(5, c49d0);
        set(6, 0);
        let mut i_var3: i32 = 0;
        let mut local_8: i32 = 1;
        set(7, (&raw mut LAB_100583f0) as i32);
        set(8, 0);
        set(9, -1);
        set(10, -1);
        set(11, -1);
        if 0 < ((param_3 as u32).wrapping_add(4) as *const i32).read_unaligned() {
            loop {
                let i_var2 = FUN_10018060(param_3, i_var3);
                let i_var5 = i_var4.wrapping_add(1);
                let b4 = i_var4.wrapping_mul(0xe);
                set(b4, param_1);
                set(
                    b4.wrapping_add(1),
                    row().wrapping_mul(local_8).wrapping_add(param_2),
                );
                set(b4.wrapping_add(2), 0x96);
                set(b4.wrapping_add(3), row());
                set(b4.wrapping_add(4), 0);
                set(b4.wrapping_add(5), c49d0);
                set(b4.wrapping_add(6), 0);
                set(b4.wrapping_add(7), (&raw mut LAB_100585e0) as i32);
                set(b4.wrapping_add(8), (&raw mut LAB_10058730) as i32);
                set(b4.wrapping_add(9), i_var3);
                i_var3 = i_var3.wrapping_add(1);
                set(b4.wrapping_add(10), param_4);
                set(b4.wrapping_add(11), 0);
                i_var4 = i_var4.wrapping_add(2);
                let b5 = i_var5.wrapping_mul(0xe);
                set(b5, param_1.wrapping_add(0x96));
                set(
                    b5.wrapping_add(1),
                    row().wrapping_mul(local_8).wrapping_add(param_2),
                );
                set(b5.wrapping_add(2), -1);
                set(b5.wrapping_add(3), row());
                set(b5.wrapping_add(4), 0);
                set(b5.wrapping_add(5), c49d0);
                set(b5.wrapping_add(6), 0);
                set(b5.wrapping_add(7), (&raw mut LAB_10058680) as i32);
                set(b5.wrapping_add(8), 0);
                set(
                    b5.wrapping_add(9),
                    (i_var2.wrapping_add(0x24) as *const i32).read_unaligned(),
                );
                local_8 = local_8.wrapping_add(1);
                set(b5.wrapping_add(10), param_4);
                set(b5.wrapping_add(11), 0);
                if !(i_var3 < ((param_3 as u32).wrapping_add(4) as *const i32).read_unaligned()) {
                    break;
                }
            }
        }
        set(i_var4.wrapping_mul(0xe), -1);
        p
    }
}
