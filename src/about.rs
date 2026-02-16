pub fn about() -> rust_fel::Element {
    let about_text = rust_fel::html(
        "<div>
          <h2>About</h2>
          <p> I am a Staff Software Engineer at LegalZoom in Seattle, WA where I write and maintain web applications.</p>
          <p> Outside of work I </p>
          <ul>
            <li>
             <span>Instruct full-stack web development for </span> 
              <a | href=https://techbootcamps.utexas.edu/coding/ >The Coding Boot Camp at UT Austin.</a>
            </li>
            <li> Build things in Rust, Go, Typescript, React, Swift, and more.</li>
            <li> <span>Write, play, and record perfectly normal songs about my furniture.  Here is a link to my </span>  <a | href=https://soundcloud.com/known-sound |> SoundCloud song playground </a> <span>where I try out ideas.</span> </li>
          </ul>
          <div | class=profile-img-container |>
            <img | class=profile-img src=/assets/images/torre-bw-2018.webp |></img>
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
