use crate::about::about;
use crate::handle;
use crate::posts::posts;
use crate::site_info::site_info;
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::ScrollToOptions;
#[derive(Debug, Clone, PartialEq, Default)]
pub enum ContentType {
    #[default]
    Home,
    About,
    Posts,
    SiteInfo,
    Github,
    LinkedIn,
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
        };
        handle::Handle(Rc::new(RefCell::new(content)))
    }
}

impl rust_fel::Component for handle::Handle<Content> {
    type Properties = Option<String>;
    type Message = Actions;
    type State = ContentState;

    fn add_props(&mut self, _props: Self::Properties) {}

    fn reduce_state(&mut self, message: Self::Message) {
        let window = web_sys::window().expect("no global `window` exists");
        let mut opts = ScrollToOptions::new();
        opts.top(0.0);
        window.scroll_with_scroll_to_options(&opts);

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
                    ContentType::Posts => ("<span | data-cy=nav-posts |>Posts</span>", "li"),
                    ContentType::SiteInfo => {
                        ("<span | data-cy=nav-site-info |>Site Info</span>", "li")
                    }
                    ContentType::About => ("<span | data-cy=nav-about |>About</span>", "li"),
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
                    Some("nav-item nav-item-active".to_owned())
                } else {
                    Some("nav-item".to_owned())
                };

                rust_fel::Element::new(
                    html_type.to_owned(),
                    rust_fel::Props {
                        on_click,
                        children: Some(vec![rust_fel::html(label.to_owned())]),
                        class_name: nav_item_class_name,
                        ..Default::default()
                    },
                )
            })
            .collect();

        fn navigation(list_items: Vec<rust_fel::Element>, class_name: String) -> rust_fel::Element {
            let ul = rust_fel::Element::new(
                "ul".to_owned(),
                rust_fel::Props {
                    children: Some(list_items),
                    ..Default::default()
                },
            );

            rust_fel::Element::new(
                "nav".to_owned(),
                rust_fel::Props {
                    children: Some(vec![ul]),
                    class_name: Some(class_name),
                    ..Default::default()
                },
            )
        }

        let (menu_button_action, nav_toggle_classname, body_lock) = match state.is_nav {
            true => {
                let body_lock = rust_fel::html(
            "<style>@media screen and (max-width: 900px){{body{{position:fixed; overflow:hidden;}}}}</style>".to_owned()
        );
                (Actions::HideNav, "show-nav", body_lock)
            }
            false => (
                Actions::ShowNav,
                "hide-nav",
                rust_fel::html("<style></style".to_owned()),
            ),
        };
        let mut clone_for_menu_button = self.clone();
        let menu_button_onclick =
            Some(
                Box::new(move || clone_for_menu_button.reduce_state(menu_button_action.clone()))
                    as rust_fel::ClosureProp,
            );

        let menu = rust_fel::html("<span |class=menu|></span>".to_owned());
        let menu_button_mobile = rust_fel::Element::new(
            "span".to_owned(),
            rust_fel::Props {
                class_name: Some("menu-button".to_owned()),
                on_click: menu_button_onclick,
                children: Some(vec![menu]),
                data_cy: Some("menu-button".to_owned()),
                ..Default::default()
            },
        );
        let content_footer = rust_fel::html(
            "<div |class=content-footer|><span |class=content-footer-underline|></span></div>"
                .to_owned(),
        );

        let content_children = match borrow.state.content {
            ContentType::About => Some(vec![
                navigation(
                    nav_items,
                    format!("non-home-navigation {}", nav_toggle_classname),
                ),
                menu_button_mobile,
                about(),
                content_footer,
                body_lock,
            ]),
            ContentType::SiteInfo => Some(vec![
                navigation(
                    nav_items,
                    format!("non-home-navigation {}", nav_toggle_classname),
                ),
                menu_button_mobile,
                site_info(),
                content_footer,
                body_lock,
            ]),
            ContentType::Posts => Some(vec![
                navigation(
                    nav_items,
                    format!("non-home-navigation {}", nav_toggle_classname),
                ),
                menu_button_mobile,
                posts(),
                content_footer,
                body_lock,
            ]),
            _ => Some(vec![navigation(nav_items, "home-navigation".to_owned())]),
        };

        rust_fel::Element::new(
            "section".to_owned(),
            rust_fel::Props {
                id: Some(borrow.id.clone()),
                class_name: Some("content".to_owned()),
                children: content_children,
                ..Default::default()
            },
        )
    }
}
