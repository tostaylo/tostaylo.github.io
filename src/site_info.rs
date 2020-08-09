use rust_fel;

pub fn site_info() -> rust_fel::Element {
    let site_info_text = rust_fel::html(format!(
        "<div> 
          <h2>Site Info</h2>
         <p> 
             You are viewing a site built with Rust and Web Assembly.
             Here are the reasons I chose Rust and Web Assembly.
             Additonally, I have invented my own front-end-framework ( Yes the world need another one ).
             It's not very good. But it got the job done.
        </p>
        <p>
            Here is a simple rust_fel functional component!
        </p>
<pre>
<code>
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
}}
</code>
</pre>
        </div>"
    ));

    let site_info = rust_fel::Element::new(
        "div".to_owned(),
        rust_fel::Props {
            class_name: Some(format!("site-info")),
            children: Some(vec![site_info_text]),
            ..Default::default()
        },
    );
    site_info
}
