use rust_fel;

pub fn theme_switcher(on_click: rust_fel::ClosureProp, title: String) -> rust_fel::Element {
    let text = rust_fel::html(format!(
        "<span |class=theme-switcher-text|>{}</span>",
        title
    ));
    let theme = rust_fel::Element::new(
        "span".to_owned(),
        rust_fel::Props {
            on_click: Some(on_click),
            class_name: Some(format!("theme-switcher-item")),
            children: Some(vec![text]),
            ..Default::default()
        },
    );

    theme
}
