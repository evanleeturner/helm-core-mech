// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b0b38: u32;
    static mut DAT_10098204: i32;
    static mut DAT_100a7248: u32;
    static mut DAT_10098224: *mut c_void;
}

#[allow(non_snake_case)]
unsafe extern "thiscall" {
    fn FUN_10054fe0(this: *mut c_void, param_1: i32, param_2: *mut u32, param_3: *mut u32) -> u32;
    fn FUN_10012470(this: *mut c_void, param_1: i32, param_2: i32, param_3: i32, param_4: i32);
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10069c79(param_1: i32, param_2: i32, param_3: *mut u8);
    fn FUN_10069bf7(param_1: *mut i32, param_2: i32) -> u32;
}

#[allow(non_snake_case)]
unsafe extern "fastcall" {
    fn FUN_10012620(param_1: i32);
}

unsafe extern "stdcall" {
    fn HeapFree(h: *mut c_void, flags: u32, mem: *mut c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "thiscall" fn FUN_10012ac0(
    this: *mut c_void,
    param_1: *mut c_void,
    param_2: i32,
) {
    mech_trace::enter!("10012ac0", this, param_1, param_2);
    let mut local_8: u32 = 0;
    let mut local_4: u32 = 0;
    // SAFETY: the callees and globals are the game's own; `this` points at the
    // game object whose fields are read and written at the original's offsets.
    unsafe {
        let r = mech_trace::cr!(
            "10012ac0",
            "10054fe0",
            FUN_10054fe0(param_1, param_2, &raw mut local_8, &raw mut local_4)
        );
        if r != 0 {
            return;
        }
        let src = &raw mut DAT_100b0b38 as *mut u8;
        FUN_10069c79(local_8 as i32, local_4 as i32, src);
        let dst = this.byte_add(0x92) as *mut u8;
        let mut equal = true;
        let mut i = 0usize;
        while i < 0x300 {
            if mech_trace::dr!("10012ac0", src.add(i)) != mech_trace::dr!("10012ac0", dst.add(i)) {
                equal = false;
                break;
            }
            i += 1;
        }
        if !equal {
            let mut i = 0usize;
            while i < 0xc0 {
                let v = mech_trace::rd!("10012ac0", (src as *const u32).add(i), read_unaligned);
                mech_trace::wr!("10012ac0", (dst as *mut u32).add(i), write_unaligned, v);
                i += 1;
            }
            mech_trace::wr!(
                "10012ac0",
                this.byte_add(0x1e).cast::<u32>(),
                write_unaligned,
                1
            );
            mech_trace::wr!(
                "10012ac0",
                this.byte_add(0x22).cast::<u32>(),
                write_unaligned,
                1
            );
        }
        if mech_trace::rd!(
            "10012ac0",
            this.byte_add(0x1e).cast::<i32>(),
            read_unaligned
        ) == 0
        {
            let mut res: i32 = -1;
            if mech_trace::gr!("10012ac0", "10098204", DAT_10098204) != 0 {
                let f: extern "C" fn() -> i32 = mech_trace::rd!(
                    "10012ac0",
                    ((mech_trace::gr!("10012ac0", "100a7248", DAT_100a7248).wrapping_add(0x20))
                        as *const extern "C" fn() -> i32),
                    read_unaligned
                );
                res = mech_trace::cr!("10012ac0", "ind", f());
            }
            if res == 0 {
                mech_trace::cr!(
                    "10012ac0",
                    "10069bf7",
                    FUN_10069bf7(this.byte_add(0x56).cast::<i32>(), local_8 as i32)
                );
                let mut s = mech_trace::rd!(
                    "10012ac0",
                    this.byte_add(0x2e).cast::<*const u8>(),
                    read_unaligned
                );
                let mut d = mech_trace::rd!(
                    "10012ac0",
                    this.byte_add(0x42).cast::<*mut u8>(),
                    read_unaligned
                );
                let mut n = mech_trace::rd!(
                    "10012ac0",
                    this.byte_add(0x392).cast::<i32>(),
                    read_unaligned
                )
                .wrapping_mul(mech_trace::rd!(
                    "10012ac0",
                    this.byte_add(0x396).cast::<i32>(),
                    read_unaligned
                )) as u32;
                while n != 0 {
                    mech_trace::dw!("10012ac0", d, mech_trace::dr!("10012ac0", s));
                    s = s.add(1);
                    d = d.add(1);
                    n -= 1;
                }
                FUN_10012470(
                    this,
                    mech_trace::rd!(
                        "10012ac0",
                        this.byte_add(0x5a).cast::<i32>(),
                        read_unaligned
                    ),
                    mech_trace::rd!(
                        "10012ac0",
                        this.byte_add(0x5e).cast::<i32>(),
                        read_unaligned
                    ),
                    mech_trace::rd!(
                        "10012ac0",
                        this.byte_add(0x62).cast::<i32>(),
                        read_unaligned
                    ),
                    mech_trace::rd!(
                        "10012ac0",
                        this.byte_add(0x66).cast::<i32>(),
                        read_unaligned
                    ),
                );
            }
        } else {
            mech_trace::cr!(
                "10012ac0",
                "10069bf7",
                FUN_10069bf7(this.byte_add(0x6a).cast::<i32>(), local_8 as i32)
            );
            FUN_10012620(this as i32);
        }
        mech_trace::cr!(
            "10012ac0",
            "HeapFree",
            HeapFree(
                mech_trace::gr!("10012ac0", "10098224", DAT_10098224),
                1,
                local_8 as *mut c_void
            )
        );
    }
}
