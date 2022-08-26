import { cli, copyPlugin } from "@nota-lang/esbuild-utils";
import { sassPlugin } from "esbuild-sass-plugin";
import fs from "fs";

let questionSummary = fs.readFileSync("data/question-summary.json", "utf-8");
let quizSummary = fs.readFileSync("data/quiz-summary.json", "utf-8");

let build = cli();
build({
  format: "iife",
  define: {
    QUESTION_SUMMARY: JSON.stringify(questionSummary),
    QUIZ_SUMMARY: JSON.stringify(quizSummary),
  },
  plugins: [copyPlugin({ extensions: [".html"] }), sassPlugin()],
});
