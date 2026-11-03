// SPDX-License-Identifier: MIT
//! The six Smacker thunks the shell's screens run (HC6 session 21's
//! measure): each `jmp [slot]` at its address, the slot smackw32's by
//! ordinal. Each is exported `HC5_<address>`, swapped as a port is, and
//! calls the real function through the same slot, `DAT_<slot>`, linked to
//! the shell's own address by the swap's build.
//!
//! cdecl at every call site (`add esp` after each). A thunk hands back
//! whatever the real function returned in eax, as the `jmp` did, so a
//! function whose return the header calls void (`SmackToBuffer`) still
//! logs the eax it left; only `SmackNextFrame` and `SmackGoto`, void in
//! the ports and the header alike, return nothing.
use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    /// SmackDoFrame, ordinal 17.
    static DAT_100d36f4: unsafe extern "C" fn(*mut c_void) -> u32;
    /// SmackToBuffer, ordinal 28.
    static DAT_100d36f0:
        unsafe extern "C" fn(*mut c_void, u32, u32, u32, u32, *mut c_void, u32) -> u32;
    /// SmackOpen, ordinal 22.
    static DAT_100d36ec: unsafe extern "C" fn(*const u8, u32, u32) -> *mut c_void;
    /// SmackNextFrame, ordinal 21.
    static DAT_100d36e8: unsafe extern "C" fn(*mut c_void);
    /// SmackGoto, ordinal 20.
    static DAT_100d36d4: unsafe extern "C" fn(*mut c_void, u32);
    /// SmackWait, ordinal 32.
    static DAT_100d36e4: unsafe extern "C" fn(*mut c_void) -> u32;
}

/// SmackDoFrame: decodes the current frame.
#[unsafe(export_name = "HC5_1006e74c")]
pub unsafe extern "C" fn smack_do_frame(smack: *mut c_void) -> u32 {
    // SAFETY: the shell's own slot, filled by Windows' loader; the
    // shell's argument passed on unchanged.
    let value = unsafe { DAT_100d36f4(smack) };
    imp_call!("SMACKW32!SmackDoFrame", [smack], value)
}

/// SmackToBuffer: where in which buffer the frames are drawn.
#[unsafe(export_name = "HC5_1006e752")]
pub unsafe extern "C" fn smack_to_buffer(
    smack: *mut c_void,
    left: u32,
    top: u32,
    pitch: u32,
    height: u32,
    buffer: *mut c_void,
    flags: u32,
) -> u32 {
    // SAFETY: as smack_do_frame's.
    let value = unsafe { DAT_100d36f0(smack, left, top, pitch, height, buffer, flags) };
    imp_call!(
        "SMACKW32!SmackToBuffer",
        [smack, left, top, pitch, height, buffer, flags],
        value
    )
}

/// SmackOpen: opens a video by name, its handle or null.
#[unsafe(export_name = "HC5_1006e758")]
pub unsafe extern "C" fn smack_open(name: *const u8, flags: u32, extra: u32) -> *mut c_void {
    // SAFETY: as smack_do_frame's.
    let value = unsafe { DAT_100d36ec(name, flags, extra) };
    imp_call!("SMACKW32!SmackOpen", [name, flags, extra], value)
}

/// SmackNextFrame: moves to the next frame.
#[unsafe(export_name = "HC5_1006e75e")]
pub unsafe extern "C" fn smack_next_frame(smack: *mut c_void) {
    // SAFETY: as smack_do_frame's.
    unsafe { DAT_100d36e8(smack) };
    imp_call!("SMACKW32!SmackNextFrame", [smack]);
}

/// SmackGoto: moves to a frame by number.
#[unsafe(export_name = "HC5_1006e764")]
pub unsafe extern "C" fn smack_goto(smack: *mut c_void, frame: u32) {
    // SAFETY: as smack_do_frame's.
    unsafe { DAT_100d36d4(smack, frame) };
    imp_call!("SMACKW32!SmackGoto", [smack, frame]);
}

/// SmackWait: nonzero while the current frame's time has not come.
#[unsafe(export_name = "HC5_1006e76a")]
pub unsafe extern "C" fn smack_wait(smack: *mut c_void) -> u32 {
    // SAFETY: as smack_do_frame's.
    let value = unsafe { DAT_100d36e4(smack) };
    imp_call!("SMACKW32!SmackWait", [smack], value)
}
