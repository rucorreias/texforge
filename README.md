# TexForge

A local-first LaTeX IDE that runs in the browser and works directly with your local files.

Built with:

* Rust + Axum
* React + TypeScript
* Monaco Editor
* PDF.js
* LaTeX / TeX Live

## 🚀 Run

### 🦀 Backend

```bash
cargo check
cargo test
cargo run -p texforge-server
```

The server runs at:

```text
http://127.0.0.1:3000
```

### 🌐 Frontend

```bash
cd web
pnpm install
pnpm dev
```

The frontend runs at the URL shown by Vite.

## 📁 Project structure

```text
texforge/
├── crates/
│   ├── texforge-cli/
│   ├── texforge-server/
│   └── texforge-core/
├── web/
├── templates/
├── docker/
└── docs/
```

## 🎯 Goal

Run a LaTeX project locally with:

```bash
texforge .
```

and edit, compile and preview it directly in the browser.
