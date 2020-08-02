use crate::handle;
use crate::theme::Theme;
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
}

impl Main {
    pub fn create() -> handle::Handle<Self> {
        let main = Main {
            id: "main".to_owned(),
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
        let theme_class = match state.theme {
            Theme::Dark => "dark".to_owned(),
            Theme::Light => "light".to_owned(),
        };
        // let closure = Rc::new(RefCell::new(move || clone.set_state(1000)));
        let html = rust_fel::html(format!(
            "<header |class={}|>This is my portfolio built with Rust</header",
            theme_class
        ));

        let toggle_theme_state = match state.theme {
            Theme::Dark => MainState {
                theme: Theme::Light,
            },
            Theme::Light => MainState { theme: Theme::Dark },
        };

        let main = rust_fel::create_element(
            "div".to_owned(),
            rust_fel::Props {
                id: Some(borrow.id.clone()),
                on_click: Some(Box::new(move || {
                    clone.set_state(toggle_theme_state.clone())
                })),
                class_name: Some(format!("main {}", theme_class)),
                children: Some(vec![html]),
                ..Default::default()
            },
        );

        main
    }
}
