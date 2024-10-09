use std::cell::RefCell;
use std::net::TcpStream;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tungstenite::WebSocket;
use super::{Element, Kind, New};
use crate::center::element::builder::Builder;
use crate::center::Session;

#[derive(Debug)]
pub struct Container {
    update_render: Arc<AtomicBool>,
    children: Vec<Box<dyn Element>>
}

impl Container {
    pub fn append_child(&mut self, child: impl Element + 'static) {
        self.children.push(Box::new(child));
        self.update()
    }
}

impl Element for Container {
    fn build(&mut self, builder: &Builder) {
        let component = builder.branch(Kind::Division).unwrap();
        for child in self.children.iter_mut() { child.build(&component) }
    }

    fn hydrate(&mut self) {
        todo!()
    }
    
    fn get_update_render(&self) -> &AtomicBool {
        self.update_render.as_ref()
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