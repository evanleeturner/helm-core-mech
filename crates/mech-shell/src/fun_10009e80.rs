// SPDX-License-Identifier: MIT

use core::arch::asm;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a6738: *mut core::ffi::c_void;
    static mut DAT_1008cfec: [i8; 0];
    static mut DAT_1008cff0: [i8; 0];
    static mut DAT_1008cff8: u32;
    static mut DAT_1008cffc: i8;
    static mut DAT_1008d000: [i8; 0];
    static mut DAT_1008d004: i32;
    static mut DAT_100ae1e0: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn __strnicmp(str: *const i8, param_2: *const i8, param_3: u32) -> i32;
    fn _strchr(str: *const i8, param_2: i32) -> *mut i8;
    fn FID_conflict___wfopen(str: *const i8, param_2: *const i8) -> *mut core::ffi::c_void;
    fn FUN_1006f790(
        str: *mut core::ffi::c_void,
        param_2: u32,
        param_3: u32,
        param_4: *mut core::ffi::c_void,
    ) -> u32;
    fn _fclose(str: *mut core::ffi::c_void) -> i32;
    fn FUN_100364c0(
        this_: *mut core::ffi::c_void,
        str: *mut i8,
        param_2: u32,
        param_3: *mut i32,
    ) -> *mut i8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10009e80(str: *mut i8) -> *mut u8 {
    mech_trace::enter!("10009e80", str);
    let mut local_20: [i8; 32] = [0; 32];
    // SAFETY: the globals and callees are the game's own; str points at the
    // caller's NUL-terminated string; local_20 is written where the original does.
    unsafe {
        if mech_trace::cr!(
            "10009e80",
            "__strnicmp",
            __strnicmp(str.byte_add(5), (&raw mut DAT_1008cfec).cast::<i8>(), 3)
        ) == 0
        {
            let this = mech_trace::gr!("10009e80", "100a6738", DAT_100a6738);
            let p3 = &raw mut DAT_1008d004;
            let ret: *mut i8;
            asm!(
                "push {p3}",
                "push {p2}",
                "push {p1}",
                "call {f}",
                p1 = in(reg) str,
                p2 = in(reg) 8u32,
                p3 = in(reg) p3,
                f = sym FUN_100364c0,
                in("ecx") this,
                lateout("eax") ret,
                clobber_abi("C"),
            );
            return mech_trace::rt!("10009e80", ret.cast::<u8>());
        }

        // copy prefix including terminator
        let prefix = (&raw mut DAT_1008cff0).cast::<i8>();
        let mut i = 0usize;
        loop {
            let c = mech_trace::dr!("10009e80", prefix.add(i));
            local_20[i] = c;
            i += 1;
            if c == 0 {
                break;
            }
        }

        // append str at end of local_20
        let mut end = 0usize;
        while local_20[end] != 0 {
            end += 1;
        }
        let mut j = 0usize;
        loop {
            let c = mech_trace::dr!("10009e80", str.add(j));
            local_20[end + j] = c;
            j += 1;
            if c == 0 {
                break;
            }
        }

        if mech_trace::cr!("10009e80", "_strchr", _strchr(str, 0x2e)).is_null() {
            let mut e = 0usize;
            while local_20[e] != 0 {
                e += 1;
            }
            let p = local_20.as_mut_ptr().add(e);
            mech_trace::wr!(
                "10009e80",
                p.cast::<u32>(),
                write_unaligned,
                mech_trace::gr!("10009e80", "1008cff8", DAT_1008cff8)
            );
            mech_trace::dw!(
                "10009e80",
                p.add(4),
                mech_trace::gr!("10009e80", "1008cffc", DAT_1008cffc)
            );
        }

        let file = mech_trace::cr!(
            "10009e80",
            "FID_conflict___wfopen",
            FID_conflict___wfopen(local_20.as_ptr(), (&raw mut DAT_1008d000).cast::<i8>())
        );
        if file.is_null() {
            return mech_trace::rt!("10009e80", core::ptr::null_mut());
        }
        let buf = (&raw mut DAT_100ae1e0).cast::<u8>();
        mech_trace::cr!(
            "10009e80",
            "1006f790",
            FUN_1006f790(buf.cast::<core::ffi::c_void>(), 1, 0x800, file)
        );
        mech_trace::cr!("10009e80", "_fclose", _fclose(file));
        mech_trace::rt!("10009e80", buf)
    }
}
