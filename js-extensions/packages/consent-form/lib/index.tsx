import React, { useRef } from "react";
import * as ReactDOM from "react-dom/client";

import "../consent.scss";

const CONSENT_KEY = "__wcrichto_consent";

let ConsentForm = () => {
  let ref = useRef<HTMLDivElement>(null);
  return (
    <div className="consent-form" ref={ref}>
      <div className="container">
        <h1>Rust Book experiment consent form</h1>
        <p>
          This website is an experiment by Brown University researchers{" "}
          <a href="https://willcrichton.net/">Will Crichton</a> and{" "}
          <a href="https://cs.brown.edu/~sk/">Shriram Krishnamurthi</a> that adds new kinds of
          content to Rust's language tutorial (i.e. <q>The Book</q> i.e.{" "}
          <a href="https://doc.rust-lang.org/book/">The Rust Programming Language</a>
          ). Our goal is to make it easier for developers to learn Rust.
        </p>
        <p>
          To facilitate our research, this website will gather anonymized data about your
          interactions with the content. For example, if you take a quiz, then your answers will be
          sent to our servers. By clicking <q>I consent</q> below, you are consenting to us
          gathering such data.
        </p>
        <p>Thank you for your participation in making Rust better for everyone!</p>
        <button
          onClick={() => {
            localStorage.setItem(CONSENT_KEY, "YES");
            ref.current!.style.display = "none";
          }}
        >
          I consent
        </button>
      </div>
    </div>
  );
};

if (localStorage.getItem(CONSENT_KEY) === null) {
  let el = document.createElement("div");
  document.body.appendChild(el);
  let root = ReactDOM.createRoot(el);
  root.render(<ConsentForm />);
}
