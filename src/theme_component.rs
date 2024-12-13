use crate::handle;
use crate::theme_switcher::theme_switcher;
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Debug, Clone, Default)]
pub enum Actions {
    #[default]
    DarkMode,
    LightMode,
    ShowThemes,
    HideThemes,
}

#[derive(Debug, Default, Clone)]
pub struct ThemeComponentState {
    show_themes: bool,
}

#[derive(Debug, Default, Clone)]
pub struct ThemeComponent {
    id: String,
    state: ThemeComponentState,
}

impl ThemeComponent {
    pub fn create() -> handle::Handle<Self> {
        let theme_component = ThemeComponent {
            id: "theme-component".to_owned(),
            ..Default::default()
        };
        handle::Handle(Rc::new(RefCell::new(theme_component)))
    }
}

impl rust_fel::Component for handle::Handle<ThemeComponent> {
    type Properties = Option<String>;
    type Message = Actions;
    type State = ThemeComponentState;

    fn add_props(&mut self, _props: Self::Properties) {}

    fn reduce_state(&mut self, message: Self::Message) {
        match message {
            Actions::LightMode => {
                let window = web_sys::window().expect("no global `window` exists");
                let document = window.document().expect("should have a document on window");
                document
                    .get_element_by_id("main")
                    .unwrap()
                    .set_class_name("main light");

                self.0.borrow_mut().state.show_themes = false
            }
            Actions::DarkMode => {
                let window = web_sys::window().expect("no global `window` exists");
                let document = window.document().expect("should have a document on window");
                document
                    .get_element_by_id("main")
                    .unwrap()
                    .set_class_name("main dark");

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

        let actions_vec = [Actions::DarkMode, Actions::LightMode];
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
            "div".to_owned(),
            rust_fel::Props {
                class_name: Some("theme-switcher-wrapper".to_owned()),
                children: Some(theme_items),
                ..Default::default()
            },
        );
        let mut clone_for_theme_toggle = self.clone();
        let theme_toggle_onclick =
            Box::new(move || clone_for_theme_toggle.reduce_state(theme_toggle_action.clone()))
                as rust_fel::ClosureProp;

        let theme_icon = rust_fel::html("<span |class=theme-icon|></span>".to_owned());

        let theme_toggle = rust_fel::Element::new(
            "span".to_owned(),
            rust_fel::Props {
                children: Some(vec![theme_icon]),
                on_click: Some(theme_toggle_onclick),
                class_name: Some("theme-toggle".to_owned()),
                role: Some("button".to_owned()),
                data_cy: Some("theme-toggle".to_owned()),
                ..Default::default()
            },
        );

        let mut children = vec![theme_toggle];

        if state.show_themes {
            children.push(theme_switcher_wrapper);
        }

        rust_fel::Element::new(
            "div".to_owned(),
            rust_fel::Props {
                id: Some(borrow.id.clone()),
                class_name: Some("theme-component".to_owned()),
                children: Some(children),
                ..Default::default()
            },
        )
    }
}
