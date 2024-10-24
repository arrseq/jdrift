#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use jdrift_interface::{renderer::Renderer, Window};
use ocl::{Device, Platform};

const SIZE: [u64; 2] = [400, 200];

fn main() {
    let device = Device::first(Platform::first().unwrap()).unwrap();
    let mut window = Window::new(Renderer::new(device, SIZE).expect("Window size is too large"));
    window.start();
}