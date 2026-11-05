// SPDX-License-Identifier: MIT
//! The window stand-in's step 1 (WN1, ruling 1413): the USER32 names a
//! program asks of its windows and its messages, written once. Windows'
//! window manager is never rebuilt: a front answers what the program reads
//! back of a call, its return and what it wrote, and nothing more.
//!
//! The window table, the class table and the message queue are every
//! target's, in [`desk::Desk`]. What a host's window does is the back's,
//! behind [`Screen`]: the screen's size, a window shown or placed, and the
//! events the host's window raised, handed in as [`Event`]s, never a
//! Windows type. Under Wine the original's calls stay Wine's, logged in
//! the layer.
//!
//! The 24 fronts, in [`front::Front`]: windows `RegisterClassA`,
//! `CreateWindowExA`, `DestroyWindow`, `FindWindowA`, `LoadIconA`,
//! `ShowWindow`, `SetWindowPos`, `AdjustWindowRect`, `GetWindowRect`,
//! `GetWindowLongA`, `SetWindowLongA`, `ClientToScreen`,
//! `ScreenToClient`, `SetFocus` and `GetSystemMetrics`; messages
//! `DefWindowProcA`, `PeekMessageA`, `WaitMessage`, `TranslateMessage`,
//! `DispatchMessageA`, `SendMessageA`, `PostMessageA`, `PostQuitMessage`
//! and `GetMessageTime`. Each front's doc names its callers by file: the
//! shell, the game file, the launcher; a game's rows crate names the
//! files.
//!
//! Six names a game imports that no row serves yet have no front, as the
//! registry's `RegCreateKeyA`: each comes with its first measured caller.
//! `GetForegroundWindow`, `GetMessageA`, `SetForegroundWindow`,
//! `SetRect`, `SetWindowTextA` and `ShowWindowAsync`. `GetMessageA`
//! blocks until a message comes, which a page cannot do: its loop is the
//! page's to turn, named when its caller is cut.
//!
//! Each call writes one `imp.call` line: `c` the row's name, `a` the
//! call's numbers, `s` the name or title it was given, `o` what it wrote,
//! `v` its answer.
//!
//! The walls, named: the stand-in draws no frame, so a window's client
//! area is its whole rectangle; no message is sent while a window is made,
//! placed or destroyed, the program reading none back; z-order and the
//! owner chain are not kept; the key-to-character map is `window-input`'s
//! (step 2), so `TranslateMessage` posts nothing; icons and painting are
//! chunk 3's, so an icon is a handle and nothing more.
//!
//! The crate names no game and nothing of any one target, so it builds for
//! `wasm32-unknown-unknown`; a game's facts live in its rows crate.
#![cfg_attr(not(test), no_std)]

#[cfg(any(test, feature = "desk"))]
extern crate alloc;

#[cfg(any(test, feature = "desk"))]
pub mod desk;
#[cfg(any(test, feature = "desk"))]
pub mod front;
pub mod table;

use core::fmt;

/// A rectangle in screen pixels, Windows' `RECT`: right and bottom one
/// past the last pixel.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl Rect {
    /// The rectangle at `left`, `top`, `width` wide and `height` high.
    #[must_use]
    pub const fn sized(left: i32, top: i32, width: i32, height: i32) -> Self {
        Self {
            left,
            top,
            right: left + width,
            bottom: top + height,
        }
    }
}

/// One queued message, the 7 fields `PeekMessageA` writes: the window
/// (0 a message to the program, not a window), its number and two words,
/// the time it was queued and where the pointer was.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Message {
    pub window: u32,
    pub message: u32,
    pub w: u32,
    pub l: u32,
    pub time: u32,
    pub x: i32,
    pub y: i32,
}

impl Message {
    /// The fields as the line shows them, in the record's order.
    #[must_use]
    pub const fn fields(&self) -> [u32; 7] {
        [
            self.window,
            self.message,
            self.w,
            self.l,
            self.time,
            bits(self.x),
            bits(self.y),
        ]
    }
}

/// What a host's window did, as a back hands it in: the seam's own words,
/// never a host's or Windows' type. Input is step 2's (`window-input`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Event {
    /// The host asked `window` closed: the program's `WM_CLOSE`.
    Close { window: u32 },
    /// The host's window took or lost the user's attention: the program's
    /// `WM_ACTIVATEAPP`.
    Active { window: u32, active: bool },
}

/// Where one program's windows are shown: the seam between the calls and
/// a target. A window is named by the handle the desk gave it.
pub trait Screen {
    /// The screen's width and height in pixels.
    fn size(&self) -> (i32, i32);
    /// Shows `window` at `rect` when `shown`, else hides it.
    fn place(&self, window: u32, rect: Rect, shown: bool);
    /// `window` is gone: the back lets its host's window go.
    fn gone(&self, window: u32);
    /// Hands each event the host raised since the last pump to `into`,
    /// never waiting for one.
    fn pump(&self, into: &mut dyn FnMut(Event));
    /// Milliseconds since the back began, a message's time.
    fn now(&self) -> u32;
    /// Writes one line around whatever the target must keep across it.
    fn quiet(&self, line: impl FnOnce()) {
        line();
    }
}

/// A borrowed back: each of a program's files keeps its own front and all
/// share the one back.
impl<S: Screen> Screen for &S {
    fn size(&self) -> (i32, i32) {
        (**self).size()
    }

    fn place(&self, window: u32, rect: Rect, shown: bool) {
        (**self).place(window, rect, shown);
    }

    fn gone(&self, window: u32) {
        (**self).gone(window);
    }

    fn pump(&self, into: &mut dyn FnMut(Event)) {
        (**self).pump(into);
    }

    fn now(&self) -> u32 {
        (**self).now()
    }

    fn quiet(&self, line: impl FnOnce()) {
        (**self).quiet(line);
    }
}

/// A signed number's bits as the file reads them, two's complement.
#[must_use]
pub const fn bits(value: i32) -> u32 {
    u32::from_ne_bytes(value.to_ne_bytes())
}

/// Bytes as the line shows them: a printable byte as its character, any
/// other `\xNN`.
pub struct Bytes<'a>(pub &'a [u8]);

impl fmt::Display for Bytes<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.iter().try_for_each(|byte| match byte {
            b' '..=b'~' => fmt::Write::write_char(f, char::from(*byte)),
            _ => write!(f, "\\x{byte:02x}"),
        })
    }
}

/// Numbers as the line shows them, comma-joined: empty for none.
pub struct Numbers<'a>(pub &'a [u32]);

impl fmt::Display for Numbers<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (at, number) in self.0.iter().enumerate() {
            if at > 0 {
                f.write_str(",")?;
            }
            write!(f, "{}", mech_trace::__shown!(*number))?;
        }
        Ok(())
    }
}
