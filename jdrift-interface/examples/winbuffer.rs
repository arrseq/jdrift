use std::{num::NonZeroU32, rc::Rc};

use softbuffer::{Context, Surface};
use jdrift_interface::Window;

fn rgb_h(r: u8, g: u8, b: u8) -> u32 {
    let mut out = (r as u32) << 16;
    out |= (g as u32) << 8;
    out |= b as u32;
    out
}

fn main() {
    let mut window = Window::new("Hello World");
    window.start();
    loop {}
}