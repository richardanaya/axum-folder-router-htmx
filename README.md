# HTMX and Rust Example Repo

This repository is an example that shows my dream setup for HTMX and Rust. It demonstrates a folder router based system inspired by NextJS. The structure leverages Rust with the Axum framework and Askama for neat, clean, typed templates.

## Overview

- **Folder Router System:** The routing logic is handled by a folder-based structure, ensuring that routes are automatically generated from files in `src/api`.
- **HTMX Integration:** The HTML templates make use of HTMX to power dynamic, hypermedia-driven interactions.
- **Askama Templates:** Server-rendered pages use Askama for easy and typed template management.

## What This Example Shows

- A simple landing page that processes a form submission.
- A clean project structure that can be adapted to scale for larger projects.
- A combination of modern frontend interactivity with powerful Rust tooling.

## Getting Started

To run the example:

```bash
cargo run
```

Then visit [http://localhost:3000](http://localhost:3000) in your browser to see the example in action.

## License

MIT License
