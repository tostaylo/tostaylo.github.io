use rust_fel;

pub fn posts() -> rust_fel::Element {
    let post_text = rust_fel::html(format!(
        "<div>
          <h2>Posts</h2>
          <p>Posts Go Here</p>
        </div>"
    ));

    let posts = rust_fel::Element::new(
        "div".to_owned(),
        rust_fel::Props {
            class_name: Some(format!("posts")),
            children: Some(vec![post_text]),
            ..Default::default()
        },
    );
    posts
}
