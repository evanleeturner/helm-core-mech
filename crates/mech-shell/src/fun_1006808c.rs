// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
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

#[inline(always)]
unsafe fn plot(
    x: i32,
    y: i32,
    minx: i32,
    maxx: i32,
    miny: i32,
    maxy: i32,
    stride: i32,
    base: i32,
    v: u8,
) {
    if minx <= x && x <= maxx && miny <= y && y <= maxy {
        let addr = y.wrapping_mul(stride).wrapping_add(base).wrapping_add(x);
        // SAFETY: the address is inside the caller's clipped pixel buffer.
        unsafe {
            (addr as *mut u8).write(v);
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1006808c(
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
    // SAFETY: param_1 points at the caller's surface descriptor and its
    // header; the pixel writes are clipped to the surface bounds.
    unsafe {
        let p1 = *param_1 as *const i32;
        let i13 = p1.add(1).read();
        let i9 = i13.wrapping_add(1);
        if i9 == 0 || i13 < -1 {
            return 0xffff_ffff;
        }
        let i2 = p1.add(2).read();
        if i2 < 0 {
            return 0xffff_ffff;
        }
        let i3 = param_1.add(1).read();
        let mut i10 = i3;
        if i3 < 1 {
            i10 = 0;
        }
        let i4 = param_1.add(2).read();
        let mut i11 = i4;
        if i4 < 1 {
            i11 = 0;
        }
        let mut i12 = param_1.add(3).read();
        if i13 <= param_1.add(3).read() {
            i12 = i13;
        }
        let mut i13b = param_1.add(4).read();
        if i2 <= param_1.add(4).read() {
            i13b = i2;
        }
        if i12 < i10 || i13b < i11 {
            return 0xffff_fffe;
        }
        let base = p1.read();
        let v = param_6 as u8;
        let cx = param_2.wrapping_add(i3);
        let cy = param_3.wrapping_add(i4);
        let mut local_10: i32 = 0;
        let mut local_14: i32 = param_5;
        let i5 = param_5.wrapping_mul(param_5);
        let u6 = (param_4.wrapping_mul(param_4)) as u32;
        let mut local_28: i32 = 0;
        let mut local_2c: i32 = u6.wrapping_mul(2).wrapping_mul(param_5 as u32) as i32;
        let mut local_30: i32 = ((u6 >> 2)
            .wrapping_add(i5 as u32)
            .wrapping_sub(u6.wrapping_mul(param_5 as u32))) as i32;
        let neg2u6 = u6.wrapping_mul(2).wrapping_neg() as i32;

        while local_28.wrapping_sub(local_2c) < 0 {
            plot(
                cx.wrapping_add(local_10),
                cy.wrapping_add(local_14),
                i10,
                i12,
                i11,
                i13b,
                i9,
                base,
                v,
            );
            plot(
                cx.wrapping_add(local_10),
                cy.wrapping_sub(local_14),
                i10,
                i12,
                i11,
                i13b,
                i9,
                base,
                v,
            );
            plot(
                cx.wrapping_sub(local_10),
                cy.wrapping_add(local_14),
                i10,
                i12,
                i11,
                i13b,
                i9,
                base,
                v,
            );
            plot(
                cx.wrapping_sub(local_10),
                cy.wrapping_sub(local_14),
                i10,
                i12,
                i11,
                i13b,
                i9,
                base,
                v,
            );
            if -1 < local_30 {
                local_14 = local_14.wrapping_sub(1);
                param_5 = param_5.wrapping_sub(1);
                local_2c = local_2c.wrapping_add(neg2u6);
                local_30 = local_30.wrapping_sub(local_2c);
            }
            local_10 = local_10.wrapping_add(1);
            local_28 = local_28.wrapping_add(i5.wrapping_mul(2));
            local_30 = local_30.wrapping_add(local_28).wrapping_add(i5);
        }
        let d = u6.wrapping_sub(i5 as u32);
        let t = (((d as i32) >> 1) as u32)
            .wrapping_add(d)
            .wrapping_sub(local_28 as u32)
            .wrapping_sub(local_2c as u32);
        local_30 = local_30.wrapping_add((t as i32) >> 1);

        let mut u8_ret: u32;
        loop {
            plot(
                cx.wrapping_add(local_10),
                cy.wrapping_add(local_14),
                i10,
                i12,
                i11,
                i13b,
                i9,
                base,
                v,
            );
            plot(
                cx.wrapping_add(local_10),
                cy.wrapping_sub(local_14),
                i10,
                i12,
                i11,
                i13b,
                i9,
                base,
                v,
            );
            plot(
                cx.wrapping_sub(local_10),
                cy.wrapping_add(local_14),
                i10,
                i12,
                i11,
                i13b,
                i9,
                base,
                v,
            );
            plot(
                cx.wrapping_sub(local_10),
                cy.wrapping_sub(local_14),
                i10,
                i12,
                i11,
                i13b,
                i9,
                base,
                v,
            );
            if local_30 < 0 {
                local_10 = local_10.wrapping_add(1);
                local_28 = local_28.wrapping_add(i5.wrapping_mul(2));
                local_30 = local_30.wrapping_add(local_28);
            }
            local_14 = local_14.wrapping_sub(1);
            local_2c = local_2c.wrapping_add(neg2u6);
            u8_ret = (local_2c as u32).wrapping_sub(u6);
            local_30 = local_30.wrapping_sub(u8_ret as i32);
            param_5 = param_5.wrapping_sub(1);
            if !(-1 < param_5) {
                break;
            }
        }
        u8_ret
    }
}
