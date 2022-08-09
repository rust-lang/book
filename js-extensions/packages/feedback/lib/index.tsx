import React from "react";
import * as ReactDOM from "react-dom/client";
import Highlighter from "web-highlighter";
import HighlightSource from "web-highlighter/dist/model/source";

import "../feedback.scss";
import FeedbackRenderer from "./renderer";
import SelectionRenderer from "./selection";
import { HIGHLIGHT_STORAGE_KEY, HighlightExtra } from "./utils";

let initFeedback = () => {
  let highlighter = new Highlighter();

  // fetch stored highlights for current page from local storage
  let stored = localStorage.getItem(HIGHLIGHT_STORAGE_KEY);
  let stored_parsed: HighlightSource[] = JSON.parse(stored || "[]");
  stored_parsed = stored_parsed.filter(h => {
    let extra: HighlightExtra = JSON.parse(h.extra as string);
    return extra.page === window.location.pathname;
  });

  let div = document.createElement("div");
  let page = document.querySelector(".page")!;
  page.appendChild(div);
  let root = ReactDOM.createRoot(div);

  root.render(
    <>
      {/* render tooltip over existing feedback */}
      <FeedbackRenderer highlighter={highlighter} />

      {/* render tooltip over user's current selection */}
      <SelectionRenderer highlighter={highlighter} stored={stored_parsed} />
    </>
  );
};

initFeedback();
