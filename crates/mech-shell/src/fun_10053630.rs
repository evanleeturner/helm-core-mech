// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1009fc68: u8;
    static mut DAT_1009fc80: u8;
    static mut DAT_1009fc88: u8;
    static mut DAT_1009fc70: u8;
    static mut DAT_1009fc78: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn strstr(haystack: *const i8, needle: *const i8) -> *mut i8;
    fn FUN_10053590(param_1: *mut core::ffi::c_void, param_2: *mut i8, param_3: i32);
}

unsafe fn str_len(mut p: *const u8) -> usize {
    let mut n = 0usize;
    // SAFETY: p points at a NUL-terminated string.
    unsafe {
        while *p != 0 {
            p = p.add(1);
            n += 1;
        }
    }
    n
}

unsafe fn copy_bytes(dst: *mut u8, src: *const u8, n: usize) {
    let mut i = 0usize;
    // SAFETY: caller guarantees both ranges are valid for n bytes.
    unsafe {
        while i < n {
            *dst.add(i) = *src.add(i);
            i += 1;
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10053630(
    param_1: *mut core::ffi::c_void,
    param_2: *mut i8,
    param_3: *mut u32,
) {
    let mut local_b8: [u32; 6] = [0; 6];
    let mut local_a0: [u8; 80] = [0; 80];
    let mut local_50: [u8; 80] = [0; 80];

    // SAFETY: the globals and callees are the game's own; param_2 is a NUL-terminated
    // string and param_3 points at six dwords, used exactly as the original does.
    unsafe {
        let mut i = 0usize;
        while i < 6 {
            local_b8[i] = param_3.add(i).read_unaligned();
            i += 1;
        }

        let a = local_a0.as_mut_ptr();
        let b = local_50.as_mut_ptr();

        let n = str_len(param_2 as *const u8) + 1;
        copy_bytes(a, param_2 as *const u8, n);

        let p = strstr(a as *const i8, &raw const DAT_1009fc68 as *const i8);
        if !p.is_null() {
            *p = 0;
        }

        let n = str_len(a as *const u8) + 1;
        copy_bytes(b, a as *const u8, n);

        let (second_src, second_dst);
        if (param_3.add(2).read_unaligned() & 0x8000_0000) == 0 {
            let l = str_len(a as *const u8);
            copy_bytes(a.add(l), &raw const DAT_1009fc80 as *const u8, 7);
            second_src = &raw const DAT_1009fc88 as *const u8;
            second_dst = b;
        } else {
            let l = str_len(b as *const u8);
            copy_bytes(b.add(l), &raw const DAT_1009fc70 as *const u8, 7);
            second_src = &raw const DAT_1009fc78 as *const u8;
            second_dst = a;
        }
        let l = str_len(second_dst as *const u8);
        copy_bytes(second_dst.add(l), second_src, 6);

        FUN_10053590(param_1, a as *mut i8, param_3 as i32);
        param_3
            .add(4)
            .write_unaligned(param_3.add(5).read_unaligned());
        param_3
            .add(2)
            .write_unaligned(param_3.add(3).read_unaligned());
        FUN_10053590(param_1, b as *mut i8, param_3 as i32);

        let mut i = 0usize;
        while i < 6 {
            param_3.add(i).write_unaligned(local_b8[i]);
            i += 1;
        }
    }
}
