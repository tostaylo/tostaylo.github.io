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

    // Project data
    let projects_data = vec![
        Project {
            title: "SecureViz",
            description: "A desktop Mac application for AI-powered data visualization that processes everything locally with zero cloud uploads. Features natural language querying, supports CSV/Excel import, and handles 100,000+ rows with sub-100ms query performance using local Ollama AI.",
            image_path: "/assets/images/secure-viz.png",
            github_url: None,
            live_url: Some("https://secure-suite.github.io/secure-viz/"),
            tech_stack: "Go, SQLite, Apache ECharts, React, Ollama, VitePress",
        },
        Project {
            title: "WhisperClip",
            description: "A lightweight macOS menu bar application for instant voice-to-text transcription using OpenAI's Whisper speech recognition model. Features global hotkey recording, automatic clipboard integration, and completely offline processing with whisper.cpp for fast, accurate transcription.",
            image_path: "/assets/images/whisper-clip.png",
            github_url: None,
            live_url: None,
            tech_stack: "Swift, Whisper.cpp, OpenAI Whisper",
        },
        Project {
            title: "rust-fel",
            description: "An experimental frontend library built on rustwasm that enables client-side development using Rust compiled to WebAssembly. Features a Virtual DOM, JSX-like syntax, and component-based architecture with state management.",
            image_path: "/assets/images/rust-fel.png",
            github_url: Some("https://github.com/tostaylo/rust-fel"),
            live_url: Some("/site-info"),
            tech_stack: "Rust, WebAssembly, wasm-bindgen",
        },
        Project {
            title: "Front-End Framework Bench Viewer",
            description: "A Vue 3 application for visualizing benchmark results from front-end framework performance testing. Displays comparative metrics across multiple frameworks to help developers make informed technology choices.",
            image_path: "/assets/images/front-end-framework-bench.png",
            github_url: Some("https://github.com/tostaylo/front-end-framework-bench-viewer"),
            live_url: Some("https://tostaylo.github.io/front-end-framework-bench-viewer/"),
            tech_stack: "Vue 3, TypeScript, Vite",
        },
        Project {
            title: "Front-End Framework Bench",
            description: "Automated testing suite that compares rendering performance across client-side frameworks. Uses Puppeteer to simulate user interactions and Chrome DevTools Performance Timeline to record timings for fair comparison.",
            image_path: "/assets/images/front-end-framework-bench.png",
            github_url: Some("https://github.com/tostaylo/front-end-framework-bench"),
            live_url: Some("https://tostaylo.github.io/front-end-framework-bench-viewer/"),
            tech_stack: "Rust, TypeScript, Puppeteer, Chrome DevTools",
        },
        Project {
            title: "Audio Effects",
            description: "A guitar effects simulator implementing amps, pedals, and mixing effects. Features decoupled core logic for portability across different audio context environments, deployed on Cloudflare Workers.",
            image_path: "/assets/images/audio-effects.png",
            github_url: Some("https://github.com/tostaylo/audio-effects"),
            live_url: Some("https://audio-effects.torretaylor.workers.dev/"),
            tech_stack: "Preact, Redux, TypeScript, Web Audio API, Cloudflare Workers",
        },
        Project {
            title: "Object Detection to Speech",
            description: "A Flask-based web API that integrates machine learning object detection libraries to identify objects in images and convert results to speech output. Supports deployment via Docker, Google Cloud Run, and local environments.",
            image_path: "/assets/images/object-detection-to-speech.png",
            github_url: Some("https://github.com/tostaylo/object-detection-to-speech"),
            live_url: Some("https://object-detection-qq4ppihdsa-uc.a.run.app/"),
            tech_stack: "Python, Flask, YOLOv5, Detectron2, Docker",
        },
        Project {
            title: "Chrome Measure User Perf",
            description: "Automates performance testing of user interactions using Puppeteer and Chrome DevTools Performance Timeline. Records click event durations and browser render processes with configurable timing thresholds to determine pass/fail outcomes.",
            image_path: "/assets/images/perf-timeline-full.webp",
            github_url: Some("https://github.com/tostaylo/chrome-measure-user-perf"),
            live_url: None,
            tech_stack: "TypeScript, Node.js, Puppeteer, Chrome DevTools",
        },
        Project {
            title: "Song Recommender",
            description: "A recommendation engine for songs built with Apache Kafka Streams and Scala. Users submit a song ID via Kafka, and the system processes the request and outputs a recommended track to a CSV file.",
            image_path: "/assets/images/song-recommender.png",
            github_url: Some("https://github.com/tostaylo/song-recommender"),
            live_url: Some("https://www.youtube.com/watch?v=DyHWdd0oyME"),
            tech_stack: "Scala, Apache Kafka Streams, Docker",
        },
        Project {
            title: "Lighthouse Captcha Plugin",
            description: "A Google Lighthouse plugin designed to detect captcha scripts loading at page load time. Helps identify potentially invalid Lighthouse performance results caused by captcha interference during automated testing.",
            image_path: "/assets/images/lighthouse-captcha-plugin.png",
            github_url: Some("https://github.com/tostaylo/lighthouse-plugin-has-captcha-on-page-load"),
            live_url: None,
            tech_stack: "TypeScript, Node.js, Google Lighthouse",
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
                    links_html.push_str(" <span |class=link-separator|>•</span> ");
                }
                links_html.push_str(&format!(
                    "<a | href={} target=_blank class=project-link |>Live Demo</a>",
                    live
                ));
            }

            let project_html = format!(
                "<div |class=project-card|>
                  <div |class=project-image-container|>
                    <img |src={} alt={} class=project-image|></img>
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
          <p>A collection of projects exploring Rust, WebAssembly, performance engineering, machine learning, and full-stack development.</p>
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
