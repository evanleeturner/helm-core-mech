// SPDX-License-Identifier: MIT
//! A host's back through winit 0.30.13: Linux's X11 and Wayland, and
//! Windows; macOS builds untried, the gap (1392). Each pump turns winit's
//! loop once and never waits, as `PeekMessageA` does: the host's close and
//! focus, its keys, buttons and pointer, and the pointer's relative motion
//! are handed on as [`Event`]s. A window is made in the loop's turn, so
//! one placed shows at the next pump. The screen's size is the one the
//! back is made with: the program is answered the size it was built for,
//! and its windows are placed on the host's desktop as the host allows.
//! The host draws the frame: the program's rectangle is the window's
//! inside, the frame's corner at its top left. Linux turns the loop on the
//! main thread alone, and one loop a program.
//!
//! A key is the virtual key number of its place on the keyboard,
//! Windows' own numbering, whatever the host's layout; Shift, Ctrl and
//! Alt either side are `0x10`, `0x11` and `0x12`. The pointer stands where
//! it is in the window's inside, offset by the program's rectangle. The
//! lock is the program's to keep (`window-input`): its script asks
//! [`Native::grab`] and [`Native::cursor_shown`] as it changes.
//!
//! The walls, named: a key outside [`virtual_key`]'s rows is dropped, as
//! is a button other than the 3; a key winit raises for itself as the
//! focus moves (its synthetic presses and releases) is dropped, the
//! program's queue never taking it; relative motion comes while a window
//! of the program's has the focus alone, winit's default; X11 takes
//! `Confined` alone, `Locked` not implemented there (winit's
//! `CursorGrabMode` notes), so a confined pointer still moves the host's
//! cursor inside the window while the program reads its motion.
use std::cell::RefCell;
use std::time::{Duration, Instant};

use winit::application::ApplicationHandler;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::error::{EventLoopError, OsError};
use winit::event::{DeviceEvent, DeviceId, ElementState, MouseButton, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::platform::pump_events::EventLoopExtPumpEvents;
use winit::window::{CursorGrabMode, Window, WindowId};

use crate::{Event, Rect, Screen};

/// Where a program's window stands with its host's.
enum Made {
    /// Placed, not yet made: the loop's next turn makes it.
    Waiting,
    Window(Window),
    /// The host would not make it: its reason kept, never asked again.
    Refused(OsError),
}

/// One program window as last placed.
struct Placed {
    window: u32,
    rect: Rect,
    shown: bool,
    made: Made,
}

/// How the host holds the pointer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hold {
    /// Free to leave the window.
    Free,
    /// Held where it stood, moving by relative motion alone: Wayland's
    /// and Windows', not X11's.
    Locked,
    /// Kept inside the window: X11's, Wayland's and Windows'.
    Confined,
}

/// The windows, the events raised since the last pump, and the pointer's
/// motion not yet a whole pixel: winit's handler.
#[derive(Default)]
struct Desktop {
    placed: Vec<Placed>,
    events: Vec<Event>,
    motion: (f64, f64),
}

impl Desktop {
    /// Makes each window placed since the loop's last turn.
    fn make(&mut self, event_loop: &ActiveEventLoop) {
        for placed in &mut self.placed {
            if matches!(placed.made, Made::Waiting) {
                let attributes = Window::default_attributes()
                    .with_inner_size(inside(placed.rect))
                    .with_position(corner(placed.rect))
                    .with_visible(placed.shown);
                placed.made = match event_loop.create_window(attributes) {
                    Ok(host) => Made::Window(host),
                    Err(refusal) => Made::Refused(refusal),
                };
            }
        }
    }
}

