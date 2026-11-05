// SPDX-License-Identifier: MIT
//! A host's back through winit 0.30.13: Linux's X11 and Wayland, and
//! Windows; macOS builds untried, the gap (1392). Each pump turns winit's
//! loop once and never waits, as `PeekMessageA` does: the host's close and
//! focus are handed on as [`Event`]s. A window is made in the loop's turn,
//! so one placed shows at the next pump. The screen's size is the one the
//! back is made with: the program is answered the size it was built for,
//! and its windows are placed on the host's desktop as the host allows.
//! The host draws the frame: the program's rectangle is the window's
//! inside, the frame's corner at its top left. Linux turns the loop on the
//! main thread alone, and one loop a program.
use std::cell::RefCell;
use std::time::{Duration, Instant};

use winit::application::ApplicationHandler;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::error::{EventLoopError, OsError};
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::platform::pump_events::EventLoopExtPumpEvents;
use winit::window::{Window, WindowId};

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

/// The windows, and the events raised since the last pump: winit's
/// handler.
#[derive(Default)]
struct Desktop {
    placed: Vec<Placed>,
    events: Vec<Event>,
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

    /// The host asked a window closed, or moved the user's attention:
    /// queued; the host's window stays until the program lets it go.
    fn window_event(&mut self, _: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        let Some(window) = self.placed.iter().find_map(|placed| match &placed.made {
            Made::Window(host) if host.id() == id => Some(placed.window),
            _ => None,
        }) else {
            return;
        };
        match event {
            WindowEvent::CloseRequested => self.events.push(Event::Close { window }),
            WindowEvent::Focused(active) => self.events.push(Event::Active { window, active }),
            _ => {}
        }
    }
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
