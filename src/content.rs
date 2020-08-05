use crate::about::about;
use crate::handle;
use crate::posts::posts;
use crate::site_info::site_info;
use rust_fel;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub enum ContentType {
    Home,
    About,
    Posts,
    SiteInfo,
    Github,
    LinkedIn,
}

impl Default for ContentType {
    fn default() -> Self {
        ContentType::Home
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
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
    type Message = ContentType;
    type State = ContentState;

    fn add_props(&mut self, _props: Self::Properties) {
        ()
    }

    fn reduce_state(&mut self, message: Self::Message) {
        match message {
            _ => self.0.borrow_mut().state.content = message,
        }
        rust_fel::re_render(self.render(), Some(self.0.borrow().id.clone()));
    }

    fn render(&self) -> rust_fel::Element {
        let borrow = self.0.borrow_mut();
        let state = borrow.state.clone();
        let content_vec = vec![
            ContentType::Home,
            ContentType::About,
            ContentType::SiteInfo,
            ContentType::Posts,
            ContentType::Github,
            ContentType::LinkedIn,
        ];
        let mut nav_items: Vec<rust_fel::Element> = vec![];

        for content in content_vec.iter() {
            let mut clone = self.clone();

            let (label, html_type) = match content {
                ContentType::Home => ("<span>Home</span>", "li"),
                ContentType::Posts => ("<span>Posts</span>", "li"),
                ContentType::SiteInfo => ("<span>Site Info</span>", "li"),
                ContentType::About => ("<span>About</span>", "li"),
                ContentType::Github => ("<a | href=https://github.com/tostaylo |>Github</a>", "li"),
                ContentType::LinkedIn => (
                    "<a | href=https://www.linkedin.com/in/taylortorre |>LinkedIn</a>",
                    "li",
                ),
            };
            let new_content = content.to_owned();
            let on_click = match content {
                ContentType::Github => None,
                _ => Some(Box::new(move || clone.reduce_state(new_content.clone()))
                    as rust_fel::ClosureProp),
            };

            let class_name = if content == &state.content.clone() {
                Some(format!("list-item list-item-active"))
            } else {
                Some("list-item".to_owned())
            };

            let nav_item = rust_fel::wrapper(
                html_type.to_owned(),
                None,
                on_click,
                class_name,
                Some(rust_fel::html(label.to_owned())),
            );
            nav_items.push(nav_item);
        }

        fn list(list_items: Vec<rust_fel::Element>, class_name: String) -> rust_fel::Element {
            let list = rust_fel::Element::new(
                "ul".to_owned(),
                rust_fel::Props {
                    children: Some(list_items),
                    class_name: Some(class_name),
                    ..Default::default()
                },
            );
            list
        }

        let content_children = match borrow.state.content {
            ContentType::About => Some(vec![about(), list(nav_items, "bottom-list".to_owned())]),
            ContentType::SiteInfo => {
                Some(vec![site_info(), list(nav_items, "bottom-list".to_owned())])
            }
            ContentType::Posts => Some(vec![posts(), list(nav_items, "bottom-list".to_owned())]),
            _ => Some(vec![list(nav_items, "side-list".to_owned())]),
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
