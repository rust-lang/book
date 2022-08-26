import { cli, copyPlugin, peerfixPlugin } from "@nota-lang/esbuild-utils";
import { sassPlugin } from "esbuild-sass-plugin";
import fs from "fs";
import * as url from 'url';

const __dirname = url.fileURLToPath(new URL('.', import.meta.url));
fs.mkdirSync("dist", {recursive: true});
try {
    fs.symlinkSync(__dirname + "/" + "data", "dist/data")
} catch(e) {
  // pass
}

let build = cli();
build({
  format: "iife",
  plugins: [
    copyPlugin({ extensions: [".html"] }),
    sassPlugin(),
    peerfixPlugin({
      modules: ["react", "react-dom", "mobx", "mobx-react"],
      meta: import.meta,
    }),
  ],
});
