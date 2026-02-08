import { test, expect, type Browser, type Page } from "@playwright/test";
import { mkdir, rm, writeFile } from "node:fs/promises";
import path from "node:path";

type OverlayTone = "score" | "economy" | "control" | "meta" | "neutral";

type OverlayArrow = {
  from: string;
  to: string;
  label: string;
  tone?: OverlayTone;
};

type OverlayCue = {
  title: string;
  body: string;
  focus: string[];
  panelAnchor?: string;
  tone?: OverlayTone;
  arrows?: OverlayArrow[];
};

type ChapterCue = {
  kicker: string;
  title: string;
  body: string;
  tone?: OverlayTone;
};

type InfoCue = {
  title: string;
  subtitle: string;
  quote: string;
  details: Array<{ label: string; value: string }>;
  tone?: OverlayTone;
};

type TutorialScene = {
  id: string;
  title: string;
  caption: string;
  voiceover: string;
  run: (page: Page) => Promise<void>;
};

type SceneManifest = {
  generated_at: string;
  base_url: string;
  size: { width: number; height: number };
  scenes: Array<{
    index: number;
    id: string;
    title: string;
    caption: string;
    voiceover: string;
    clip: string;
    frame: string;
  }>;
};

const ENABLED = process.env.TUTORIAL_VIDEO === "1";
const OUT_ROOT = process.env.TUTORIAL_VIDEO_OUT_DIR || "artifacts/tutorial-video";
const SCENE_DIR = path.join(OUT_ROOT, "scenes");
const FRAME_DIR = path.join(OUT_ROOT, "frames");
const SIZE = { width: 1280, height: 720 };

async function waitForBoot(page: Page) {
  await page.goto("/", { waitUntil: "domcontentloaded" });
  await expect(page.getByTestId("play")).toBeVisible();
}

async function waitMs(page: Page, ms: number) {
  await page.waitForTimeout(ms);
}

async function queueDeckCardToHand(page: Page, name: string): Promise<string> {
  const deck = page.getByTestId("deck-zone");
  const card = deck.locator(".card", { hasText: name }).first();
  await expect(card).toBeVisible();
  const cardId = await card.getAttribute("id");
  const selector = cardId ? `#${cardId}` : '[data-testid="deck-zone"] .card';

  await card.click();
  await waitMs(page, 260);
  await page.keyboard.press("ArrowUp");
  await waitMs(page, 460);
  return selector;
}

async function clearCoachOverlay(page: Page) {
  await page.evaluate(() => {
    const root = document.getElementById("__tutorial-coach-root");
    if (root) root.remove();
  });
}

async function clearChapterCard(page: Page) {
  await page.evaluate(() => {
    const root = document.getElementById("__tutorial-chapter-root");
    if (root) root.remove();
  });
}

async function clearInfoCard(page: Page) {
  await page.evaluate(() => {
    const root = document.getElementById("__tutorial-info-root");
    if (root) root.remove();
  });
}

async function showChapterCard(page: Page, cue: ChapterCue) {
  await page.evaluate((data: ChapterCue) => {
    const escapeHtml = (input: string): string =>
      input
        .replaceAll("&", "&amp;")
        .replaceAll("<", "&lt;")
        .replaceAll(">", "&gt;")
        .replaceAll('"', "&quot;");

    const toneColor = (tone: OverlayTone | undefined): string => {
      switch (tone) {
        case "score":
          return "rgba(255, 193, 77, 0.96)";
        case "economy":
          return "rgba(97, 231, 201, 0.96)";
        case "meta":
          return "rgba(255, 122, 193, 0.96)";
        case "control":
          return "rgba(64, 233, 255, 0.96)";
        default:
          return "rgba(64, 233, 255, 0.96)";
      }
    };

    const styleId = "__tutorial-chapter-style";
    if (!document.getElementById(styleId)) {
      const style = document.createElement("style");
      style.id = styleId;
      style.textContent = `
.tv-chapter-root {
  position: fixed;
  inset: 0;
  z-index: 999990;
  pointer-events: none;
  display: grid;
  place-items: center;
}

.tv-chapter-backdrop {
  position: absolute;
  inset: 0;
  background:
    radial-gradient(1100px 720px at 12% -12%, rgba(64, 233, 255, 0.24), transparent 62%),
    radial-gradient(1000px 760px at 95% 8%, rgba(255, 143, 45, 0.22), transparent 64%),
    linear-gradient(165deg, rgba(6, 10, 18, 0.76), rgba(8, 12, 24, 0.82));
  animation: tvChapterBackdrop 6200ms ease-in-out infinite;
}

.tv-chapter-noise {
  position: absolute;
  inset: 0;
  opacity: 0.08;
  mix-blend-mode: screen;
  background:
    repeating-linear-gradient(
      to bottom,
      rgba(255, 255, 255, 0.08),
      rgba(255, 255, 255, 0.08) 1px,
      rgba(0, 0, 0, 0) 2px,
      rgba(0, 0, 0, 0) 6px
    );
  animation: tvChapterNoise 3.6s linear infinite;
}

.tv-chapter-card {
  position: relative;
  width: min(920px, calc(100vw - 84px));
  border-radius: 28px;
  border: 2px solid rgba(64, 233, 255, 0.74);
  clip-path: polygon(0 16%, 6% 0, 94% 0, 100% 16%, 100% 84%, 94% 100%, 6% 100%, 0 84%);
  background:
    radial-gradient(520px 220px at 18% 2%, rgba(255, 255, 255, 0.11), transparent 62%),
    linear-gradient(160deg, rgba(9, 15, 28, 0.86), rgba(6, 10, 20, 0.92));
  box-shadow:
    0 36px 78px rgba(0, 0, 0, 0.56),
    0 0 64px rgba(64, 233, 255, 0.20),
    inset 0 0 0 1px rgba(255, 255, 255, 0.18);
  padding: 30px 40px 34px;
  color: rgba(242, 249, 255, 0.96);
  transform: translateY(8px) scale(0.96);
  animation: tvChapterCardIn 760ms cubic-bezier(0.2, 1, 0.2, 1) forwards;
}

.tv-chapter-kicker {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  padding: 8px 14px;
  border-radius: 999px;
  border: 1px solid rgba(64, 233, 255, 0.66);
  background: rgba(8, 15, 28, 0.58);
  text-transform: uppercase;
  letter-spacing: 1.2px;
  font-size: 13px;
  font-weight: 900;
}

.tv-chapter-title {
  margin-top: 14px;
  font-size: clamp(34px, 4.4vw, 52px);
  letter-spacing: 0.8px;
  line-height: 1.03;
  text-transform: uppercase;
  font-weight: 980;
  text-shadow: 0 0 42px rgba(64, 233, 255, 0.30);
}

.tv-chapter-body {
  margin-top: 14px;
  max-width: 760px;
  font-size: clamp(15px, 1.6vw, 22px);
  line-height: 1.4;
  color: rgba(214, 230, 245, 0.92);
}

@keyframes tvChapterCardIn {
  0% { opacity: 0; transform: translateY(24px) scale(0.94); }
  100% { opacity: 1; transform: translateY(0) scale(1); }
}

@keyframes tvChapterBackdrop {
  0%, 100% { opacity: 0.84; }
  50% { opacity: 0.94; }
}

@keyframes tvChapterNoise {
  0% { transform: translateY(0); }
  100% { transform: translateY(6px); }
}
`;
      document.head.append(style);
    }

    const rootId = "__tutorial-chapter-root";
    const old = document.getElementById(rootId);
    if (old) old.remove();

    const root = document.createElement("div");
    root.id = rootId;
    root.className = "tv-chapter-root";
    document.body.append(root);

    const backdrop = document.createElement("div");
    backdrop.className = "tv-chapter-backdrop";
    root.append(backdrop);

    const noise = document.createElement("div");
    noise.className = "tv-chapter-noise";
    root.append(noise);

    const card = document.createElement("div");
    card.className = "tv-chapter-card";
    card.style.borderColor = toneColor(data.tone);
    card.innerHTML = `
      <div class="tv-chapter-kicker" style="border-color:${toneColor(data.tone)}; color:${toneColor(data.tone)};">${escapeHtml(data.kicker)}</div>
      <div class="tv-chapter-title">${escapeHtml(data.title)}</div>
      <div class="tv-chapter-body">${escapeHtml(data.body)}</div>
    `;
    root.append(card);
  }, cue);
}

