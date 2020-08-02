use rust_fel;

pub fn text_wrapper(
    html_type: String,
    text: String,
    on_click: Option<rust_fel::ClosureProp>,
    class_name: Option<String>,
) -> rust_fel::Element {
    let text = rust_fel::create_element(
        "TEXT_ELEMENT".to_owned(),
        rust_fel::Props {
            text: Some(format!("{}", text)),
            ..Default::default()
        },
    );
    let text_container = rust_fel::create_element(
        html_type,
        rust_fel::Props {
            on_click,
            class_name,
            children: Some(vec![text]),
            ..Default::default()
        },
    );

    text_container
}
