import { cli } from "@nota-lang/esbuild-utils";
import sassPlugin from "esbuild-plugin-sass";

let build = cli();
build({
  format: "iife",
  plugins: [sassPlugin()],
});
