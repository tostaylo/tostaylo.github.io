use crate::handle;
use rust_fel;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum DarkMode {
  Dark,
  Light,
}

impl Default for DarkMode {
  fn default() -> Self {
    DarkMode::Light
  }
}

#[derive(Debug, Default, Clone)]
pub struct MainState {
  dark_mode: DarkMode,
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
    self.0.borrow_mut().state.dark_mode = new_state.dark_mode;
    rust_fel::re_render(self.render(), Some(self.0.borrow().id.clone()));
  }

  fn render(&self) -> rust_fel::Element {
    let mut clone = self.clone();
    let borrow = self.0.borrow_mut();
    let state = borrow.state.clone();
    let theme_class = match state.dark_mode {
      DarkMode::Dark => "dark".to_owned(),
      DarkMode::Light => "light".to_owned(),
    };
    // let closure = Rc::new(RefCell::new(move || clone.set_state(1000)));
    let html = rust_fel::html(format!(
      "<header |class={}|>This is my portfolio built with Rust</header",
      theme_class
    ));

    let toggle_dark_mode_state = match state.dark_mode {
      DarkMode::Dark => MainState {
        dark_mode: DarkMode::Light,
      },
      DarkMode::Light => MainState {
        dark_mode: DarkMode::Dark,
      },
    };

    let main = rust_fel::create_element(
      "div".to_owned(),
      rust_fel::Props {
        id: Some(borrow.id.clone()),
        on_click: Some(Box::new(move || {
          clone.set_state(toggle_dark_mode_state.clone())
        })),
        class_name: Some(format!("main {}", theme_class)),
        children: Some(vec![html]),
        ..Default::default()
      },
    );

    main
  }
}
