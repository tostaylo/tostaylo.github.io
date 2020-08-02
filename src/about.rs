use rust_fel;

pub fn about() -> rust_fel::Element {
    let about = rust_fel::html(format!(
        "<div |class=about|>
          <p>I am currently a Senior Software Developer at CoStar Group in Austin, TX where I write and mantain web applications.</p>
          <p> Outside of work I</p>
          <p> Am taking courses to attain a computer science master's degree from Harvard Extension.</p>
          <p> Founded and co-organize the Austin Web Performance Meetup</p>
          <p>Build things in Rust, Typescript, React, and C#.</p>
        </div>"
    ));
    about
}
