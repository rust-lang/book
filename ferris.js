// @ts-check

/**
 * @typedef {{ attr: string, title: string }} FerrisType
 */

/** @type {Array<FerrisType>} */
const FERRIS_TYPES = [
  {
    attr: "does_not_compile",
    title: "This code does not compile!",
  },
  {
    attr: "panics",
    title: "This code panics!",
  },
  {
    attr: "not_desired_behavior",
    title: "This code does not produce the desired behavior.",
  },
];

document.addEventListener("DOMContentLoaded", () => {
  for (let ferrisType of FERRIS_TYPES) {
    attachFerrises(ferrisType);
  }
});

/**
 * @param {FerrisType} type
 */
function attachFerrises(type) {
  let elements = document.getElementsByClassName(type.attr);

  for (let codeBlock of elements) {
    // Skip SVG etc.: in principle, these should never be attached to those, but
    // this means if someone happens to have a browser extension which *is*
    // attaching them, it will not break the code.
    if (!(codeBlock instanceof HTMLElement)) {
      continue;
    }

    let codeLines = codeBlock.innerText;
    let extra = codeLines.endsWith("\n") ? 1 : 0;
    let numLines = codeLines.split("\n").length - extra;

    /** @type {'small' | 'large'} */
    let size = numLines < 4 ? "small" : "large";

    let container = prepareFerrisContainer(codeBlock, size == "small");
    if (!container) {
      continue;
    }

    let ferris = createFerris(type, size)
    container.appendChild(ferris);
    giveFerrisSpace(codeBlock, ferris, size);
  }
}

/**
 * @param {HTMLElement} element - Code block element to attach a Ferris to.
 * @param {boolean} useButtons - Whether to attach to existing buttons.
 * @returns {Element | null} - The container element to use.
 */
function prepareFerrisContainer(element, useButtons) {
  let foundButtons = element.parentElement?.querySelector(".buttons");
  if (useButtons && foundButtons) {
    return foundButtons;
  }

  let div = document.createElement("div");
  div.classList.add("ferris-container");

  if (!element.parentElement) {
    console.error(`Could not install Ferris on ${element}, which is missing a parent`);
    return null;
  }

  element.parentElement.insertBefore(div, element);

  return div;
}

/**
 * @param {FerrisType} type
 * @param {'small' | 'large'} size
 * @returns {HTMLAnchorElement} - The generated anchor element.
 */
function createFerris(type, size) {
  let a = document.createElement("a");
  a.setAttribute("href", "ch00-00-introduction.html#ferris");
  a.setAttribute("target", "_blank");

  let img = document.createElement("img");
  img.setAttribute("src", "img/ferris/" + type.attr + ".svg");
  img.setAttribute("title", type.title);
  img.classList.add("ferris");
  img.classList.add("ferris-" + size);

  a.appendChild(img);

  return a;
}

/**
 * Put each line ending in a span. For each of those spans,
 * if Ferris might hide it, give it a safety buffer.
 * @param {HTMLElement} codeBlock
 * @param {HTMLAnchorElement} ferris
 * @param {'small' | 'large'} size
 */
function giveFerrisSpace(codeBlock, ferris, size) {
  // sanity checking + lint awareness
  const ferrisImage = ferris.firstChild;
  if (!(ferrisImage instanceof HTMLImageElement)) {
    console.error("ferris should be <a> containing <img>", ferris);
    return;
  }

  /** @type {HTMLSpanElement[]} */
  const lineEndings = [];  // line endings which might be hidden by Ferris

  const walker = document.createTreeWalker(codeBlock, NodeFilter.SHOW_TEXT);
  const re = /^(.*?)\n(.*)$/s

  while (walker.nextNode()) {
    const current = walker.currentNode;
    const parent = current.parentNode;

    // sanity checking + lint awareness
    if (!(current instanceof Text) || !parent) {
      continue;
    }

    let re_results;
    while (re_results = current.textContent.match(re)) {
      // text node contains newline
      const [_, beforeNewline, afterNewline] = re_results;

      // line ending gets a span
      const lineEnd = document.createElement("span");
      lineEnd.textContent = beforeNewline;
      lineEndings.push(lineEnd);
      parent.insertBefore(lineEnd, current);

      // newline now stands alone
      parent.insertBefore(document.createTextNode("\n"), current);

      // rest of the text
      current.textContent = afterNewline;
      // current might still contain newlines, so we go again until it doesn't
    }
  }

  codeBlock.normalize(); // not strictly necessary, but good practice to leave the DOM normalized

  // setTimeout so getBoundingClientRect returns valid results
  setTimeout(() => {
    const f = ferrisImage.getBoundingClientRect();
    lineEndings.forEach((s) => {
      const {bottom, top} = s.getBoundingClientRect();
      if ( // vertical overlap between ferris and span
        (bottom >= f.top && bottom <= f.bottom)
        || (top >= f.top && top <= f.bottom)
        || (f.top >= top && f.top <= bottom)
      ) {
        // buffer needed!
        s.classList.add("ferris-buffer-" + size);
      }
    });
  });
}
