import { test, expect } from "@playwright/test";

test("large decks/hands keep topbar + play button accessible", async ({ page }, testInfo) => {
  await page.goto("/?prefill=28", { waitUntil: "domcontentloaded" });

  const deck = page.getByTestId("deck-zone");
  const hand = page.getByTestId("hand-zone");
  const play = page.getByTestId("play");

  // With prefill, deck should be large enough to overflow horizontally.
  await expect
    .poll(async () => deck.locator(".card").count(), { timeout: 15000 })
    .toBeGreaterThanOrEqual(20);

  // Move a bunch of cards into hand.
  for (let i = 0; i < 12; i++) {
    const c = deck.locator(".card").nth(i);
    await c.click();
    await page.keyboard.press("ArrowUp");
  }
  await expect(hand.locator(".card")).toHaveCount(12);

  const vb = page.viewportSize();
  expect(vb).toBeTruthy();
  const box = await play.boundingBox();
  expect(box).toBeTruthy();
  if (box && vb) {
    expect(box.y).toBeGreaterThanOrEqual(0);
    expect(box.y + box.height).toBeLessThanOrEqual(vb.height);
  }

  // With a large deck, the deck row should be horizontally scrollable.
  const deckRow = deck.locator(".row-scroll").first();
  const sizes = await deckRow.evaluate((el: HTMLElement) => ({
    scrollWidth: el.scrollWidth,
    clientWidth: el.clientWidth,
    scrollLeft: el.scrollLeft,
  }));
  expect(sizes.scrollWidth).toBeGreaterThan(sizes.clientWidth);

  // Arrow navigation should auto-scroll the selected card into view.
  const first = deck.locator(".card").first();
  await first.click();
  for (let i = 0; i < 18; i++) {
    await page.keyboard.press("ArrowRight");
  }
  const after = await deckRow.evaluate((el: HTMLElement) => el.scrollLeft);
  expect(after).toBeGreaterThan(sizes.scrollLeft);

  await page.screenshot({ path: testInfo.outputPath("01-layout-large.png"), fullPage: true });
});


