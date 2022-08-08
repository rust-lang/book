import { VirtualElement } from "@popperjs/core";
import React, { useCallback, useEffect, useState } from "react";
import Highlighter from "web-highlighter";

import FeedbackModal from "./modal";
import FeedbackTooltip from "./tooltip";
import { HIGHLIGHT_STORAGE_KEY } from "./utils";

type SelectionRendererProps = { highlighter: Highlighter; stored?: any[] };
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
    let range =
      selection && !selection.isCollapsed && selection.rangeCount && selection.getRangeAt(0);

    setCurrRange(range || null);
  }, []);

  useEffect(() => {
    // load highlights from local storage
    stored?.map(s => highlighter.fromStore(s.startMeta, s.endMeta, s.text, s.id, s.extra));

    // store new highlights in localStorage when created
    highlighter.on(Highlighter.event.CREATE, ({ sources }) => {
      let stored_str = localStorage.getItem(HIGHLIGHT_STORAGE_KEY);
      let stored_highlights = JSON.parse(stored_str || "[]");
      stored_highlights.push(...sources);

      localStorage.setItem(HIGHLIGHT_STORAGE_KEY, JSON.stringify(stored_highlights));
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

      return (
        <FeedbackTooltip reference={reference}>
          <div
            className="pop-button"
            onClick={() => setModalOpen(true)}
            title="Provide feedback on this content"
          >
            &#10068;
          </div>
        </FeedbackTooltip>
      );
    }
  }

  return null;
};

export default SelectionRenderer;
