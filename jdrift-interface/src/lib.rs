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
use std::sync::Arc;

use softbuffer::{Context, Surface, RGBX, RGBA};
use winit::application::ApplicationHandler;
use winit::event::{ElementState, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window;
use winit::window::{ResizeDirection, WindowId};

pub mod server;
pub mod center;

struct AppState {
    window: Arc<window::Window>,
    context: Context<Arc<window::Window>>,
    surface: Surface<Arc<window::Window>, Arc<window::Window>>
}

struct App {
    state: Option<AppState>
}

impl Default for App {
    fn default() -> Self {
        Self { state: None }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(event_loop.create_window(window::Window::default_attributes()).unwrap());
        let context = Context::new(window.clone()).unwrap();

        window.set_decorations(false);
        window.set_transparent(true);

        self.state = Some(AppState {
            surface: Surface::new(&context, window.clone()).unwrap(),
            window, context
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
                let size = inner.window.inner_size();
                let mut buffer = inner.surface.buffer_mut().unwrap();

                let mut index = 0usize;
                for y in 0..size.height {
                    for x in 0..size.width {
                        if x == 0 || x == size.width - 1
                            || y == 0 || y == size.height - 1 {
                            buffer[index] = u32::MAX;
                        }

                        index += 1;
                    }
                }

                buffer.present();
            },
            WindowEvent::Resized(size) => {
                if id != inner.window.id() { return }
                let Some(width) = NonZeroU32::new(size.width) else { return };
                let Some(height) = NonZeroU32::new(size.height) else { return };
                inner.surface.resize(width, height).unwrap();
            },
            WindowEvent::MouseInput { state: ElementState::Pressed, .. } => {
                inner.window.drag_resize_window(ResizeDirection::East);
            }
            _ => (),
        }
    }
}

pub struct Window {
    app: App,
    title: String
}

impl Window {
    pub fn new(title: &str) -> Self {
        let mut app = App::default();
        Self {
            app, title: String::from(title)
        }
    }

    pub fn start(&mut self) {
        let r#loop = EventLoop::new().unwrap();
        r#loop.run_app(&mut self.app).unwrap();
    }
}