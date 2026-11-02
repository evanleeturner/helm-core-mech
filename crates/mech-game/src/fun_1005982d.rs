// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10056489(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
        param_7: i32,
    ) -> u32;
}

unsafe fn fill_row(row: i32, stride: i32, base: i32, left: i32, right: i32, word: i32, byte: u8) {
    let total = (right as u32).wrapping_sub(left as u32).wrapping_add(1);
    let mut rem = total & 3;
    let addr = (row as u32)
        .wrapping_mul(stride as u32)
        .wrapping_add(base as u32)
        .wrapping_add(left as u32);
    let mut p = addr as *mut i32;
    let mut cnt = total >> 2;
    // SAFETY: the destination is the game's pixel buffer, written exactly
    // where the original writes.
    unsafe {
        while cnt != 0 {
            p.write_unaligned(word);
            p = p.add(1);
            cnt -= 1;
        }
        let mut pb = p as *mut u8;
        while rem != 0 {
            pb.write(byte);
            pb = pb.add(1);
            rem -= 1;
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005982d(
    param_1: *mut i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
) -> u32 {
    // SAFETY: param_1 points at the game's clip/surface structure; all
    // reads and writes follow the original.
    unsafe {
        if param_4 == 0 || param_5 == 0 {
            return FUN_10056489(
                param_1,
                param_2.wrapping_sub(param_4),
                param_3.wrapping_sub(param_5),
                param_2.wrapping_add(param_4),
                param_3.wrapping_add(param_5),
                0,
                param_6,
            );
        }
        let mut param_5 = param_5;
        let surf = param_1.read() as *mut i32;
        let mut iv12 = surf.add(1).read();
        let iv8 = iv12.wrapping_add(1);
        if iv8 == 0 || iv12 < -1 {
            return 0xffff_ffff;
        }
        let iv1 = surf.add(2).read();
        if iv1 < 0 {
            return 0xffff_ffff;
        }
        let iv2 = param_1.add(1).read();
        let mut iv9 = iv2;
        if iv2 < 1 {
            iv9 = 0;
        }
        let iv3 = param_1.add(2).read();
        let mut iv10 = iv3;
        if iv3 < 1 {
            iv10 = 0;
        }
        let mut iv11 = param_1.add(3).read();
        if iv12 <= param_1.add(3).read() {
            iv11 = iv12;
        }
        let iv12b = iv12;
        iv12 = param_1.add(4).read();
        if iv1 <= param_1.add(4).read() {
            iv12 = iv1;
        }
        let _ = iv12b;
        if iv11 < iv9 || iv12 < iv10 {
            return 0xffff_fffe;
        }
        let base = surf.read();
        let b = param_6 as u8;
        let word =
            (((b as u32) << 24) | ((b as u32) << 16) | ((b as u32) << 8) | (b as u32)) as i32;
        let cx = param_2.wrapping_add(iv2);
        let cy = param_3.wrapping_add(iv3);
        let mut local_10: i32 = 0;
        let mut local_14: i32 = param_5;
        let iv4 = param_5.wrapping_mul(param_5);
        let u5 = (param_4 as u32).wrapping_mul(param_4 as u32);
        let mut local_28: i32 = 0;
        let mut local_2c: i32 = u5.wrapping_mul(2).wrapping_mul(param_5 as u32) as i32;
        let mut local_30: i32 = ((u5 >> 2).wrapping_add(iv4 as u32))
            .wrapping_sub(u5.wrapping_mul(param_5 as u32)) as i32;

        while local_28.wrapping_sub(local_2c) < 0 {
            let mut iv15 = cx.wrapping_add(local_10);
            if iv9 <= iv15 {
                if iv11 <= iv15 {
                    iv15 = iv11;
                }
                let mut iv16 = cx.wrapping_sub(local_10);
                if iv16 <= iv11 {
                    if iv16 <= iv9 {
                        iv16 = iv9;
                    }
                    let iv13 = cy.wrapping_add(local_14);
                    if iv10 <= iv13 {
                        if iv13 <= iv12 {
                            fill_row(iv13, iv8, base, iv16, iv15, word, b);
                        }
                        let iv13 = cy.wrapping_sub(local_14);
                        if iv10 <= iv13 && iv13 <= iv12 {
                            fill_row(iv13, iv8, base, iv16, iv15, word, b);
                        }
                    }
                }
            }
            if -1 < local_30 {
                local_14 = local_14.wrapping_sub(1);
                param_5 = param_5.wrapping_sub(1);
                local_2c = local_2c.wrapping_add(u5.wrapping_mul(-2i32 as u32) as i32);
                local_30 = local_30.wrapping_sub(local_2c);
            }
            local_10 = local_10.wrapping_add(1);
            local_28 = local_28.wrapping_add(iv4.wrapping_mul(2));
            local_30 = local_30.wrapping_add(local_28).wrapping_add(iv4);
        }
        let d = u5.wrapping_sub(iv4 as u32);
        let t = (((d as i32) >> 1) as u32)
            .wrapping_add(d)
            .wrapping_sub(local_28 as u32)
            .wrapping_sub(local_2c as u32);
        local_30 = local_30.wrapping_add(((t as i32) >> 1) as u32 as i32);

        let mut u7: u32;
        loop {
            let mut iv15 = cx.wrapping_add(local_10);
            if iv9 <= iv15 {
                if iv11 <= iv15 {
                    iv15 = iv11;
                }
                let mut iv16 = cx.wrapping_sub(local_10);
                if iv16 <= iv11 {
                    if iv16 <= iv9 {
                        iv16 = iv9;
                    }
                    let iv13 = cy.wrapping_add(local_14);
                    if iv10 <= iv13 {
                        if iv13 <= iv12 {
                            fill_row(iv13, iv8, base, iv16, iv15, word, b);
                        }
                        let iv13 = cy.wrapping_sub(local_14);
                        if iv10 <= iv13 && iv13 <= iv12 {
                            fill_row(iv13, iv8, base, iv16, iv15, word, b);
                        }
                    }
                }
            }
            if local_30 < 0 {
                local_10 = local_10.wrapping_add(1);
                local_28 = local_28.wrapping_add(iv4.wrapping_mul(2));
                local_30 = local_30.wrapping_add(local_28);
            }
            local_14 = local_14.wrapping_sub(1);
            local_2c = local_2c.wrapping_add(u5.wrapping_mul(-2i32 as u32) as i32);
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
