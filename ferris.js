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

  // Rust Playground IDE button
  const rightButtons = document.querySelector(".right-buttons");
  if (rightButtons) {
    const ideLink = document.createElement("a");
    ideLink.setAttribute("href", "https://play.rust-lang.org/");
    ideLink.setAttribute("target", "_blank");
    ideLink.setAttribute("rel", "noopener noreferrer");
    ideLink.setAttribute("title", "Rust Playground");
    ideLink.setAttribute("aria-label", "Rust Playground");

    // code-editor SVG icon 
    ideLink.innerHTML =
      '<span class="fa-svg">' +
      '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 576 512">' +
      "<!--! Font Awesome Free 6.2.0 by @fontawesome - https://fontawesome.com " +
      "License - https://fontawesome.com/license/free " +
      "(Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) " +
      "Copyright 2022 Fonticons, Inc. -->" +
      '<path d="M9.4 86.6C-3.1 74.1-3.1 53.9 9.4 41.4s32.8-12.5 45.3 0l192 192c12.5 12.5 12.5 32.8 0 45.3l-192 192c-12.5 12.5-32.8 12.5-45.3 0s-12.5-32.8 0-45.3L178.7 256 9.4 86.6zM256 416H544c17.7 0 32 14.3 32 32s-14.3 32-32 32H256c-17.7 0-32-14.3-32-32s14.3-32 32-32z"/>' +
      "</svg>" +
      "</span>";

    rightButtons.appendChild(ideLink);
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

    container.appendChild(createFerris(type, size));
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
