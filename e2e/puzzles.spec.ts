import { test, expect } from "@playwright/test";

type PuzzleCase = {
  id: string;
  status: string;
};

const CASES: PuzzleCase[] = [
  { id: "lesson_score_ping", status: "Solved" },
  { id: "lesson_money_loop", status: "Solved" },
  { id: "lesson_draw_math", status: "Solved" },
  { id: "lesson_adaptive_branch", status: "Solved" },
  { id: "lesson_meta_clone", status: "Solved" },
  { id: "lesson_fibo_sprint", status: "Solved" },
];

async function waitForStableUi(page: any) {
  await expect(page.getByTestId("play")).toBeVisible();
}

test("all tutorial puzzle cards can be launched and solved in one play", async ({ page }, testInfo) => {
  test.setTimeout(120_000);
  await page.goto("/", { waitUntil: "domcontentloaded" });
  await waitForStableUi(page);

  for (const [idx, c] of CASES.entries()) {
    await page.getByTestId(`puzzle-${c.id}`).click();
    await waitForStableUi(page);

    await expect(page.getByText("Puzzles / Tutorials")).toBeVisible();
    await expect(page.getByText("In Progress")).toBeVisible();

    // Snapshot the lesson start state (hint + initial hand/deck).
    await page.screenshot({
      path: testInfo.outputPath(`${String(idx + 1).padStart(2, "0")}-${c.id}-start.png`),
      fullPage: true,
    });

    await page.getByTestId("play").click();
    await expect(page.locator(".kv", { hasText: "Status" }).getByText(c.status, { exact: true })).toBeVisible();

    // Snapshot solved state.
    await page.screenshot({
      path: testInfo.outputPath(`${String(idx + 1).padStart(2, "0")}-${c.id}-solved.png`),
      fullPage: true,
    });
  }
});