async function showInfoCard(page: Page, cue: InfoCue) {
  await page.evaluate((data: InfoCue) => {
    const escapeHtml = (input: string): string =>
      input
        .replaceAll("&", "&amp;")
        .replaceAll("<", "&lt;")
        .replaceAll(">", "&gt;")
        .replaceAll('"', "&quot;");

    const toneColor = (tone: OverlayTone | undefined): string => {
      switch (tone) {
        case "score":
          return "rgba(255, 193, 77, 0.96)";
        case "economy":
          return "rgba(97, 231, 201, 0.96)";
        case "meta":
          return "rgba(255, 122, 193, 0.96)";
        case "control":
          return "rgba(64, 233, 255, 0.96)";
        default:
          return "rgba(64, 233, 255, 0.96)";
      }
    };

    const styleId = "__tutorial-info-style";
    if (!document.getElementById(styleId)) {
      const style = document.createElement("style");
      style.id = styleId;
      style.textContent = `
.tv-info-root {
  position: fixed;
  inset: 0;
  z-index: 999995;
  pointer-events: none;
  display: grid;
  place-items: center;
}

.tv-info-backdrop {
  position: absolute;
  inset: 0;
  background:
    radial-gradient(1200px 760px at 8% -14%, rgba(64, 233, 255, 0.24), transparent 62%),
    radial-gradient(980px 700px at 92% 10%, rgba(255, 143, 45, 0.22), transparent 64%),
    linear-gradient(165deg, rgba(6, 10, 18, 0.78), rgba(8, 12, 24, 0.86));
  animation: tvInfoPulse 5.2s ease-in-out infinite;
}

.tv-info-card {
  position: relative;
  width: min(980px, calc(100vw - 68px));
  border-radius: 26px;
  border: 2px solid rgba(64, 233, 255, 0.80);
  clip-path: polygon(0 9%, 8% 0, 100% 0, 100% 92%, 92% 100%, 0 100%);
  background:
    radial-gradient(560px 240px at 12% -2%, rgba(255, 255, 255, 0.10), transparent 60%),
    linear-gradient(160deg, rgba(9, 15, 28, 0.88), rgba(6, 10, 20, 0.92));
  box-shadow:
    0 38px 84px rgba(0, 0, 0, 0.60),
    0 0 62px rgba(64, 233, 255, 0.22),
    inset 0 0 0 1px rgba(255, 255, 255, 0.16);
  color: rgba(244, 250, 255, 0.97);
  padding: 28px 34px 30px;
  animation: tvInfoIn 820ms cubic-bezier(0.2, 1, 0.2, 1) forwards;
}

.tv-info-title {
  font-size: clamp(31px, 4.1vw, 50px);
  line-height: 1.02;
  letter-spacing: 0.8px;
  text-transform: uppercase;
  font-weight: 980;
}

.tv-info-subtitle {
  margin-top: 10px;
  color: rgba(213, 229, 245, 0.92);
  font-size: clamp(14px, 1.55vw, 21px);
  line-height: 1.38;
}

.tv-info-grid {
  margin-top: 16px;
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 8px 18px;
}

.tv-info-item {
  border-radius: 13px;
  border: 1px solid rgba(120, 180, 255, 0.34);
  background: rgba(8, 14, 24, 0.56);
  padding: 8px 10px;
}

.tv-info-label {
  font-size: 11px;
  text-transform: uppercase;
  letter-spacing: 0.78px;
  color: rgba(188, 211, 233, 0.78);
}

.tv-info-value {
  margin-top: 2px;
  font-size: 13px;
  color: rgba(235, 246, 255, 0.95);
  line-height: 1.35;
}

.tv-info-quote {
  margin-top: 16px;
  border-radius: 18px;
  border: 2px solid rgba(255, 143, 45, 0.68);
  background: rgba(12, 16, 24, 0.74);
  padding: 14px 15px;
  font-size: clamp(20px, 2.1vw, 31px);
  line-height: 1.18;
  font-weight: 920;
  letter-spacing: 0.24px;
  color: rgba(255, 231, 198, 0.97);
  box-shadow: 0 0 36px rgba(255, 143, 45, 0.24);
}

@keyframes tvInfoIn {
  0% { opacity: 0; transform: translateY(18px) scale(0.96); }
  100% { opacity: 1; transform: translateY(0) scale(1); }
}

@keyframes tvInfoPulse {
  0%, 100% { opacity: 0.80; }
  50% { opacity: 0.90; }
}
`;
      document.head.append(style);
    }

    const rootId = "__tutorial-info-root";
    const old = document.getElementById(rootId);
    if (old) old.remove();

    const root = document.createElement("div");
    root.id = rootId;
    root.className = "tv-info-root";
    document.body.append(root);

    const backdrop = document.createElement("div");
    backdrop.className = "tv-info-backdrop";
    root.append(backdrop);

    const card = document.createElement("div");
    card.className = "tv-info-card";
    card.style.borderColor = toneColor(data.tone);
    const detailHtml = data.details
      .map(
        (item) => `
      <div class="tv-info-item">
        <div class="tv-info-label">${escapeHtml(item.label)}</div>
        <div class="tv-info-value">${escapeHtml(item.value)}</div>
      </div>`,
      )
      .join("");
    card.innerHTML = `
      <div class="tv-info-title">${escapeHtml(data.title)}</div>
      <div class="tv-info-subtitle">${escapeHtml(data.subtitle)}</div>
      <div class="tv-info-grid">${detailHtml}</div>
      <div class="tv-info-quote">${escapeHtml(data.quote)}</div>
    `;
    root.append(card);
  }, cue);
}

