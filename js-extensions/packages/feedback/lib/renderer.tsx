import { VirtualElement } from "@popperjs/core";
import React from "react";
import { useEffect, useState } from "react";
import Highlighter from "web-highlighter";
import HighlightSource from "web-highlighter/dist/model/source";
import type {} from "telemetry";

import FeedbackTooltip from "./tooltip";
import { HIGHLIGHT_STORAGE_KEY, HighlightTelemetryAction } from "./utils";

type FeedbackRendererProps = { highlighter: Highlighter };
const FeedbackRenderer: React.FC<FeedbackRendererProps> = ({ highlighter }) => {
  // id of feedback highlight and tooltip currently hovered over
  const [highlightHovered, setHighlightHovered] = useState<string | null>(null);
  const [tooltipHovered, setTooltipHovered] = useState<string | null>(null);

  useEffect(() => {
    highlighter.on(Highlighter.event.REMOVE, ({ ids }) => {
      // remove highlights from local storage when deleted
      let stored_str = localStorage.getItem(HIGHLIGHT_STORAGE_KEY);
      let stored_highlights = JSON.parse(stored_str || "[]") as HighlightSource[];
      let filtered_highlights = stored_highlights.filter(hl => !ids.includes(hl.id));

      localStorage.setItem(HIGHLIGHT_STORAGE_KEY, JSON.stringify(filtered_highlights));

      // log removed highlights to telemetry server
      ids.forEach(id => {
        let data = { action: HighlightTelemetryAction.remove, data: id };
        window.telemetry.log("feedback", data);
      });
    });

    // update state on hover changes
    highlighter.on(Highlighter.event.HOVER, ({ id }) => setHighlightHovered(id));
    highlighter.on(Highlighter.event.HOVER_OUT, () => setHighlightHovered(null));
  }, []);

  const removeFeedback = () => {
    // cursor must be over tooltip when deleting, so use tooltip ID
    highlighter.remove(tooltipHovered!);
    setTooltipHovered(null);

    // on mobile, the highlight isn't unhovered, so also reset
    setHighlightHovered(null);
  };

  // If hovering over existing highlight or tooltip, render tooltip
  let id = highlightHovered || tooltipHovered;
  if (id) {
    let el = highlighter.getDoms(id);
    let extra = highlighter.cache.get(id).extra as string;
    let feedback = JSON.parse(extra || "{}").text;

    const reference: VirtualElement = {
      getBoundingClientRect: el[0].getBoundingClientRect.bind(el[0]),
    };

    return (
      <FeedbackTooltip
        reference={reference}
        onHoverChange={isHovered => setTooltipHovered(isHovered ? highlightHovered : null)}
        placement="top"
      >
        <div className="pop-feedback-container">
          {feedback != "" ? <div className="pop-feedback-text">{feedback}</div> : null}
          <div
            className="pop-button"
            onClick={removeFeedback}
            onTouchStart={removeFeedback}
            title="Delete feedback"
          >
            &#128465;
          </div>
        </div>
      </FeedbackTooltip>
    );
  }

  return null;
};

export default FeedbackRenderer;
