// @ts-check

// Open mdBook's built-in search with Cmd+K (macOS) or Ctrl+K (everywhere else).
// mdBook's own key handler ignores modified keys, so this doesn't clash with
// its `S` / `/` shortcuts.

const IS_MAC = /Mac|iPhone|iPad/.test(navigator.platform);
const SHORTCUT_LABEL = IS_MAC ? "⌘K" : "Ctrl+K";

document.addEventListener(
  "keydown",
  (event) => {
    if (event.key.toLowerCase() !== "k" || event.altKey || event.shiftKey) {
      return;
    }
    if (!(IS_MAC ? event.metaKey : event.ctrlKey)) {
      return;
    }

    let wrapper = document.getElementById("mdbook-search-wrapper");
    let toggle = document.getElementById("mdbook-search-toggle");
    let searchbar = /** @type {HTMLInputElement | null} */ (
      document.getElementById("mdbook-searchbar")
    );
    if (!wrapper || !toggle || !searchbar) {
      return;
    }

    // Stop the browser's own Cmd/Ctrl+K (focus the address bar, etc.).
    event.preventDefault();

    if (wrapper.classList.contains("hidden")) {
      // The toggle's click handler opens the bar, loads the index, and
      // selects the input.
      toggle.click();
    } else {
      window.scrollTo(0, 0);
      searchbar.select();
    }
  },
  true,
);

document.addEventListener("DOMContentLoaded", () => {
  let searchbar = document.getElementById("mdbook-searchbar");
  if (searchbar) {
    searchbar.setAttribute(
      "placeholder",
      `Search this book ... (${SHORTCUT_LABEL})`,
    );
  }

  let toggle = document.getElementById("mdbook-search-toggle");
  if (toggle) {
    toggle.setAttribute("title", `Search (${SHORTCUT_LABEL})`);
    let keys = toggle.getAttribute("aria-keyshortcuts") || "";
    toggle.setAttribute(
      "aria-keyshortcuts",
      `${keys} ${IS_MAC ? "Meta+K" : "Control+K"}`.trim(),
    );
  }
});
