use crate::about::about;
use crate::handle;
use rust_fel;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum ContentType {
    List,
    About,
    Posts,
}

impl Default for ContentType {
    fn default() -> Self {
        ContentType::List
    }
}

#[derive(Debug, Default, Clone)]
pub struct ContentState {
    content: ContentType,
}

#[derive(Debug, Default, Clone)]
pub struct Content {
    id: String,
    state: ContentState,
}

impl Content {
    pub fn create() -> handle::Handle<Self> {
        let content = Content {
            id: "content".to_owned(),
            ..Default::default()
        };
        handle::Handle(Rc::new(RefCell::new(content)))
    }
}

impl rust_fel::Component for handle::Handle<Content> {
    type Properties = Option<String>;
    type Message = String;
    type State = ContentState;

    fn add_props(&mut self, _props: Self::Properties) {
        ()
    }

    fn set_state(&mut self, new_state: Self::State) {
        self.0.borrow_mut().state.content = new_state.content;
        rust_fel::re_render(self.render(), Some(self.0.borrow().id.clone()));
    }

    fn render(&self) -> rust_fel::Element {
        let mut clone = self.clone();
        let borrow = self.0.borrow_mut();
        let _state = borrow.state.clone();
        let list_item_onclick = Some(Box::new(move || {
            clone.set_state(ContentState {
                content: ContentType::About,
            })
        }) as rust_fel::ClosureProp);

        let list_item_1 = rust_fel::wrapper(
            "li".to_owned(),
            Some("About".to_owned()),
            list_item_onclick,
            None,
            None,
        );

        let list = rust_fel::create_element(
            "ul".to_owned(),
            rust_fel::Props {
                children: Some(vec![list_item_1]),
                ..Default::default()
            },
        );

        let content_children = match borrow.state.content {
            ContentType::About => Some(vec![about()]),
            _ => Some(vec![list]),
        };

        let content = rust_fel::create_element(
            "div".to_owned(),
            rust_fel::Props {
                id: Some(borrow.id.clone()),
                class_name: Some(format!("content")),
                children: content_children,
                ..Default::default()
            },
        );

        content
    }
}
