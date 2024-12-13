pub fn posts() -> rust_fel::Element {
    let post_text = rust_fel::html(
        "<div>
          <h2>Posts</h2>
          <div |id=injected-app class=injected-app |><span |id=loading-placeholder| >Loading...</span</div>
         <script |src=/assets/main-bundle.js|></script>
        </div>"
            .to_owned(),
    );

    rust_fel::Element::new(
        "div".to_owned(),
        rust_fel::Props {
            class_name: Some("posts".to_owned()),
            children: Some(vec![post_text]),
            ..Default::default()
        },
    )
}
