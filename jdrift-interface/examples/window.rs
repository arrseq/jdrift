#![feature(let_chains)]

use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use jdrift_interface::center::element::container::Container;
use jdrift_interface::center::element::text::Text;
use jdrift_interface::center::element::New;
use jdrift_interface::center::{Center, Renderer};

fn host_session(center: &mut Center) {
    let renderer = Renderer::spawn(center.stream().expect("Could not start session"));

    {
        let mut source = renderer.get_session().unwrap();
        let session = source.as_mut().unwrap();
        ensure_session_is_Safe(session);
        
        for _ in 0..50 {
            let mut container = session.root.create::<Container>();
            let mut text = container.create::<Text>();
            text.set_text("Hello World: Body > Root: Container > Text: Text");
            container.append_child(text);
            session.root.append_child(container);
        }
    }

    renderer.join().expect("Cannot join");
}

fn ensure_session_is_safe<S: Send + Sync>(s: &mut S) {}

fn main() {
     let mut center = Center::new(4417).expect("Failed start center");
     loop {
         host_session(&mut center);
         println!("ok: previous session ended, attempting to restart");
     }
}