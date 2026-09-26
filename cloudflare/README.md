# mismatch Cloudflare Worker

Thin Worker that forwards every request to the mismatch Rust server running in a
[Cloudflare Container](https://developers.cloudflare.com/containers/). The image is
built from `../Dockerfile` by `wrangler deploy`.

```bash
npm install
npx wrangler login   # once
npm run dev          # local Worker + container (needs Docker running)
npm run deploy       # build image (linux/amd64), push, deploy
npm run tail         # stream logs
```
