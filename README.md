# rust-portfolio

My [portfolio website](https://tostaylo.github.io), built with [rust-fel](https://github.com/tostaylo/rust-fel) and React

1. Install Cargo and Rust https://doc.rust-lang.org/cargo/getting-started/installation.html
2. Install WASM Pack using Cargo
3. `npm run build` builds the rust app, translates markdown files to html, builds react app, copies assets to root, runs a local node server.

The site is being built from a Rust Package that is transpiled to WASM/JS. The index.html requests the JS produced by Rust WASM. The Posts page requests a React App which attaches itself to a DOM Node on the page.

Assets are being built into a root level assets directory and pkg directory.

### Gotchas

Using the 404.html page duplicate of index.html to allow for routing on Github Pages.
