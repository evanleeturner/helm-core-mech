// SPDX-License-Identifier: MIT
use core::arch::asm;
use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10083508: u32;
    static mut DAT_1009b3e0: [i8; 0];
    static mut DAT_1009b3fc: [i8; 0];
    static mut DAT_1009b428: [i8; 0];
    static mut DAT_1009b44c: [i8; 0];
    static mut DAT_1009b498: [i8; 0];
    static mut DAT_100a6744: *mut c_void;
    static mut DAT_100bacc4: u32;
    static mut DAT_100baccc: u32;
    static mut DAT_1009b49c: [i8; 0];
    static mut DAT_1009b400: [i8; 0];
    static mut DAT_1009b3e4: [i8; 0];
    static mut DAT_1009b42c: [i8; 0];
    static mut DAT_1009b450: [i8; 0];
    static mut DAT_1009b474: [i8; 0];
    static mut DAT_1009b484: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000c4e0(param_1: u32, param_2: i32) -> i32;
    fn sprintf(dest: *mut i8, format: *const i8, ...) -> i32;
    fn FUN_1004b180(param_1: i8) -> i32;
    fn FUN_10050b20();
}

#[allow(non_snake_case)]
unsafe fn measure(this: *mut c_void, text: *mut i8) -> i32 {
    let r: i32;
    // SAFETY: calls the game's thiscall routine with this in ecx and one
    // stack argument, which the callee pops.
    unsafe {
        mech_trace::log::debug!(
            f = "1004b200", b = "10050b20", a:% = mech_trace::Args([
                mech_trace::__shown!(text),
                mech_trace::__shown!(this),
            ]); "asm.in"
        );
        asm!(
            "push {arg}",
            "call {f}",
            arg = in(reg) text,
            f = sym FUN_10050b20,
            in("ecx") this,
            out("eax") r,
            clobber_abi("C"),
        );
        mech_trace::cr!("1004b200", "10050b20", r);
    }
    r
}

