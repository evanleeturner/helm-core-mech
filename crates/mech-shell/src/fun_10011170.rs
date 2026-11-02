// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b07f0: u32;
    static mut DAT_100b07f8: u32;
    static mut DAT_100b0a30: [i8; 0];
    static mut DAT_1008e964: u32;
    static mut DAT_1008e968: i8;
    static mut DAT_1008e96c: u32;
    static mut DAT_1008e950: i32;
    static mut DAT_1008e970: [i8; 0];
    static mut DAT_1008e974: [i8; 0];
    static mut DAT_100a6b90: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10035eb0();
    fn FUN_10035f00_placeholder_never_used();
}

#[allow(non_snake_case)]
unsafe extern "C" {
    #[link_name = "FID_conflict___wfopen"]
    fn fid_wfopen(param_1: *const i8, param_2: *const i8) -> *mut u8;
    #[link_name = "_fread"]
    fn c_fread(param_1: *mut u8, param_2: u32, param_3: u32, param_4: *mut u8) -> u32;
    #[link_name = "_fclose"]
    fn c_fclose(param_1: *mut u8) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10011170(param_1: u32, param_2: u8, param_3: *mut i8) {
    mech_trace::enter!("10011170", param_1, param_2, param_3);
    // SAFETY: the globals, buffers and callees are the game's own; the
    // buffer at DAT_100b0a30 is large enough in the original, and the
    // strings are NUL-terminated as the original assumes.
    unsafe {
        mech_trace::gw!("10011170", "100b07f0", DAT_100b07f0, param_1);
        mech_trace::gw!("10011170", "100b07f8", DAT_100b07f8, param_2 as u32);

        let buf = (&raw mut DAT_100b0a30).cast::<u8>();

        // copy param_3 including NUL
        let mut src = param_3 as *const u8;
        let mut dst = buf;
        loop {
            let c = mech_trace::rd!("10011170", src, read);
            mech_trace::wr!("10011170", dst, write, c);
            src = src.add(1);
            dst = dst.add(1);
            if c == 0 {
                break;
            }
        }

        let c1 = mech_trace::gr!("10011170", "1008e968", DAT_1008e968);

        // find end of buffer string
        let mut p = buf;
        while mech_trace::rd!("10011170", p, read) != 0 {
            p = p.add(1);
        }
        mech_trace::wr!(
            "10011170",
            p.cast::<u32>(),
            write_unaligned,
            mech_trace::gr!("10011170", "1008e964", DAT_1008e964)
        );
        mech_trace::wr!("10011170", p.add(3), write, c1 as u8);

        // append s_launch_scr
        let mut src = (&raw const DAT_100a6b90).cast::<u8>();
        let mut p = buf;
        while mech_trace::rd!("10011170", p, read) != 0 {
            p = p.add(1);
        }
        loop {
            let c = mech_trace::rd!("10011170", src, read);
            mech_trace::wr!("10011170", p, write, c);
            src = src.add(1);
            p = p.add(1);
            if c == 0 {
                break;
            }
        }

        if mech_trace::gr!("10011170", "1008e950", DAT_1008e950) != 0 {
            let mut p = buf;
            while mech_trace::rd!("10011170", p, read) != 0 {
                p = p.add(1);
            }
            mech_trace::wr!(
                "10011170",
                p.cast::<u32>(),
                write_unaligned,
                mech_trace::gr!("10011170", "1008e96c", DAT_1008e96c)
            );
        }

        FUN_10035eb0();
        let file = mech_trace::cr!(
            "10011170",
            "fid_wfopen",
            fid_wfopen(
                (&raw const DAT_1008e974).cast::<i8>(),
                (&raw const DAT_1008e970).cast::<i8>(),
            )
        );
        if !file.is_null() {
            mech_trace::cr!(
                "10011170",
                "c_fread",
                c_fread((&raw mut DAT_100b07f0).cast::<u8>(), 0x344, 1, file)
            );
            mech_trace::cr!("10011170", "c_fclose", c_fclose(file));
        }
    }
}
