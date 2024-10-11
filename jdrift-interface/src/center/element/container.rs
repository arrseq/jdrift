use super::{Element, Kind, New};
use crate::center::element::builder::Builder;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, RwLock};
use std::thread::JoinHandle;

#[derive(Debug)]
pub struct Container {
    renderer_thread: Arc<RwLock<JoinHandle<()>>>,
    children: Vec<Box<dyn Element + Send + Sync>>
}

impl Container {
    pub fn append_child(&mut self, child: impl Element + 'static + Send + Sync) {
        self.children.push(Box::new(child));
        self.update();
    }
}

impl Element for Container {
    fn build(&mut self, builder: &Builder) {
        let component = builder.branch(Kind::Division).unwrap();
        for child in self.children.iter_mut() { child.build(&component) }
    }
    
    fn get_renderer_thread(&self) -> &Arc<RwLock<JoinHandle<()>>> {
        &self.renderer_thread
    }
}

impl New for Container {
    fn new(update_render: Arc<RwLock<JoinHandle<()>>>) -> Self {
        Self {
            renderer_thread: update_render,
            children: Vec::new()
        }
    }
}