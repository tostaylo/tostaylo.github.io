use rust_fel;

pub fn header() -> rust_fel::Element {
    let header_class = "header";
    let header = rust_fel::html(format!(
        "<header |class={}|>This is my portfolio built with Rust</header",
        header_class
    ));
    header
}
