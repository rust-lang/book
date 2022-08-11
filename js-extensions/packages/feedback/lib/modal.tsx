import React, { useRef } from "react";
import Modal from "react-modal";
import type {} from "telemetry";
import Highlighter from "web-highlighter";

Modal.setAppElement("body");

const modalStyles = {
  content: {
    top: "50%",
    left: "50%",
    right: "auto",
    bottom: "auto",
    marginRight: "-50%",
    transform: "translate(-50%, -50%)",
  },
};

type FeedbackModalProps = {
  range: Range;
  highlighter: Highlighter;
  closeModal: () => void;
};
const FeedbackModal: React.FC<FeedbackModalProps> = ({ range, highlighter, closeModal }) => {
  const feedback = useRef<HTMLTextAreaElement>(null);

  const handleSubmit = () => {
    // add feedback to serialized highlighter data (dispose hook after use)
    let data = {
      text: feedback.current!.value,
      page: window.location.pathname,
    };
    window.telemetry.log("feedback", data);

    let dispose = highlighter.hooks.Serialize.RecordInfo.tap(() => JSON.stringify(data));
    highlighter.fromRange(range);
    dispose();

    closeModal();
  };

  return (
    <Modal style={modalStyles} contentLabel="Feedback Modal" onRequestClose={closeModal} isOpen>
      <textarea autoFocus ref={feedback} rows={4} placeholder="Your note..." required></textarea>
      <br />
      <button onClick={handleSubmit}>Submit</button>
    </Modal>
  );
};

export default FeedbackModal;
