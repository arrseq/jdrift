use super::{Element, Kind, New};
use crate::center::element::builder::Builder;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, RwLock};
use std::thread::{JoinHandle, Thread};
use crate::center::element::event::Event;

#[derive(Debug)]
pub struct Text {
    renderer_thread: Thread,
    text: String
}

impl Text {
    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_owned();
        self.update();
    }
}

impl Element for Text {
    fn build(&self, builder: &Builder) {
        let component = builder.branch(Kind::Span).unwrap();
        component.set_text(self.text.clone());
    }

    fn get_renderer_thread(&self) -> &Thread {
        &self.renderer_thread
    }

    fn handle_event(&mut self, event: Event) {
        println!("(acorn) SHOTS FIRED: text event");
    }
}

impl New for Text {
    fn new(update_render: Thread) -> Self {
        Self {
            renderer_thread: update_render,
            text: String::new()
        }
    }
}