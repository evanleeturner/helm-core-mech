// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a9de4: u32;
    static mut DAT_100a9de8: i8;
    static mut DAT_100c6088: [i8; 1];
    static mut DAT_100a9dec: u16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10010010(param_1: *mut i8, param_2: i32) -> *mut i8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn _FILE_read_8(param_1: *mut i8, param_2: i32) -> u32;
}

unsafe fn str_len(p: *const i8) -> usize {
    let mut n = 0usize;
    // SAFETY: p points at a nul-terminated string.
    unsafe {
        while mech_trace::rd!("1001028b", p.add(n), read) != 0 {
            n += 1;
        }
    }
    n
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001028b(param_1: *mut i8) -> u32 {
    mech_trace::enter!("1001028b", param_1);
    let mut local_204 = [0i8; 256];
    let mut local_104 = [0i8; 256];
    let p104 = local_104.as_mut_ptr();
    let p204 = local_204.as_mut_ptr();

    // SAFETY: the buffers are local; the string pointers and globals are the
    // game's own, accessed as the original does.
    unsafe {
        let n = str_len(param_1) + 1;
        mech_trace::cp!(
            "1001028b",
            core::ptr::copy_nonoverlapping,
            param_1 as *const i8,
            p104,
            n
        );

        let e = p104.add(str_len(p104));
        mech_trace::wr!(
            "1001028b",
            e.cast::<u32>(),
            write_unaligned,
            mech_trace::gr!("1001028b", "100a9de4", DAT_100a9de4)
        );
        mech_trace::wr!(
            "1001028b",
            e.add(4),
            write,
            mech_trace::gr!("1001028b", "100a9de8", DAT_100a9de8)
        );

        let r = mech_trace::cr!("1001028b", "10010010", FUN_10010010(p104, 0));
        if r.is_null() {
            return mech_trace::rt!("1001028b", 0);
        }

        let src = (&raw mut DAT_100c6088) as *const i8;
        let n = str_len(src) + 1;
        mech_trace::cp!("1001028b", core::ptr::copy_nonoverlapping, src, p204, n);

        let e = p204.add(str_len(p204));
        mech_trace::wr!(
            "1001028b",
            e.cast::<u16>(),
            write_unaligned,
            mech_trace::gr!("1001028b", "100a9dec", DAT_100a9dec)
        );

        let n = str_len(p104) + 1;
        let e = p204.add(str_len(p204));
        mech_trace::cp!(
            "1001028b",
            core::ptr::copy_nonoverlapping,
            p104 as *const i8,
            e,
            n
        );

        mech_trace::rt!(
            "1001028b",
            mech_trace::cr!("1001028b", "_FILE_read_8", _FILE_read_8(p204, 0))
        )
    }
}
