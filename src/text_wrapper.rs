use rust_fel;

pub fn text_wrapper(
    html_type: String,
    children: Option<Vec<rust_fel::Element>>,
    on_click: Option<rust_fel::ClosureProp>,
    class_name: Option<String>,
) -> rust_fel::Element {
    let text_container = rust_fel::create_element(
        html_type,
        rust_fel::Props {
            on_click,
            class_name,
            children,
            ..Default::default()
        },
    );

    text_container
}