impl ApplicationHandler for Desktop {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.make(event_loop);
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        self.make(event_loop);
    }

    /// The host asked a window closed, moved the user's attention, or
    /// took a key, a button or the pointer: queued; the host's window
    /// stays until the program lets it go.
    fn window_event(&mut self, _: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        let Some((window, rect)) = self.placed.iter().find_map(|placed| match &placed.made {
            Made::Window(host) if host.id() == id => Some((placed.window, placed.rect)),
            _ => None,
        }) else {
            return;
        };
        let event = match event {
            WindowEvent::CloseRequested => Event::Close { window },
            WindowEvent::Focused(active) => Event::Active { window, active },
            WindowEvent::KeyboardInput {
                event,
                is_synthetic: false,
                ..
            } => {
                let PhysicalKey::Code(code) = event.physical_key else {
                    return;
                };
                let Some(key) = virtual_key(code) else {
                    return;
                };
                Event::Key {
                    window,
                    key,
                    down: event.state == ElementState::Pressed,
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let key = match button {
                    MouseButton::Left => 1,
                    MouseButton::Right => 2,
                    MouseButton::Middle => 4,
                    _ => return,
                };
                Event::Button {
                    window,
                    key,
                    down: state == ElementState::Pressed,
                }
            }
            WindowEvent::CursorMoved { position, .. } => Event::Pointer {
                window,
                x: rect.left.saturating_add(whole(position.x.round())),
                y: rect.top.saturating_add(whole(position.y.round())),
            },
            _ => return,
        };
        self.events.push(event);
    }

    /// The pointer's relative motion, its whole pixels handed on and the
    /// rest kept for the next.
    fn device_event(&mut self, _: &ActiveEventLoop, _: DeviceId, event: DeviceEvent) {
        let DeviceEvent::MouseMotion { delta } = event else {
            return;
        };
        let (x, y) = (self.motion.0 + delta.0, self.motion.1 + delta.1);
        let (dx, dy) = (x.trunc(), y.trunc());
        self.motion = (x - dx, y - dy);
        if dx != 0.0 || dy != 0.0 {
            self.events.push(Event::Motion {
                dx: whole(dx),
                dy: whole(dy),
            });
        }
    }
}

/// A whole number of pixels as an `i32`, saturating past either end.
#[expect(
    clippy::cast_possible_truncation,
    reason = "every caller rounds or truncates first, and `as` saturates"
)]
const fn whole(pixels: f64) -> i32 {
    pixels as i32
}

