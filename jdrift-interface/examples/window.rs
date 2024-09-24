use jdrift_interface::center::Center;

fn main() {
    let mut center = Center::new(4417).expect("Failed start center");
    let session = center.session().expect("Could not start session");
    
    dbg!(session);
}