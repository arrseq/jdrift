#![feature(let_chains)]

use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use jdrift_interface::center::element::container::Container;
use jdrift_interface::center::element::text::Text;
use jdrift_interface::center::element::New;
use jdrift_interface::center::{Center, Session};

fn host_session(center: &mut Center) {
    let renderer = Session::spawn(center.stream().expect("Could not start session"));

    {
        let mut source = renderer.get_root().unwrap();
        // ensure_session_is_safe(source);

        for _ in 0..50 {
            let mut container = source.create::<Container>();
            let mut text = container.create::<Text>();
            text.set_text("Hello World: Body > Root: Container > Text: Text");
            container.append_child(text);
            source.append_child(container);
        }
    }
    
    renderer.start();

    while renderer.tick().is_ok() {}
    renderer.join().expect("Cannot join");
}

// fn ensure_session_is_safe<S: Send + Sync>(s: &mut S) {}

fn main() {
     let mut center = Center::new(4417).expect("Failed start center");
     loop {
         host_session(&mut center);
         println!("ok: previous session ended, attempting to restart");
     }
}