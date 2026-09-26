# AGENTS.md

Guidance for AI coding agents working in this repo.

## What this is

**Mismatch** is a word-similarity game. You submit a word, and the server scores it against a target word
by taking the cosine similarity of ONNX embeddings from the [`potion-base-8M`](https://huggingface.co/minishlab/potion-base-8M)
model. There's a free-play mode at `/` and a daily mode at `/daily`.

- **Backend:** Rust (edition 2021), axum 0.7, tokio, `ort` (ONNX Runtime) + `tokenizers`, minijinja templates.
- **Frontend:** server-rendered HTML with htmx, and Tailwind v4 + daisyUI styling. The small TypeScript scripts are bundled with esbuild.
- **Hosting:** Cloudflare Containers. A thin TS Worker in `cloudflare/` forwards every request to the Rust server running in a container.
- **Statefulness:** none. There's no database and no runtime file writes. The daily word is round-tripped by the client, and high scores
  live in browser `localStorage`. Instances can restart or scale freely.

## Layout

| Path | Purpose |
| --- | --- |
| `src/main.rs`, `src/cmd/` | clap CLI (`serve`, `hello-world`), config loading, logging setup. `serve.rs` builds the router and binds `0.0.0.0:<port>` |
| `src/api/` | JSON API under `/api` (`GET /api/healthz`) |
| `src/app/` | Core logic: model loading (`mod.rs`, `model.rs`), scoring (`score.rs`), daily word, config |
| `src/store/` | In-memory English word list loaded from `gen/english_words.txt` |
| `src/ui/` | UI routes, middleware, `*.html.j2` templates (embedded via `include_str!`), and a Node project for CSS/JS |
| `src/ui/public/` | Static files served at `/public` (build output in `public/static/` is gitignored) |
| `src/bin/gen_word_dict/` | Build-time tool that downloads SCOWL and writes `gen/english_words.txt` |
| `cloudflare/` | Worker + `wrangler.jsonc` for Cloudflare Containers deployment |
| `Dockerfile` | Multi-stage build: UI, models, Rust + ONNX Runtime 1.16.3 (x64), then a slim runtime image |
| `mise.toml` | Pinned toolchain versions (Rust, Node, dev tools) |

## First-time setup

`models/`, `gen/`, `config.yaml` and `src/ui/public/static/` are gitignored and must be generated:

Toolchains are managed with [mise](https://mise.jdx.dev) (`mise.toml`: Rust 1.94 + clippy/rustfmt, Node 20, systemfd,
cargo-watch). CI installs Rust through `jdx/mise-action`.

```bash
make setup           # mise install, then setup-ui and models
make gen_word_dict   # writes gen/english_words.txt (needs network)
```

You also need ONNX Runtime installed locally (the `ort` crate links against it). A local `config.yaml` looks like this:

```yaml
App:
  Model: potion-base-8M
  Store:
    BlobStorage: LOCAL
```

## Common commands

```bash
make serve        # dev server with hot reload (browser-sync :8080 -> app :3000)
make ui           # rebuild CSS/JS (src/ui: tsc + esbuild + postcss)
make test         # cargo test (requires gen/english_words.txt)
make lint         # cargo clippy --all-targets --all-features && cargo fmt --check
make lint-fix     # auto-fix fmt + clippy
make cf-dev       # run the Worker + container locally via wrangler (Docker required)
make deploy       # wrangler deploy (builds and pushes the image, then deploys the Worker)
```

CI (`.github/workflows/`) runs build, test (after `make models`), fmt and clippy on pushes and PRs to `main`.
Run `make lint` and `make test` before finishing a change.

## Configuration

- CLI: `mismatch [-v] [-c config.yaml] serve [--port 8080]`. Without `-c`, it reads `./config.yaml` if present.
- Env vars override the file and use the `MISMATCH__` prefix with `__` as the separator: `MISMATCH__APP__MODEL=potion-base-8M`,
  `MISMATCH__APP__STORE__BLOB_STORAGE=LOCAL`. If the model is unset, no model is loaded.
- Logs are pretty when stdin is a TTY, JSON otherwise.

## Gotchas

- **Paths are fixed at compile time.** Models, the word list and `public/` are resolved through `env!("CARGO_MANIFEST_DIR")`.
  The runtime Docker image therefore puts these files under `/usr/src/app/mismatch/...`, which is the build-time path. Keep them in sync.
- `BlobStorage` defaults to `S3`, but no S3 code path exists. Always set it to `LOCAL`.
- The Docker image downloads the **linux-x64** ONNX Runtime, and Cloudflare Containers require **linux/amd64** images. On Apple
  Silicon, Docker builds under emulation, which is slow.
- The Dockerfile uses `ENTRYPOINT ["mismatch"]` + `CMD ["serve", "--port", "8080"]`. Port 8080 must match `defaultPort` in
  `cloudflare/src/index.ts`.
- The Rust version is pinned in two places: `mise.toml` and the `rust:<version>-slim-bookworm` image in `Dockerfile`.
  Bump them together.
- Templates are compiled into the binary, so after editing any `.html.j2` you need to rebuild (`make serve` handles this).

## Deployment (Cloudflare Containers)

- `cloudflare/wrangler.jsonc` defines a `MismatchContainer` Durable Object class backed by the container image built from
  `../Dockerfile` with build context `..`. It uses instance type `basic` and `max_instances: 3`.
- `cloudflare/src/index.ts` load-balances each request with `getRandom(env.MISMATCH, 2)`. Containers sleep after 10 minutes idle, so the
  first request after that has a cold start (container boot + model load).
- CI (`.github/workflows/release.yml`) separates build and deploy:
  - `image` builds the Dockerfile on every PR (no push) and every push to `main`, pushing
    `ghcr.io/dskart/mismatch:sha-<commit>` + `latest`. The Docker layer cache is kept in the GitHub Actions cache.
  - `deploy` runs only when release-please creates a release (its release PR is merged), or manually via "Run workflow".
    Cloudflare can't pull from GHCR, so it copies the image into the Cloudflare registry (`wrangler containers push`),
    rewrites `image` in `wrangler.jsonc` to that ref, and runs `wrangler deploy`, with no rebuild.
  - The deploy needs the repo secrets `CLOUDFLARE_API_TOKEN` and `CLOUDFLARE_ACCOUNT_ID`. Merging to `main` without a
    release does not deploy.
  - If you change the `image` line in `wrangler.jsonc`, update the `sed` in the deploy job.
- Local deploys (`make deploy`) need a Workers Paid plan, Docker running locally, and `npx wrangler login`.
- After changing `wrangler.jsonc`, run `npm run cf-typegen` in `cloudflare/`.

## Conventions

- Use Conventional Commits (`feat:`, `fix:`, `chore:` ...). release-please (`release-type: rust`) generates `CHANGELOG.md` and version bumps,
  so don't edit those by hand.
- rustfmt `max_width = 120`.
