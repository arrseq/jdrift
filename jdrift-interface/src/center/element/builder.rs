use crate::center::element::Kind;
use crate::center::message;
use crate::center::message::Message;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct Builder {
    class: u32,
    parent: u32,
    new_class: Rc<Cell<u32>>,
    commands: Rc<RefCell<Vec<Message>>>
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            class: 2,
            parent: 1,
            new_class: Rc::new(Cell::new(3)),
            commands: Rc::new(RefCell::new(Vec::new()))
        }
    }
}

impl Builder {
    /// Adds a create element command and assigns it a unique class ID.
    ///
    /// # Result
    /// If the class ID could not be assigned, then [None] is returned.
    pub fn create_element(&mut self, kind: Kind) -> Option<u32> {
        let class = self.get_next_class()?;
        self.commands.borrow_mut().push(Message {
            class,
            kind: message::Kind::Create { parent: self.parent, kind }
        });

        Some(class)
    }

    /// Adds a delete element command and assigns it a unique class ID.
    pub fn delete_element(&mut self, class: u32) {
        self.commands.borrow_mut().push(Message { class, kind: message::Kind::Delete });
    }

    fn get_next_class(&self) -> Option<u32> {
        let return_class = self.new_class.get();
        self.new_class.set(self.new_class.get().checked_add(1)?);
        Some(return_class)
    }

    /// Branch this builder off.
    ///
    /// # Result
    /// If the class ID could not be assigned, then [None] is returned.
    pub fn branch(&self) -> Option<Self> {
        Some(Builder {
            class: self.get_next_class()?,
            parent: self.class,
            new_class: self.new_class.clone(),
            commands: self.commands.clone()
        })
    }
}