use super::{Element, Kind, New};
use crate::center::element::builder::Builder;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

#[derive(Debug)]
pub struct Text {
    update_render: Arc<AtomicBool>,
    text: String
}

impl Text {
    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_owned();
        self.update()
    }
}

impl Element for Text {
    fn build(&mut self, builder: &Builder) {
        let component = builder.branch(Kind::Span).unwrap();
        component.set_text(self.text.clone());
    }

    fn get_update_render(&self) -> &Arc<AtomicBool> {
        &self.update_render
    }
}

impl New for Text {
    fn new(update_render: Arc<AtomicBool>) -> Self {
        Self {
            update_render,
            text: String::new()
        }
    }
}