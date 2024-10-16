use crate::center::element::Kind;
use crate::center::message::{self, EventMessage, Message, PropertyKind};
use std::cell::{Cell, Ref, RefCell};
use std::rc::Rc;
use crate::center::element::event::Event;

#[derive(Debug, PartialEq)]
pub struct Builder {
    class: u32,
    parent: u32,
    new_class: Rc<Cell<u32>>,
    commands: Rc<RefCell<Vec<Message>>>,
    kind: Kind,
    event: Option<EventMessage>
}

impl Builder {
    pub fn new(event: Option<EventMessage>) -> Self {
        Self {
            class: 1,
            parent: 0,
            new_class: Rc::new(Cell::new(2)),
            commands: Rc::new(RefCell::new(Vec::new())),
            kind: Kind::Division,
            event
        }
    }
    
    pub const fn get_event(&self) -> Option<Event> {
        let Some(event) = self.event else { return None };
        if event.class != self.class { None }
        else { Some(event.kind) }
    }
    
    /// Adds a create element command and assigns it a unique class ID.
    ///
    /// # Result
    /// If the class ID could not be assigned, then [None] is returned.
    pub fn create_element(&self, kind: Kind) -> Option<u32> {
        let class = self.get_next_class()?;
        self.commands.borrow_mut().push(Message {
            class,
            kind: message::Kind::Create { parent: self.class, kind }
        });

        Some(class)
    }
    
    pub(super) fn send_command(&self, message: message::Kind) {
        self.commands.borrow_mut().push(Message { class: self.class, kind: message });
    } 
    
    pub(super) fn set_text(&self, text: String) {
        self.send_command(message::Kind::SetText { text });
    }

    pub(super) fn set_property(&self, kind: PropertyKind, property: &str, value: &str) {
        self.send_command(message::Kind::SetProperty { kind, property: String::from(property), value: String::from(value) });
    }

    /// Adds a delete element command and assigns it a unique class ID.
    pub fn delete_element(&self, class: u32) {
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
    pub fn branch(&self, kind: Kind) -> Option<Self> {
        Some(Builder {
            class: self.create_element(kind)?,
            parent: self.class,
            new_class: self.new_class.clone(),
            commands: self.commands.clone(),
            event: self.event,
            kind
        })
    }

    pub fn get_commands(&self) -> Ref<Vec<Message>> {
        self.commands.borrow()
    }
    
    pub const fn get_kind(&self) -> Kind {
        self.kind
    }
}