async function showCoachOverlay(page: Page, cue: OverlayCue) {
  await page.evaluate((data: OverlayCue) => {
    const escapeHtml = (input: string): string =>
      input
        .replaceAll("&", "&amp;")
        .replaceAll("<", "&lt;")
        .replaceAll(">", "&gt;")
        .replaceAll('"', "&quot;");

    const styleId = "__tutorial-coach-style";
    if (!document.getElementById(styleId)) {
      const style = document.createElement("style");
      style.id = styleId;
      style.textContent = `
.tv-coach-root {
  position: fixed;
  inset: 0;
  z-index: 999999;
  pointer-events: none;
}

.tv-coach-backdrop {
  position: absolute;
  inset: 0;
  background:
    radial-gradient(900px 620px at 8% -10%, rgba(64, 233, 255, 0.18), transparent 62%),
    radial-gradient(900px 600px at 92% 8%, rgba(255, 143, 45, 0.18), transparent 64%),
    linear-gradient(145deg, rgba(6, 10, 18, 0.22), rgba(7, 11, 20, 0.38));
  animation: tvCoachBackdropPulse 3800ms ease-in-out infinite;
}

.tv-coach-backdrop.tone-score {
  background:
    radial-gradient(900px 620px at 8% -10%, rgba(255, 193, 77, 0.20), transparent 62%),
    radial-gradient(900px 600px at 92% 8%, rgba(255, 143, 45, 0.16), transparent 64%),
    linear-gradient(145deg, rgba(14, 10, 6, 0.30), rgba(15, 10, 6, 0.34));
}

.tv-coach-backdrop.tone-economy {
  background:
    radial-gradient(900px 620px at 8% -10%, rgba(97, 231, 201, 0.20), transparent 62%),
    radial-gradient(900px 600px at 92% 8%, rgba(64, 233, 255, 0.15), transparent 64%),
    linear-gradient(145deg, rgba(6, 14, 12, 0.30), rgba(7, 13, 11, 0.34));
}

.tv-coach-backdrop.tone-meta {
  background:
    radial-gradient(900px 620px at 8% -10%, rgba(255, 122, 193, 0.22), transparent 62%),
    radial-gradient(900px 600px at 92% 8%, rgba(146, 92, 255, 0.17), transparent 64%),
    linear-gradient(145deg, rgba(16, 8, 18, 0.32), rgba(14, 8, 16, 0.36));
}

.tv-coach-svg {
  position: absolute;
  inset: 0;
  width: 100%;
  height: 100%;
}

.tv-coach-bloom {
  position: absolute;
  border-radius: 28px;
  filter: blur(14px);
  opacity: 0.44;
  animation: tvCoachBloom 2400ms ease-in-out infinite;
}

.tv-coach-bloom.tone-score { background: rgba(255, 193, 77, 0.56); }
.tv-coach-bloom.tone-economy { background: rgba(97, 231, 201, 0.50); }
.tv-coach-bloom.tone-control { background: rgba(64, 233, 255, 0.52); }
.tv-coach-bloom.tone-meta { background: rgba(255, 122, 193, 0.54); }

.tv-coach-ring {
  position: absolute;
  border-radius: 22px;
  border: 3px solid rgba(64, 233, 255, 0.95);
  clip-path: polygon(0 10%, 10% 0, 90% 0, 100% 10%, 100% 90%, 90% 100%, 10% 100%, 0 90%);
  box-shadow:
    0 0 0 2px rgba(10, 16, 28, 0.68),
    0 0 34px rgba(64, 233, 255, 0.34),
    inset 0 0 0 2px rgba(255, 255, 255, 0.24);
  animation: tvCoachPulse 1650ms ease-in-out infinite;
}

.tv-coach-ring.tone-score {
  border-color: rgba(255, 193, 77, 0.96);
  box-shadow: 0 0 0 2px rgba(20, 16, 8, 0.66), 0 0 38px rgba(255, 193, 77, 0.40), inset 0 0 0 2px rgba(255, 244, 214, 0.30);
}

.tv-coach-ring.tone-economy {
  border-color: rgba(97, 231, 201, 0.96);
  box-shadow: 0 0 0 2px rgba(10, 18, 16, 0.62), 0 0 38px rgba(97, 231, 201, 0.40), inset 0 0 0 2px rgba(216, 255, 248, 0.26);
}

.tv-coach-ring.tone-meta {
  border-color: rgba(255, 122, 193, 0.96);
  box-shadow: 0 0 0 2px rgba(20, 8, 18, 0.66), 0 0 38px rgba(255, 122, 193, 0.40), inset 0 0 0 2px rgba(255, 221, 240, 0.24);
}

.tv-coach-panel-backdrop {
  position: absolute;
  border-radius: 24px;
  background: rgba(8, 14, 24, 0.44);
  filter: blur(10px);
  opacity: 0.92;
}

.tv-coach-panel {
  position: absolute;
  width: min(440px, calc(100vw - 32px));
  border-radius: 20px;
  border: 2px solid rgba(64, 233, 255, 0.78);
  background: linear-gradient(160deg, rgba(8, 15, 28, 0.86), rgba(8, 11, 20, 0.90));
  clip-path: polygon(0 9%, 10% 0, 100% 0, 100% 86%, 90% 100%, 0 100%);
  box-shadow: 0 20px 50px rgba(0, 0, 0, 0.56), 0 0 30px rgba(64, 233, 255, 0.20);
  color: rgba(243, 250, 255, 0.98);
  padding: 16px 18px 18px;
  animation: tvCoachFloat 2400ms ease-in-out infinite;
}

.tv-coach-panel::after {
  content: "";
  position: absolute;
  left: 0;
  top: 0;
  width: 96px;
  height: 3px;
  background: rgba(255, 255, 255, 0.62);
  box-shadow: 0 0 18px rgba(255, 255, 255, 0.48);
}

.tv-coach-panel.tone-score { border-color: rgba(255, 193, 77, 0.82); box-shadow: 0 20px 56px rgba(0, 0, 0, 0.58), 0 0 38px rgba(255, 193, 77, 0.26); }
.tv-coach-panel.tone-economy { border-color: rgba(97, 231, 201, 0.82); box-shadow: 0 20px 56px rgba(0, 0, 0, 0.58), 0 0 38px rgba(97, 231, 201, 0.24); }
.tv-coach-panel.tone-meta { border-color: rgba(255, 122, 193, 0.82); box-shadow: 0 20px 56px rgba(0, 0, 0, 0.58), 0 0 38px rgba(255, 122, 193, 0.26); }

.tv-coach-title {
  font-size: 14px;
  font-weight: 920;
  text-transform: uppercase;
  letter-spacing: 0.74px;
}

.tv-coach-body {
  margin-top: 8px;
  font-size: 13px;
  line-height: 1.5;
  color: rgba(218, 232, 246, 0.92);
}

.tv-coach-link-label {
  position: absolute;
  padding: 7px 12px;
  border-radius: 14px;
  border: 2px solid rgba(64, 233, 255, 0.75);
  clip-path: polygon(8% 0, 100% 0, 92% 100%, 0 100%);
  background: rgba(8, 15, 28, 0.88);
  color: rgba(241, 249, 255, 0.96);
  font-size: 11px;
  text-transform: uppercase;
  letter-spacing: 0.6px;
  font-weight: 860;
  transform: translate(-50%, -50%);
  white-space: nowrap;
  box-shadow: 0 12px 30px rgba(0, 0, 0, 0.48);
  animation: tvCoachLabelBob 1600ms ease-in-out infinite;
}

.tv-coach-link-label.tone-score { border-color: rgba(255, 193, 77, 0.80); }
.tv-coach-link-label.tone-economy { border-color: rgba(97, 231, 201, 0.80); }
.tv-coach-link-label.tone-meta { border-color: rgba(255, 122, 193, 0.80); }

.tv-coach-arrow-beam {
  fill: none;
  stroke-width: 11;
  stroke-linecap: round;
  stroke-linejoin: round;
  opacity: 0.24;
  filter: blur(2px);
}

.tv-coach-arrow-core {
  fill: none;
  stroke-width: 5;
  stroke-linecap: round;
  stroke-linejoin: round;
  stroke-dasharray: 14 8;
  animation: tvCoachFlow 1200ms linear infinite;
}

.tv-coach-arrow-ribbon {
  fill: none;
  stroke-width: 2;
  stroke-linecap: round;
  stroke-linejoin: round;
  stroke-dasharray: 5 12;
  opacity: 0.72;
  animation: tvCoachFlowReverse 1900ms linear infinite;
}

.tv-coach-dot {
  stroke-width: 2.5;
  animation: tvCoachPulse 1650ms ease-in-out infinite;
}

@keyframes tvCoachPulse {
  0%, 100% { transform: scale(0.99); opacity: 0.86; }
  50% { transform: scale(1.03); opacity: 0.97; }
}

@keyframes tvCoachFlow {
  0% { stroke-dashoffset: 70; }
  100% { stroke-dashoffset: 0; }
}

@keyframes tvCoachFlowReverse {
  0% { stroke-dashoffset: 0; }
  100% { stroke-dashoffset: 70; }
}

@keyframes tvCoachFloat {
  0%, 100% { transform: translateY(0px); }
  50% { transform: translateY(-2px); }
}

@keyframes tvCoachLabelBob {
  0%, 100% { transform: translate(-50%, -50%); }
  50% { transform: translate(-50%, calc(-50% - 1px)); }
}

@keyframes tvCoachBloom {
  0%, 100% { opacity: 0.30; transform: scale(1); }
  50% { opacity: 0.46; transform: scale(1.03); }
}

@keyframes tvCoachBackdropPulse {
  0%, 100% { opacity: 0.72; }
  50% { opacity: 0.88; }
}
`;
      document.head.append(style);
    }

    const toneColor = (tone: OverlayTone | undefined): string => {
      switch (tone) {
        case "score":
          return "rgba(255, 193, 77, 0.96)";
        case "economy":
          return "rgba(97, 231, 201, 0.96)";
        case "meta":
          return "rgba(255, 122, 193, 0.96)";
        case "control":
          return "rgba(64, 233, 255, 0.96)";
        default:
          return "rgba(64, 233, 255, 0.96)";
      }
    };

    const bySelector = (selector: string): Element | null => document.querySelector(selector);
    const rectOf = (selector: string): DOMRect | null => {
      const el = bySelector(selector);
      return el ? el.getBoundingClientRect() : null;
    };

    const rootId = "__tutorial-coach-root";
    const old = document.getElementById(rootId);
    if (old) old.remove();

    const root = document.createElement("div");
    root.id = rootId;
    root.className = "tv-coach-root";
    document.body.append(root);

    const backdrop = document.createElement("div");
    backdrop.className = `tv-coach-backdrop tone-${data.tone || "neutral"}`;
    root.append(backdrop);

    const svg = document.createElementNS("http://www.w3.org/2000/svg", "svg");
    svg.setAttribute("class", "tv-coach-svg");
    svg.setAttribute("viewBox", `0 0 ${window.innerWidth} ${window.innerHeight}`);
    root.append(svg);

    const defs = document.createElementNS("http://www.w3.org/2000/svg", "defs");
    svg.append(defs);

    const createArrowMarker = (color: string): string => {
      const markerId = `tvCoachArrow-${Math.random().toString(36).slice(2, 10)}`;
      const marker = document.createElementNS("http://www.w3.org/2000/svg", "marker");
      marker.setAttribute("id", markerId);
      marker.setAttribute("viewBox", "0 0 18 14");
      marker.setAttribute("refX", "15");
      marker.setAttribute("refY", "7");
      marker.setAttribute("markerWidth", "11");
      marker.setAttribute("markerHeight", "11");
      marker.setAttribute("orient", "auto-start-reverse");
      defs.append(marker);

      const arrowPath = document.createElementNS("http://www.w3.org/2000/svg", "path");
      arrowPath.setAttribute("d", "M 0 0 L 18 7 L 0 14 L 4.8 7 z");
      arrowPath.setAttribute("fill", color);
      marker.append(arrowPath);
      return markerId;
    };

    for (const selector of data.focus) {
      const r = rectOf(selector);
      if (!r) continue;

      const bloom = document.createElement("div");
      bloom.className = `tv-coach-bloom tone-${data.tone || "neutral"}`;
      bloom.style.left = `${Math.max(0, r.left - 32)}px`;
      bloom.style.top = `${Math.max(0, r.top - 32)}px`;
      bloom.style.width = `${r.width + 64}px`;
      bloom.style.height = `${r.height + 64}px`;
      root.append(bloom);

      const ring = document.createElement("div");
      ring.className = `tv-coach-ring tone-${data.tone || "neutral"}`;
      ring.style.left = `${Math.max(0, r.left - 9)}px`;
      ring.style.top = `${Math.max(0, r.top - 9)}px`;
      ring.style.width = `${r.width + 18}px`;
      ring.style.height = `${r.height + 18}px`;
      root.append(ring);
    }

    for (const arrow of data.arrows || []) {
      const from = rectOf(arrow.from);
      const to = rectOf(arrow.to);
      if (!from || !to) continue;

      const x1 = from.left + from.width * 0.5;
      const y1 = from.top + from.height * 0.5;
      const x2 = to.left + to.width * 0.5;
      const y2 = to.top + to.height * 0.5;
      const tone = arrow.tone || data.tone;
      const color = toneColor(tone);

      const dx = x2 - x1;
      const dy = y2 - y1;
      const dist = Math.max(Math.hypot(dx, dy), 1);
      const nx = -dy / dist;
      const ny = dx / dist;
      const bend = Math.max(26, Math.min(96, dist * 0.28));
      const cx = (x1 + x2) / 2 + nx * bend;
      const cy = (y1 + y2) / 2 + ny * bend;
      const d = `M ${x1.toFixed(2)} ${y1.toFixed(2)} Q ${cx.toFixed(2)} ${cy.toFixed(2)} ${x2.toFixed(2)} ${y2.toFixed(2)}`;
      const markerId = createArrowMarker(color);

      const beam = document.createElementNS("http://www.w3.org/2000/svg", "path");
      beam.setAttribute("class", "tv-coach-arrow-beam");
      beam.setAttribute("d", d);
      beam.setAttribute("stroke", color);
      svg.append(beam);

      const core = document.createElementNS("http://www.w3.org/2000/svg", "path");
      core.setAttribute("class", "tv-coach-arrow-core");
      core.setAttribute("d", d);
      core.setAttribute("stroke", color);
      core.setAttribute("marker-end", `url(#${markerId})`);
      svg.append(core);

      const ribbon = document.createElementNS("http://www.w3.org/2000/svg", "path");
      ribbon.setAttribute("class", "tv-coach-arrow-ribbon");
      ribbon.setAttribute("d", d);
      ribbon.setAttribute("stroke", "rgba(245, 250, 255, 0.92)");
      ribbon.setAttribute("marker-end", `url(#${markerId})`);
      svg.append(ribbon);

      const start = document.createElementNS("http://www.w3.org/2000/svg", "circle");
      start.setAttribute("class", "tv-coach-dot");
      start.setAttribute("cx", `${x1}`);
      start.setAttribute("cy", `${y1}`);
      start.setAttribute("r", "6.5");
      start.setAttribute("fill", color);
      start.setAttribute("stroke", "rgba(8, 15, 28, 0.88)");
      svg.append(start);

      const end = document.createElementNS("http://www.w3.org/2000/svg", "circle");
      end.setAttribute("class", "tv-coach-dot");
      end.setAttribute("cx", `${x2}`);
      end.setAttribute("cy", `${y2}`);
      end.setAttribute("r", "5.2");
      end.setAttribute("fill", "rgba(245, 250, 255, 0.92)");
      end.setAttribute("stroke", color);
      svg.append(end);

      const t = 0.56;
      const inv = 1 - t;
      const labelX = inv * inv * x1 + 2 * inv * t * cx + t * t * x2;
      const labelY = inv * inv * y1 + 2 * inv * t * cy + t * t * y2;

      const label = document.createElement("div");
      label.className = `tv-coach-link-label tone-${tone || "neutral"}`;
      label.textContent = arrow.label;
      label.style.left = `${labelX}px`;
      label.style.top = `${labelY}px`;
      root.append(label);
    }

    const anchor = rectOf(data.panelAnchor || data.focus[0]) || new DOMRect(24, 24, 1, 1);
    const panel = document.createElement("div");
    panel.className = `tv-coach-panel tone-${data.tone || "neutral"}`;
    panel.innerHTML = `
      <div class="tv-coach-title">${escapeHtml(data.title)}</div>
      <div class="tv-coach-body">${escapeHtml(data.body)}</div>
    `;
    root.append(panel);

    const panelBackdrop = document.createElement("div");
    panelBackdrop.className = "tv-coach-panel-backdrop";
    root.append(panelBackdrop);

    const panelRect = panel.getBoundingClientRect();
    const spaceRight = window.innerWidth - anchor.right;
    let left =
      spaceRight > panelRect.width + 24
        ? anchor.right + 16
        : Math.max(16, anchor.left - panelRect.width - 16);
    left = Math.min(Math.max(16, left), window.innerWidth - panelRect.width - 16);
    let top = anchor.top + anchor.height * 0.5 - panelRect.height * 0.5;
    top = Math.min(Math.max(16, top), window.innerHeight - panelRect.height - 16);

    panel.style.left = `${left}px`;
    panel.style.top = `${top}px`;

    panelBackdrop.style.left = `${left - 10}px`;
    panelBackdrop.style.top = `${top - 10}px`;
    panelBackdrop.style.width = `${panelRect.width + 20}px`;
    panelBackdrop.style.height = `${panelRect.height + 20}px`;
  }, cue);
}

