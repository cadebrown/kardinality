import { test, expect } from "@playwright/test";

test("play/shop hover does not resize buttons", async ({ page }) => {
  await page.goto("/", { waitUntil: "domcontentloaded" });

  const play = page.getByTestId("play");
  const shop = page.getByTestId("shop");

  const b1 = await play.boundingBox();
  await play.hover();
  const b2 = await play.boundingBox();

  expect(b1).not.toBeNull();
  expect(b2).not.toBeNull();
  expect(Math.abs((b1!.width ?? 0) - (b2!.width ?? 0))).toBeLessThan(0.5);
  expect(Math.abs((b1!.height ?? 0) - (b2!.height ?? 0))).toBeLessThan(0.5);

  const s1 = await shop.boundingBox();
  await shop.hover();
  const s2 = await shop.boundingBox();

  expect(s1).not.toBeNull();
  expect(s2).not.toBeNull();
  expect(Math.abs((s1!.width ?? 0) - (s2!.width ?? 0))).toBeLessThan(0.5);
  expect(Math.abs((s1!.height ?? 0) - (s2!.height ?? 0))).toBeLessThan(0.5);
});

test("keyboard selection keeps a visible selected card", async ({ page }) => {
  await page.goto("/", { waitUntil: "domcontentloaded" });

  const sel1 = page.locator('.card[data-selected="true"]').first();
  await expect(sel1).toHaveCount(1);
  const id1 = await sel1.getAttribute("id");

  await page.keyboard.press("ArrowRight");
  const sel2 = page.locator('.card[data-selected="true"]').first();
  await expect(sel2).toHaveCount(1);
  const id2 = await sel2.getAttribute("id");

  expect(id2).not.toEqual(id1);
});

test("dragging a card hides the source and shows a custom drag ghost", async ({ page }) => {
  await page.goto("/", { waitUntil: "domcontentloaded" });

  const deck = page.getByTestId("deck-zone");
  const handDrop = page.getByTestId("hand-dropzone");

  const source = deck.locator(".card").first();
  await expect(source).toBeVisible();

  // Synthetic drag events are more reliable across browsers than mouse-driven HTML5 drag gestures in headless mode.
  await source.evaluate((el) => {
    el.dispatchEvent(new DragEvent("dragstart", { bubbles: true, cancelable: true }));
  });

  await expect(page.getByTestId("drag-fx")).toBeVisible();
  await expect(source).toHaveAttribute("data-drag-hidden", "true");

  // Drop into the Hand.
  await handDrop.evaluate((el) => {
    el.dispatchEvent(new DragEvent("drop", { bubbles: true, cancelable: true }));
  });

  // Ghost animates briefly; then the real card appears in Hand.
  await page.waitForTimeout(260);
  await expect(page.getByTestId("drag-fx")).toHaveCount(0);
  await expect(page.getByTestId("hand-zone").locator(".card")).toHaveCount(1);
});


