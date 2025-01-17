import * as esbuild from "esbuild";

const scriptConfig = {
  entryPoints: ["pages/components/scripts.ts"],
  bundle: true,
  minify: false,
  globalName: "scripts",
  target: ["chrome58", "firefox57", "safari11", "edge16"],
};

await esbuild.build({
  ...scriptConfig,
  outfile: "public/static/scripts.js",
});
