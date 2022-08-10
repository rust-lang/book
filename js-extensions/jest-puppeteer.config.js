module.exports = {
  server: {
    command: "cd .. && mdbook serve",
    host: process.env.CI ? "[::1]" : "localhost",
    port: 3000,
    path: "/",
    launchTimeout: 20000,
  },
};
