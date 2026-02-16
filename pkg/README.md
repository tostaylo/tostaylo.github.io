# rust-portfolio

My [portfolio website](https://tostaylo.github.io), built with [rust-fel](https://github.com/tostaylo/rust-fel), Rust WASM, and React.

## Overview

This is a hybrid portfolio site that combines multiple technologies:

- **Main Site**: Built with Rust using [rust-fel](https://github.com/tostaylo/rust-fel) framework, compiled to WebAssembly
- **Posts/Projects Pages**: React SPA built with Vite and styled with Tailwind CSS
- **Blog Engine**: Markdown-to-HTML conversion pipeline using Showdown
- **Theme System**: Dynamic light/dark theme switcher
- **Deployment**: GitHub Pages with client-side routing

### Architecture

The site uses a dual-runtime approach:

- **Rust WASM Layer**: Renders the main site structure (home, navigation, theme)
- **React SPA Layer**: Handles interactive pages (/posts, /projects) injected into the DOM
- Both communicate through shared DOM nodes and localStorage

## Tech Stack

### Backend/Static Generation

- **Rust** - Main application logic compiled to WASM
- **rust-fel** - Rust framework for functional UI
- **wasm-pack** - Rust to WebAssembly compiler
- **wee_alloc** - Minimal memory allocator for WASM

### Frontend

- **React 18** - Posts and projects interactive UI
- **Vite** - Fast build tool and dev server
- **TypeScript** - Type-safe JavaScript
- **Tailwind CSS** - Utility-first styling
- **PostCSS** - CSS processing with autoprefixer
- **wouter** - Lightweight client-side router
- **Lucide React** - Icon library

### Build & Tooling

- **Showdown** - Markdown to HTML converter
- **ESLint** - Code linting
- **Cypress** - E2E testing framework
- **http-server** - Local development server

## Prerequisites

- [Rust & Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [Node.js](https://nodejs.org/) & npm
- `wasm-pack` - Install with: `cargo install wasm-pack`

## Setup & Running

1. Install dependencies in all directories:

   ```bash
   npm install
   cd injected-app && npm install && cd ..
   cd converter && npm install && cd ..
   ```

2. Build and run the development server:
   ```bash
   npm run build
   ```

This single command:

- Compiles Rust code to WASM using wasm-pack (`build.sh`)
- Converts markdown posts to HTML (`converter/index.js`)
- Builds the React SPA with Vite (`injected-app/vite.config.ts`)
- Copies generated assets to root-level `assets/` directory
- Starts a local http-server on port 8000

## Available Scripts

- `npm run start` - Watch TypeScript files for changes
- `npm run build` - Full production build (same as described above)
- `npm run build-ts` - Compile TypeScript only
- `npm run build-injected` - Build React app in isolation
- `npm run convert` - Convert markdown posts to HTML
- `npm run node-server` - Start local server on port 8000

## Development Workflow

### Adding New Posts

1. Create a markdown file in `converter/posts/md/` (e.g., `post.3.md`)
2. Run `npm run convert` to generate HTML
3. The React component will detect and display the new post

### Theme Switching

The site includes a dynamic theme switcher (`src/theme_switcher.rs`) that toggles between light and dark modes and persists to localStorage.

### Styling

The React app uses Tailwind CSS with custom components in `injected-app/src/ui/`. ESLint enforces code quality.

## Testing

Cypress configuration exists in `cypress.json` for E2E testing.

## Deployment Notes

### GitHub Pages Routing

The `404.html` file is a duplicate of `index.html` to enable client-side routing on GitHub Pages. When a user navigates to a non-existent path, GitHub Pages serves `404.html`, which contains the full SPA routing logic.

### Asset Generation

- Generated WASM files are in `pkg/` directory
- Built React assets are copied to `assets/` in the bundle step
- CSS and JS are optimized for production in the Vite build

## Performance Optimizations

- **wee_alloc**: Minimal WASM memory footprint
- **Release build**: Rust compiled with LTO (Link-Time Optimization) and opt-level 3
- **Code splitting**: Separate WASM runtime and React bundle
- **Asset bundling**: Vite's rollup configuration bundles React app into single JS file
