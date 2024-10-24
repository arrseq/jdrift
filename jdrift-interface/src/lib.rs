#![allow(incomplete_features)]
#![feature(seek_stream_len)]
#![feature(test)]
#![feature(let_chains)]
#![feature(const_trait_impl)]
#![feature(fn_traits)]
#![deny(clippy::trivially_copy_pass_by_ref)]
#![deny(clippy::large_types_passed_by_value)]
#![allow(clippy::unusual_byte_groupings)]
#![deny(clippy::missing_const_for_fn)]
#![allow(const_evaluatable_unchecked)]
#![allow(clippy::unused_io_amount)]
#![allow(soft_unstable)]
#![allow(clippy::should_implement_trait)]

use std::num::NonZeroU32;
use std::ops::Deref;
use std::sync::{Arc, RwLock, RwLockWriteGuard};
use renderer::Renderer;
use softbuffer::{Context, Surface};
use winit::application::ApplicationHandler;
use winit::event::{ElementState, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window;
use winit::window::{ResizeDirection, WindowId};

pub mod renderer;
// pub mod server;
// pub mod center;

#[derive(Clone)]
struct AppState {
    window: Arc<window::Window>,
    surface: Arc<RwLock<Surface<Arc<window::Window>, Arc<window::Window>>>>
}

impl AppState {
    pub fn surface_mut(&self) -> RwLockWriteGuard<Surface<Arc<window::Window>, Arc<window::Window>>> {
        self.surface.write().unwrap()
    }
}

#[derive(Clone)]
struct App {
    state: Option<AppState>,
    renderer: Arc<RwLock<Renderer>>
}

impl App {
    pub fn new(renderer: Renderer) -> Self {
        Self {
            state: None,
            renderer: Arc::new(RwLock::new(renderer))
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(event_loop.create_window(window::Window::default_attributes()).unwrap());
        let context = Context::new(window.clone()).unwrap();

        // window.set_decorations(false);
        window.set_resizable(true);

        self.state = Some(AppState {
            surface: Arc::new(RwLock::new(Surface::new(&context, window.clone()).unwrap())),
            window
        });
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        let inner = self.state.as_mut().unwrap();

        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping;");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                self.renderer.write().unwrap().fill();

                let size = inner.window.inner_size();
                let mut surface = inner.surface_mut();
                let mut buffer = surface.buffer_mut().unwrap();

                let frame = self.renderer.read().unwrap();
                for (index, pixel) in frame.get_frame().iter().enumerate() {
                    buffer[index] = Self::rgb_to_hex(pixel[0], pixel[1], pixel[2]);
                }

                buffer.present();
            },
            WindowEvent::Resized(size) => {
                if id != inner.window.id() { return }
                let Some(width) = NonZeroU32::new(size.width) else { return };
                let Some(height) = NonZeroU32::new(size.height) else { return };

                inner.surface_mut().resize(width, height).unwrap();
                println!("Event: Window resized {:?}", [size.width as u64, size.height as u64]);
                self.renderer.write().unwrap().resize([size.width as u64, size.height as u64]).unwrap()
            },
            WindowEvent::CursorMoved { position, .. } => {
                let p0 = [100f64, 100.0];
                let p1 = [200f64, 200.0];
                self.renderer.write().unwrap().glow = position.x >= p0[0] && position.x <= p1[0]
                    && position.y >= p0[1] && position.y <= p1[1];
                inner.window.request_redraw();
            }
            _ => (),
        }
    }
}

impl App {
    fn rgb_to_hex(r: u8, g: u8, b: u8) -> u32 {
        let mut out = (r as u32) << 16;
        out |= (g as u32) << 8;
        out |= b as u32;
        out
    }
}

pub struct Window(App);

impl Window {
    pub fn new(renderer: Renderer) -> Self {
        Self(App::new(renderer))
    }

    pub fn start(&mut self) {
        let r#loop = EventLoop::new().unwrap();
        r#loop.run_app(&mut self.0).unwrap();
    }
}

