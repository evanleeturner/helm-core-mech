// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10065f68(clip_target: *mut i32, image_table: i32, image_index: i32, draw_x: i32);
}

#[inline(always)]
unsafe fn rd32(a: u32) -> i32 {
    // SAFETY: the address is the game's own memory, read where the original reads.
    unsafe { mech_trace::rd!("10065b64", (a as *const i32), read_unaligned) }
}

#[inline(always)]
unsafe fn rd8(a: u32) -> u32 {
    // SAFETY: the address is the game's own memory, read where the original reads.
    unsafe { u32::from(mech_trace::rd!("10065b64", (a as *const u8), read)) }
}

#[inline(always)]
unsafe fn wr8(a: u32, v: u32) {
    // SAFETY: the address is the game's own pixel memory, written where the original writes.
    unsafe { mech_trace::wr!("10065b64", (a as *mut u8), write, v as u8) }
}

#[inline(always)]
unsafe fn wr32(a: u32, v: u32) {
    // SAFETY: the address is the game's own pixel memory, written where the original writes.
    unsafe { mech_trace::wr!("10065b64", (a as *mut u32), write_unaligned, v) }
}

const DISPATCH: u8 = 0;
const FILL: u8 = 1;
const LIT: u8 = 2;
const GAP: u8 = 3;
const EOR: u8 = 4;
const SDISPATCH: u8 = 5;
const SFILL: u8 = 6;
const SLIT: u8 = 7;
const SGAP: u8 = 8;

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10065b64(
    clip_target: *mut i32,
    image_table: i32,
    image_index: i32,
    draw_x: i32,
    draw_y: i32,
) -> u32 {
    mech_trace::enter!(
        "10065b64",
        clip_target,
        image_table,
        image_index,
        draw_x,
        draw_y
    );
    // SAFETY: all pointers and addresses are the game's own memory; the
    // original's reads and writes happen in the same order.
    unsafe {
        let surface = mech_trace::rd!("10065b64", clip_target, read) as u32;
        let mut clip_bottom = rd32(surface.wrapping_add(4));
        let row_pitch = clip_bottom.wrapping_add(1);
        if row_pitch == 0 || clip_bottom < -1 {
            return mech_trace::rt!("10065b64", 0xffff_ffff);
        }
        let mut image_left = rd32(surface.wrapping_add(8));
        if image_left < 0 {
            return mech_trace::rt!("10065b64", 0xffff_ffff);
        }
        let ct1 = mech_trace::rd!("10065b64", clip_target.byte_add(4), read);
        let ct2 = mech_trace::rd!("10065b64", clip_target.byte_add(8), read);
        let ct3 = mech_trace::rd!("10065b64", clip_target.byte_add(12), read);
        let ct4 = mech_trace::rd!("10065b64", clip_target.byte_add(16), read);
        let origin_x0 = ct1;
        let clip_left = if origin_x0 < 1 { 0 } else { origin_x0 };
        let origin_y0 = ct2;
        let clip_top = if origin_y0 < 1 { 0 } else { origin_y0 };
        let mut clip_right = ct3;
        if clip_bottom <= ct3 {
            clip_right = clip_bottom;
        }
        clip_bottom = ct4;
        if image_left <= ct4 {
            clip_bottom = image_left;
        }
        if clip_right < clip_left || clip_bottom < clip_top {
            return mech_trace::rt!("10065b64", 0xffff_fffe);
        }
        let origin_x = draw_x.wrapping_add(origin_x0);
        let origin_y = draw_y.wrapping_add(origin_y0);
        let rec_off = (image_index as u32)
            .wrapping_mul(8)
            .wrapping_add(8)
            .wrapping_add(image_table as u32);
        let image_record = (rd32(rec_off) as u32).wrapping_add(image_table as u32);
        image_left = rd32(image_record.wrapping_add(8)).wrapping_add(origin_x);
        let mut row_y = rd32(image_record.wrapping_add(0xc)).wrapping_add(origin_y);
        let image_right = rd32(image_record.wrapping_add(0x10)).wrapping_add(origin_x);
        let image_bottom = rd32(image_record.wrapping_add(0x14)).wrapping_add(origin_y);
        let mut src = image_record.wrapping_add(0x18);
        if image_right < image_left || image_bottom < row_y {
            return mech_trace::rt!("10065b64", 0xffff_fffc);
        }
        let neg = |a: i32, b: i32| -> u32 { (a.wrapping_sub(b) < 0) as u32 };
        let tl: u32 = (((neg(image_left, clip_left) * 2 + neg(clip_right, image_left)) * 2
            + neg(row_y, clip_top))
            * 2
            + neg(clip_bottom, row_y))
            & 0xff;
        let br: u32 = (((neg(image_right, clip_left) * 2 + neg(clip_right, image_right)) * 2
            + neg(image_bottom, clip_top))
            * 2
            + neg(clip_bottom, image_bottom))
            & 0xff;
        if (br & tl) != 0 {
            return mech_trace::rt!("10065b64", 0xffff_fffd);
        }
        if tl == 0 && br == 0 {
            let a =
                origin_x.wrapping_sub(mech_trace::rd!("10065b64", clip_target.byte_add(4), read));
            let b =
                origin_y.wrapping_sub(mech_trace::rd!("10065b64", clip_target.byte_add(8), read));
            FUN_10065f68(clip_target, image_record as i32, a, b);
            return mech_trace::rt!("10065b64", 0);
        }
        let pitch = row_pitch as u32;
        let mut row_start = (row_y as u32)
            .wrapping_mul(pitch)
            .wrapping_add(rd32(surface) as u32)
            .wrapping_add(image_left as u32);
        while row_y < clip_top {
            loop {
                let b = rd8(src);
                src = src.wrapping_add(1);
                let low = b & 1;
                let c = b >> 1;
                if low != 0 || c == 0 {
                    if c != 0 {
                        src = src.wrapping_add(c - 1);
                    } else if low == 0 {
                        break;
                    }
                }
                src = src.wrapping_add(1);
            }
            row_start = row_start.wrapping_add(pitch);
            row_y = row_y.wrapping_add(1);
        }
        let mut rcl = row_start.wrapping_add(clip_left.wrapping_sub(image_left) as u32);
        let mut rcr = row_start.wrapping_add(clip_right.wrapping_sub(image_left) as u32);

        loop {
            if image_bottom < row_y {
                return mech_trace::rt!("10065b64", 0);
            }
            if clip_bottom < row_y {
                return mech_trace::rt!("10065b64", 0);
            }
            let mut dest = row_start;
            let mut mode: u32 = if (tl & 8) == 0 {
                if (br & 4) == 0 { 0 } else { 1 }
            } else {
                2
            };
            let b = rd8(src);
            src = src.wrapping_add(1);
            let mut low = b & 1;
            let mut cnt = b >> 1;
            let mut run: u32 = 0;
            let mut st = DISPATCH;
            loop {
                match st {
                    DISPATCH => {
                        if low == 0 && cnt != 0 {
                            run = cnt;
                            st = FILL;
                        } else if cnt != 0 {
                            run = cnt;
                            st = LIT;
                        } else if low != 0 {
                            st = GAP;
                        } else {
                            st = EOR;
                        }
                    }
                    GAP => {
                        dest = dest.wrapping_add(rd8(src));
                        let b = rd8(src.wrapping_add(1));
                        src = src.wrapping_add(2);
                        low = b & 1;
                        cnt = b >> 1;
                        st = DISPATCH;
                    }
                    FILL => {
                        let mut ov: u32 = 0;
                        if mode == 2 {
                            let left = rcl.wrapping_sub(dest) as i32;
                            if left < run as i32 {
                                if left >= 0 {
                                    dest = dest.wrapping_add(left as u32);
                                    run = run.wrapping_sub(left as u32);
                                }
                                mode = if (br & 4) == 0 { 0 } else { 1 };
                                continue;
                            }
                            dest = dest.wrapping_add(run);
                            let b = rd8(src.wrapping_add(1));
                            src = src.wrapping_add(2);
                            low = b & 1;
                            cnt = b >> 1;
                            st = DISPATCH;
                            continue;
                        }
                        if mode == 1 {
                            if (rcr as i32) < (dest as i32) {
                                st = SFILL;
                                continue;
                            }
                            let raw =
                                dest.wrapping_add(run.wrapping_sub(1).wrapping_sub(rcr)) as i32;
                            ov = (!(raw >> 31) & raw) as u32;
                        }
                        run = run.wrapping_sub(ov);
                        let v = rd8(src);
                        let next = src.wrapping_add(1);
                        let mut rem = run & 3;
                        while rem != 0 {
                            wr8(dest, v);
                            dest = dest.wrapping_add(1);
                            rem -= 1;
                        }
                        let word = v.wrapping_mul(0x0101_0101);
                        let mut n = run >> 2;
                        while n != 0 {
                            wr32(dest, word);
                            dest = dest.wrapping_add(4);
                            n -= 1;
                        }
                        dest = dest.wrapping_add(ov);
                        src = src.wrapping_add(2);
                        let b = rd8(next);
                        low = b & 1;
                        cnt = b >> 1;
                        st = DISPATCH;
                    }
                    LIT => {
                        let mut ov: u32 = 0;
                        if mode == 2 {
                            let left = rcl.wrapping_sub(dest) as i32;
                            if left < run as i32 {
                                if left >= 0 {
                                    dest = dest.wrapping_add(left as u32);
                                    run = run.wrapping_sub(left as u32);
                                    src = src.wrapping_add(left as u32);
                                }
                                mode = if (br & 4) == 0 { 0 } else { 1 };
                                continue;
                            }
                            dest = dest.wrapping_add(run);
                            let b = rd8(src.wrapping_add(run));
                            src = src.wrapping_add(run).wrapping_add(1);
                            low = b & 1;
                            cnt = b >> 1;
                            st = DISPATCH;
                            continue;
                        }
                        if mode == 1 {
                            if (rcr as i32) < (dest as i32) {
                                st = SLIT;
                                continue;
                            }
                            let raw =
                                dest.wrapping_add(run.wrapping_sub(1).wrapping_sub(rcr)) as i32;
                            ov = (!(raw >> 31) & raw) as u32;
                        }
                        let total = run.wrapping_sub(ov);
                        let mut rem = total & 3;
                        while rem != 0 {
                            wr8(dest, rd8(src));
                            src = src.wrapping_add(1);
                            dest = dest.wrapping_add(1);
                            rem -= 1;
                        }
                        let mut n = total >> 2;
                        while n != 0 {
                            let w =
                                mech_trace::rd!("10065b64", (src as *const u32), read_unaligned);
                            wr32(dest, w);
                            src = src.wrapping_add(4);
                            dest = dest.wrapping_add(4);
                            n -= 1;
                        }
                        dest = dest.wrapping_add(ov);
                        let b = rd8(src.wrapping_add(ov));
                        src = src.wrapping_add(ov).wrapping_add(1);
                        low = b & 1;
                        cnt = b >> 1;
                        st = DISPATCH;
                    }
                    SDISPATCH => {
                        if low == 0 && cnt != 0 {
                            st = SFILL;
                        } else if cnt != 0 {
                            run = cnt;
                            st = SLIT;
                        } else if low != 0 {
                            st = SGAP;
                        } else {
                            st = EOR;
                        }
                    }
                    SFILL => {
                        loop {
                            let tmp = src.wrapping_add(1);
                            src = src.wrapping_add(2);
                            let b = rd8(tmp);
                            low = b & 1;
                            cnt = b >> 1;
                            if !(low == 0 && cnt != 0) {
                                break;
                            }
                        }
                        if low != 0 {
                            if cnt == 0 {
                                st = SGAP;
                            } else {
                                run = cnt;
                                st = SLIT;
                            }
                        } else {
                            st = EOR;
                        }
                    }
                    SLIT => {
                        let b = rd8(src.wrapping_add(run));
                        src = src.wrapping_add(run).wrapping_add(1);
                        low = b & 1;
                        cnt = b >> 1;
                        st = SDISPATCH;
                    }
                    SGAP => {
                        let tmp = src.wrapping_add(1);
                        src = src.wrapping_add(2);
                        let b = rd8(tmp);
                        low = b & 1;
                        cnt = b >> 1;
                        st = SDISPATCH;
                    }
                    _ => break,
                }
            }
            row_start = row_start.wrapping_add(pitch);
            rcl = rcl.wrapping_add(pitch);
            rcr = rcr.wrapping_add(pitch);
            row_y = row_y.wrapping_add(1);
        }
    }
}
