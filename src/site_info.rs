use rust_fel;

pub fn site_info() -> rust_fel::Element {
    let intro_text = format!(
        "
<div |classname=site-info-intro|> 
  <h2>Site Info</h2>
  <p>  
    You are viewing a site built with Rust and Web Assembly.
    Here are the reasons I chose Rust and Web Assembly.
    Additonally, I have invented my own front-end-framework ( Yes the world needed another one ).
    It's not very good. But it got the job done.
  </p>
</div>"
    );
    let rust_fel_html_text = format!(
        "let html = 
rust_fel::html({});",
        intro_text.clone()
    );
    let intro_el = rust_fel::html(intro_text);

    let rust_fel_fc_text = format!(
        "
use rust_fel;

pub fn site_info() -> rust_fel::Element {{
  let site_info = rust_fel::Element::new(
    \"div\".to_owned(),
    rust_fel::Props {{}}
        class_name: Some(format!(\"site-info\")),
        ..Default::default()
    }},
  );

site_info
}}"
    );

    let rust_fel_fc_code_block =
        code_pre_block(rust_fel_fc_text, "Here is a rust_fel functional component.");

    let rust_fel_component_text = format!(
        "
use crate::action::Action;
use crate::handle;
use crate::text_wrapper::text_wrapper;
use rust_fel;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Default, Clone)]
pub struct ChildProps {{
  pub string_props: String,
}}

#[derive(Debug, Default, Clone)]
pub struct MainSibling {{
  state: i32,
  props: ChildProps,
  id: String,
}}

impl MainSibling {{
pub fn create() -> handle::Handle<Self> {{
    let main_child = MainSibling {{
        id: \"main-sibling\".to_owned(),
        ..Default::default()
    }};
    handle::Handle(Rc::new(RefCell::new(main_child)))
}}
}}

impl rust_fel::Component for handle::Handle<MainSibling> {{

type Properties = ChildProps;
type Message = Action;
type State = i32;

fn add_props(&mut self, props: Self::Properties) {{
    self.0.borrow_mut().props = props;
}}

fn reduce_state(&mut self, message: Action) {{
    match message {{
        Action::Increment => self.0.borrow_mut().state += 5,
        Action::Decrement => self.0.borrow_mut().state -= 5,
    }}

    rust_fel::re_render(self.render(), Some(self.0.borrow().id.clone()));
}}

fn render(&self) -> rust_fel::Element {{
    let mut clone = self.clone();
    let borrow = self.0.borrow();

    let main_text = rust_fel::Element::new(
        \"TEXT_ELEMENT\".to_owned(),
        rust_fel::Props {{
            text: Some(format!(
                \"Hi, From Main Child Sibling {{}}\",
                borrow.state.to_string()
            )),
            ..Default::default()
        }},
    );

    let main_el = text_wrapper(
        \"div\".to_owned(),
        Some(vec![main_text]),
        None,
        Some(\"main-text\".to_owned()),
    );

    let closure = move || clone.reduce_state(Action::Decrement);

    let main = rust_fel::Element::new(
        \"div\".to_owned(),
        rust_fel::Props {{
            id: Some(self.0.borrow().id.clone()),
            on_click: Some(Box::new(closure.clone())),
            class_name: Some(\"main-child\".to_owned()),
            children: Some(vec![main_el]),
            ..Default::default()
        }},
    );

    main
  }}
}}"
    );

    let rust_fel_struct_code_block = code_pre_block(
        rust_fel_component_text,
        "Here is a rust_fel struct component.",
    );

    let rust_fel_rsx_code_block =
        code_pre_block(rust_fel_html_text, "Here is a rust_fel rsx element.");

    let site_info = rust_fel::Element::new(
        "div".to_owned(),
        rust_fel::Props {
            class_name: Some(format!("site-info")),
            children: Some(vec![
                intro_el,
                rust_fel_struct_code_block,
                rust_fel_fc_code_block,
                rust_fel_rsx_code_block,
            ]),
            ..Default::default()
        },
    );
    site_info
}

fn code_pre_block(code_text: String, heading_text: &str) -> rust_fel::Element {
    let heading_el = rust_fel::html(format!("<h5>{}</h5>", heading_text));

    let code_text_el = rust_fel::Element::new(
        "TEXT_ELEMENT".to_owned(),
        rust_fel::Props {
            text: Some(code_text),
            ..Default::default()
        },
    );

    let code_el = rust_fel::Element::new(
        "code".to_owned(),
        rust_fel::Props {
            children: Some(vec![code_text_el]),
            ..Default::default()
        },
    );
    let pre_el = rust_fel::Element::new(
        "pre".to_owned(),
        rust_fel::Props {
            children: Some(vec![code_el]),
            ..Default::default()
        },
    );

    let container = rust_fel::Element::new(
        "div".to_owned(),
        rust_fel::Props {
            children: Some(vec![heading_el, pre_el]),
            ..Default::default()
        },
    );
    container
}
