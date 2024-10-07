pub mod builder;
pub mod container;
pub mod text;
pub mod canvas;

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use xbinser_macros::EnumEncoded;
use crate::center;
use crate::center::element::builder::Builder;

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

pub struct Element<InnerElement: ?Sized> {
    pub(super) session: Rc<RefCell<center::Inner>>,
    pub inner: Box<InnerElement>,
    pub(super) parent: Option<Arc<Mutex<Element<dyn Buildable>>>>,
    pub(super) is_hydrated: bool // todo
}

pub trait Buildable {
    fn build(&mut self, builder: Builder);
    fn hydrate(&mut self);
}