unsafe fn append(dst: *mut i8, src: *const i8) {
    // SAFETY: both are NUL-terminated buffers supplied by the caller/local.
    unsafe {
        let mut n: usize = 0;
        while mech_trace::dr!("1004b200", src.add(n)) != 0 {
            n += 1;
        }
        n += 1;
        let mut d = dst;
        while mech_trace::dr!("1004b200", d) != 0 {
            d = d.add(1);
        }
        let mut i = 0;
        while i < n {
            mech_trace::dw!(
                "1004b200",
                d.add(i),
                mech_trace::dr!("1004b200", src.add(i))
            );
            i += 1;
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004b200(param_1: *mut i8) {
    mech_trace::enter!("1004b200", param_1);
    let mut local_200 = [0i8; 256];
    let mut local_100 = [0i8; 256];
    let l2 = local_200.as_mut_ptr();
    let l1 = local_100.as_mut_ptr();
    // SAFETY: the globals and callees are the game's own; the local buffers
    // are 256 bytes and param_1 is the caller's NUL-terminated buffer.
    unsafe {
        let funds = mech_trace::rd!(
            "1004b200",
            (mech_trace::gr!("1004b200", "10083508", DAT_10083508).wrapping_add(0x12a)
                as *const i32),
            read_unaligned
        );

        let mut i2 = mech_trace::cr!(
            "1004b200",
            "1000c4e0",
            FUN_1000c4e0(funds as u32, l2 as i32)
        );
        mech_trace::cr!(
            "1004b200",
            "sprintf",
            sprintf(l2, (&raw mut DAT_1009b3e0).cast::<i8>(), i2)
        );
        i2 = measure(mech_trace::gr!("1004b200", "100a6744", DAT_100a6744), l2);
        mech_trace::cr!(
            "1004b200",
            "sprintf",
            sprintf(l1, (&raw mut DAT_1009b3e4).cast::<i8>(), 0x202i32, i2, l2)
        );
        append(param_1, l1);

        i2 = mech_trace::cr!(
            "1004b200",
            "1000c4e0",
            FUN_1000c4e0(
                mech_trace::gr!("1004b200", "100baccc", DAT_100baccc),
                l2 as i32
            )
        );
        mech_trace::cr!(
            "1004b200",
            "sprintf",
            sprintf(l2, (&raw mut DAT_1009b3fc).cast::<i8>(), i2)
        );
        mech_trace::gw!(
            "1004b200",
            "100bacc4",
            DAT_100bacc4,
            mech_trace::gr!("1004b200", "100bacc4", DAT_100bacc4).wrapping_add(mech_trace::gr!(
                "1004b200",
                "100baccc",
                DAT_100baccc
            ))
        );
        i2 = measure(mech_trace::gr!("1004b200", "100a6744", DAT_100a6744), l2);
        mech_trace::cr!(
            "1004b200",
            "sprintf",
            sprintf(l1, (&raw mut DAT_1009b400).cast::<i8>(), 0x202i32, i2, l2)
        );
        append(param_1, l1);

        i2 = mech_trace::cr!(
            "1004b200",
            "1000c4e0",
            FUN_1000c4e0(
                mech_trace::gr!("1004b200", "100bacc4", DAT_100bacc4),
                l2 as i32
            )
        );
        mech_trace::cr!(
            "1004b200",
            "sprintf",
            sprintf(l2, (&raw mut DAT_1009b428).cast::<i8>(), i2)
        );
        i2 = measure(mech_trace::gr!("1004b200", "100a6744", DAT_100a6744), l2);
        mech_trace::cr!(
            "1004b200",
            "sprintf",
            sprintf(l1, (&raw mut DAT_1009b42c).cast::<i8>(), 0x202i32, i2, l2)
        );
        append(param_1, l1);

        i2 = mech_trace::cr!("1004b200", "1004b180", FUN_1004b180(0));
        i2 = mech_trace::cr!(
            "1004b200",
            "1000c4e0",
            FUN_1000c4e0(i2.wrapping_neg() as u32, l2 as i32)
        );
        mech_trace::cr!(
            "1004b200",
            "sprintf",
            sprintf(l2, (&raw mut DAT_1009b44c).cast::<i8>(), i2)
        );
        i2 = measure(mech_trace::gr!("1004b200", "100a6744", DAT_100a6744), l2);
        mech_trace::cr!(
            "1004b200",
            "sprintf",
            sprintf(l1, (&raw mut DAT_1009b450).cast::<i8>(), 0x202i32, i2, l2)
        );
        append(param_1, l1);

        mech_trace::cr!(
            "1004b200",
            "sprintf",
            sprintf(l2, (&raw mut DAT_1009b474).cast::<i8>())
        );
        i2 = measure(mech_trace::gr!("1004b200", "100a6744", DAT_100a6744), l2);
        mech_trace::cr!(
            "1004b200",
            "sprintf",
            sprintf(l1, (&raw mut DAT_1009b484).cast::<i8>(), 0x202i32, i2, l2)
        );
        append(param_1, l1);

        let bal = mech_trace::rd!(
            "1004b200",
            (mech_trace::gr!("1004b200", "10083508", DAT_10083508).wrapping_add(0x12a)
                as *const i32),
            read_unaligned
        );
        let i3 = mech_trace::cr!("1004b200", "1004b180", FUN_1004b180(0));
        i2 = mech_trace::cr!(
            "1004b200",
            "1000c4e0",
            FUN_1000c4e0(
                bal.wrapping_sub(i3).wrapping_add(mech_trace::gr!(
                    "1004b200",
                    "100bacc4",
                    DAT_100bacc4
                ) as i32) as u32,
                l2 as i32,
            )
        );
        mech_trace::cr!(
            "1004b200",
            "sprintf",
            sprintf(l2, (&raw mut DAT_1009b498).cast::<i8>(), i2)
        );
        i2 = measure(mech_trace::gr!("1004b200", "100a6744", DAT_100a6744), l2);
        mech_trace::cr!(
            "1004b200",
            "sprintf",
            sprintf(l1, (&raw mut DAT_1009b49c).cast::<i8>(), 0x202i32, i2, l2)
        );
        append(param_1, l1);
    }
}
