module.exports = {
  server: {
    command: "cd .. && QUIZ_LIGHTWEIGHT_VALIDATE= mdbook serve -n 0.0.0.0",
    host: "0.0.0.0",
    port: 3000,
    launchTimeout: 35000,
  },
};
