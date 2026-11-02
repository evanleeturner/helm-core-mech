// SPDX-License-Identifier: MIT

unsafe extern "C" {
    fn FUN_10065029(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
        param_7: i32,
    ) -> u32;
}

unsafe fn fill(addr: u32, count: u32, val: i32, b: u8) {
    let mut p = addr as *mut u8;
    let mut n = count >> 2;
    let mut r = count & 3;
    // SAFETY: the destination is the caller's surface memory, written exactly
    // as the original does.
    unsafe {
        while n != 0 {
            p.cast::<i32>().write_unaligned(val);
            p = p.byte_add(4);
            n = n.wrapping_sub(1);
        }
        while r != 0 {
            p.write(b);
            p = p.byte_add(1);
            r = r.wrapping_sub(1);
        }
    }
}

#[allow(clippy::too_many_arguments)]
unsafe fn plot(
    cx: i32,
    dx: i32,
    min_x: i32,
    max_x: i32,
    cy: i32,
    dy: i32,
    min_y: i32,
    max_y: i32,
    stride: i32,
    base: i32,
    val: i32,
    b: u8,
) {
    let mut x1 = cx.wrapping_add(dx);
    if min_x <= x1 {
        if max_x <= x1 {
            x1 = max_x;
        }
        let mut x0 = cx.wrapping_sub(dx);
        if x0 <= max_x {
            if x0 <= min_x {
                x0 = min_x;
            }
            let y = cy.wrapping_add(dy);
            if min_y <= y {
                if y <= max_y {
                    let cnt = (x1.wrapping_sub(x0).wrapping_add(1)) as u32;
                    let addr = (y as u32)
                        .wrapping_mul(stride as u32)
                        .wrapping_add(base as u32)
                        .wrapping_add(x0 as u32);
                    // SAFETY: see fill.
                    unsafe { fill(addr, cnt, val, b) };
                }
                let y = cy.wrapping_sub(dy);
                if min_y <= y && y <= max_y {
                    let cnt = (x1.wrapping_sub(x0).wrapping_add(1)) as u32;
                    let addr = (y as u32)
                        .wrapping_mul(stride as u32)
                        .wrapping_add(base as u32)
                        .wrapping_add(x0 as u32);
                    // SAFETY: see fill.
                    unsafe { fill(addr, cnt, val, b) };
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100683cd(
    param_1: *mut i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
) -> u32 {
    let mut param_5 = param_5;
    if param_4 == 0 || param_5 == 0 {
        // SAFETY: the callee is the game's own function.
        return unsafe {
            FUN_10065029(
                param_1,
                param_2.wrapping_sub(param_4),
                param_3.wrapping_sub(param_5),
                param_2.wrapping_add(param_4),
                param_3.wrapping_add(param_5),
                0,
                param_6,
            )
        };
    }
    // SAFETY: param_1 points at the game's surface descriptor; its fields and
    // the pixel memory are read and written where the original does.
    unsafe {
        let piv = param_1.read() as *const i32;
        let i12 = piv.add(1).read_unaligned();
        let i8 = i12.wrapping_add(1);
        if i8 == 0 || i12 < -1 {
            return 0xffff_ffff;
        }
        let i1h = piv.add(2).read_unaligned();
        if i1h < 0 {
            return 0xffff_ffff;
        }
        let i2 = param_1.add(1).read_unaligned();
        let i9 = if i2 < 1 { 0 } else { i2 };
        let i3 = param_1.add(2).read_unaligned();
        let i10 = if i3 < 1 { 0 } else { i3 };
        let mut i11 = param_1.add(3).read_unaligned();
        if i12 <= i11 {
            i11 = i12;
        }
        let mut i12b = param_1.add(4).read_unaligned();
        if i1h <= i12b {
            i12b = i1h;
        }
        if i11 < i9 || i12b < i10 {
            return 0xffff_fffe;
        }
        let i12 = i12b;
        let i1 = piv.read_unaligned();
        let u6 = param_6 as u8;
        let p6 =
            (((u6 as u32) << 24) | ((u6 as u32) << 16) | ((u6 as u32) << 8) | (u6 as u32)) as i32;
        let i2 = param_2.wrapping_add(i2);
        let i3 = param_3.wrapping_add(i3);
        let mut local_10: i32 = 0;
        let mut local_14: i32 = param_5;
        let i4 = param_5.wrapping_mul(param_5);
        let u5 = (param_4 as u32).wrapping_mul(param_4 as u32);
        let mut local_28: i32 = 0;
        let mut local_2c: i32 = u5.wrapping_mul(2).wrapping_mul(param_5 as u32) as i32;
        let mut local_30: i32 = ((u5 >> 2).wrapping_add(i4 as u32))
            .wrapping_sub(u5.wrapping_mul(param_5 as u32)) as i32;
        while local_28.wrapping_sub(local_2c) < 0 {
            plot(
                i2, local_10, i9, i11, i3, local_14, i10, i12, i8, i1, p6, u6,
            );
            if -1 < local_30 {
                local_14 = local_14.wrapping_sub(1);
                param_5 = param_5.wrapping_sub(1);
                local_2c = local_2c.wrapping_add(u5.wrapping_mul(0xffff_fffe) as i32);
                local_30 = local_30.wrapping_sub(local_2c);
            }
            local_10 = local_10.wrapping_add(1);
            local_28 = local_28.wrapping_add(i4.wrapping_mul(2));
            local_30 = local_30.wrapping_add(local_28).wrapping_add(i4);
        }
        let d = u5.wrapping_sub(i4 as u32);
        let inner = ((d as i32) >> 1) as u32;
        let x = inner
            .wrapping_add(d)
            .wrapping_sub(local_28 as u32)
            .wrapping_sub(local_2c as u32);
        local_30 = local_30.wrapping_add(((x as i32) >> 1));
        let mut u7: u32;
        loop {
            plot(
                i2, local_10, i9, i11, i3, local_14, i10, i12, i8, i1, p6, u6,
            );
            if local_30 < 0 {
                local_10 = local_10.wrapping_add(1);
                local_28 = local_28.wrapping_add(i4.wrapping_mul(2));
                local_30 = local_30.wrapping_add(local_28);
            }
            local_14 = local_14.wrapping_sub(1);
            local_2c = local_2c.wrapping_add(u5.wrapping_mul(0xffff_fffe) as i32);
            u7 = (local_2c as u32).wrapping_sub(u5);
            local_30 = local_30.wrapping_sub(u7 as i32);
            param_5 = param_5.wrapping_sub(1);
            if !(-1 < param_5) {
                break;
            }
        }
        u7
    }
}
