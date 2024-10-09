pub mod builder;
pub mod container;
pub mod text;
pub mod canvas;

use crate::center::element::builder::Builder;
use crate::center::Session;
use std::fmt::Debug;
use std::ops::Deref;
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicBool, Ordering};
use xbinser_macros::EnumEncoded;

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

pub trait Element: Debug {
    fn build(&mut self, builder: &Builder);
    fn get_update_render(&self) -> &Arc<AtomicBool>;
    
    fn update(&self) {
        self.get_update_render().store(true, Ordering::Release);
    }
}

pub trait New: Debug + Element {
    fn new(update_render: Arc<AtomicBool>) -> Self;
    
    fn create<T: New>(&self) -> T {
        T::new(self.get_update_render().clone())
    }
}