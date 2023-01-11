import _ from "lodash";
import React, { useEffect, useState } from "react";
import * as ReactDOM from "react-dom/client";
import Highlighter from "web-highlighter";

import "../feedback.scss";
import FeedbackRenderer from "./renderer";
import SelectionRenderer from "./selection";
import { highlightIsValid, loadHighlights } from "./utils";

type FeedbackMainProps = { highlighter: Highlighter };
let FeedbackMain: React.FC<FeedbackMainProps> = ({ highlighter }) => {
  // initially load all highlights on page from local storage
  let [stored_highlights, set_stored_highlights] = useState(loadHighlights);

  // remove highlights that have been invalidated by content updates
  // (in useEffect since highlights must be rendered to compare DOM hashes)
  useEffect(() => {
    // find highlights to keep and remove
    let [keep, remove] = _.partition(stored_highlights, h => highlightIsValid(highlighter, h));

    remove.forEach(h => highlighter.remove(h.id));
    set_stored_highlights(keep);
  }, []);

  return (
    <>
      {/* render tooltip over existing feedback */}
      <FeedbackRenderer highlighter={highlighter} />

      {/* render tooltip over user's current selection */}
      <SelectionRenderer highlighter={highlighter} stored={stored_highlights} />
    </>
  );
};

let initFeedback = () => {
  let highlighter = new Highlighter();

  let div = document.createElement("div");
  let page = document.querySelector(".page")!;
  page.appendChild(div);
  let root = ReactDOM.createRoot(div);

  root.render(<FeedbackMain highlighter={highlighter} />);
};

initFeedback();
