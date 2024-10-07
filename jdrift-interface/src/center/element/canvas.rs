// use std::str::FromStr;
// use crate::center::message;
// use crate::center::message::PropertyKind;
// use super::Kind;
// 
// pub struct Canvas {
//     update_listener: Box<dyn FnMut()>,
//     text: String
// }
// 
// impl Canvas {
//     pub fn from_str(text: &str) -> Self {
//         Self {
//             update_listener: Box::new(|| {}),
//             text: String::from_str(text).unwrap()
//         }
//     }
// }
// 
// impl super::Buildable for Canvas {
//     fn build(&self, builder: &mut super::builder::Builder) {
//         let component = builder.branch(Kind::Canvas).unwrap();
//         component.set_property(PropertyKind::Attribute, "width".to_string(), "50px".to_string());
//         component.set_property(PropertyKind::Attribute, "height".to_string(), "50px".to_string());
//     }
// 
//     fn get_kind(&self) -> super::Kind {
//         Kind::Canvas
//     }
// 
//     fn set_update_listener(&mut self, listener: Box<dyn FnMut()>) {
//         self.update_listener = listener;
//     }
// }