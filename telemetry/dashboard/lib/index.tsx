import React from "react";
import ReactDOM from "react-dom/client";

let App = () => {
  return <div>
    Hello world
  </div>;
}

let root = ReactDOM.createRoot(document.getElementById('root'));
root.render(<App />);