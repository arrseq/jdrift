#![windows_subsystem = "windows"]

use std::{num::NonZeroU32, rc::Rc};

use ocl::{Platform, DeviceType, Device};
use softbuffer::{Context, Surface};
use jdrift_interface::{Window, renderer::Renderer};

const SIZE: [usize; 2] = [400, 200];

fn main() {
    let device = Device::first(Platform::first().unwrap()).unwrap();
    let mut renderer = Renderer::new(device, SIZE).expect("Window size is too large");

    renderer.fill();

    let mut window = Window::new(renderer);

    window.start();
}