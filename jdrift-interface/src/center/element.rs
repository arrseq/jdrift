pub trait Element {
    
}

pub trait Tree {
    fn append_child(&mut self, child: Box<dyn Element>);
    fn get_children(&self) -> &[&dyn Element];
    fn get_children_mut(&mut self); // todo: Implement ChildrenGuard
}

pub struct Container {
    children: Vec<Box<dyn Element>>
}

impl Container {
    pub const fn new() -> Self {
        Self { children: Vec::new() }
    }
    
    pub const fn from_children(children: Vec<Box<dyn Element>>) -> Self {
        Self { children }
    }
}

impl Element for Container {
    
}


impl Tree for Container {
    fn append_child(&mut self, child: Box<dyn Element>) {
        self.children.push(child);
    }

    fn get_children(&self) -> &[&dyn Element] {
        todo!()
    }

    fn get_children_mut(&mut self) {
        todo!()
    }
}