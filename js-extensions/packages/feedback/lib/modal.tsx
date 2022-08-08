import React, { useRef, useState } from "react";
import Modal from "react-modal";
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
  const [feedbackEmpty, setFeedbackEmpty] = useState(false);

  const handleSubmit = () => {
    // prevent empty feedback submissions
    if (!feedback.current!.value.trim()) {
      return setFeedbackEmpty(true);
    }

    // add feedback to serialized highlighter data (dispose hook after use)
    let dispose = highlighter.hooks.Serialize.RecordInfo.tap(() => feedback.current!.value);
    highlighter.fromRange(range);
    dispose();

    closeModal();
  };

  return (
    <Modal style={modalStyles} contentLabel="Feedback Modal" onRequestClose={closeModal} isOpen>
      <textarea
        ref={feedback}
        style={{ minWidth: "250px" }}
        rows={4}
        placeholder="Your feedback..."
        required
      ></textarea>
      {feedbackEmpty && <div className="modal-textarea-helper">Feedback is required</div>}
      <br />
      <button onClick={handleSubmit}>Submit</button>
    </Modal>
  );
};

export default FeedbackModal;
