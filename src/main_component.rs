use crate::content::Content;
use crate::handle;
use crate::theme_component::ThemeComponent;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Default, Clone)]
pub struct Main {
    id: String,
    child_content_component: handle::Handle<Content>,
    child_theme_component: handle::Handle<ThemeComponent>,
}

impl Main {
    pub fn create() -> handle::Handle<Self> {
        let main = Main {
            id: "main".to_owned(),
            child_content_component: Content::create(),
            child_theme_component: ThemeComponent::create(),
        };
        handle::Handle(Rc::new(RefCell::new(main)))
    }
}

impl rust_fel::Component for handle::Handle<Main> {
    type Properties = Option<String>;
    type Message = Option<String>;
    type State = ();

    fn add_props(&mut self, _props: Self::Properties) {}

    fn reduce_state(&mut self, _message: Self::Message) {}

    fn render(&self) -> rust_fel::Element {
        let borrow = self.0.borrow_mut();
        let child_content_component = borrow.child_content_component.render();
        let theme_component = borrow.child_theme_component.render();

        let children = vec![theme_component, child_content_component];

        let window = web_sys::window().expect("no global `window` exists");
        let storage = window.local_storage();

        let theme = storage
            .unwrap()
            .unwrap()
            .get_item("theme")
            .unwrap()
            .unwrap_or("dark".to_owned());

        let class_name = Some(format!("main {}", theme));

        rust_fel::Element::new(
            "main".to_owned(),
            rust_fel::Props {
                id: Some(borrow.id.clone()),
                class_name,
                children: Some(children),
                ..Default::default()
            },
        )
    }
}
