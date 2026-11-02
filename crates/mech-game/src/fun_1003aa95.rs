// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38fc: u32;
    static mut DAT_100aaf68: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000fc30(param_1: i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_10039980(param_1: i32) -> i32;
    fn FUN_10024008(param_1: u32, param_2: u32, param_3: u32) -> *mut u32;
    fn FUN_100399ec(param_1: i32, param_2: i32) -> u8;
    fn FUN_1002f783(param_1: i32) -> u8;
    fn FUN_1002f37a(param_1: i32, param_2: u32, param_3: u32, param_4: u32) -> u8;
    fn FUN_1002fbc0(param_1: *mut i32) -> u8;
    fn FUN_10051f43(param_1: i32) -> i32;
    fn FUN_1003a783(param_1: i32) -> u8;
    fn FUN_1003a8ca(param_1: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003aa95(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
    param_7: i32,
) {
    // SAFETY: the globals and callees are the game's own; all pointers are
    // the game's structures, read and written at the original's offsets.
    unsafe {
        let base = DAT_100aaf68 as usize as *const u8;
        let rd = |off: usize| base.byte_add(off).cast::<i32>().read_unaligned();
        let mut i_var1 = FUN_1000fc30(
            param_1.wrapping_sub(rd(0)),
            param_2.wrapping_sub(rd(4)),
            param_3.wrapping_sub(rd(8)),
        );
        if i_var1 > rd(0x40) {
            return;
        }
        let pi2 = FUN_10039980(param_7) as usize as *mut i32;
        if pi2.is_null() {
            return;
        }
        let pu3 = FUN_10024008(
            FUN_1003a783 as usize as u32,
            FUN_1003a8ca as usize as u32,
            pi2 as usize as u32,
        );
        if pu3.is_null() {
            FUN_100399ec(param_7, pi2.read_unaligned());
            return;
        }
        let p = pu3 as *mut u8;
        pi2.byte_add(4)
            .write_unaligned(pu3.byte_add(8).read_unaligned() as i32);
        pi2.byte_add(28).write_unaligned(0);
        pi2.byte_add(24).write_unaligned(0);
        pi2.byte_add(20).write_unaligned(0);
        p.byte_add(0x10).cast::<u16>().write_unaligned(1);
        let obj = p.byte_add(0x1d2).cast::<i32>().read_unaligned();
        i_var1 = (obj as usize as *const u8)
            .byte_add(0x6c)
            .cast::<i32>()
            .read_unaligned();
        let i_var4 = FUN_1000fc30(param_4, param_5, param_6);
        let q = i_var1 as usize as *mut u8;
        q.byte_add(0x40).cast::<i32>().write_unaligned(i_var4);
        q.byte_add(0x34).cast::<i32>().write_unaligned(param_1);
        q.byte_add(0x38).cast::<i32>().write_unaligned(param_2);
        q.byte_add(0x3c).cast::<i32>().write_unaligned(param_3);
        q.byte_add(0x28).cast::<i32>().write_unaligned(0);
        q.byte_add(0x2c).cast::<i32>().write_unaligned(0);
        q.byte_add(0x30).cast::<i32>().write_unaligned(0);
        p.byte_add(0x1ea).cast::<i32>().write_unaligned(param_4);
        p.byte_add(0x1ee).cast::<i32>().write_unaligned(param_5);
        p.byte_add(0x1f2).cast::<i32>().write_unaligned(param_6);
        FUN_1002f783(p.byte_add(0x1d2).cast::<i32>().read_unaligned());
        FUN_1002f37a(
            p.byte_add(0x1d2).cast::<i32>().read_unaligned(),
            param_1 as u32,
            param_2 as u32,
            param_3 as u32,
        );
        FUN_1002fbc0(p.byte_add(0x1d2).cast::<i32>().read_unaligned() as usize as *mut i32);
        p.byte_add(0x1de)
            .cast::<u32>()
            .write_unaligned(DAT_100b38fc);
        let mut local_20 = param_6 >> 4;
        let mut local_1c = param_5 >> 4;
        let mut local_18 = param_4 >> 4;
        let mut i: i32 = 0;
        while i < 0x20 {
            let off = (i as usize) * 0xe;
            p.byte_add(off + 0x1e).cast::<u16>().write_unaligned(1);
            local_18 = local_18.wrapping_add(param_4 >> 4);
            local_1c = local_1c.wrapping_add(param_5 >> 4);
            local_20 = local_20.wrapping_add(param_6 >> 4);
            let a = FUN_10051f43(10);
            p.byte_add(off + 0x12)
                .cast::<i32>()
                .write_unaligned(local_18.wrapping_add(a).wrapping_add(-5));
            let b = FUN_10051f43(0x32);
            p.byte_add(off + 0x16)
                .cast::<i32>()
                .write_unaligned(local_1c.wrapping_add(b).wrapping_add(-0x19));
            let c = FUN_10051f43(10);
            p.byte_add(off + 0x1a)
                .cast::<i32>()
                .write_unaligned(local_20.wrapping_add(c).wrapping_add(-5));
            i = i.wrapping_add(1);
        }
    }
}