/// The virtual key number of `code`'s place on the keyboard, Windows'
/// `VK_` numbering; `None` for a key with no row: the 26 letters, the 10
/// digits, F1 to F12, the arrows, Return, Escape, Backspace, Tab, space,
/// Shift, Ctrl and Alt, the 6 keys above the arrows, Pause and the 3
/// locks, the number pad, and the 11 `VK_OEM` keys of the US layout.
#[must_use]
pub const fn virtual_key(code: KeyCode) -> Option<u8> {
    use KeyCode as K;
    Some(match code {
        K::KeyA => b'A',
        K::KeyB => b'B',
        K::KeyC => b'C',
        K::KeyD => b'D',
        K::KeyE => b'E',
        K::KeyF => b'F',
        K::KeyG => b'G',
        K::KeyH => b'H',
        K::KeyI => b'I',
        K::KeyJ => b'J',
        K::KeyK => b'K',
        K::KeyL => b'L',
        K::KeyM => b'M',
        K::KeyN => b'N',
        K::KeyO => b'O',
        K::KeyP => b'P',
        K::KeyQ => b'Q',
        K::KeyR => b'R',
        K::KeyS => b'S',
        K::KeyT => b'T',
        K::KeyU => b'U',
        K::KeyV => b'V',
        K::KeyW => b'W',
        K::KeyX => b'X',
        K::KeyY => b'Y',
        K::KeyZ => b'Z',
        K::Digit0 => b'0',
        K::Digit1 => b'1',
        K::Digit2 => b'2',
        K::Digit3 => b'3',
        K::Digit4 => b'4',
        K::Digit5 => b'5',
        K::Digit6 => b'6',
        K::Digit7 => b'7',
        K::Digit8 => b'8',
        K::Digit9 => b'9',
        K::F1 => 0x70,
        K::F2 => 0x71,
        K::F3 => 0x72,
        K::F4 => 0x73,
        K::F5 => 0x74,
        K::F6 => 0x75,
        K::F7 => 0x76,
        K::F8 => 0x77,
        K::F9 => 0x78,
        K::F10 => 0x79,
        K::F11 => 0x7a,
        K::F12 => 0x7b,
        K::ArrowLeft => 0x25,
        K::ArrowUp => 0x26,
        K::ArrowRight => 0x27,
        K::ArrowDown => 0x28,
        K::Enter | K::NumpadEnter => 0x0d,
        K::Escape => 0x1b,
        K::Backspace => 0x08,
        K::Tab => 0x09,
        K::Space => 0x20,
        K::ShiftLeft | K::ShiftRight => 0x10,
        K::ControlLeft | K::ControlRight => 0x11,
        K::AltLeft | K::AltRight => 0x12,
        K::PageUp => 0x21,
        K::PageDown => 0x22,
        K::End => 0x23,
        K::Home => 0x24,
        K::Insert => 0x2d,
        K::Delete => 0x2e,
        K::Pause => 0x13,
        K::CapsLock => 0x14,
        K::NumLock => 0x90,
        K::ScrollLock => 0x91,
        K::Numpad0 => 0x60,
        K::Numpad1 => 0x61,
        K::Numpad2 => 0x62,
        K::Numpad3 => 0x63,
        K::Numpad4 => 0x64,
        K::Numpad5 => 0x65,
        K::Numpad6 => 0x66,
        K::Numpad7 => 0x67,
        K::Numpad8 => 0x68,
        K::Numpad9 => 0x69,
        K::NumpadMultiply => 0x6a,
        K::NumpadAdd => 0x6b,
        K::NumpadSubtract => 0x6d,
        K::NumpadDecimal => 0x6e,
        K::NumpadDivide => 0x6f,
        K::Semicolon => 0xba,
        K::Equal => 0xbb,
        K::Comma => 0xbc,
        K::Minus => 0xbd,
        K::Period => 0xbe,
        K::Slash => 0xbf,
        K::Backquote => 0xc0,
        K::BracketLeft => 0xdb,
        K::Backslash => 0xdc,
        K::BracketRight => 0xdd,
        K::Quote => 0xde,
        _ => return None,
    })
}

/// The rectangle's top left, where the host's frame goes.
fn corner(rect: Rect) -> PhysicalPosition<i32> {
    PhysicalPosition::new(rect.left, rect.top)
}

/// The rectangle's size, the window's inside: 1 pixel at least, a host
/// refusing an empty window.
fn inside(rect: Rect) -> PhysicalSize<u32> {
    let side = |span: i32| u32::try_from(span).unwrap_or(0).max(1);
    PhysicalSize::new(
        side(rect.right.saturating_sub(rect.left)),
        side(rect.bottom.saturating_sub(rect.top)),
    )
}

/// The back on a host's desktop.
pub struct Native {
    size: (i32, i32),
    began: Instant,
    event_loop: RefCell<EventLoop<()>>,
    desktop: RefCell<Desktop>,
}

impl Native {
    /// A back answering a `width` by `height` screen, its loop made now.
    /// Linux refuses a loop off the main thread, or a second one.
    pub fn new(width: i32, height: i32) -> Result<Self, EventLoopError> {
        Ok(Self {
            size: (width, height),
            began: Instant::now(),
            event_loop: RefCell::new(EventLoop::new()?),
            desktop: RefCell::new(Desktop::default()),
        })
    }

    /// Whether the host has made `window`'s window: `Ok(false)` for one
    /// waiting for the next pump, gone or never placed; `Err` the host's
    /// reason it would not.
    pub fn made(&self, window: u32) -> Result<bool, String> {
        let desktop = self.desktop.borrow();
        match desktop.placed.iter().find(|placed| placed.window == window) {
            Some(Placed {
                made: Made::Window(_),
                ..
            }) => Ok(true),
            Some(Placed {
                made: Made::Refused(refusal),
                ..
            }) => Err(refusal.to_string()),
            _ => Ok(false),
        }
    }

