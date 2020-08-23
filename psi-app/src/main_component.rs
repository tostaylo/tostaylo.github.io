use crate::handle;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Default, Clone)]
pub struct Main {
    id: String,
}

impl Main {
    pub fn create() -> handle::Handle<Self> {
        let main = Main {
            id: "psi-app-main".to_owned(),
            ..Default::default()
        };
        handle::Handle(Rc::new(RefCell::new(main)))
    }
}

impl rust_fel::Component for handle::Handle<Main> {
    type Properties = ();
    type Message = ();
    type State = ();

    fn add_props(&mut self, _props: Self::Properties) {
        ();
    }

    fn reduce_state(&mut self, _message: Self::Message) {
        rust_fel::re_render(self.render(), Some(self.0.borrow().id.clone()));
    }

    fn render(&self) -> rust_fel::Element {
        let borrow = self.0.borrow_mut();

        let main_text = rust_fel::html(
            "<span | data-cy=psi-main-text| >PageSpeed Insights Report coming soon! </span>"
                .to_owned(),
        );

        let main_el = rust_fel::Element::new(
            "div".to_owned(),
            rust_fel::Props {
                class_name: Some("psi-main-el".to_owned()),
                children: Some(vec![main_text]),
                ..Default::default()
            },
        );

        rust_fel::Element::new(
            "div".to_owned(),
            rust_fel::Props {
                id: Some(borrow.id.clone()),
                class_name: Some("psi-app-main".to_owned()),
                children: Some(vec![main_el]),
                ..Default::default()
            },
        )
    }
}
