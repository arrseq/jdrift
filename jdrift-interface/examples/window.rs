use jdrift_interface::center::element::{Container, Tree};
use jdrift_interface::center::Center;

fn host_session(center: &mut Center) {
    let mut session = center.session().expect("Could not start session");
    let inner = Container::new();
    session.append_child(inner);
    
    // Maintain session open
    loop {}
}

fn main() {
    let mut center = Center::new(4417).expect("      Failed start center");
    loop {
        host_session(&mut center);
        println!("ok: previous session ended, attempting to restart");
    }
}