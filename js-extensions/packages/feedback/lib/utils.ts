import md5 from "md5";
import Highlighter from "web-highlighter";
import HighlightSource from "web-highlighter/dist/model/source";

export const HIGHLIGHT_STORAGE_KEY = "mdbook-quiz:highlights";

/** Stored in the `extra` field of a `HighlightSource` as a string */
export type HighlightExtra = {
  /** Feedback submitted by user */
  text: string;
  /** Path to page on which feedback was given */
  page: string;
  /** Hash of highlighted DOM node (used to invalidate outdated feedback) */
  domHash?: string;
};

export enum HighlightTelemetryAction {
  create = "create",
  remove = "remove",
}

/** Add md5 hash of DOM node surrounding a highlight */
export const addDOMHash = (
  highlighter: Highlighter,
  highlight: HighlightSource
): HighlightSource => {
  // compute hash of DOM node innerHTML
  let dom_nodes = highlighter.getDoms(highlight.id);
  let domHash = md5(dom_nodes[0].innerHTML);

  // add `domHash` to highlight metadata
  let extra = JSON.parse((highlight.extra as string) || "{}");
  highlight.extra = JSON.stringify({ ...extra, domHash });

  return highlight;
};

/** Load all highlights on current page from local storage */
export const loadHighlights = (): HighlightSource[] => {
  let stored = localStorage.getItem(HIGHLIGHT_STORAGE_KEY);
  let parsed: HighlightSource[] = JSON.parse(stored || "[]");

  return parsed.filter(h => {
    // parse highlight's metadata
    let extra: HighlightExtra = JSON.parse(h.extra as string);

    // check highlight is on current page
    return extra.page === window.location.pathname;
  });
};

/** Ensure highlight hasn't been invalidated by content changes */
export const highlightIsValid = (highlighter: Highlighter, highlight: HighlightSource) => {
  // parse highlight's metadata
  let extra: HighlightExtra = JSON.parse(highlight.extra as string);

  // if highlight has no DOM hash, don't invalidate
  if (!extra.domHash) return true;

  // get highlight's enclosing DOM node
  let dom_nodes = highlighter.getDoms(highlight.id);

  // if highlight has no DOM node, filter
  if (!dom_nodes) return false;

  // check highlight's DOM node content hasn't changed
  let domHash = md5(dom_nodes[0].innerHTML);
  return domHash === extra.domHash;
};
