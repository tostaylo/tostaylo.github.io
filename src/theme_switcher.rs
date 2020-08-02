use rust_fel;

pub fn theme_switcher(on_click: Option<rust_fel::ClosureProp>) -> rust_fel::Element {
    let theme_switcher_text = rust_fel::html(format!(
        "<span |class=theme-switcher-text|>Switch Themes!</span>"
    ));
    let theme_switcher = rust_fel::create_element(
        "div".to_owned(),
        rust_fel::Props {
            on_click,
            class_name: Some(format!("theme-switcher")),
            children: Some(vec![theme_switcher_text]),
            ..Default::default()
        },
    );

    theme_switcher
}
