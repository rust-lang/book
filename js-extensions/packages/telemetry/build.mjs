import { cli } from "@nota-lang/esbuild-utils";
import * as cp from "child_process";

let commitHash = cp.execSync("git rev-parse HEAD").toString("utf-8").trim();
let branch = cp.execSync("git rev-parse --abbrev-ref HEAD").toString("utf-8").trim();
let build = cli();
build({
  format: "iife",
  define: {
    COMMIT_HASH: JSON.stringify(commitHash),
    BRANCH: JSON.stringify(branch),
    TELEMETRY_URL: JSON.stringify("https://mindover.computer"),
  },
});
