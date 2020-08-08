use rust_fel;

pub fn site_info() -> rust_fel::Element {
    let site_info_text = rust_fel::html(format!(
        "<div> 
          <h2>Site Info</h2>
          <pre>
            <code>pub fn what(){{
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
