# HTMX + Rust + NextJS Folder-based Router

This repository is an example that shows my dream setup for HTMX and Rust. It demonstrates a folder router based system inspired by NextJS. The structure leverages Rust with the Axum framework and Askama for neat, clean, typed templates.

## Overview

- **Folder Router System:** The routing logic is handled by a folder-based structure, ensuring that routes are automatically generated from files in `src/api`. It supports nested routes and segments/slugs (i.e. `foo/[id]` , `blah/[...]`). It does this by generating the routes builder with the `build.rs`
- **Askama Templates:** Server-rendered pages use Askama for easy and typed template management.
- Shows how to use axum_extra for strongly typed Forms

## Getting Started

To run the example:

```bash
cargo run
```

Then visit [http://localhost:3000](http://localhost:3000) in your browser to see the example in action.

## License

MIT License
