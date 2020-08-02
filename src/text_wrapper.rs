use rust_fel;

pub fn text_wrapper(
    html_type: String,
    text: Option<String>,
    on_click: Option<rust_fel::ClosureProp>,
    class_name: Option<String>,
    elements: Option<rust_fel::Element>,
) -> rust_fel::Element {
    let mut children = None;

    match (elements, text) {
        (Some(el), None) => {
            children = Some(vec![el]);
        }
        (None, Some(t)) => {
            let text_el = rust_fel::create_element(
                "TEXT_ELEMENT".to_owned(),
                rust_fel::Props {
                    text: Some(format!("{}", t)),
                    ..Default::default()
                },
            );
            children = Some(vec![text_el]);
        }
        _ => panic!("Have to have at least (text) or (elements) but not both."),
    };

    rust_fel::create_element(
        html_type,
        rust_fel::Props {
            on_click,
            class_name,
            children,
            ..Default::default()
        },
    )
}
