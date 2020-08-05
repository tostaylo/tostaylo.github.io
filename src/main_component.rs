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
}

impl Default for Actions {
    fn default() -> Self {
        Actions::DarkMode
    }
}

#[derive(Debug, Default, Clone)]
pub struct MainState {
    theme: Theme,
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
            Actions::LightMode => self.0.borrow_mut().state.theme = Theme::Light,
            Actions::DarkMode => self.0.borrow_mut().state.theme = Theme::Dark,
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
        let mut items: Vec<rust_fel::Element> = actions_vec
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
                };
                theme_switcher(theme_onclick, theme_title)
            })
            .collect();

        items.push(child_content_component);

        let main = rust_fel::Element::new(
            "div".to_owned(),
            rust_fel::Props {
                id: Some(borrow.id.clone()),
                class_name: Some(format!("main {}", theme_class)),
                children: Some(items),
                ..Default::default()
            },
        );

        main
    }
}
