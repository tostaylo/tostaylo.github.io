use rust_fel;

pub fn site_info() -> rust_fel::Element {
    let site_info = rust_fel::html(format!(
        "<div |class=site-info|> 
          <pre>
            <code>pub fn what(){{
                        }}
            </code>
         </pre>
        </div>"
    ));
    site_info
}
