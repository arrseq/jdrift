// use std::str::FromStr;
// use super::{Color, Kind, StyleProperty};
// use paste::paste;
// use crate::center::message::PropertyKind;
// 
// pub struct Text {
//     update_listener: Box<dyn FnMut()>,
//     text: String,
//     color: StyleProperty<Color>
// }
// 
// impl Text {
//     pub fn from_str(text: &str) -> Self {
//         Self {
//             update_listener: Box::new(|| {}),
//             text: String::from_str(text).unwrap(),
//             color: StyleProperty::Inherit
//         }
//     }
// }
// 
// impl super::Buildable for Text {
//     fn build(&self, builder: &mut super::builder::Builder) {
//         let component = builder.branch(Kind::Division).unwrap();
//         component.set_text(self.text.clone());
//         // component.set_property(PropertyKind::Style, "color".to_string(), self.color.as_css())
//     }
// 
//     fn get_kind(&self) -> super::Kind {
//         Kind::Division
//     }
// 
//     fn set_update_listener(&mut self, listener: Box<dyn FnMut()>) {
//         self.update_listener = listener;
//     }
// }
// 
// macro_rules! implement_get_and_set {
//     ($property: ident, $ty: ty, $tys: ty) => {
//         paste! {
//             impl Text {
//                 pub const fn [<get_ $property>](&self) -> $ty {
//                     self.$property
//                 }
//             
//                 pub fn [<set_ $property>](&mut self, value: $tys) {
//                     self.$property = value;
//                     (self.update_listener)();
//                 }
//             }
//         }
//     };
//     
//     ($property: ident, $ty: ty) => {
//         paste! {
//             impl Text {
//                 pub const fn [<get_ $property>](&self) -> $ty {
//                     self.$property
//                 }
//             
//                 pub fn [<set_ $property>](&mut self, value: $ty) {
//                     self.$property = value;
//                     (self.update_listener)();
//                 }
//             }
//         }
//     };
// }
// 
// implement_get_and_set!(color, StyleProperty<Color>);