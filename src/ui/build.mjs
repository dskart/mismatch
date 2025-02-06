import * as esbuild from "esbuild";

const baseConfig = {
  bundle: true,
  minify: false,
  globalName: "scripts",
  target: ["chrome58", "firefox57", "safari11", "edge16"],
};

const builds = [
  {
    entryPoints: ["pages/components/scripts.ts"],
    outfile: "public/static/scripts.js",
  },
  {
    entryPoints: ["pages/daily/components/scripts.ts"],
    outfile: "public/static/daily/scripts.js",
  },
];

await Promise.all(
  builds.map((buildConfig) =>
    esbuild.build({
      ...baseConfig,
      ...buildConfig,
    })
  )
);
