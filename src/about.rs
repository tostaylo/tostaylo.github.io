pub fn about() -> rust_fel::Element {
    let about_text = rust_fel::html(
        "<div>
          <h2>About</h2>
          <p> I am a Senior Software Developer at CoStar Group in Austin, TX where I write and mantain web applications.</p>
          <p> Outside of work I </p>
          <ul>
            <li>
             <span>Am a degree applicant for the </span> 
              <a | href=https://www.extension.harvard.edu/academics/graduate-degrees/software-engineering-degree |>Masters of Liberal Arts, Software Engineering, from Harvard Extension College.</a>
            </li>
            <li> 
              <span>Founded and co-organize the </span> 
                <a | href=https://www.meetup.com/austin-web-performance |>Austin Web Performance Meetup.</a>
            </li>
            <li> Build things in Rust, Typescript, React, and C#.</li>
            <li> Play loud, overly distorted, barely listenable, guitar. </li>
          </ul>
          <div | class=profile-img-container |>
            <img | class=profile-img src=assets/images/torre-bw-2018.webp |></img>
          </div>
        </div>".to_owned()
    );

    rust_fel::Element::new(
        "div".to_owned(),
        rust_fel::Props {
            class_name: Some("about".to_owned()),
            children: Some(vec![about_text]),
            data_cy: Some("about".to_owned()),
            ..Default::default()
        },
    )
}
