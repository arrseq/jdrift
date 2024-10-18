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
        source.set_direction();
        
        let mut l = source.create::<Container>();
        let mut r = source.create::<Container>();

        for _ in 0..10 {
            let mut container = source.create::<Container>();
            let mut text = container.create::<Text>();
            text.set_text("Hello World: Body > Root: Container > Text: Text");
            container.append_child(text);
            l.append_child(container);
        }

        for _ in 0..10 {
            let mut container = source.create::<Container>();
            let mut text = container.create::<Text>();
            text.set_text("Hello World: Body > Root: Container > Text: Text");
            container.append_child(text);
            r.append_child(container);
        }
        
        source.append_child(l);
        source.append_child(r);
    }

    renderer.start();

    while renderer.tick().is_ok() {}
    renderer.stop();
    renderer.join().expect("Cannot join");
}

fn main() {
     let mut center = Center::new(4417).expect("Failed start center");
     loop {
         host_session(&mut center);
         println!("ok: previous session ended, attempting to restart");
     }
}