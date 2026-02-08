import { test, expect } from "@playwright/test";

test("boots with starter deck visible", async ({ page }, testInfo) => {
  await page.goto("/", { waitUntil: "domcontentloaded" });

  const deck = page.getByTestId("deck-zone");
  await expect(deck).toBeVisible();

  // Starter deck should be present immediately.
  await expect(deck.locator(".card")).toHaveCount(3);

  await page.screenshot({ path: testInfo.outputPath("01-boot.png"), fullPage: true });
});

test("move Score card to hand and play hand updates score", async ({ page }, testInfo) => {
  await page.goto("/", { waitUntil: "domcontentloaded" });

  const deck = page.getByTestId("deck-zone");
  const hand = page.getByTestId("hand-zone");

  // Select the starter score card and move it into hand via keyboard (ArrowUp).
  const scoreCard = deck.locator(".card", { hasText: "Tap Score" }).first();
  await scoreCard.click();
  await page.keyboard.press("ArrowUp");

  await expect(hand.locator(".card")).toHaveCount(1);
  await page.screenshot({ path: testInfo.outputPath("02-after-move-to-hand.png"), fullPage: true });

  // Play hand via the big right-rail button.
  await page.getByTestId("play").click();

  await expect(page.getByTestId("score-value")).toHaveText("2/10");
  await expect(hand.locator(".card")).toHaveCount(0);

  await page.screenshot({ path: testInfo.outputPath("03-after-play.png"), fullPage: true });
});

test("kardinomicon has the requested tabs", async ({ page }, testInfo) => {
  await page.goto("/", { waitUntil: "domcontentloaded" });
  await page.getByTestId("open-docs").click();

  await expect(page.getByText("Kardinomicon")).toBeVisible();
  await expect(page.getByRole("button", { name: "Overview" })).toBeVisible();
  await expect(page.getByRole("button", { name: "Functions" })).toBeVisible();
  await expect(page.getByRole("button", { name: "Examples" })).toBeVisible();
  await expect(page.getByRole("button", { name: "Reference" })).toBeVisible();

  await page.screenshot({ path: testInfo.outputPath("04-kardinomicon.png"), fullPage: true });
});

