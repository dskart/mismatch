# Mismatch ⚖

A word-similarity game powered by ONNX embeddings, served by a Rust (axum) backend with an htmx UI.

## Getting Started

### Prerequisites

Toolchains (Rust, Node, systemfd, cargo-watch) are pinned in `mise.toml`. Install [mise](https://mise.jdx.dev), then:

```bash
mise install
```

You also need `wget` and ONNX Runtime.

```bash
make setup
make gen_word_dict
make serve
```

## Deployment

The app runs on [Cloudflare Containers](https://developers.cloudflare.com/containers/). A Worker in `cloudflare/` forwards requests to the
container built from `Dockerfile`. Deploying requires a Workers Paid plan and Docker running locally.

```bash
make cf-setup
cd cloudflare && npx wrangler login   # once
make deploy
```

See [AGENTS.md](AGENTS.md) for more details on the project layout and conventions.
