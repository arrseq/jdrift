pub mod builder;
pub mod container;
pub mod text;
pub mod canvas;
pub mod event;

use crate::center::element::builder::Builder;
use crate::center::Session;
use std::fmt::Debug;
use std::ops::Deref;
use std::sync::{Arc, LockResult, Mutex, PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::{JoinHandle, Thread};
use xbinser_macros::EnumEncoded;
use crate::center::element::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, EnumEncoded)]
pub enum Kind {
    Division,
    Span,
    Paragraph,
    Button,
    Header,
    Canvas
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InnerColor {
    Hsl { hue: u8, saturation: u8, brightness: u8 },
    Rgb { red:  u8, green: u8, blue: u8 }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub inner: InnerColor,
    pub alpha: u8
}

impl Color {
    pub const fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            inner: InnerColor::Rgb { red: r, green: g, blue: b },
            alpha: a
        }
    }

    pub const fn from_hsla(h: u8, s: u8, l: u8, a: u8) -> Self {
        Self {
            inner: InnerColor::Hsl { hue: h, saturation: s, brightness: l },
            alpha: a
        }
    }
    
    pub fn as_css(self) -> String {
        match self.inner {
            InnerColor::Hsl { hue, saturation, brightness } => format!("hsla({hue}, {saturation}, {brightness}, {})", self.alpha),
            InnerColor::Rgb { red, green, blue } => format!("rgba({red}, {green}, {blue}, {})", self.alpha)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StyleProperty<T> {
    Immediate(T),
    Inherit
}

/// # Warning
/// Implementations should ensure that the fields can be consistent across threads. Clone is 
/// required and will be used to clone any reference counters. If the type is not consistent, then 
/// external modifications to an element will not be reflected.
pub trait Element: Debug + Sync {
    fn build(&self, builder: &Builder);
    fn get_renderer_thread(&self) -> &Thread;
    fn handle_event(&mut self, event: Event);

    fn update(&self) {
        self.get_renderer_thread().unpark();
    }
}

pub trait New: Element {
    fn new(renderer_thread: Thread) -> Self;
    
    fn create<T: New>(&self) -> T {
        T::new(self.get_renderer_thread().clone())
    }
}

pub trait Parent: Element {
    fn get_children(&self) -> Box<[Box<dyn Element>]>;
}