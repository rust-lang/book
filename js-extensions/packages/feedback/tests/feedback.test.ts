import "expect-puppeteer";
import { BoundingBox } from "puppeteer";

const EXAMPLE_TEXT = "Try highlighting this text!";
const EXAMPLE_FEEDBACK = "my feedback...";

const TEST_SELECTORS = {
  tooltip: ".pop",
  modal: ".ReactModal__Overlay",
  modalTextarea: ".ReactModal__Overlay textarea",
  modalButton: ".ReactModal__Overlay button",
  removeButton: ".pop-button",
};

describe("feedback", () => {
  // get all web-highlighter elements
  const getHighlights = () => page.$$(".highlight-mengshou-wrap");

  // get bounding box of element containing `text`
  const getExampleBB = async (text: string) => {
    const xpath = `//*[contains(text(), '${text}')]`;
    const el = await page.waitForXPath(xpath);
    const box = await el.boundingBox();
    return box!;
  };

  // select text within bounding box with mouse
  const selectBB = async (box: BoundingBox) => {
    const { x, y, width, height } = box;
    await page.mouse.move(x, y);
    await page.mouse.down();
    await page.mouse.move(x + width, y + height);
    await page.mouse.up();
  };

  beforeAll(async () => {
    await page.goto("http://localhost:3000");

    // avoid consent page
    await page.evaluate(() => {
      localStorage.setItem("__wcrichto_consent", "YES");
    });
  });

  beforeEach(async () => {
    await page.goto("http://localhost:3000");
  });

  it("defaults to no highlights", async () => {
    await expect(page).toMatch("Experiment Introduction");

    let highlights = await getHighlights();
    expect(highlights.length).toBe(0);
  });

  it("opens tooltip on text highlight", async () => {
    const rect = await getExampleBB(EXAMPLE_TEXT);
    await selectBB(rect);

    const tooltip = await page.$(TEST_SELECTORS.tooltip);
    await expect(tooltip).not.toBeNull();
  });

  it("opens modal when tooltip clicked", async () => {
    const rect = await getExampleBB(EXAMPLE_TEXT);
    await selectBB(rect);

    const tooltip = await page.$(TEST_SELECTORS.tooltip);
    await tooltip?.click();

    const modal = await page.$(TEST_SELECTORS.modal);
    expect(modal).not.toBeNull();
  });

  it("stores and displays highlight when created", async () => {
    const rect = await getExampleBB(EXAMPLE_TEXT);
    await selectBB(rect);

    const tooltip = await page.$(TEST_SELECTORS.tooltip);
    await tooltip?.click();

    const textarea = await page.$(TEST_SELECTORS.modalTextarea);
    await textarea?.type(EXAMPLE_FEEDBACK);

    const button = await page.$(TEST_SELECTORS.modalButton);
    await button?.click();

    let highlights = await getHighlights();
    expect(highlights.length).toEqual(1);
  });

  // it("removes highlight when deleted", async () => {
  //   let highlights = await getHighlights();
  //   expect(highlights.length).toEqual(1);

  //   await highlights[0].hover();
  //   expect(page).toMatch(EXAMPLE_FEEDBACK);

  //   let remove = await page.$(TEST_SELECTORS.removeButton);
  //   await remove?.click();

  //   highlights = await getHighlights();
  //   expect(highlights.length).toEqual(0);

  //   await page.reload();

  //   highlights = await getHighlights();
  //   expect(highlights.length).toEqual(0);
  // });
});
