use xbinser_macros::{EnumEncoded, StructEncoded};

#[derive(Debug, Clone, Copy, PartialEq, EnumEncoded)]
pub enum ElementKind {
    Division,
    Span,
    Paragraph,
    Button,
    Header,
    Canvas
}

#[derive(Debug, Clone, PartialEq, EnumEncoded)]
pub enum PropertyKind {
    Style,
    Attribute
}

#[derive(Debug, Clone, PartialEq, EnumEncoded)]
pub enum Kind {
    Create { parent: u32, kind: ElementKind },
    Delete,
    SetText { text: String },
    SetProperty { kind: PropertyKind, property: String, value: String },
}

#[derive(Debug, Clone, PartialEq, StructEncoded)]
pub struct Message {
    pub class: u32,
    pub kind: Kind
}