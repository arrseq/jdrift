use super::{Element, Kind, New};
use crate::center::element::builder::Builder;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, RwLock};
use std::thread::{JoinHandle, Thread};

#[derive(Debug)]
pub struct Container {
    renderer_thread: Thread,
    children: Vec<Box<dyn Element + Send + Sync>>
}

impl Container {
    pub fn append_child(&mut self, child: impl Element + 'static + Send + Sync) {
        self.children.push(Box::new(child));
        self.update();
    }
}

impl Element for Container {
    fn build(&self, builder: &Builder) {
        let component = builder.branch(Kind::Division).unwrap();
        for child in self.children.iter() { child.build(&component) }
    }
    
    fn get_renderer_thread(&self) -> &Thread {
        &self.renderer_thread
    }
}

impl New for Container {
    fn new(update_render: Thread) -> Self {
        Self {
            renderer_thread: update_render,
            children: Vec::new()
        }
    }
}