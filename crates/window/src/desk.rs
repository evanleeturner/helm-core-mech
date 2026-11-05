// SPDX-License-Identifier: MIT
//! The portable part every target shares: the class table, the window
//! table and the message queue, kept for one program and read by each of
//! its files' fronts. A back raises events; the desk queues them as the
//! program's messages.

use alloc::collections::VecDeque;
use alloc::vec::Vec;
use core::cell::{RefCell, RefMut};

use crate::{Event, Message, Rect, Screen};

/// `WM_CLOSE` and `WM_ACTIVATEAPP`, the messages an [`Event`] queues.
const WM_CLOSE: u32 = 0x10;
const WM_ACTIVATEAPP: u32 = 0x1c;
/// The first window handle: never 0, which every call reads as none.
const FIRST_HANDLE: u32 = 0x0001_0020;
/// The first class atom, where Windows numbers a program's own classes.
const FIRST_ATOM: u32 = 0xc000;

/// One registered class: what a window made of it starts with.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Class {
    pub atom: u32,
    pub name: Vec<u8>,
    pub style: u32,
    /// The procedure's address, as the program gave it.
    pub procedure: u32,
    /// The bytes each window keeps for `GetWindowLongA` past its own.
    pub extra: u32,
    pub icon: u32,
}

/// One window: its handle, its class, its procedure, its rectangle, its
/// longs, its title, shown or not.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Window {
    pub handle: u32,
    pub class: Vec<u8>,
    pub procedure: u32,
    pub rect: Rect,
    pub style: u32,
    pub ex_style: u32,
    /// `GWL_USERDATA`.
    pub user: u32,
    /// The class's extra bytes, as 4-byte longs from offset 0.
    pub longs: Vec<u32>,
    pub title: Vec<u8>,
    pub parent: u32,
    pub shown: bool,
}

/// The tables and the queue.
#[derive(Debug, Default)]
pub struct State {
    pub classes: Vec<Class>,
    pub windows: Vec<Window>,
    pub queue: VecDeque<Message>,
    /// The handle the next window or icon takes.
    pub next: u32,
    /// The window holding the focus, 0 none.
    pub focus: u32,
    /// The time of the last message `PeekMessageA` took.
    pub time: u32,
}

impl State {
    /// The window `handle`, `None` for one not here.
    #[must_use]
    pub fn window(&self, handle: u32) -> Option<&Window> {
        self.windows.iter().find(|window| window.handle == handle)
    }

    /// The window `handle`, to change.
    pub fn window_mut(&mut self, handle: u32) -> Option<&mut Window> {
        self.windows
            .iter_mut()
            .find(|window| window.handle == handle)
    }

    /// The class named `name`, its case ignored as Windows ignores it.
    #[must_use]
    pub fn class(&self, name: &[u8]) -> Option<&Class> {
        self.classes
            .iter()
            .find(|class| class.name.eq_ignore_ascii_case(name))
    }

    /// A fresh handle, never 0 and never one given before.
    pub fn handle(&mut self) -> u32 {
        if self.next == 0 {
            self.next = FIRST_HANDLE;
        }
        let handle = self.next;
        self.next += 2;
        handle
    }

    /// The atom the next class takes.
    #[must_use]
    pub fn atom(&self) -> u32 {
        FIRST_ATOM + u32::try_from(self.classes.len()).unwrap_or(u32::MAX - FIRST_ATOM)
    }
}

/// One program's windows: the tables and the queue, and the back that
/// shows them.
pub struct Desk<S> {
    screen: S,
    state: RefCell<State>,
}

impl<S: Screen> Desk<S> {
    pub fn new(screen: S) -> Self {
        Self {
            screen,
            state: RefCell::new(State::default()),
        }
    }

    pub const fn screen(&self) -> &S {
        &self.screen
    }

    /// The tables, to read or change; never held across a procedure's
    /// call, which may call a front again.
    pub fn state(&self) -> RefMut<'_, State> {
        self.state.borrow_mut()
    }

    /// Queues each event the back raised since the last pump as the
    /// program's message, stamped with the back's time.
    pub fn pump(&self) {
        let mut events = Vec::new();
        self.screen.pump(&mut |event| events.push(event));
        let time = self.screen.now();
        let mut state = self.state();
        for event in events {
            let (window, message, w) = match event {
                Event::Close { window } => (window, WM_CLOSE, 0),
                Event::Active { window, active } => (window, WM_ACTIVATEAPP, u32::from(active)),
            };
            state.queue.push_back(Message {
                window,
                message,
                w,
                time,
                ..Message::default()
            });
        }
    }
}