async function recordScene(
  browser: Browser,
  baseUrl: string,
  scene: TutorialScene,
  index: number,
): Promise<SceneManifest["scenes"][number]> {
  const context = await browser.newContext({
    baseURL: baseUrl,
    viewport: SIZE,
    recordVideo: { dir: SCENE_DIR, size: SIZE },
  });
  const page = await context.newPage();
  await waitForBoot(page);
  await scene.run(page);

  const frameName = `${String(index + 1).padStart(2, "0")}-${scene.id}.png`;
  const framePath = path.join(FRAME_DIR, frameName);
  await page.screenshot({ path: framePath, fullPage: true });

  const video = page.video();
  await context.close();
  if (!video) {
    throw new Error(`Playwright did not produce a video for scene ${scene.id}`);
  }

  const clipName = `${String(index + 1).padStart(2, "0")}-${scene.id}.webm`;
  const clipPath = path.join(SCENE_DIR, clipName);
  await video.saveAs(clipPath);
  await video.delete();

  return {
    index: index + 1,
    id: scene.id,
    title: scene.title,
    caption: scene.caption,
    voiceover: scene.voiceover,
    clip: path.join("scenes", clipName),
    frame: path.join("frames", frameName),
  };
}

const SCENES: TutorialScene[] = [
  {
    id: "chapter-intro",
    title: "Chapter 1: Cards Are Code",
    caption: "A one-minute guided run through code-cards, puzzles, and debug trace.",
    voiceover:
      "Kardinality. Cards are code. Build a hand, run it, and watch the board respond.",
    run: async (page) => {
      await showChapterCard(page, {
        kicker: "Chapter 1",
        title: "Cards Are Code",
        body: "A rapid cinematic run: compile, execute, solve, and inspect.",
        tone: "control",
      });
      await waitMs(page, 6300);
      await clearChapterCard(page);
    },
  },
  {
    id: "core-compile",
    title: "Compile The Hand",
    caption: "Move cards into Hand to define execution order.",
    voiceover: "Queue cards into Hand. This strip is your live program.",
    run: async (page) => {
      const scoreSel = await queueDeckCardToHand(page, "Tap Score");
      await showCoachOverlay(page, {
        title: "Compile Program Order",
        body: "Move cards from Source into Hand to build your execution chain before pressing Play.",
        focus: [scoreSel, '[data-testid="hand-dropzone"]', '[data-testid="play"]'],
        panelAnchor: '[data-testid="hand-dropzone"]',
        tone: "control",
        arrows: [
          { from: scoreSel, to: '[data-testid="hand-dropzone"]', label: "Queue", tone: "control" },
          {
            from: '[data-testid="hand-dropzone"]',
            to: '[data-testid="play"]',
            label: "Then Execute",
            tone: "control",
          },
        ],
      });
      await waitMs(page, 900);

      const drawSel = await queueDeckCardToHand(page, "Spark Draw");
      await showCoachOverlay(page, {
        title: "Layer Utility",
        body: "Add draw or money cards before scorers to keep your run flexible and explosive.",
        focus: [drawSel, '[data-testid="hand-zone"]', '[data-testid="deck-zone"]'],
        panelAnchor: '[data-testid="hand-zone"]',
        tone: "economy",
        arrows: [
          { from: '[data-testid="deck-zone"]', to: '[data-testid="hand-zone"]', label: "Compile", tone: "economy" },
        ],
      });

      await waitMs(page, 1200);
      await clearCoachOverlay(page);
    },
  },
  {
    id: "core-execute",
    title: "Execute + Feedback",
    caption: "Play Hand executes code left-to-right and updates score/money immediately.",
    voiceover: "Hit Play. Execute left to right. Score and cash jump instantly.",
    run: async (page) => {
      const scoreSel = await queueDeckCardToHand(page, "Tap Score");
      await showCoachOverlay(page, {
        title: "Ready To Fire",
        body: "A tiny hand still matters. One clean scorer creates momentum.",
        focus: [scoreSel, '[data-testid="play"]'],
        panelAnchor: '[data-testid="play"]',
        tone: "score",
        arrows: [{ from: scoreSel, to: '[data-testid="play"]', label: "Run Card", tone: "score" }],
      });
      await waitMs(page, 700);

      await page.getByTestId("play").click();
      await expect
        .poll(async () => parseInt(await page.getByTestId("score-value").innerText(), 10))
        .toBeGreaterThan(0);

      await showCoachOverlay(page, {
        title: "Live Telemetry",
        body: "Every play updates score and bankroll instantly so you can tune strategy in the same turn.",
        focus: ['[data-testid="score-value"]', '[data-testid="money-value"]', '[data-testid="play"]'],
        panelAnchor: '[data-testid="score-value"]',
        tone: "score",
        arrows: [
          { from: '[data-testid="play"]', to: '[data-testid="score-value"]', label: "Score", tone: "score" },
          { from: '[data-testid="play"]', to: '[data-testid="money-value"]', label: "Cash", tone: "economy" },
        ],
      });

      await waitMs(page, 1300);
      await clearCoachOverlay(page);
    },
  },
  {
    id: "chapter-puzzles",
    title: "Chapter 2: Puzzle Lessons",
    caption: "Deterministic tutorials with clear goals and reusable patterns.",
    voiceover: "Now switch to Puzzle Lessons. One setup, one pattern, one fast win.",
    run: async (page) => {
      await showChapterCard(page, {
        kicker: "Chapter 2",
        title: "Puzzle Lessons",
        body: "Focused card sets. One objective. One pattern to master.",
        tone: "economy",
      });
      await waitMs(page, 5200);
      await clearChapterCard(page);
    },
  },
  {
    id: "puzzle-hints",
    title: "Hints + Goal",
    caption: "Hints are prominent and outcomes are explicit.",
    voiceover: "Each lesson gives a bold hint and a clear goal. Solve it once, then reuse the line.",
    run: async (page) => {
      await page.getByTestId("puzzle-lesson_score_ping").click();
      await expect(page.locator(".kv", { hasText: "Status" })).toBeVisible();

      await showCoachOverlay(page, {
        title: "Read The Lesson Frame",
        body: "Start with the big hint panel, then map it to Goal and Status before executing.",
        focus: [".puzzle-hint-hero", '.kv', '[data-testid="play"]'],
        panelAnchor: ".puzzle-hint-hero",
        tone: "meta",
        arrows: [
          { from: ".puzzle-hint-hero", to: '.kv', label: "Hint -> Goal", tone: "meta" },
          { from: '.kv', to: '[data-testid="play"]', label: "Then Run", tone: "control" },
        ],
      });
      await waitMs(page, 1600);

      await page.getByTestId("play").click();
      await expect(page.locator(".kv", { hasText: "Status" }).getByText("Solved", { exact: true })).toBeVisible();

      await showCoachOverlay(page, {
        title: "Immediate Validation",
        body: "Status and puzzle message confirm whether your line is clean or needs revision.",
        focus: ['.kv', '.puzzle-message-banner'],
        panelAnchor: '.puzzle-message-banner',
        tone: "meta",
      });
      await waitMs(page, 3900);
      await clearCoachOverlay(page);
    },
  },
  {
    id: "economy-chain",
    title: "Economy Burst",
    caption: "Convert bankroll into score through an intentional chain.",
    voiceover: "Build bankroll, convert at the right moment, then spike score in one clean burst.",
    run: async (page) => {
      await page.getByTestId("puzzle-lesson_money_loop").click();
      await expect(page.locator(".kv", { hasText: "Status" })).toBeVisible();

      await showCoachOverlay(page, {
        title: "Bankroll Engine",
        body: "This puzzle teaches a convert-and-cash pattern: build bank, then route it into score at the right moment.",
        focus: ['[data-testid="hand-zone"]', '[data-testid="money-value"]', '[data-testid="play"]'],
        panelAnchor: '[data-testid="money-value"]',
        tone: "economy",
        arrows: [
          { from: '[data-testid="hand-zone"]', to: '[data-testid="money-value"]', label: "Accumulate", tone: "economy" },
          { from: '[data-testid="money-value"]', to: '[data-testid="score-value"]', label: "Convert", tone: "score" },
        ],
      });
      await waitMs(page, 1500);

      await page.getByTestId("play").click();
      await expect(page.locator(".kv", { hasText: "Status" }).getByText("Solved", { exact: true })).toBeVisible();

      await showCoachOverlay(page, {
        title: "Combo Payoff",
        body: "Notice both bars jump. This is the rhythm for scaling hard targets without random luck.",
        focus: ['[data-testid="score-value"]', '[data-testid="money-value"]', '.puzzle-message-banner'],
        panelAnchor: '[data-testid="score-value"]',
        tone: "economy",
      });
      await waitMs(page, 3500);
      await clearCoachOverlay(page);
    },
  },
  {
    id: "chapter-trace",
    title: "Chapter 3: Debug + Meta",
    caption: "Use trace to understand complex replay behavior.",
    voiceover: "Next: Debug mode. This is where the run becomes fully transparent.",
    run: async (page) => {
      await showChapterCard(page, {
        kicker: "Chapter 3",
        title: "Debug + Meta",
        body: "Replay effects, inspect sequence, and tune like a systems designer.",
        tone: "meta",
      });
      await waitMs(page, 4300);
      await clearChapterCard(page);
    },
  },
  {
    id: "meta-trace",
    title: "Meta Cards + Trace",
    caption: "Replay logic with clone mechanics, then inspect every effect in trace.",
    voiceover: "Meta cards replay prior logic. Open trace to inspect each call and effect in order.",
    run: async (page) => {
      await page.getByTestId("puzzle-lesson_meta_clone").click();
      await expect(page.locator(".kv", { hasText: "Status" })).toBeVisible();

      await showCoachOverlay(page, {
        title: "Meta Replay",
        body: "Clone cards recursively replay earlier calls. Card order controls how wild the chain gets.",
        focus: ['[data-testid="hand-zone"]', '[data-testid="play"]'],
        panelAnchor: '[data-testid="hand-zone"]',
        tone: "meta",
        arrows: [{ from: '[data-testid="hand-zone"]', to: '[data-testid="play"]', label: "Replay", tone: "meta" }],
      });
      await waitMs(page, 250);

      await page.getByTestId("play").click();
      await expect(page.locator(".kv", { hasText: "Status" }).getByText("Solved", { exact: true })).toBeVisible();
      await page.getByRole("button", { name: "Debug" }).click();
      await expect(page.getByText("Trace (latest first)")).toBeVisible();

      await showCoachOverlay(page, {
        title: "Inspect Execution",
        body: "Trace is your truth source: every call, every derived effect, and the exact order they fired.",
        focus: [".trace-list", ".trace-item"],
        panelAnchor: ".trace-list",
        tone: "control",
        arrows: [{ from: ".trace-item", to: ".trace-list", label: "Call stack", tone: "control" }],
      });
      await waitMs(page, 150);
      await clearCoachOverlay(page);
    },
  },
  {
    id: "finale",
    title: "Finale",
    caption: "Final system card and end blurb.",
    voiceover: "Yes, this game was vibe coded and I don't give a fuck",
    run: async (page) => {
      await clearCoachOverlay(page);
      await showInfoCard(page, {
        title: "Kardinality // Runtime Notes",
        subtitle: "Jam build. Cards-as-code. Trace-first deckbuilder.",
        tone: "score",
        details: [
          { label: "Engine", value: "Rust logic + Kardlang evaluator + deterministic run state" },
          { label: "Renderer", value: "Dioxus DOM + CSS only, no traditional game renderer" },
          { label: "Loop", value: "Draw -> compile hand -> execute -> score/bankroll -> advance" },
          { label: "Onboarding", value: "Puzzle lessons, giant hints, and instrumentation overlays" },
          { label: "Debug", value: "Trace stream for call/effect order and chain reactions" },
          { label: "Video", value: "Automated scenes, transitions, subtitles, and voice synthesis" },
        ],
        quote: "Yes, this game was vibe coded and I don't give a fuck",
      });
      await waitMs(page, 10000);
      await clearInfoCard(page);
    },
  },
];

