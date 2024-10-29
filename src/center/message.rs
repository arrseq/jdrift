use xbinser_macros::{EnumEncoded, StructDecoded, StructEncoded};
use crate::center::element;
use crate::center::element::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, EnumEncoded)]
pub enum PropertyKind {
    Style,
    Attribute
}

#[derive(Debug, Clone, PartialEq, EnumEncoded)]
pub enum Kind {
    Load,
    Create { parent: u32, kind: element::Kind },
    Delete,
    SetText { text: String },
    SetProperty { kind: PropertyKind, property: String, value: String },
    SetImageData { data: Vec<u8> }
}

#[derive(Debug, Clone, PartialEq, StructEncoded)]
pub struct Message {
    pub class: u32,
    pub kind: Kind
}

#[derive(Debug, Clone, Copy, PartialEq, StructDecoded)]
pub struct EventMessage {
    pub class: u32,
    pub kind: Event
}