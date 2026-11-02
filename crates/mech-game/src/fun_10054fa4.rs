// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10055664(param_1: *mut i32, param_2: *mut i32, param_3: u32, param_4: *mut u32) -> u8;
    fn FUN_100558df(param_1: *mut i32, param_2: *mut i32, param_3: u32, param_4: *mut u32) -> u8;
}

unsafe fn copy12(dst: *mut u32, src: *const u32) {
    // SAFETY: caller guarantees both point at 12 u32 values.
    unsafe {
        for i in 0..12 {
            dst.add(i).write_unaligned(src.add(i).read_unaligned());
        }
    }
}

unsafe fn stage(
    src: *mut u32,
    dst: *mut u32,
    n: i32,
    field: usize,
    t: u32,
    greater: bool,
    clip: unsafe extern "C" fn(*mut i32, *mut i32, u32, *mut u32) -> u8,
    tmp: *mut u32,
) -> i32 {
    // SAFETY: buffers are the caller's local arrays; indexes follow the original.
    unsafe {
        let ti = t as i32;
        let pred = |v: i32| -> bool { if greater { ti < v } else { v < ti } };
        let mut count: i32 = 0;
        let mut out = dst;
        let mut last = src.offset((n.wrapping_sub(1).wrapping_mul(12)) as isize);
        let mut i: i32 = 0;
        while i < n {
            let cur = src.offset((i.wrapping_mul(12)) as isize);
            let cv = cur.add(field).read_unaligned() as i32;
            let lv = last.add(field).read_unaligned() as i32;
            if pred(cv) {
                if !pred(lv) {
                    clip(last as *mut i32, cur as *mut i32, t, tmp);
                    copy12(out, tmp);
                    out = out.add(12);
                    count = count.wrapping_add(1);
                }
            } else if pred(lv) {
                clip(last as *mut i32, cur as *mut i32, t, tmp);
                copy12(out, tmp);
                copy12(out.add(12), cur);
                out = out.add(24);
                count = count.wrapping_add(2);
            } else {
                copy12(out, cur);
                out = out.add(12);
                count = count.wrapping_add(1);
            }
            last = cur;
            i += 1;
        }
        count
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10054fa4(
    param_1: *const *const u32,
    param_2: i32,
    param_3: *mut u32,
) -> i32 {
    let mut buf_a = [0u32; 240];
    let mut buf_b = [0u32; 240];
    let mut tmp = [0u32; 12];
    let a = buf_a.as_mut_ptr();
    let b = buf_b.as_mut_ptr();
    let t = tmp.as_mut_ptr();
    // SAFETY: pointers come from the caller's list and local arrays; the
    // callees are the game's own.
    unsafe {
        let mut p1 = param_1;
        let mut dst = a;
        let mut k = param_2;
        while k != 0 {
            let s = p1.read_unaligned();
            p1 = p1.add(1);
            copy12(dst, s);
            dst = dst.add(12);
            k = k.wrapping_sub(1);
        }
        let n1 = stage(a, b, param_2, 3, 0x3fff, true, FUN_10055664, t);
        let n2 = stage(b, a, n1, 3, 0xffffc000, false, FUN_10055664, t);
        let n3 = stage(a, b, n2, 4, 0xffffc000, false, FUN_100558df, t);
        let n4 = stage(b, a, n3, 4, 0x3fff, true, FUN_100558df, t);

        let mut src = a as *const u32;
        let mut out = param_3;
        let mut k = n4;
        while k != 0 {
            out.write_unaligned(src.add(3).read_unaligned());
            out.add(1).write_unaligned(src.add(4).read_unaligned());
            out.add(3).write_unaligned(src.add(5).read_unaligned());
            out.add(4).write_unaligned(src.add(6).read_unaligned());
            out.add(2).write_unaligned(src.add(0xb).read_unaligned());
            out.add(5).write_unaligned(src.add(2).read_unaligned());
            src = src.add(12);
            out = out.add(6);
            k = k.wrapping_sub(1);
        }
        n4
    }
}
