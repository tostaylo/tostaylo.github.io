use crate::content::Content;
use crate::handle;
use crate::theme::Theme;
use crate::theme_switcher::theme_switcher;
use rust_fel;
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Debug, Clone)]
pub enum Actions {
    DarkMode,
    LightMode,
    ShowThemes,
    HideThemes,
}

impl Default for Actions {
    fn default() -> Self {
        Actions::DarkMode
    }
}

#[derive(Debug, Default, Clone)]
pub struct MainState {
    theme: Theme,
    show_themes: bool,
}

#[derive(Debug, Default, Clone)]
pub struct Main {
    id: String,
    state: MainState,
    child_content_component: handle::Handle<Content>,
}

impl Main {
    pub fn create() -> handle::Handle<Self> {
        let main = Main {
            id: "main".to_owned(),
            child_content_component: Content::create(),
            ..Default::default()
        };
        handle::Handle(Rc::new(RefCell::new(main)))
    }
}

impl rust_fel::Component for handle::Handle<Main> {
    type Properties = Option<String>;
    type Message = Actions;
    type State = MainState;

    fn add_props(&mut self, _props: Self::Properties) {
        ()
    }

    fn reduce_state(&mut self, message: Self::Message) {
        match message {
            Actions::LightMode => {
                self.0.borrow_mut().state.theme = Theme::Light;
                self.0.borrow_mut().state.show_themes = false
            }
            Actions::DarkMode => {
                self.0.borrow_mut().state.theme = Theme::Dark;
                self.0.borrow_mut().state.show_themes = false
            }
            Actions::ShowThemes => self.0.borrow_mut().state.show_themes = true,
            Actions::HideThemes => self.0.borrow_mut().state.show_themes = false,
        }

        rust_fel::re_render(self.render(), Some(self.0.borrow().id.clone()));
    }

    fn render(&self) -> rust_fel::Element {
        let borrow = self.0.borrow_mut();
        let state = borrow.state.clone();
        let child_content_component = borrow.child_content_component.render();
        let theme_class = match state.theme {
            Theme::Dark => "dark".to_owned(),
            Theme::Light => "light".to_owned(),
        };
        let actions_vec = vec![Actions::DarkMode, Actions::LightMode];
        let theme_items: Vec<rust_fel::Element> = actions_vec
            .iter()
            .map(|action| {
                let mut new_clone = self.clone();
                let (theme_onclick, theme_title) = match action {
                    Actions::LightMode => (
                        Box::new(move || new_clone.reduce_state(Actions::LightMode))
                            as rust_fel::ClosureProp,
                        "Light Mode".to_owned(),
                    ),

                    Actions::DarkMode => (
                        Box::new(move || new_clone.reduce_state(Actions::DarkMode))
                            as rust_fel::ClosureProp,
                        "Dark Mode".to_owned(),
                    ),
                    _ => (Box::new(|| ()) as rust_fel::ClosureProp, "".to_owned()),
                };
                theme_switcher(theme_onclick, theme_title)
            })
            .collect();

        let theme_toggle_action = match state.show_themes {
            true => Actions::HideThemes,
            false => Actions::ShowThemes,
        };

        let theme_switcher_wrapper = rust_fel::Element::new(
            "ul".to_owned(),
            rust_fel::Props {
                class_name: Some(format!("theme-switcher-wrapper")),
                children: Some(theme_items),
                ..Default::default()
            },
        );
        let mut clone_for_theme_toggle = self.clone();
        let theme_toggle_onclick =
            Box::new(move || clone_for_theme_toggle.reduce_state(theme_toggle_action.clone()))
                as rust_fel::ClosureProp;

        let theme_toggle = rust_fel::Element::new(
            "span".to_owned(),
            rust_fel::Props {
                role: Some("button".to_owned()),
                class_name: Some(format!("theme-toggle")),
                on_click: Some(theme_toggle_onclick),
                ..Default::default()
            },
        );

        let mut children = vec![theme_toggle, child_content_component];

        if state.show_themes {
            children.push(theme_switcher_wrapper);
        }

        let main = rust_fel::Element::new(
            "div".to_owned(),
            rust_fel::Props {
                id: Some(borrow.id.clone()),
                class_name: Some(format!("main {}", theme_class)),
                children: Some(children),
                ..Default::default()
            },
        );

        main
    }
}
