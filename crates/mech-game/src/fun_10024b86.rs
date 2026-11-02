// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ab374: u32;
    static mut DAT_100ab378: u32;
    static mut DAT_100b38fc: u32;
    static mut DAT_100aaf68: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    #[link_name = "FUN_1000fc30"]
    fn ApproxLength3_Likely(param_1: i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_10024008(param_1: u32, param_2: u32, param_3: u32) -> *mut u32;
    fn FUN_1002f783(param_1: i32);
    fn FUN_1002f37a(param_1: i32, param_2: u32, param_3: u32, param_4: u32);
    fn FUN_1002fbc0(param_1: *mut i32);
    fn FUN_10051f43(param_1: i32) -> i32;
    fn FUN_100248f2(param_1: i32) -> bool;
    fn FUN_100249cf(param_1: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10024b86(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
) {
    // SAFETY: globals and callees are the game's own; the pointers are the
    // game's structures, read and written at the offsets the original uses.
    unsafe {
        if !(DAT_100ab378.wrapping_add(1) < DAT_100ab374) {
            return;
        }
        let ptr = DAT_100aaf68 as *const u8;
        let i_var1 = ApproxLength3_Likely(
            param_1.wrapping_sub(ptr.cast::<i32>().read_unaligned()),
            param_2.wrapping_sub(ptr.byte_add(4).cast::<i32>().read_unaligned()),
            param_3.wrapping_sub(ptr.byte_add(8).cast::<i32>().read_unaligned()),
        );
        if !(i_var1 <= ptr.byte_add(0x40).cast::<i32>().read_unaligned()) {
            return;
        }
        let puv = FUN_10024008(
            FUN_100248f2 as usize as u32,
            FUN_100249cf as usize as u32,
            0,
        );
        if puv.is_null() {
            return;
        }
        let b = puv as *mut u8;
        b.byte_add(16).cast::<u16>().write_unaligned(1);
        let obj = b.byte_add(0x1d2).cast::<i32>().read_unaligned();
        let i1 = (obj as usize as *const u8)
            .byte_add(0x6c)
            .cast::<i32>()
            .read_unaligned() as usize as *mut u8;
        let i3 = ApproxLength3_Likely(param_4, param_5, param_6);
        i1.byte_add(0x40).cast::<i32>().write_unaligned(i3);
        i1.byte_add(0x34).cast::<i32>().write_unaligned(param_1);
        i1.byte_add(0x38).cast::<i32>().write_unaligned(param_2);
        i1.byte_add(0x3c).cast::<i32>().write_unaligned(param_3);
        i1.byte_add(0x28).cast::<u32>().write_unaligned(0);
        i1.byte_add(0x2c).cast::<u32>().write_unaligned(0);
        i1.byte_add(0x30).cast::<u32>().write_unaligned(0);
        b.byte_add(0x1ea).cast::<i32>().write_unaligned(param_4 / 2);
        b.byte_add(0x1ee).cast::<i32>().write_unaligned(param_5 / 2);
        b.byte_add(0x1f2).cast::<i32>().write_unaligned(param_6 / 2);
        let s1 = param_4.wrapping_add(((param_4 >> 0x1f) as u32 & 0xf) as i32) >> 4;
        let s3 = param_5.wrapping_add(((param_5 >> 0x1f) as u32 & 0xf) as i32) >> 4;
        let s4 = param_6.wrapping_add(((param_6 >> 0x1f) as u32 & 0xf) as i32) >> 4;
        let obj = b.byte_add(0x1d2).cast::<i32>().read_unaligned();
        FUN_1002f783(obj);
        let obj = b.byte_add(0x1d2).cast::<i32>().read_unaligned();
        FUN_1002f37a(obj, param_1 as u32, param_2 as u32, param_3 as u32);
        let obj = b.byte_add(0x1d2).cast::<i32>().read_unaligned();
        FUN_1002fbc0(obj as usize as *mut i32);
        DAT_100ab378 = DAT_100ab378.wrapping_add(1);
        b.byte_add(0x1de)
            .cast::<u32>()
            .write_unaligned(DAT_100b38fc);
        let mut local_1c = s4;
        let mut local_18 = s3;
        let mut local_14 = s1;
        let mut local_20: i32 = 0;
        while local_20 < 0x10 {
            let off = local_20.wrapping_mul(0xe) as isize as usize;
            b.byte_add(off)
                .byte_add(0x1e)
                .cast::<u16>()
                .write_unaligned(1);
            local_14 = local_14.wrapping_add(s1);
            local_18 = local_18.wrapping_add(s3);
            local_1c = local_1c.wrapping_add(s4);
            let r = FUN_10051f43(4);
            b.byte_add(off)
                .byte_add(0x12)
                .cast::<i32>()
                .write_unaligned(local_14.wrapping_add(r).wrapping_sub(2));
            let r = FUN_10051f43(0x32);
            b.byte_add(off)
                .byte_add(0x16)
                .cast::<i32>()
                .write_unaligned(local_18.wrapping_add(r).wrapping_sub(0x19));
            let r = FUN_10051f43(4);
            b.byte_add(off)
                .byte_add(0x1a)
                .cast::<i32>()
                .write_unaligned(local_1c.wrapping_add(r).wrapping_sub(2));
            local_20 += 1;
        }
        puv.add(3).write_unaligned(2);
    }
}
