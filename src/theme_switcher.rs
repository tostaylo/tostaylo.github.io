pub fn theme_switcher(on_click: rust_fel::ClosureProp, title: String) -> rust_fel::Element {
    let text = rust_fel::html(format!(
        "<span |class=theme-switcher-text|>{}</span>",
        title
    ));
    let theme_button = rust_fel::Element::new(
        "button".to_owned(),
        rust_fel::Props {
            on_click: Some(on_click),
            type_attr: Some("button".to_owned()),
            class_name: Some("theme-switcher-button".to_owned()),
            children: Some(vec![text]),
            data_cy: Some(title),
            ..Default::default()
        },
    );

    rust_fel::Element::new(
        "li".to_owned(),
        rust_fel::Props {
            children: Some(vec![theme_button]),
            ..Default::default()
        },
    )
}
