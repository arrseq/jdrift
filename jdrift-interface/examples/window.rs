#![feature(let_chains)]

use std::thread::sleep;
use std::time::Duration;
use jdrift_interface::center::Center;
use jdrift_interface::center::element::container::Container;

fn host_session(center: &mut Center) {
    let mut session = center.session().expect("Could not start session");
    let container = session.create::<Container>();
    session.root.append_child(container);

    session.update().expect("Failed to load");
    sleep(Duration::from_secs(2));
    for _ in 0..9 {
        let container = session.create::<Container>();
        session.root.append_child(container);
    }

    // while session.read().is_ok() {
    //
    // }

    loop {}
}

fn main() {
     let mut center = Center::new(4417).expect("Failed start center");
     loop {
         host_session(&mut center);
         println!("ok: previous session ended, attempting to restart");
     }
}