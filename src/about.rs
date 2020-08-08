use rust_fel;

pub fn about() -> rust_fel::Element {
    let about_text = rust_fel::html(format!(
        "<div>
          <h2>About</h2>
          <p> I am a Senior Software Developer at CoStar Group in Austin, TX where I write and mantain web applications.</p>
          <p> Outside of work I </p>
          <ul>
            <li> Am pursuing a Masters of Liberal Arts Computer Science from Harvard Extension College.</li>
            <li> Founded and co-organize the Austin Web Performance Meetup.</li>
            <li> Build things in Rust, Typescript, React, and C#.</li>
          </ul>
          <img | class=profile-img src=assets/torre_bw_2018.jpg |></img>
        </div>"
    ));

    let about = rust_fel::Element::new(
        "div".to_owned(),
        rust_fel::Props {
            class_name: Some(format!("about")),
            children: Some(vec![about_text]),
            ..Default::default()
        },
    );
    about
}
