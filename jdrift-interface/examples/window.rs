use std::thread::sleep;
use std::time::Duration;
use jdrift_interface::center::Center;
use jdrift_interface::center::message::Message;

fn host_session(center: &mut Center) {
    let mut session = center.session().expect("Could not start session");
    loop {
        session.send(Message::Bye);
        sleep(Duration::from_secs(2));
    }
}

fn main() {
    let mut center = Center::new(4417).expect("Failed start center");
    loop {
        host_session(&mut center);
        println!("ok: previous session ended, attempting to restart");
    }
}