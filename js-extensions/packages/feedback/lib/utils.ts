export const HIGHLIGHT_STORAGE_KEY = "mdbook-quiz:highlights";

/** Stored in the `extra` field of a `HighlightSource` as a string */
export type HighlightExtra = {
  /** Feedback submitted by user */
  text: string;
  /** Path to page on which feedback was given */
  page: string;
};
