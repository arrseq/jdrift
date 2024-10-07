use super::{Buildable, Element, Kind};
use crate::center::element::builder::Builder;

pub struct Container {
    children: Vec<Element<dyn Buildable>>
}

impl Container {
    pub fn append_child(&mut self, child: Element<dyn Buildable + 'static>) {
        self.children.push(child);
    } 
}

impl Default for Container {
    fn default() -> Self {
        Self { children: Vec::new() }
    }
}

impl Buildable for Container {
    fn build(&mut self, builder: Builder) {
        let component = builder.branch(Kind::Division);
        todo!()
    }

    fn hydrate(&mut self) {
        todo!()
    }
}