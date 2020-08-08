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

#[derive(Debug, Clone, PartialEq)]
pub enum Actions {
    ContentType(ContentType),
    ShowNav,
    HideNav,
}

impl Default for Actions {
    fn default() -> Self {
        Actions::ContentType(ContentType::Home)
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ContentState {
    content: ContentType,
    is_nav: bool,
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
            state: ContentState {
                content: ContentType::Home,
                is_nav: false,
            },
            ..Default::default()
        };
        handle::Handle(Rc::new(RefCell::new(content)))
    }
}

impl rust_fel::Component for handle::Handle<Content> {
    type Properties = Option<String>;
    type Message = Actions;
    type State = ContentState;

    fn add_props(&mut self, _props: Self::Properties) {
        ()
    }

    fn reduce_state(&mut self, message: Self::Message) {
        match message {
            Actions::ContentType(x) => {
                self.0.borrow_mut().state.content = x;
                self.0.borrow_mut().state.is_nav = false;
            }
            Actions::ShowNav => self.0.borrow_mut().state.is_nav = true,
            Actions::HideNav => self.0.borrow_mut().state.is_nav = false,
        }
        rust_fel::re_render(self.render(), Some(self.0.borrow().id.clone()));
    }

    fn render(&self) -> rust_fel::Element {
        let borrow = self.0.borrow_mut();
        let state = borrow.state.clone();
        let mut content_type_vec = vec![
            ContentType::About,
            ContentType::SiteInfo,
            ContentType::Posts,
            ContentType::Github,
            ContentType::LinkedIn,
        ];
        if state.content != ContentType::Home {
            content_type_vec.push(ContentType::Home)
        }

        let nav_items: Vec<rust_fel::Element> = content_type_vec
            .iter()
            .map(|content_type| {
                let mut clone = self.clone();

                let (label, html_type) = match content_type {
                    ContentType::Home => ("<span>Home</span>", "li"),
                    ContentType::Posts => ("<span>Posts</span>", "li"),
                    ContentType::SiteInfo => ("<span>Site Info</span>", "li"),
                    ContentType::About => ("<span>About</span>", "li"),
                    ContentType::Github => {
                        ("<a | href=https://github.com/tostaylo |>Github</a>", "li")
                    }
                    ContentType::LinkedIn => (
                        "<a | href=https://www.linkedin.com/in/taylortorre |>LinkedIn</a>",
                        "li",
                    ),
                };
                let owned_content_type = content_type.to_owned();
                let on_click = match content_type {
                    ContentType::Github => None,
                    ContentType::LinkedIn => None,
                    _ => Some(Box::new(move || {
                        clone.reduce_state(Actions::ContentType(owned_content_type.clone()))
                    }) as rust_fel::ClosureProp),
                };

                let nav_item_class_name = if content_type == &state.content.clone() {
                    Some(format!("nav-item nav-item-active"))
                } else {
                    Some("nav-item".to_owned())
                };

                let nav_item = rust_fel::wrapper(
                    html_type.to_owned(),
                    None,
                    on_click,
                    nav_item_class_name,
                    Some(rust_fel::html(label.to_owned())),
                );
                nav_item
            })
            .collect();

        fn navigation(list_items: Vec<rust_fel::Element>, class_name: String) -> rust_fel::Element {
            let nav = rust_fel::Element::new(
                "ul".to_owned(),
                rust_fel::Props {
                    children: Some(list_items),
                    class_name: Some(class_name),
                    ..Default::default()
                },
            );
            nav
        }

        let (menu_button_action, nav_toggle_classname) = match state.is_nav {
            true => (Actions::HideNav, "show-nav"),
            false => (Actions::ShowNav, "hide-nav"),
        };
        let mut clone_for_menu_button = self.clone();
        let menu_button_onclick =
            Some(
                Box::new(move || clone_for_menu_button.reduce_state(menu_button_action.clone()))
                    as rust_fel::ClosureProp,
            );

        let menu = rust_fel::Element::new(
            "span".to_owned(),
            rust_fel::Props {
                class_name: Some("menu".to_owned()),
                ..Default::default()
            },
        );

        let menu_button_mobile = rust_fel::Element::new(
            "span".to_owned(),
            rust_fel::Props {
                class_name: Some("menu-button".to_owned()),
                on_click: menu_button_onclick,
                children: Some(vec![menu]),
                ..Default::default()
            },
        );

        let content_children = match borrow.state.content {
            ContentType::About => Some(vec![
                about(),
                menu_button_mobile,
                navigation(
                    nav_items,
                    format!("non-home-navigation {}", nav_toggle_classname),
                ),
            ]),
            ContentType::SiteInfo => Some(vec![
                site_info(),
                menu_button_mobile,
                navigation(
                    nav_items,
                    format!("non-home-navigation {}", nav_toggle_classname),
                ),
            ]),
            ContentType::Posts => Some(vec![
                posts(),
                menu_button_mobile,
                navigation(
                    nav_items,
                    format!("non-home-navigation {}", nav_toggle_classname),
                ),
            ]),
            _ => Some(vec![navigation(nav_items, "home-navigation".to_owned())]),
        };

        let content = rust_fel::Element::new(
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
