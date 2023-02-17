import { VirtualElement } from "@popperjs/core";
import React, { useCallback, useEffect, useLayoutEffect, useState } from "react";
import type {} from "telemetry";
import Highlighter from "web-highlighter";
import HighlightSource from "web-highlighter/dist/model/source";

import FeedbackModal from "./modal";
import FeedbackTooltip from "./tooltip";
import { HIGHLIGHT_STORAGE_KEY, HighlightTelemetryAction, addDOMHash } from "./utils";

// TODO: this is duplicated with mdbook-quiz AND aquascope-embed!
let useCaptureMdbookShortcuts = (capture: boolean) => {
  useLayoutEffect(() => {
    if (capture) {
      let captureKeyboard = (e: KeyboardEvent) => e.stopPropagation();

      // This gets added specifically to document.documentElement rather than document
      // so bubbling events will hit this listener before ones added via document.addEventListener(...).
      // All of the problematic mdBook interactions are created that way, so we ensure that
      // the keyboard event does not propagate to those listeners.
      //
      // However, some widgets like Codemirror require keydown events but on local elements.
      // So we can't just stopPropagation in the capture phase, or those widgets will break.
      // This is the compromise!
      document.documentElement.addEventListener(
        "keydown",
        captureKeyboard,
        false
      );

      return () =>
        document.documentElement.removeEventListener(
          "keydown",
          captureKeyboard,
          false
        );
    }
  }, [capture]);
};

type SelectionRendererProps = { highlighter: Highlighter; stored?: HighlightSource[] };
let SelectionRenderer: React.FC<SelectionRendererProps> = ({ highlighter, stored }) => {
  // current highlighted range of text
  const [currRange, setCurrRange] = useState<Range | null>(null);

  // whether feedback modal is open
  const [modalOpen, setModalOpen] = useState(false);

  // update selected range when selection changes
  // wrapped in useCallback to retain a stable reference to the callback
  // so addEventListener and removeEventListener use the same function
  const handleSelection = useCallback(() => {
    // get current selection (falsy value if no selection)
    let selection = document.getSelection();
    if (!selection) return;

    let anchor = selection.anchorNode;
    if (!anchor) return;

    let parentElement = anchor;
    while (!(parentElement instanceof Element)) {
      if (!parentElement.parentNode) return;
      parentElement = parentElement.parentNode;
    }

    if (!parentElement.closest(".content") || parentElement.closest(".aquascope, .mdbook-quiz"))
      return;

    let range = !selection.isCollapsed && selection.rangeCount && selection.getRangeAt(0);

    setCurrRange(range || null);
  }, []);

  useEffect(() => {
    // load highlights from local storage
    stored?.map(s => highlighter.fromStore(s.startMeta, s.endMeta, s.text, s.id, s.extra));

    highlighter.on(Highlighter.event.CREATE, ({ sources }) => {
      // add DOM node hash to each source's metadata
      sources = sources.map(h => addDOMHash(highlighter, h));

      // store new highlights in localStorage when created
      let stored_str = localStorage.getItem(HIGHLIGHT_STORAGE_KEY);
      let stored_highlights = JSON.parse(stored_str || "[]");
      stored_highlights.push(...sources);

      localStorage.setItem(HIGHLIGHT_STORAGE_KEY, JSON.stringify(stored_highlights));

      // log new highlights to telemetry server
      sources.forEach(src => {
        // `extra` contains JSON about the highlight but is stringified (need to parse)
        let parsed_src = { ...src, extra: JSON.parse(src.extra as string) };
        let data = { action: HighlightTelemetryAction.create, data: parsed_src };

        window.telemetry?.log("feedback", data);
      });
    });
  }, []);

  useEffect(() => {
    // handle selection events only when modal closed
    if (!modalOpen) {
      document.addEventListener("selectionchange", handleSelection);
    } else {
      document.removeEventListener("selectionchange", handleSelection);
    }
  }, [modalOpen]);

  // Remove modal and tooltip when closing modal
  const handleCloseModal = () => {
    setModalOpen(false);
    setCurrRange(null);
  };

  useCaptureMdbookShortcuts(modalOpen);

  if (currRange) {
    if (modalOpen) {
      // If tooltip feedback icon pressed, render modal
      return (
        <FeedbackModal range={currRange} highlighter={highlighter} closeModal={handleCloseModal} />
      );
    } else {
      // If modal not open, show tooltip over selected text
      const reference: VirtualElement = {
        getBoundingClientRect: currRange.getBoundingClientRect.bind(currRange),
      };

      let isMobile = window.innerWidth < 800;
      let placement: "top" | "bottom" = isMobile ? "bottom" : "top";

      return (
        <FeedbackTooltip reference={reference} placement={placement}>
          <div
            className="pop-button"
            onClick={() => setModalOpen(true)}
            onTouchStart={() => setModalOpen(true)}
            title="Provide feedback on this content"
          >
            ✏️
          </div>
        </FeedbackTooltip>
      );
    }
  }

  return null;
};

export default SelectionRenderer;
