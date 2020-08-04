use rust_fel;

pub fn posts() -> rust_fel::Element {
    let posts = rust_fel::html(format!(
        "<div |class=posts|>
          Posts Go Here
        </div>"
    ));
    posts
}
