pub mod builder;
pub mod container;

use xbinser_macros::EnumEncoded;
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

pub struct Element {
    class: Option<u32>
}

pub trait Inner {
    fn build(&self, builder: &mut Builder);
}