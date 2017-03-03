// Checks cache first for page, then updates from the server and refreshes.
//
// See https://serviceworke.rs/strategy-cache-update-and-refresh_service-worker_doc.html

var CACHE = 'rust-lang/book';

var REQUIRED_FILES = [
  'book.css',
  'book.js',
  'favicon.png',
  'highlight.css',
  'highlight.js',
  'index.html',
  'jquery.js',
  'tomorrow-night.css',
  '_FontAwesome/css/font-awesome.css',
  '_FontAwesome/fonts/FontAwesome.ttf',
  '_FontAwesome/fonts/fontawesome-webfont.eot',
  '_FontAwesome/fonts/fontawesome-webfont.svg',
  '_FontAwesome/fonts/fontawesome-webfont.ttf',
  '_FontAwesome/fonts/fontawesome-webfont.woff',
  '_FontAwesome/fonts/fontawesome-webfont.woff2',
  'img/trpl04-01.svg',
  'img/trpl04-02.svg',
  'img/trpl04-03.svg',
  'img/trpl04-04.svg',
  'img/trpl04-05.svg',
  'img/trpl04-06.svg',
  'appendix-00.html',
  'appendix-01-keywords.html',
  'appendix-02-operators.html',
  'appendix-03-derivable-traits.html',
  'appendix-04-nightly-rust.html',
  'appendix-05-macros.html',
  'appendix-06-translation.html',
  'ch01-00-introduction.html',
  'ch01-01-installation.html',
  'ch01-02-hello-world.html',
  'ch02-00-guessing-game-tutorial.html',
  'ch03-00-common-programming-concepts.html',
  'ch03-01-variables-and-mutability.html',
  'ch03-02-data-types.html',
  'ch03-03-how-functions-work.html',
  'ch03-04-comments.html',
  'ch03-05-control-flow.html',
  'ch04-00-understanding-ownership.html',
  'ch04-01-what-is-ownership.html',
  'ch04-02-references-and-borrowing.html',
  'ch04-03-slices.html',
  'ch05-00-structs.html',
  'ch05-01-method-syntax.html',
  'ch06-00-enums.html',
  'ch06-01-defining-an-enum.html',
  'ch06-02-match.html',
  'ch06-03-if-let.html',
  'ch07-00-modules.html',
  'ch07-01-mod-and-the-filesystem.html',
  'ch07-02-controlling-visibility-with-pub.html',
  'ch07-03-importing-names-with-use.html',
  'ch08-00-common-collections.html',
  'ch08-01-vectors.html',
  'ch08-02-strings.html',
  'ch08-03-hash-maps.html',
  'ch09-00-error-handling.html',
  'ch09-01-unrecoverable-errors-with-panic.html',
  'ch09-02-recoverable-errors-with-result.html',
  'ch09-03-to-panic-or-not-to-panic.html',
  'ch10-00-generics.html',
  'ch10-01-syntax.html',
  'ch10-02-traits.html',
  'ch10-03-lifetime-syntax.html',
  'ch11-00-testing.html',
  'ch11-01-writing-tests.html',
  'ch11-02-running-tests.html',
  'ch11-03-test-organization.html',
  'ch12-00-an-io-project.html',
  'ch12-01-accepting-command-line-arguments.html',
  'ch12-02-reading-a-file.html',
  'ch12-03-improving-error-handling-and-modularity.html',
  'ch12-04-testing-the-librarys-functionality.html',
  'ch12-05-working-with-environment-variables.html',
  'ch12-06-writing-to-stderr-instead-of-stdout.html',
  'ch13-00-functional-features.html',
  'ch13-01-closures.html',
  'ch13-02-iterators.html',
  'ch13-03-improving-our-io-project.html',
  'ch13-04-performance.html',
  'ch14-00-more-about-cargo.html',
  'ch14-01-release-profiles.html',
  'ch14-02-publishing-to-crates-io.html',
  'ch14-03-cargo-workspaces.html',
  'ch14-04-installing-binaries.html',
  'ch14-05-extending-cargo.html',
  'ch15-00-smart-pointers.html',
  'ch16-00-concurrency.html',
  'ch17-00-oop.html',
  'ch18-00-patterns.html',
  'ch19-00-advanced-features.html',
  'ch19-01-unsafe-rust.html',
  'ch19-02-advanced-lifetimes.html',
  'ch19-03-advanced-traits.html',
  'ch20-00-unnamed-project.html'
];

self.addEventListener('install', function (evt) {
  evt.waitUntil(caches.open(CACHE).then(function (cache) {
    cache.addAll(REQUIRED_FILES);
  }));
});

self.addEventListener('fetch', function (evt) {
  evt.respondWith(fromCache(evt.request));
  evt.waitUntil(update(evt.request).then(refresh));
});

function fromCache(request) {
  return caches.open(CACHE).then(function (cache) {
    return cache.match(request);
  });
}

function update(request) {
  return caches.open(CACHE).then(function (cache) {
    return fetch(request).then(function (response) {
      return cache.put(request, response.clone()).then(function () {
        return response;
      });
    });
  });
}

function refresh(response) {
  return self.clients.matchAll().then(function (clients) {
    clients.forEach(function (client) {
      var message = {
        type: 'refresh',
        url: response.url,
        eTag: response.headers.get('Expires')
      };

      client.postMessage(JSON.stringify(message));
    });
  });
}
