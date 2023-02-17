module.exports = {
  server: {
    command: "cd .. && QUIZ_LIGHTWEIGHT_VALIDATE= mdbook build && cd book && python3 -m http.server 3000",
    host: "0.0.0.0",
    port: 3000,
    launchTimeout: 35000,
  },
};
