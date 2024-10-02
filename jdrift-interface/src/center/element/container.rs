use super::Kind;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Container {
    
}

impl super::Inner for Container {
    fn build(&self, builder: &mut super::builder::Builder) {
        let component = builder.branch(Kind::Division).unwrap();
        let btn_inner = component.create_element(Kind::Button).unwrap();
//        builder.
//        todo!()
    }

    fn get_kind(&self) -> super::Kind {
        Kind::Division
    }
}