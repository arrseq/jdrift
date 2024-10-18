use super::{Element, Kind, New};
use crate::center::element::builder::Builder;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, RwLock};
use std::thread::{JoinHandle, Thread};
use crate::center::element::event::{Event, MouseButton};
use crate::center::message::PropertyKind;

#[derive(Debug)]
pub struct Container {
    renderer_thread: Thread,
    children: Vec<Box<dyn Element + Send + Sync>>,
    direction: bool
}

impl Container {
    pub fn append_child(&mut self, child: impl Element + 'static + Send + Sync) {
        self.children.push(Box::new(child));
        self.update();
    }
    
    /// TODO: Temporary method that should be removed. Intended for developer testing.
    pub fn set_direction(&mut self) {
        self.direction = !self.direction;
        self.update();
    }
}

impl Element for Container {
    fn build(&self, builder: &Builder) {
        let component = builder.branch(Kind::Division).unwrap();
        if let Some(Event::MouseClick { button: MouseButton::Primary, pressed: true }) = component.get_event() {
            component.set_property(PropertyKind::Style, "background", "rgba(255, 255, 255, 10%)");
            println!("click");
        }
        for child in self.children.iter() { child.build(&component) }
        if self.direction {
            component.set_property(PropertyKind::Attribute, "style", "display: flex; flex-direction: row; gap: 10px;");
        }
    }
    
    fn get_renderer_thread(&self) -> &Thread {
        &self.renderer_thread
    }

    fn handle_event(&mut self, event: Event) {
        println!("(acorn) SHOTS FIRED: container event");
    }
}

impl New for Container {
    fn new(update_render: Thread) -> Self {
        Self {
            renderer_thread: update_render,
            children: Vec::new(),
            direction: false
        }
    }
}