    /// Runs `ask` on `window`'s host window; `Err` for one not made.
    fn host<T>(&self, window: u32, ask: impl FnOnce(&Window) -> T) -> Result<T, String> {
        let desktop = self.desktop.borrow();
        match desktop.placed.iter().find(|placed| placed.window == window) {
            Some(Placed {
                made: Made::Window(host),
                ..
            }) => Ok(ask(host)),
            _ => Err(format!("window {window} not made")),
        }
    }

    /// Asks the host to hold the pointer over `window` as `hold`; `Err`
    /// the host's refusal, or a window not made.
    pub fn hold(&self, window: u32, hold: Hold) -> Result<(), String> {
        let mode = match hold {
            Hold::Free => CursorGrabMode::None,
            Hold::Locked => CursorGrabMode::Locked,
            Hold::Confined => CursorGrabMode::Confined,
        };
        self.host(window, |host| host.set_cursor_grab(mode))?
            .map_err(|refusal| refusal.to_string())
    }

    /// The program's lock as the host can hold it: when `locked`,
    /// `Locked`, else `Confined` on its refusal; unlocked, `Free`. The
    /// hold taken, or `Err` every refusal.
    pub fn grab(&self, window: u32, locked: bool) -> Result<Hold, String> {
        if !locked {
            return self.hold(window, Hold::Free).map(|()| Hold::Free);
        }
        match self.hold(window, Hold::Locked) {
            Ok(()) => Ok(Hold::Locked),
            Err(refused) => self
                .hold(window, Hold::Confined)
                .map(|()| Hold::Confined)
                .map_err(|confined| format!("locked: {refused}; confined: {confined}")),
        }
    }

    /// Shows the host's cursor over `window`, or hides it: `ShowCursor`'s
    /// count 0 or more, or below 0.
    pub fn cursor_shown(&self, window: u32, shown: bool) -> Result<(), String> {
        self.host(window, |host| host.set_cursor_visible(shown))
    }
}

impl Screen for Native {
    fn size(&self) -> (i32, i32) {
        self.size
    }

    /// A window the host has made is placed at once, its new inside asked
    /// of the host, the host's answer never read back: the program keeps
    /// its own rectangle. Any other is made at the next pump.
    fn place(&self, window: u32, rect: Rect, shown: bool) {
        let mut desktop = self.desktop.borrow_mut();
        match desktop.placed.iter_mut().find(|was| was.window == window) {
            Some(was) => {
                was.rect = rect;
                was.shown = shown;
                if let Made::Window(host) = &was.made {
                    host.set_outer_position(corner(rect));
                    let _ = host.request_inner_size(inside(rect));
                    host.set_visible(shown);
                }
            }
            None => desktop.placed.push(Placed {
                window,
                rect,
                shown,
                made: Made::Waiting,
            }),
        }
    }

    /// The host's window closes as it is let go.
    fn gone(&self, window: u32) {
        self.desktop
            .borrow_mut()
            .placed
            .retain(|placed| placed.window != window);
    }

    /// One turn of winit's loop, waiting for nothing; the events it raised
    /// taken out before any is handed on, so a handler may place or let
    /// go.
    fn pump(&self, into: &mut dyn FnMut(Event)) {
        let mut desktop = self.desktop.borrow_mut();
        self.event_loop
            .borrow_mut()
            .pump_app_events(Some(Duration::ZERO), &mut *desktop);
        let events = core::mem::take(&mut desktop.events);
        drop(desktop);
        events.into_iter().for_each(into);
    }

    fn now(&self) -> u32 {
        u32::try_from(self.began.elapsed().as_millis()).unwrap_or(u32::MAX)
    }
}
