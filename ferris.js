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