test.describe("tutorial video capture", () => {
  test.skip(!ENABLED, "Set TUTORIAL_VIDEO=1 to enable tutorial scene capture.");

  test("captures deterministic tutorial scenes and emits a manifest", async ({ browser }) => {
    test.setTimeout(240_000);
    const baseUrl = process.env.E2E_BASE_URL || "http://127.0.0.1:8080";

    await rm(SCENE_DIR, { recursive: true, force: true });
    await rm(FRAME_DIR, { recursive: true, force: true });
    await mkdir(SCENE_DIR, { recursive: true });
    await mkdir(FRAME_DIR, { recursive: true });
    await mkdir(OUT_ROOT, { recursive: true });

    const warmupContext = await browser.newContext({ baseURL: baseUrl, viewport: SIZE });
    const warmupPage = await warmupContext.newPage();
    await waitForBoot(warmupPage);
    await waitMs(warmupPage, 250);
    await warmupContext.close();

    const scenes: SceneManifest["scenes"] = [];
    for (const [index, scene] of SCENES.entries()) {
      scenes.push(await recordScene(browser, baseUrl, scene, index));
    }

    const manifest: SceneManifest = {
      generated_at: new Date().toISOString(),
      base_url: baseUrl,
      size: SIZE,
      scenes,
    };

    await writeFile(
      path.join(OUT_ROOT, "scene-manifest.json"),
      `${JSON.stringify(manifest, null, 2)}\n`,
      "utf8",
    );
  });
});
