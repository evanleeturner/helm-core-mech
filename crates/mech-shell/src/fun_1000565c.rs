// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10083878: [u32; 0];
    static mut DAT_10083874: i32;
    static mut DAT_10083078: [i8; 0];
    static mut DAT_100ad560: [i8; 0];
    static mut DAT_1008517c: [*const i8; 0];
    static mut DAT_10082bec: [u8; 0];
    static mut DAT_1008307c: [i8; 0];
    static mut DAT_10083084: [i8; 0];
    static mut DAT_10083088: [i8; 0];
    static mut DAT_10083098: [i8; 0];
    static mut DAT_100a6740: *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1006e7b0(param_1: *mut i8, param_2: *const i8, ...) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "thiscall" {
    fn FUN_10050c30(
        this_: *mut core::ffi::c_void,
        param_1: i32,
        param_2: i32,
        param_3: *mut i8,
        param_4: i32,
    ) -> *mut i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000565c(param_1: i32, param_2: *mut i32) -> *mut i32 {
    mech_trace::enter!("1000565c", param_1, param_2);
    let _ = param_1;
    // SAFETY: all globals, tables and callees are the game's own memory;
    // param_2 points at the caller's record, read where the original reads.
    unsafe {
        let buf = (&raw mut DAT_100ad560).cast::<i8>();
        let table = (&raw mut DAT_10083878).cast::<u32>();
        let idx = mech_trace::rd!("1000565c", param_2.add(9), read_unaligned)
            .wrapping_add(mech_trace::gr!("1000565c", "10083874", DAT_10083874).wrapping_mul(0xc));
        let entry = mech_trace::rd!(
            "1000565c",
            table.wrapping_offset(idx as isize),
            read_unaligned
        );
        if entry == 0xffff_ffff {
            return mech_trace::rt!("1000565c", core::ptr::null_mut());
        }
        let u_var4 = entry & 0x7fff_ffff;
        if u_var4 == 0 {
            let mut src = (&raw mut DAT_10083078).cast::<i8>().cast_const();
            let mut dst = buf;
            loop {
                let c = mech_trace::rd!("1000565c", src, read);
                mech_trace::wr!("1000565c", dst, write, c);
                src = src.add(1);
                dst = dst.add(1);
                if c == 0 {
                    break;
                }
            }
        } else if u_var4 < 7000 {
            let names = (&raw mut DAT_1008517c).cast::<*const i8>();
            let name = mech_trace::rd!(
                "1000565c",
                names.add((u_var4 / 100) as usize * 0x10),
                read_unaligned
            );
            mech_trace::cr!(
                "1000565c",
                "1006e7b0",
                FUN_1006e7b0(
                    buf,
                    (&raw mut DAT_1008307c).cast::<i8>().cast_const(),
                    name,
                    u_var4 % 100,
                )
            );
        } else if u_var4 < 10000 {
            let base = (&raw mut DAT_10082bec).cast::<u8>();
            let mut i_var7: i32 = 0x17;
            let mut p = base.add(0x110);
            loop {
                let v = mech_trace::rd!("1000565c", p.cast::<i32>(), read_unaligned);
                if ((v / 0x32) as u32) == u_var4 / 0x32 {
                    let s = mech_trace::rd!(
                        "1000565c",
                        base.add((i_var7 as usize).wrapping_mul(0xc))
                            .cast::<*const i8>(),
                        read_unaligned
                    );
                    mech_trace::cr!(
                        "1000565c",
                        "1006e7b0",
                        FUN_1006e7b0(buf, (&raw mut DAT_10083084).cast::<i8>().cast_const(), s)
                    );
                    break;
                }
                p = p.wrapping_sub(12);
                i_var7 = i_var7.wrapping_sub(1);
                if (p as usize) <= (base.wrapping_sub(5) as usize) {
                    break;
                }
            }
            if i_var7 < 0 {
                mech_trace::cr!(
                    "1000565c",
                    "1006e7b0",
                    FUN_1006e7b0(
                        buf,
                        (&raw mut DAT_10083088).cast::<i8>().cast_const(),
                        u_var4,
                    )
                );
            }
        } else {
            let i_var7 = u_var4.wrapping_sub(10000) as i32;
            let names = (&raw mut DAT_1008517c).cast::<*const i8>();
            let name = mech_trace::rd!(
                "1000565c",
                names.wrapping_offset(((i_var7 / 10000) as isize).wrapping_mul(0x10)),
                read_unaligned
            );
            mech_trace::cr!(
                "1000565c",
                "1006e7b0",
                FUN_1006e7b0(
                    buf,
                    (&raw mut DAT_10083098).cast::<i8>().cast_const(),
                    name,
                    (i_var7 / 100) % 100,
                    i_var7 % 100,
                )
            );
        }
        mech_trace::rt!(
            "1000565c",
            mech_trace::cr!(
                "1000565c",
                "10050c30",
                FUN_10050c30(
                    mech_trace::gr!("1000565c", "100a6740", DAT_100a6740),
                    mech_trace::rd!("1000565c", param_2, read_unaligned),
                    mech_trace::rd!("1000565c", param_2.add(1), read_unaligned),
                    buf,
                    mech_trace::rd!("1000565c", param_2.add(5), read_unaligned),
                )
            )
        )
    }
}
