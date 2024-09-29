use jdrift_interface::center::Center;

fn host_session(center: &mut Center) {
    let session = center.session().expect("Could not start session");

    dbg!(&session);
    session.join().unwrap();
}

fn main() {
    let mut center = Center::new(4417).expect("Failed start center");
    loop { 
        host_session(&mut center);
        println!("ok: previous session ended, attempting to restart");
    }
}