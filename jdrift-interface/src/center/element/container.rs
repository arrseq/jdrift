use super::{Element, Kind, New};
use crate::center::element::builder::Builder;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

#[derive(Debug)]
pub struct Container {
    update_render: Arc<AtomicBool>,
    children: Vec<Box<dyn Element + Send + Sync>>
}

impl Container {
    pub fn append_child(&mut self, child: impl Element + 'static + Send + Sync) {
        self.children.push(Box::new(child));
        self.update()
    }
}

impl Element for Container {
    fn build(&mut self, builder: &Builder) {
        let component = builder.branch(Kind::Division).unwrap();
        for child in self.children.iter_mut() { child.build(&component) }
    }
    
    fn get_update_render(&self) -> &Arc<AtomicBool> {
        &self.update_render
    }
}

impl New for Container {
    fn new(update_render: Arc<AtomicBool>) -> Self {
        Self {
            update_render,
            children: Vec::new()
        }
    }
}