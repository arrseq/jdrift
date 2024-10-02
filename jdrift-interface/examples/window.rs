use jdrift_interface::center::Center;
use jdrift_interface::center::element::container::{Container, self};
use jdrift_interface::center::element::{Element, Kind, Inner};
use jdrift_interface::center::element::builder::{Builder};

fn host_session(center: &mut Center) {
    let mut session = center.session().expect("Could not start session");
    // let inner = ContainerContent::new();
    // session.append_child(inner);

    // Maintain session open
    loop {}
}

fn main() {
     let mut center = Center::new(4417).expect("Failed start center");
     loop {
         host_session(&mut center);
         println!("ok: previous session ended, attempting to restart");
     }
}