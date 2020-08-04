use crate::about::about;
use crate::handle;
use crate::posts::posts;
use crate::site_info::site_info;
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

            let (label, html_type) = match content {
                ContentType::Posts => ("<span>Posts</span>", "li"),
                ContentType::SiteInfo => ("<span>Site Info</span>", "li"),
                ContentType::About => ("<span>About</span>", "li"),
                ContentType::Github => ("<a | href=https://github.com/tostaylo |>Github</a>", "li"),
                _ => ("", ""),
            };
            let new_content = content.to_owned();
            let on_click = match content {
                ContentType::Github => None,
                _ => Some(Box::new(move || clone.reduce_state(new_content.clone()))
                    as rust_fel::ClosureProp),
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

        fn list(list_items: Vec<rust_fel::Element>, class_name: String) -> rust_fel::Element {
            let list = rust_fel::create_element(
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
            ContentType::About => Some(vec![about(), list(list_items, "bottom-list".to_owned())]),
            ContentType::SiteInfo => Some(vec![
                site_info(),
                list(list_items, "bottom-list".to_owned()),
            ]),
            ContentType::Posts => Some(vec![posts(), list(list_items, "bottom-list".to_owned())]),
            _ => Some(vec![list(list_items, "side-list".to_owned())]),
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
