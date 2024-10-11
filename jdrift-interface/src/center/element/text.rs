use super::{Element, Kind, New};
use crate::center::element::builder::Builder;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, RwLock};
use std::thread::JoinHandle;

#[derive(Debug)]
pub struct Text {
    renderer_thread: Arc<RwLock<JoinHandle<()>>>,
    text: String
}

impl Text {
    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_owned();
        self.update();
    }
}

impl Element for Text {
    fn build(&mut self, builder: &Builder) {
        let component = builder.branch(Kind::Span).unwrap();
        component.set_text(self.text.clone());
    }

    fn get_renderer_thread(&self) -> &Arc<RwLock<JoinHandle<()>>> {
        &self.renderer_thread
    }
}

impl New for Text {
    fn new(update_render: Arc<RwLock<JoinHandle<()>>>) -> Self {
        Self {
            renderer_thread: update_render,
            text: String::new()
        }
    }
}