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
    SiteInfo,
    Github,
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
        let borrow = self.0.borrow_mut();
        let _state = borrow.state.clone();
        let content_vec = vec![
            ContentType::About,
            ContentType::SiteInfo,
            ContentType::Posts,
            ContentType::Github,
        ];
        let mut list_items: Vec<rust_fel::Element> = vec![];

        for content in content_vec.iter() {
            let mut clone = self.clone();
            let state = ContentState {
                content: content.to_owned(),
            };

            let (label, html_type) = match content {
                ContentType::Posts => ("<span>Posts</span>", "li"),
                ContentType::SiteInfo => ("<span>Site Info</span>", "li"),
                ContentType::About => ("<span>About</span>", "li"),
                ContentType::Github => ("<a | href=https://github.com/tostaylo |>Github</a>", "li"),
                _ => ("", ""),
            };

            let on_click = match content {
                ContentType::Github => None,
                _ => {
                    Some(Box::new(move || clone.set_state(state.clone())) as rust_fel::ClosureProp)
                }
            };

            let list_item = rust_fel::wrapper(
                html_type.to_owned(),
                None,
                on_click,
                Some("list-item".to_owned()),
                //href
                Some(rust_fel::html(label.to_owned())),
            );
            list_items.push(list_item);
        }

        let list = rust_fel::create_element(
            "ul".to_owned(),
            rust_fel::Props {
                children: Some(list_items),
                ..Default::default()
            },
        );

        let content_children = match borrow.state.content {
            ContentType::About => Some(vec![list, about()]),
            ContentType::SiteInfo => Some(vec![about(), list]),
            ContentType::Posts => Some(vec![]),
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
