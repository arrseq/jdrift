use xbinser_macros::EnumDecoded;

#[derive(Debug, Clone, Copy, PartialEq, EnumDecoded)]
pub enum MouseButton {
    Primary,
    Scroll,
    Secondary
}

#[derive(Debug, Clone, Copy, PartialEq, EnumDecoded)]
pub enum Event {
    MouseClick { button: MouseButton, pressed: bool },
    
}