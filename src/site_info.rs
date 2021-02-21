pub fn site_info() -> rust_fel::Element {
    let intro_text = "
<div |class=site-info-text-block|> 
  <h2>Site Info</h2>
  <p> You are viewing a site built with
     <a | href=https://www.rust-lang.org/ |>Rust</a> <span>,
     </span> <a | href=https://webassembly.org/ |>Web Assembly</a><span>, </span> 
      <span>and </span> <a | href=https://svelte.dev/ |>Svelte</a><span>.</span></p>
   <p>There are a million articles online about why Rust is a good choice but I'll only link to one.
    <a | href=https://rustwasm.github.io/docs/book/why-rust-and-webassembly.html |> Why Rust and Web Assembly? </a>
  </p>
  <p> To assist in building this site, I created another front-end-library the world does not need  -- 
    <a | href=https://github.com/tostaylo/rust-fel |><pre><code>rust_fel</code></pre></a>
  </p>
  <p> It's not very good, but I like showing it off!</p>
</div>".to_owned()
    ;
    let rust_fel_html_text = format!(
        "let html = 
rust_fel::html({});",
        intro_text
    );
    let intro_el = rust_fel::html(intro_text);

    let rust_fel_fc_text = "
use rust_fel;

pub fn site_info() -> rust_fel::Element {{
  let site_info = rust_fel::Element::new(
    \"div\".to_owned(),
    rust_fel::Props {{
        class_name: Some(format!(\"site-info\")),
        ..Default::default()
    }},
  );

  site_info
}}"
    .to_owned();

    let rust_fel_fc_code_block =
        code_pre_block(rust_fel_fc_text, "Here is a rust_fel functional component.");

    let rust_fel_component_text = "
use crate::action::Action;
use crate::handle;
use rust_fel;
use std::cell::RefCell;
use std::rc::Rc;


#[derive(Debug, Default, Clone)]
pub struct MainProps {{
  pub string_props: String,
}}

#[derive(Debug, Default, Clone)]
pub struct Main {{
  state: i32,
  props: MainProps,
  id: String,
}}


impl Main {{
  pub fn create() -> handle::Handle<Self> {{
      let main = Main {{
          id: \"main\".to_owned(),
          ..Default::default()
      }};
      handle::Handle(Rc::new(RefCell::new(main)))
  }}
}}


impl rust_fel::Component for handle::Handle<Main> {{

type Properties = MainProps;
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

    let main_el = rust_fel::Element::new(
        \"div\".to_owned(),
        rust_fel::Props {{
            children: Some(vec![main_text]),
            ..Default::default()
        }},
    );

    let handle_click = move || clone.reduce_state(Action::Decrement);

    let main = rust_fel::Element::new(
        \"div\".to_owned(),
        rust_fel::Props {{
            id: Some(self.0.borrow().id.clone()),
            on_click: Some(Box::new(handle_click)),
            class_name: Some(\"main\".to_owned()),
            children: Some(vec![main_el]),
            ..Default::default()
        }},
    );

    main
  }}
}}"
    .to_owned();

    let rust_fel_struct_code_block = code_pre_block(
        rust_fel_component_text,
        "Here is a rust_fel struct component.",
    );

    let rust_fel_rsx_code_block =
        code_pre_block(rust_fel_html_text, "Here is a rust_fel html function.");

    let post_component_text = rust_fel::html(
        "
<div |class=site-info-text-block|>
  <p>
    <span>My influences in creating this library were </span> 
    <a | href=https://github.com/facebook/react |>React </a> 
    <span> and </span>
    <a | href=https://github.com/yewstack/yew |> Yew</a>s
    <span>.  </span>
    <span>In case you were wondering, </span>
    <pre><code>rust_fel</code></pre>  
    <span> uses a Virtual DOM.  But nothing fancy.  </span>
    <span>If a </span>
    <pre><code>rust_fel</code></pre>  
    <span>component chooses to update itself, a new Virtual DOM is constructed.
      The DOM Node the component represents and all of it's children are removed from the DOM.
      Finally, a new DOM tree is constructed from the position of the root DOM Node removed.
    </span>
  </p>
  <p>
    <pre><code>rust_fel</code></pre> 
      <span> includes support for state management at the component level.  
      Data flows one direction, down, from parent to children.
      A parent can update it's children by updating itself and passing new data to the child. 
      A child can update it's parent, if the parent gives the child the means to. 
      Siblings cannot update one another directly. 
      Any communication between siblings must be done through the parent.
    </span>
  </p>
</div>"
            .to_owned(),
    );

    let pre_rsx_text = rust_fel::html(
        "
<div |class=site-info-text-block|>
  <p>
    <pre><code>rust_fel</code></pre> 
    <span>includes a small </span>    
    <a | href=https://github.com/facebook/jsx |>jsx</a>
    <span>-like library in order to write basic html elements with less syntactic effort.</span>
  </p>
</div>"
            .to_owned(),
    );

    let outro_text = rust_fel::html(
        "
<div |class=site-info-text-block|> 
  <p> 
   <span><pre><code>rust_fel</code></pre></span>
    <span> has one major dependency</span>
    <a | href=https://github.com/rustwasm/wasm-bindgen |>
      <pre><code>rustwasm/wasm-bindgen</code></pre>
    </a>
  </p>
</div>"
            .to_owned(),
    );

    let comparison = rust_fel::html(
        "
<div |class=site-info-text-block|> 
  <a | data-cy=comparison-main-text href=https://front-end-framework-bench.torretaylor.me/comparison |>
    Performance comparison with other front-end frameworks
  </a>
</div>"
            .to_owned(),
    );

    rust_fel::Element::new(
        "div".to_owned(),
        rust_fel::Props {
            class_name: Some("site-info".to_owned()),
            children: Some(vec![
                intro_el,
                rust_fel_struct_code_block,
                post_component_text,
                rust_fel_fc_code_block,
                pre_rsx_text,
                rust_fel_rsx_code_block,
                outro_text,
                comparison,
            ]),
            data_cy: Some("site-info".to_owned()),
            ..Default::default()
        },
    )
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

    rust_fel::Element::new(
        "div".to_owned(),
        rust_fel::Props {
            children: Some(vec![heading_el, pre_el]),
            ..Default::default()
        },
    )
}
