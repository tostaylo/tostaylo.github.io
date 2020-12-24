// <script |src=./svelte-app/public/build/bundle.js|></script>
pub fn posts() -> rust_fel::Element {
    let post_text = rust_fel::html(
        "<div>
          <h2>Posts</h2>
          <div |id=svelte-app class=svelte-app |><span |id=loading-placeholder| >Loading...</span</div>
         <script |src=./svelte-app/public/build/bundle.js|></script>
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
