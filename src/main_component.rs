use crate::content::Content;
use crate::handle;
use crate::header::header;
use crate::theme::Theme;
use crate::theme_switcher::theme_switcher;
use rust_fel;
use std::cell::RefCell;
use std::rc::Rc;

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
    type Message = String;
    type State = MainState;

    fn add_props(&mut self, _props: Self::Properties) {
        ()
    }

    fn set_state(&mut self, new_state: Self::State) {
        self.0.borrow_mut().state.theme = new_state.theme;
        rust_fel::re_render(self.render(), Some(self.0.borrow().id.clone()));
    }

    fn render(&self) -> rust_fel::Element {
        let mut clone = self.clone();
        let borrow = self.0.borrow_mut();
        let state = borrow.state.clone();
        let child_content_component = borrow.child_content_component.render();
        let (theme_class, toggle_theme_state) = match state.theme {
            Theme::Dark => (
                "dark".to_owned(),
                MainState {
                    theme: Theme::Light,
                },
            ),
            Theme::Light => ("light".to_owned(), MainState { theme: Theme::Dark }),
        };
        let theme_onclick = Some(
            Box::new(move || clone.set_state(toggle_theme_state.clone())) as rust_fel::ClosureProp,
        );

        let main = rust_fel::create_element(
            "div".to_owned(),
            rust_fel::Props {
                id: Some(borrow.id.clone()),
                class_name: Some(format!("main {}", theme_class)),
                children: Some(vec![
                    header(),
                    theme_switcher(theme_onclick),
                    child_content_component,
                ]),
                ..Default::default()
            },
        );

        main
    }
}
