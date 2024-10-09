#![feature(let_chains)]

use jdrift_interface::center::element::container::Container;
use jdrift_interface::center::element::text::Text;
use jdrift_interface::center::element::New;
use jdrift_interface::center::{Center, Renderer};
use std::thread::sleep;
use std::time::Duration;

fn host_session(center: &mut Center) {
    let renderer = Renderer::new(center.session().expect("Could not start session"));
    let session = renderer.get_session();

    let thread = renderer.spawn();
    
    {
        let mut session = session.write().unwrap();

        let mut container = session.root.create::<Container>();
        let mut text = container.create::<Text>();
        text.set_text("Hello World: Body > Root: Container > Text: Text");
        container.append_child(text);
        session.root.append_child(container);
    }
    
    sleep(Duration::from_secs(2));
    {
        let mut session = session.write().unwrap();
        let mut text = session.root.create::<Text>();
        text.set_text("This text is new and later added");
        session.root.append_child(text);
    }

    // while session.read().is_ok() {
    //
    // }

    thread.join().expect("Failed to join thread");
}

fn main() {
     let mut center = Center::new(4417).expect("Failed start center");
     loop {
         host_session(&mut center);
         println!("ok: previous session ended, attempting to restart");
     }
}