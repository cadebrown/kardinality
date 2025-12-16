import { test, expect } from "@playwright/test";

test("boots with starter deck visible", async ({ page }) => {
  await page.goto("/");

  const deck = page.getByTestId("deck-zone");
  await expect(deck).toBeVisible();

  // Starter deck should be present immediately.
  await expect(deck.locator(".card")).toHaveCount(3);
});

test("move Score card to hand and play hand updates score", async ({ page }) => {
  await page.goto("/");

  const deck = page.getByTestId("deck-zone");
  const hand = page.getByTestId("hand-zone");

  // Select the "Score +4" card and move it into hand via keyboard (ArrowUp).
  const scoreCard = deck.locator(".card", { hasText: "Score +4" }).first();
  await scoreCard.click();
  await page.keyboard.press("ArrowUp");

  await expect(hand.locator(".card")).toHaveCount(1);

  // Play hand via the big right-rail button.
  await page.getByTestId("play-hand").click();

  await expect(page.getByTestId("score-value")).toHaveText("4");
  await expect(hand.locator(".card")).toHaveCount(0);
});

test("kardinomicon has the requested tabs", async ({ page }) => {
  await page.goto("/");
  await page.getByTestId("open-docs").click();

  await expect(page.getByText("Kardinomicon")).toBeVisible();
  await expect(page.getByRole("button", { name: "Overview" })).toBeVisible();
  await expect(page.getByRole("button", { name: "Functions" })).toBeVisible();
  await expect(page.getByRole("button", { name: "Examples" })).toBeVisible();
  await expect(page.getByRole("button", { name: "Reference" })).toBeVisible();
});


