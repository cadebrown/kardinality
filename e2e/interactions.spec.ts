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

test("dragging a card moves the real element and drops into Hand", async ({ page }) => {
  await page.goto("/", { waitUntil: "domcontentloaded" });

  const deck = page.getByTestId("deck-zone");
  const handDrop = page.getByTestId("hand-dropzone");

  const source = deck.locator(".card").first();
  await expect(source).toBeVisible();

  const sb = await source.boundingBox();
  const hb = await handDrop.boundingBox();
  expect(sb).not.toBeNull();
  expect(hb).not.toBeNull();

  // Pointer-based drag: move the real card element via CSS transform.
  await page.mouse.move(sb!.x + sb!.width / 2, sb!.y + sb!.height / 2);
  await page.mouse.down();
  await page.mouse.move(sb!.x + sb!.width / 2 + 60, sb!.y + sb!.height / 2 + 6, { steps: 6 });
  await expect(source).toHaveAttribute("data-dragging", "true");

  await page.mouse.move(hb!.x + hb!.width / 2, hb!.y + hb!.height / 2, { steps: 10 });
  await page.mouse.up();

  await expect(page.getByTestId("hand-zone").locator(".card")).toHaveCount(1);
});

async function dragFromTo(page: any, from: any, to: any, toEdge: "left" | "center" | "right" = "center") {
  const fb = await from.boundingBox();
  const tb = await to.boundingBox();
  expect(fb).not.toBeNull();
  expect(tb).not.toBeNull();

  await page.mouse.move(fb!.x + fb!.width / 2, fb!.y + fb!.height / 2);
  await page.mouse.down();
  await page.mouse.move(fb!.x + fb!.width / 2 + 50, fb!.y + fb!.height / 2 + 5, { steps: 5 });

  const tx =
    toEdge === "left" ? tb!.x + 2 : toEdge === "right" ? tb!.x + tb!.width - 2 : tb!.x + tb!.width / 2;
  const ty = tb!.y + tb!.height / 2;
  await page.mouse.move(tx, ty, { steps: 10 });
  await page.mouse.up();
  await page.waitForTimeout(120);
}

test("drag: insert-before vs swap works inside Deck", async ({ page }) => {
  await page.goto("/", { waitUntil: "domcontentloaded" });

  const deck = page.getByTestId("deck-zone");
  const score = deck.locator(".card", { hasText: "Score +4" }).first();
  const draw = deck.locator(".card", { hasText: "Draw 5" }).first();

  // Insert-before: drag Score onto left edge of Draw -> Score becomes before Draw.
  await dragFromTo(page, score, draw, "left");

  const titles1 = await deck.locator(".card .card-title").allTextContents();
  const idxScore1 = titles1.findIndex((t) => t.includes("Score +4"));
  const idxDraw1 = titles1.findIndex((t) => t.includes("Draw 5"));
  expect(idxScore1).toBeGreaterThanOrEqual(0);
  expect(idxDraw1).toBeGreaterThanOrEqual(0);
  expect(idxScore1).toBeLessThan(idxDraw1);

  // Swap: drag Bank onto center of Draw -> their positions swap.
  const bank = deck.locator(".card", { hasText: "Bank +6" }).first();
  await dragFromTo(page, bank, draw, "center");

  const titles2 = await deck.locator(".card .card-title").allTextContents();
  const idxBank2 = titles2.findIndex((t) => t.includes("Bank +6"));
  const idxDraw2 = titles2.findIndex((t) => t.includes("Draw 5"));
  expect(idxBank2).toBeGreaterThanOrEqual(0);
  expect(idxDraw2).toBeGreaterThanOrEqual(0);
  expect(idxBank2).not.toEqual(idxDraw2);
});

test("drag: insert-after works inside Hand (no drop slots)", async ({ page }) => {
  await page.goto("/", { waitUntil: "domcontentloaded" });

  const deck = page.getByTestId("deck-zone");
  const hand = page.getByTestId("hand-zone");
  const handRow = hand.locator(".row-scroll").first();
  const handDrop = page.getByTestId("hand-dropzone");

  const draw = deck.locator(".card", { hasText: "Draw 5" }).first();
  const score = deck.locator(".card", { hasText: "Score +4" }).first();

  // Put two specific cards into Hand in known order: Draw then Score.
  await dragFromTo(page, draw, handDrop, "center");
  await dragFromTo(page, score, handRow, "center");

  const titles0 = await hand.locator(".card .card-title").allTextContents();
  expect(titles0.some((t) => t.includes("Draw 5"))).toBeTruthy();
  expect(titles0.some((t) => t.includes("Score +4"))).toBeTruthy();

  // Drag Draw onto the RIGHT edge of Score => insert-after, so order becomes Score then Draw.
  const handDraw = hand.locator(".card", { hasText: "Draw 5" }).first();
  const handScore = hand.locator(".card", { hasText: "Score +4" }).first();

  await dragFromTo(page, handDraw, handScore, "right");

  const titles1 = await hand.locator(".card .card-title").allTextContents();
  const idxScore = titles1.findIndex((t) => t.includes("Score +4"));
  const idxDraw = titles1.findIndex((t) => t.includes("Draw 5"));
  expect(idxScore).toBeGreaterThanOrEqual(0);
  expect(idxDraw).toBeGreaterThanOrEqual(0);
  expect(idxScore).toBeLessThan(idxDraw);
});


