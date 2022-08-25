import {cli} from "@nota-lang/esbuild-utils"

let build = cli();
build({
  format: "iife"
})