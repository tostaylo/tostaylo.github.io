pub fn projects() -> rust_fel::Element {
    // Define project data structure inline
    struct Project {
        title: &'static str,
        description: &'static str,
        image_path: &'static str,
        github_url: Option<&'static str>,
        live_url: Option<&'static str>,
        tech_stack: &'static str,
    }

    // Mock project data
    let projects_data = vec![
        Project {
            title: "rust-fel",
            description: "A front-end library for building web applications with Rust and WebAssembly. Features a Virtual DOM, component-based architecture, and JSX-like syntax.",
            image_path: "/assets/images/composite-layers.webp",
            github_url: Some("https://github.com/tostaylo/rust-fel"),
            live_url: None,
            tech_stack: "Rust, WebAssembly, wasm-bindgen",
        },
        Project {
            title: "Portfolio Site",
            description: "This portfolio website built entirely with Rust and WebAssembly, demonstrating the capabilities of rust-fel framework.",
            image_path: "/assets/images/torre-bw-2018.webp",
            github_url: Some("https://github.com/tostaylo/tostaylo.github.io"),
            live_url: Some("https://tostaylo.github.io"),
            tech_stack: "Rust, WebAssembly, rust-fel",
        },
        Project {
            title: "Framework Performance Benchmarks",
            description: "A comprehensive performance comparison tool for front-end frameworks, measuring render times, memory usage, and DOM operations.",
            image_path: "/assets/images/perf-timeline-full.webp",
            github_url: Some("https://github.com/tostaylo/front-end-framework-bench-viewer"),
            live_url: Some("https://tostaylo.github.io/front-end-framework-bench-viewer/"),
            tech_stack: "TypeScript, React, Performance API",
        },
    ];

    // Build project cards
    let project_elements: Vec<rust_fel::Element> = projects_data
        .iter()
        .map(|project| {
            // Build links section
            let mut links_html = String::new();
            if let Some(github) = project.github_url {
                links_html.push_str(&format!(
                    "<a | href={} target=_blank class=project-link |>GitHub</a>",
                    github
                ));
            }
            if let Some(live) = project.live_url {
                if !links_html.is_empty() {
                    links_html.push_str("<span> | </span>");
                }
                links_html.push_str(&format!(
                    "<a | href={} target=_blank class=project-link |>Live Demo</a>",
                    live
                ));
            }

            let project_html = format!(
                "<div |class=project-card|>
                  <div |class=project-image-container|>
                    <img |class=project-image src={} alt={}|></img>
                  </div>
                  <div |class=project-content|>
                    <h3>{}</h3>
                    <p>{}</p>
                    <div |class=project-tech|>
                      <strong>Tech Stack:</strong>
                      <span> {}</span>
                    </div>
                    <div |class=project-links|>
                      {}
                    </div>
                  </div>
                </div>",
                project.image_path,
                project.title,
                project.title,
                project.description,
                project.tech_stack,
                links_html
            );

            rust_fel::html(project_html)
        })
        .collect();

    // Header section
    let header = rust_fel::html(
        "<div |class=projects-header|>
          <h2>Projects</h2>
          <p>Here are some of the projects I've built exploring Rust, WebAssembly, and modern web technologies.</p>
        </div>"
        .to_owned(),
    );

    // Projects grid container
    let projects_grid = rust_fel::Element::new(
        "div".to_owned(),
        rust_fel::Props {
            class_name: Some("projects-grid".to_owned()),
            children: Some(project_elements),
            ..Default::default()
        },
    );

    // Main projects container
    rust_fel::Element::new(
        "div".to_owned(),
        rust_fel::Props {
            class_name: Some("projects".to_owned()),
            children: Some(vec![header, projects_grid]),
            data_cy: Some("projects".to_owned()),
            ..Default::default()
        },
    )
}
