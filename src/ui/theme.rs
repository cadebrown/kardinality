pub const CSS: &str = r#"
:root {
  --bg0: #05070c;
  --bg1: #070b12;
  --panel: rgba(10, 14, 20, 0.78);
  --panel2: rgba(8, 11, 16, 0.82);
  --border: rgba(0, 255, 156, 0.24);
  --border-strong: rgba(0, 255, 156, 0.55);
  --accent: #00ff9c;
  --accent2: #00c8ff;
  --danger: #ff4d6d;
  --text: rgba(238, 255, 248, 0.92);
  --muted: rgba(238, 255, 248, 0.66);
  --shadow: rgba(0, 0, 0, 0.55);
  --radius: 16px;
  --radius-sm: 12px;
  --glow: 0 0 12px rgba(0, 255, 156, 0.22), 0 0 44px rgba(0, 200, 255, 0.10);
  /* Card sizing is viewport-responsive so Hand + Deck can each occupy ~half the screen */
  --card-w: clamp(140px, 15.5vw, 184px);
  --card-h: clamp(210px, 28vh, 320px);
  --card-r: 18px;
  --topbar-h: 162px;
}

@media (max-height: 820px) {
  :root {
    --card-w: clamp(132px, 15.0vw, 172px);
    --card-h: clamp(192px, 26vh, 280px);
    --card-r: 16px;
    --topbar-h: 148px;
  }
}

html, body {
  height: 100%;
}

body {
  margin: 0;
  overflow: hidden;
  color: var(--text);
  font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", monospace;
  background:
    radial-gradient(1200px 900px at 18% 10%, rgba(0, 255, 156, 0.12), transparent 55%),
    radial-gradient(900px 700px at 82% 20%, rgba(0, 200, 255, 0.10), transparent 52%),
    radial-gradient(700px 520px at 60% 110%, rgba(255, 77, 109, 0.06), transparent 55%),
    linear-gradient(180deg, var(--bg1), var(--bg0));
}

* {
  box-sizing: border-box;
}

button, input, select {
  font-family: inherit;
}

/* Selection policy:
   * Default: no text selection in the game UI (prevents drag-select / random highlights)
   * Opt-in: `.selectable` for trace/docs/blocks where copying is useful
   * During drag: force-disable selection everywhere (even selectable areas) */
.app {
  -webkit-user-select: none;
  user-select: none;
}

.selectable,
.selectable * {
  -webkit-user-select: text;
  user-select: text;
}

.app.is-dragging,
.app.is-dragging * {
  -webkit-user-select: none !important;
  user-select: none !important;
}

.app.is-dragging {
  cursor: grabbing;
}

.app {
  height: 100vh;
  display: grid;
  grid-template-columns: 308px 1fr;
  position: relative;
}

.app.theme-terminal {
  --accent: #7CFF00;
  --accent2: #00ff9c;
  --bg0: #020402;
  --bg1: #030604;
  --border: rgba(124, 255, 0, 0.22);
  --border-strong: rgba(124, 255, 0, 0.55);
  --glow: 0 0 12px rgba(124, 255, 0, 0.22), 0 0 44px rgba(0, 255, 156, 0.08);
}

.app.theme-magic {
  --accent: #ff4dff;
  --accent2: #7a5cff;
  --bg0: #07030c;
  --bg1: #090417;
  --border: rgba(255, 77, 255, 0.20);
  --border-strong: rgba(255, 77, 255, 0.55);
  --glow: 0 0 14px rgba(255, 77, 255, 0.22), 0 0 54px rgba(122, 92, 255, 0.14);
}

.app.effects-off::before,
.app.effects-off::after {
  display: none;
}

.app.effects-off {
  filter: none;
  transform: none;
}

.app.theme-crt:not(.effects-off) {
  filter: contrast(1.22) saturate(1.34) brightness(1.06);
  transform: perspective(1100px) rotateX(0.95deg) rotateY(-0.85deg) scale(1.02);
  transform-origin: 50% 50%;
  will-change: transform, filter;
}

.app.theme-crt:not(.effects-off)::before {
  opacity: 0.60;
  background:
    repeating-linear-gradient(
      to bottom,
      rgba(255, 255, 255, 0.090),
      rgba(255, 255, 255, 0.090) 1px,
      rgba(0, 0, 0, 0.00) 2px,
      rgba(0, 0, 0, 0.00) 5px
    ),
    repeating-linear-gradient(
      to right,
      rgba(255, 0, 0, 0.045),
      rgba(255, 0, 0, 0.045) 1px,
      rgba(0, 255, 0, 0.040) 2px,
      rgba(0, 0, 255, 0.045) 3px
    );
  mix-blend-mode: screen;
  animation: scanlines 3.4s linear infinite;
}

.app.theme-crt:not(.effects-off)::after {
  background:
    radial-gradient(closest-side at 50% 50%, rgba(0, 0, 0, 0.00) 58%, rgba(0, 0, 0, 0.62) 100%),
    radial-gradient(1200px 900px at 50% 50%, rgba(0, 255, 156, 0.080), transparent 60%),
    radial-gradient(1100px 820px at 56% 46%, rgba(0, 200, 255, 0.060), transparent 62%),
    linear-gradient(90deg, rgba(0, 255, 156, 0.06), rgba(0, 200, 255, 0.03));
  opacity: 0.98;
  mix-blend-mode: overlay;
  animation: flicker 1.6s infinite steps(40);
}

.app::before {
  content: "";
  position: fixed;
  inset: 0;
  pointer-events: none;
  opacity: 0.14;
  background:
    repeating-linear-gradient(
      to bottom,
      rgba(255, 255, 255, 0.035),
      rgba(255, 255, 255, 0.035) 1px,
      rgba(0, 0, 0, 0.00) 2px,
      rgba(0, 0, 0, 0.00) 4px
    );
  mix-blend-mode: overlay;
  animation: scanlines 8s linear infinite;
}

.app::after {
  content: "";
  position: fixed;
  inset: 0;
  pointer-events: none;
  background:
    radial-gradient(closest-side at 50% 50%, rgba(0, 0, 0, 0.00) 62%, rgba(0, 0, 0, 0.52) 100%),
    linear-gradient(90deg, rgba(0, 255, 156, 0.02), rgba(0, 200, 255, 0.01));
  opacity: 0.9;
  animation: flicker 3.2s infinite steps(60);
}

/* During drag, disable CRT/scanline overlays so the dragged card reads cleanly above everything. */
.app.is-dragging::before,
.app.is-dragging::after,
.app.is-dragging.theme-crt:not(.effects-off)::before,
.app.is-dragging.theme-crt:not(.effects-off)::after {
  opacity: 0 !important;
  animation: none !important;
}

@keyframes scanlines {
  0% { transform: translateY(0); }
  100% { transform: translateY(4px); }
}

@keyframes flicker {
  0% { opacity: 0.88; }
  13% { opacity: 0.93; }
  17% { opacity: 0.86; }
  27% { opacity: 0.94; }
  52% { opacity: 0.90; }
  73% { opacity: 0.95; }
  100% { opacity: 0.89; }
}

.sidebar {
  border-right: 1px solid var(--border);
  background:
    linear-gradient(180deg, rgba(9, 12, 18, 0.90), rgba(6, 9, 13, 0.86));
  padding: 18px 16px;
  display: flex;
  flex-direction: column;
  gap: 14px;
  position: relative;
  z-index: 1;
}

.brand {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.brand-title {
  font-size: 20px;
  letter-spacing: 0.6px;
  text-transform: uppercase;
  margin: 0;
  text-shadow: 0 0 18px rgba(0, 255, 156, 0.22);
}

.brand-subtitle {
  color: var(--muted);
  font-size: 12px;
  letter-spacing: 0.2px;
}

.tabs {
  display: flex;
  gap: 10px;
}

.tab {
  flex: 1;
  border-radius: 999px;
  border: 1px solid var(--border);
  background: rgba(0, 0, 0, 0.20);
  color: var(--muted);
  padding: 10px 12px;
  cursor: pointer;
  transition: transform 120ms ease, border-color 120ms ease, box-shadow 120ms ease, background 120ms ease;
}

.tab:hover {
  transform: translateY(-1px);
  border-color: rgba(0, 255, 156, 0.45);
}

.tab.active {
  color: var(--text);
  border-color: var(--border-strong);
  background: rgba(0, 255, 156, 0.08);
  box-shadow: var(--glow);
}

.panel {
  border-radius: var(--radius);
  border: 1px solid var(--border);
  background: var(--panel);
  box-shadow: 0 18px 44px rgba(0, 0, 0, 0.36);
  backdrop-filter: blur(10px);
}

.sidebar-panel {
  padding: 14px;
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.sidebar-panel h3 {
  margin: 0;
  font-size: 12px;
  text-transform: uppercase;
  letter-spacing: 0.4px;
  color: var(--muted);
}

.btn {
  border-radius: 12px;
  border: 1px solid rgba(0, 255, 156, 0.28);
  background:
    linear-gradient(180deg, rgba(0, 255, 156, 0.14), rgba(0, 0, 0, 0.20));
  color: var(--text);
  padding: 10px 12px;
  cursor: pointer;
  transition: transform 120ms ease, border-color 120ms ease, box-shadow 120ms ease;
  text-align: left;
}

.btn:hover {
  transform: translateY(-1px);
  border-color: rgba(0, 255, 156, 0.56);
  box-shadow: 0 0 18px rgba(0, 255, 156, 0.12);
}

.btn.focused {
  border-color: rgba(0, 255, 156, 0.92);
  box-shadow: 0 0 0 1px rgba(0, 255, 156, 0.10) inset, 0 0 26px rgba(0, 255, 156, 0.18);
}

.btn.secondary.focused {
  border-color: rgba(0, 200, 255, 0.85);
  box-shadow: 0 0 0 1px rgba(0, 200, 255, 0.10) inset, 0 0 26px rgba(0, 200, 255, 0.16);
}

.btn.danger.focused {
  border-color: rgba(255, 77, 109, 0.90);
  box-shadow: 0 0 0 1px rgba(255, 77, 109, 0.10) inset, 0 0 26px rgba(255, 77, 109, 0.16);
}

.btn:active {
  transform: translateY(0px);
}

.btn.secondary {
  border-color: rgba(0, 200, 255, 0.28);
  background: linear-gradient(180deg, rgba(0, 200, 255, 0.12), rgba(0, 0, 0, 0.20));
}

.btn.secondary:hover {
  border-color: rgba(0, 200, 255, 0.55);
  box-shadow: 0 0 18px rgba(0, 200, 255, 0.12);
}

.btn.danger {
  border-color: rgba(255, 77, 109, 0.30);
  background: linear-gradient(180deg, rgba(255, 77, 109, 0.12), rgba(0, 0, 0, 0.20));
}

.btn.danger:hover {
  border-color: rgba(255, 77, 109, 0.65);
  box-shadow: 0 0 18px rgba(255, 77, 109, 0.12);
}

.hint {
  color: var(--muted);
  font-size: 12px;
  line-height: 1.35;
}

.main {
  padding: 18px;
  display: flex;
  flex-direction: column;
  gap: 14px;
  position: relative;
  z-index: 1;
  min-height: 0;
  overflow: hidden;
  overflow-x: hidden;
}

/* Floating drag layer: dragged cards are truly unparented + fixed-positioned here. */
.drag-layer {
  position: fixed;
  inset: 0;
  pointer-events: none;
  z-index: 20000;
}

.app.is-dragging .topbar {
  /* Let dragged cards float over the topbar (stacking contexts are atomic). */
  z-index: 10;
}

.app.is-dragging .content {
  /* Critical: lift the entire content layer above the topbar. */
  position: relative;
  z-index: 1000;
}

/* Backdrop-filter can cause odd compositing/stacking in some browsers while dragging transformed elements. */
.app.is-dragging .panel {
  backdrop-filter: none !important;
}

.topbar {
  display: grid;
  grid-template-columns: minmax(320px, 360px) 1fr 238px;
  gap: 14px;
  align-items: stretch;
  /* Don't force a fixed height: allow contents to size naturally to avoid clipping. */
  flex: 0 0 auto;
  z-index: 200;
}

.run-pane {
  display: flex;
  flex-direction: column;
  gap: 10px;
  padding: 12px;
  min-height: 0;
  overflow: hidden;
}

.run-top {
  flex: 0 0 auto;
}

.run-bottom {
  margin-top: auto;
  flex: 0 0 auto;
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.run-buttons {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 10px;
}

.play-btn2 {
  height: 92px;
  display: grid;
  align-items: stretch;
  justify-items: stretch;
  padding: 12px 14px 12px;
}

.shop-btn {
  height: 92px;
  display: grid;
  align-items: stretch;
  justify-items: stretch;
  padding: 12px 14px 12px;
  border-color: rgba(0, 200, 255, 0.40);
  box-shadow: 0 18px 50px rgba(0, 0, 0, 0.52), 0 0 44px rgba(0, 200, 255, 0.16);
}
.shop-btn:hover {
  border-color: rgba(0, 200, 255, 0.62);
  box-shadow: 0 22px 60px rgba(0, 0, 0, 0.55), 0 0 64px rgba(0, 200, 255, 0.22);
}
.shop-btn.focused {
  border-color: rgba(0, 200, 255, 0.92);
  box-shadow: 0 22px 60px rgba(0, 0, 0, 0.55), 0 0 84px rgba(0, 200, 255, 0.28);
}
.shop-btn .play-icon {
  color: rgba(0, 200, 255, 0.95);
  text-shadow: 0 0 22px rgba(0, 200, 255, 0.28);
}
.play-head {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 10px;
}
.run-strip {
  display: flex;
  gap: 0px;
  padding: 0;
  border-radius: 12px;
  border: 1px solid rgba(0, 255, 156, 0.14);
  background: rgba(0, 0, 0, 0.16);
  overflow: hidden;
}
.strip-item {
  flex: 1 1 0;
  padding: 8px 10px;
  display: grid;
  grid-template-rows: auto auto;
  gap: 3px;
  text-align: center;
}
.strip-item + .strip-item {
  border-left: 1px solid rgba(0, 255, 156, 0.10);
}
.strip-k {
  font-size: 10px;
  letter-spacing: 0.4px;
  text-transform: uppercase;
  color: rgba(238, 255, 248, 0.60);
}
.strip-v {
  font-size: 13px;
  font-weight: 900;
  letter-spacing: 0.35px;
  color: rgba(238, 255, 248, 0.94);
  text-shadow: 0 0 18px rgba(0, 255, 156, 0.14), 0 0 54px rgba(0, 200, 255, 0.10);
}
.strip-item:nth-child(1) .strip-v { color: rgba(0, 255, 156, 0.95); }
.strip-item:nth-child(2) .strip-v { color: rgba(255, 158, 0, 0.95); }
.strip-item:nth-child(3) .strip-v { color: rgba(0, 200, 255, 0.95); }

.run-progress {
  border-radius: 999px;
  border: 1px solid rgba(0, 255, 156, 0.18);
  background: rgba(0, 0, 0, 0.30);
  overflow: hidden;
  box-shadow: 0 0 0 1px rgba(0, 0, 0, 0.24) inset;
}
.run-progress-fill {
  height: 100%;
  background: linear-gradient(90deg, rgba(255, 77, 255, 0.55), rgba(0, 255, 156, 0.42), rgba(0, 200, 255, 0.55));
  box-shadow: 0 0 18px rgba(0, 255, 156, 0.14);
}

.content {
  flex: 1 1 auto;
  min-height: 0;
  overflow: visible;
  display: grid;
  grid-template-rows: 1fr 1fr;
  gap: 14px;
}

.handbar,
.deckbar {
  min-height: 0;
  /* Allow selected-card glow to escape panel bounds. */
  overflow: visible;
  display: flex;
  flex-direction: column;
}

.handrow {
  min-height: 0;
  display: flex;
  gap: 14px;
  align-items: stretch;
}

.handrow .handbar {
  flex: 1 1 auto;
  min-width: 0;
}

.pile-widget {
  /* Keep right column aligned with the topbar right rail. */
  width: 238px;
  flex: 0 0 238px;
  min-height: 0;
  overflow: visible;
  position: relative;
  padding: 10px;
}

.card-wrap {
  position: relative;
  display: block;
}

/* Drop sliver between cards (hit target) */
.drop-slit {
  position: absolute;
  top: 6px;
  bottom: 6px;
  width: 16px;
  border-radius: 999px;
  opacity: 0;
  pointer-events: none;
  background: rgba(0, 255, 156, 0.08);
  border: 1px solid rgba(0, 255, 156, 0.10);
  box-shadow: 0 0 0 rgba(0, 0, 0, 0);
  transition: opacity 120ms ease, box-shadow 120ms ease, border-color 120ms ease, background 120ms ease;
}

.drop-slit.left { left: -10px; }
.drop-slit.right { right: -10px; }

/* Only show drop slivers while dragging */
.app.is-dragging .drop-slit {
  opacity: 0.22;
  pointer-events: auto;
}

.app.is-dragging .drop-slit.active {
  opacity: 1;
  background: rgba(0, 255, 156, 0.16);
  border-color: rgba(0, 255, 156, 0.62);
  box-shadow: 0 0 26px rgba(0, 255, 156, 0.18), 0 0 70px rgba(0, 200, 255, 0.10);
}

/* Swap target feedback (drop centered on a card) */
.app.is-dragging .card-wrap.swap-target > .card {
  outline: 3px solid rgba(0, 200, 255, 0.72);
  outline-offset: 2px;
  box-shadow:
    0 30px 76px rgba(0, 0, 0, 0.62),
    0 0 0 2px rgba(0, 200, 255, 0.18) inset,
    0 0 42px rgba(0, 200, 255, 0.22),
    0 0 110px rgba(0, 255, 156, 0.12);
}

.pile-face {
  margin-top: 6px;
  height: calc(var(--card-h) * 0.64);
  border-radius: 16px;
  border: 1px solid rgba(255, 77, 255, 0.24);
  background:
    radial-gradient(160px 110px at 20% 25%, rgba(255, 77, 255, 0.20), transparent 60%),
    radial-gradient(150px 110px at 78% 22%, rgba(0, 200, 255, 0.14), transparent 62%),
    linear-gradient(135deg, rgba(0, 0, 0, 0.15), rgba(0, 0, 0, 0.44));
  box-shadow: 0 18px 44px rgba(0, 0, 0, 0.38), 0 0 0 1px rgba(255, 77, 255, 0.05) inset;
  padding: 10px;
  display: grid;
  align-content: end;
  gap: 6px;
  overflow: hidden;
}

.pile-face-title {
  font-size: 13px;
  font-weight: 900;
  letter-spacing: 0.35px;
  text-transform: uppercase;
  color: rgba(238, 255, 248, 0.92);
  text-shadow: 0 0 20px rgba(255, 77, 255, 0.14), 0 0 44px rgba(0, 200, 255, 0.10);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
.pile-face-sub {
  font-size: 11px;
  color: rgba(238, 255, 248, 0.60);
}

.pile-meta {
  margin-top: 10px;
}

.pile-menu {
  position: absolute;
  left: 10px;
  right: 10px;
  bottom: 10px;
  transform: translateY(8px) scale(0.98);
  opacity: 0;
  pointer-events: none;
  border-radius: 14px;
  border: 1px solid rgba(255, 77, 255, 0.22);
  background: rgba(0, 0, 0, 0.70);
  box-shadow: 0 18px 50px rgba(0, 0, 0, 0.55), 0 0 30px rgba(255, 77, 255, 0.10);
  padding: 10px;
  transition: opacity 160ms ease, transform 160ms cubic-bezier(0.16, 1, 0.3, 1);
}
.pile-widget:hover .pile-menu {
  opacity: 1;
  transform: translateY(0px) scale(1);
  pointer-events: auto;
}
.pile-menu-title {
  font-size: 11px;
  letter-spacing: 0.4px;
  text-transform: uppercase;
  color: rgba(238, 255, 248, 0.62);
  margin-bottom: 8px;
}
.pile-item {
  font-size: 12px;
  color: rgba(238, 255, 248, 0.86);
  padding: 6px 8px;
  border-radius: 10px;
  border: 1px solid rgba(238, 255, 248, 0.08);
  background: rgba(0, 0, 0, 0.22);
  margin-bottom: 6px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
.pile-item.empty {
  opacity: 0.6;
}

.row-scroll {
  flex: 1 1 auto;
  min-height: 0;
  overflow-x: auto;
  overflow-y: hidden;
  scroll-behavior: smooth;
}

.right-rail {
  width: 238px;
  display: grid;
  grid-template-rows: auto 1fr;
  gap: 12px;
}

.play-btn {
  width: 100%;
  border-radius: 16px;
  border: 1px solid rgba(0, 255, 156, 0.42);
  background:
    radial-gradient(240px 140px at 20% 20%, rgba(0, 255, 156, 0.22), transparent 60%),
    linear-gradient(180deg, rgba(0, 0, 0, 0.10), rgba(0, 0, 0, 0.30));
  box-shadow: 0 18px 50px rgba(0, 0, 0, 0.52), 0 0 44px rgba(0, 255, 156, 0.18);
  color: rgba(238, 255, 248, 0.96);
  padding: 12px 14px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 10px;
  transition: transform 140ms ease, border-color 140ms ease, box-shadow 140ms ease, filter 140ms ease;
}
.play-btn:hover {
  border-color: rgba(0, 255, 156, 0.62);
  box-shadow: 0 22px 60px rgba(0, 0, 0, 0.55), 0 0 64px rgba(0, 255, 156, 0.28);
  filter: saturate(1.12) contrast(1.08);
}
.play-btn.focused {
  outline: none;
  border-color: rgba(0, 255, 156, 0.92);
  box-shadow: 0 22px 60px rgba(0, 0, 0, 0.55), 0 0 84px rgba(0, 255, 156, 0.32);
}

/* Playback / FX layer (animated execution) */
.fx-blocker {
  position: fixed;
  inset: 0;
  z-index: 9997;
  pointer-events: auto;
  background: rgba(0, 0, 0, 0.00);
}

.fx-layer {
  position: fixed;
  inset: 0;
  z-index: 9998;
  pointer-events: none;
}

.fx-card {
  position: fixed;
  transform-origin: 50% 50%;
  will-change: transform, opacity;
  pointer-events: none;
  transform: translate(var(--fx-tx, 0px), var(--fx-ty, 0px)) scale(var(--fx-scale, 1));
  transition: transform 420ms cubic-bezier(0.16, 1, 0.3, 1), opacity 220ms ease;
}

.fx-card.executing {
  animation: fxshake 140ms infinite;
  filter: saturate(1.22) contrast(1.10) brightness(1.04);
  outline: 3px solid rgba(255, 77, 255, 0.72);
  box-shadow:
    0 0 0 2px rgba(0, 0, 0, 0.32) inset,
    0 0 62px rgba(255, 77, 255, 0.24),
    0 0 110px rgba(0, 200, 255, 0.14);
}

.fx-call {
  position: fixed;
  z-index: 9999;
  transform: translate(-50%, -50%);
  font-size: 12px;
  color: rgba(238, 255, 248, 0.92);
  padding: 8px 10px;
  border-radius: 999px;
  border: 1px solid rgba(0, 255, 156, 0.24);
  background: rgba(0, 0, 0, 0.55);
  box-shadow: 0 0 22px rgba(0, 255, 156, 0.14);
}

/* Playback step tooltip (near executing card) */
.fx-step {
  position: fixed;
  z-index: 9999;
  transform: translate(-50%, -50%);
  font-size: 12px;
  font-weight: 800;
  letter-spacing: 0.35px;
  padding: 8px 10px;
  border-radius: 999px;
  border: 1px solid rgba(0, 255, 156, 0.26);
  background: rgba(0, 0, 0, 0.62);
  box-shadow: 0 0 22px rgba(0, 255, 156, 0.16);
  pointer-events: none;
  animation: stepPop 520ms cubic-bezier(0.16, 1, 0.3, 1) forwards;
}

.fx-step.card { border-color: rgba(255, 77, 255, 0.34); box-shadow: 0 0 24px rgba(255, 77, 255, 0.14); }
.fx-step.call { border-color: rgba(0, 200, 255, 0.34); box-shadow: 0 0 24px rgba(0, 200, 255, 0.14); }
.fx-step.pos  { border-color: rgba(0, 255, 156, 0.34); box-shadow: 0 0 24px rgba(0, 255, 156, 0.16); }
.fx-step.neg  { border-color: rgba(255, 77, 109, 0.38); box-shadow: 0 0 24px rgba(255, 77, 109, 0.16); }
.fx-step.mul  { border-color: rgba(0, 200, 255, 0.38); box-shadow: 0 0 24px rgba(0, 200, 255, 0.16); }
.fx-step.info { border-color: rgba(238, 255, 248, 0.20); box-shadow: 0 0 24px rgba(238, 255, 248, 0.10); }

@keyframes stepPop {
  0%   { opacity: 0; transform: translate(-50%, -50%) translateY(10px) scale(0.92); }
  15%  { opacity: 1; transform: translate(-50%, -50%) translateY(0px) scale(1.02); }
  100% { opacity: 1; transform: translate(-50%, -50%) translateY(0px) scale(1.00); }
}

.fx-bump {
  position: fixed;
  z-index: 9999;
  transform: translate(-50%, -50%);
  font-size: 14px;
  font-weight: 700;
  letter-spacing: 0.4px;
  padding: 6px 10px;
  border-radius: 999px;
  border: 1px solid rgba(0, 255, 156, 0.22);
  background: rgba(0, 0, 0, 0.55);
  box-shadow: 0 0 18px rgba(0, 255, 156, 0.12);
  animation: bumpFloat 900ms ease-out forwards;
}
.fx-bump.pos { color: rgba(0, 255, 156, 0.95); }
.fx-bump.neg { color: rgba(255, 77, 109, 0.95); border-color: rgba(255, 77, 109, 0.30); }
.fx-bump.mul { color: rgba(0, 200, 255, 0.95); border-color: rgba(0, 200, 255, 0.30); }
.fx-bump.info { color: rgba(238, 255, 248, 0.85); }

.fx-proj {
  position: fixed;
  z-index: 9999;
  transform: translate(-50%, -50%);
  font-size: 13px;
  font-weight: 800;
  letter-spacing: 0.4px;
  padding: 6px 10px;
  border-radius: 999px;
  border: 1px solid rgba(0, 255, 156, 0.22);
  background: rgba(0, 0, 0, 0.62);
  box-shadow: 0 0 18px rgba(0, 255, 156, 0.12);
  will-change: transform, opacity;
  transition: transform 520ms cubic-bezier(0.16, 1, 0.3, 1), opacity 260ms ease;
}
.fx-proj.pos { color: rgba(0, 255, 156, 0.95); }
.fx-proj.neg { color: rgba(255, 77, 109, 0.95); border-color: rgba(255, 77, 109, 0.30); }
.fx-proj.mul { color: rgba(0, 200, 255, 0.95); border-color: rgba(0, 200, 255, 0.30); }
.fx-proj.info { color: rgba(238, 255, 248, 0.85); }

.fx-burst {
  position: fixed;
  z-index: 9999;
  left: var(--x, 0px);
  top: var(--y, 0px);
  width: 4px;
  height: 4px;
  pointer-events: none;
}
.fx-burst span {
  position: absolute;
  left: 0;
  top: 0;
  width: 3px;
  height: 10px;
  border-radius: 999px;
  background: rgba(0, 255, 156, 0.95);
  box-shadow: 0 0 16px rgba(0, 255, 156, 0.20);
  transform: rotate(var(--a)) translateY(0px);
  animation: spark 520ms ease-out forwards;
  opacity: 0.95;
}
.fx-burst.neg span { background: rgba(255, 77, 109, 0.95); box-shadow: 0 0 16px rgba(255, 77, 109, 0.18); }
.fx-burst.mul span { background: rgba(0, 200, 255, 0.95); box-shadow: 0 0 16px rgba(0, 200, 255, 0.18); }

@keyframes spark {
  0% { transform: rotate(var(--a)) translateY(0px) scaleY(0.8); opacity: 0.95; }
  100% { transform: rotate(var(--a)) translateY(var(--d)) scaleY(1.0); opacity: 0.0; }
}

.deck-shake .deck-widget {
  animation: deckShake 420ms cubic-bezier(0.16, 1, 0.3, 1);
}

@keyframes bumpFloat {
  0%   { opacity: 0; transform: translate(-50%, -50%) scale(0.88); }
  12%  { opacity: 1; transform: translate(-50%, -50%) scale(1.02); }
  100% { opacity: 0; transform: translate(-50%, calc(-50% - 42px)) scale(1.00); }
}

@keyframes fxshake {
  0%   { transform: translate(var(--fx-tx, 0px), var(--fx-ty, 0px)) rotate(-0.9deg) scale(var(--fx-scale, 1)); }
  50%  { transform: translate(var(--fx-tx, 0px), var(--fx-ty, 0px)) rotate(0.9deg) scale(var(--fx-scale, 1)); }
  100% { transform: translate(var(--fx-tx, 0px), var(--fx-ty, 0px)) rotate(-0.9deg) scale(var(--fx-scale, 1)); }
}

@keyframes deckShake {
  0% { transform: translate(0px, 0px) rotate(0deg); }
  20% { transform: translate(-2px, 1px) rotate(-0.25deg); }
  40% { transform: translate(3px, -1px) rotate(0.25deg); }
  60% { transform: translate(-2px, 1px) rotate(-0.18deg); }
  100% { transform: translate(0px, 0px) rotate(0deg); }
}
.play-btn:active {
  transform: none;
}
.play-icon {
  font-size: 18px;
  color: rgba(0, 255, 156, 0.95);
  text-shadow: 0 0 22px rgba(0, 255, 156, 0.35);
}
.play-text {
  font-size: 14px;
  letter-spacing: 0.4px;
  text-transform: uppercase;
}

.hud {
  flex: 1;
  display: grid;
  grid-template-columns: 1.2fr 1fr;
  gap: 14px;
}

.hud-panel {
  padding: 6px 8px 6px;
}

.hud-title {
  margin: 0 0 6px 0;
  font-size: 12px;
  text-transform: uppercase;
  letter-spacing: 0.45px;
  color: var(--muted);
}

.scoreline {
  display: flex;
  justify-content: space-between;
  align-items: flex-end;
  gap: 14px;
  margin-bottom: 6px;
}

.scorebig {
  font-size: 24px;
  letter-spacing: 0.6px;
  color: rgba(238, 255, 248, 0.96);
  text-shadow: 0 0 22px rgba(0, 255, 156, 0.16), 0 0 64px rgba(0, 200, 255, 0.08);
}

.scoremeta {
  text-align: right;
}

.scoretarget {
  font-size: 14px;
  color: rgba(0, 255, 156, 0.88);
}

.progress {
  height: 10px;
  border-radius: 999px;
  border: 1px solid rgba(0, 255, 156, 0.18);
  background: rgba(0, 0, 0, 0.28);
  overflow: hidden;
  box-shadow: 0 0 0 1px rgba(0, 0, 0, 0.24) inset;
}

.progress-fill {
  height: 100%;
  border-radius: 999px;
  background: linear-gradient(90deg, rgba(0, 255, 156, 0.15), rgba(0, 200, 255, 0.55));
  box-shadow: 0 0 22px rgba(0, 255, 156, 0.14);
}

.progress-fill.low {
  background: linear-gradient(90deg, rgba(255, 77, 109, 0.55), rgba(255, 158, 0, 0.45));
}

.progress-fill.mid {
  background: linear-gradient(90deg, rgba(255, 158, 0, 0.55), rgba(0, 200, 255, 0.50));
}

.progress-fill.high {
  background: linear-gradient(90deg, rgba(0, 200, 255, 0.55), rgba(0, 255, 156, 0.70));
}

.progress-fill.clear {
  background: linear-gradient(90deg, rgba(0, 255, 156, 0.75), rgba(255, 77, 255, 0.55));
  box-shadow: 0 0 28px rgba(0, 255, 156, 0.22), 0 0 64px rgba(255, 77, 255, 0.10);
}

.bankline {
  display: flex;
  justify-content: space-between;
  align-items: baseline;
  margin-top: 8px;
}

.bankbig {
  font-size: 18px;
  color: rgba(238, 255, 248, 0.94);
  text-shadow: 0 0 18px rgba(0, 255, 156, 0.12);
}

.stats {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 10px 14px;
}

.stat {
  display: flex;
  justify-content: space-between;
  gap: 10px;
  font-size: 13px;
  color: var(--muted);
}

.stat strong {
  color: var(--text);
  text-shadow: 0 0 14px rgba(0, 255, 156, 0.14);
}

.kv {
  display: flex;
  justify-content: space-between;
  gap: 10px;
  font-size: 12px;
  color: var(--muted);
}

.kv code {
  color: rgba(0, 255, 156, 0.88);
  text-shadow: 0 0 18px rgba(0, 255, 156, 0.18);
}

.deck-widget {
  width: 238px;
  padding: 8px;
  position: relative;
  overflow: hidden;
}

.deck-title {
  margin: 0 0 6px 0;
  font-size: 12px;
  text-transform: uppercase;
  letter-spacing: 0.45px;
  color: var(--muted);
}

.deck-stack {
  position: relative;
  height: 58px;
  margin-top: 6px;
}

.deck-card {
  position: absolute;
  inset: 0;
  border-radius: 14px;
  border: 1px solid rgba(0, 255, 156, 0.20);
  background:
    linear-gradient(135deg, rgba(0, 255, 156, 0.10), rgba(0, 200, 255, 0.06) 45%, rgba(0, 0, 0, 0.35));
  box-shadow: 0 18px 44px rgba(0, 0, 0, 0.40);
}

.deck-card:nth-child(2) { transform: translate(10px, 9px); opacity: 0.55; }

.deck-meta {
  position: relative;
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 8px 12px;
  margin-top: 6px;
}

.pill {
  display: flex;
  justify-content: space-between;
  align-items: center;
  border-radius: 999px;
  border: 1px solid rgba(0, 255, 156, 0.18);
  background: rgba(0, 0, 0, 0.22);
  padding: 8px 10px;
  font-size: 12px;
  color: var(--muted);
}

.pill strong {
  color: var(--text);
}

.deckbar {
  border-radius: var(--radius);
  border: 1px solid rgba(0, 255, 156, 0.14);
  background: rgba(0, 0, 0, 0.18);
  padding: 12px;
  min-height: 0;
  position: relative;
  z-index: 8;
}

.handbar {
  border-radius: var(--radius);
  border: 1px solid rgba(0, 255, 156, 0.18);
  background: linear-gradient(180deg, rgba(0, 0, 0, 0.12), rgba(0, 0, 0, 0.28));
  padding: 12px;
  position: relative;
  z-index: 10;
  min-height: 0;
}

.hand-title {
  margin: 0 0 10px 0;
  font-size: 12px;
  text-transform: uppercase;
  letter-spacing: 0.45px;
  color: var(--muted);
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 10px;
}

.row-scroll {
  overflow-x: auto;
  overflow-y: visible;
  padding: 10px 0;
  scrollbar-gutter: stable both-edges;
  scrollbar-color: rgba(0, 255, 156, 0.35) rgba(0, 0, 0, 0.20);
  /* Prevent layout shift: row never shrinks below viewport */
  min-width: 100%;
}

.row {
  display: flex;
  gap: 12px;
  align-items: flex-end;
  /* Fixed layout: cards never wrap, row grows to fit all cards */
  flex-wrap: nowrap;
  width: fit-content;
  min-width: 100%;
}

/* Drop slots removed - drag onto cards directly for simpler, jitter-free layout */

.ghost-card {
  width: var(--card-w);
  min-width: var(--card-w);
  height: var(--card-h);
  border-radius: var(--card-r);
  border: 1px dashed rgba(0, 255, 156, 0.30);
  background:
    radial-gradient(220px 160px at 50% 30%, rgba(0, 255, 156, 0.06), transparent 60%),
    linear-gradient(135deg, rgba(0, 255, 156, 0.04), rgba(0, 200, 255, 0.02) 35%, rgba(0, 0, 0, 0.18));
  display: grid;
  place-items: center;
  gap: 8px;
  color: rgba(238, 255, 248, 0.72);
  box-shadow: 0 18px 44px rgba(0, 0, 0, 0.40);
  transition: transform 140ms ease, border-color 140ms ease, box-shadow 140ms ease;
}
.ghost-card:hover {
  transform: translateY(-2px);
  border-color: rgba(0, 255, 156, 0.55);
  box-shadow: 0 24px 56px rgba(0, 0, 0, 0.46), 0 0 44px rgba(0, 255, 156, 0.14);
}

@media (max-height: 820px) {
  .card-actions {
    bottom: 8px;
    left: 8px;
    right: 8px;
    gap: 6px;
  }
  .card-btn {
    padding: 7px 7px;
    border-radius: 11px;
  }
  .card-body {
    padding: 8px 10px 10px;
  }
  .card-script {
    margin-top: 8px;
    font-size: 11px;
  }
}
.ghost-plus {
  width: 44px;
  height: 44px;
  border-radius: 999px;
  display: grid;
  place-items: center;
  border: 1px solid rgba(0, 255, 156, 0.32);
  background: rgba(0, 0, 0, 0.22);
  font-size: 24px;
  color: rgba(0, 255, 156, 0.88);
  text-shadow: 0 0 22px rgba(0, 255, 156, 0.25);
}
.ghost-hint {
  font-size: 12px;
  color: rgba(238, 255, 248, 0.62);
}

.row-scroll::-webkit-scrollbar {
  height: 10px;
}
.row-scroll::-webkit-scrollbar-thumb {
  background: rgba(0, 255, 156, 0.25);
  border-radius: 999px;
}
.row-scroll::-webkit-scrollbar-track {
  background: rgba(0, 0, 0, 0.18);
  border-radius: 999px;
}

.card {
  width: var(--card-w);
  min-width: var(--card-w);
  height: var(--card-h);
  border-radius: var(--card-r);
  border: 1px solid rgba(0, 255, 156, 0.22);
  background:
    radial-gradient(260px 180px at 10% 20%, rgba(255, 77, 255, 0.14), transparent 58%),
    radial-gradient(240px 160px at 85% 18%, rgba(0, 200, 255, 0.12), transparent 60%),
    radial-gradient(220px 160px at 60% 110%, rgba(0, 255, 156, 0.10), transparent 62%),
    linear-gradient(135deg, rgba(0, 255, 156, 0.10), rgba(0, 200, 255, 0.06) 35%, rgba(0, 0, 0, 0.38));
  box-shadow: 0 22px 54px rgba(0, 0, 0, 0.50), 0 0 0 1px rgba(0, 255, 156, 0.05) inset;
  overflow: hidden;
  position: relative;
  z-index: 1;
  transition: transform 220ms cubic-bezier(0.16, 1, 0.3, 1), box-shadow 220ms cubic-bezier(0.16, 1, 0.3, 1), border-color 220ms ease, filter 220ms ease;
  will-change: transform;
}

.card::before {
  content: "";
  position: absolute;
  inset: -60px -60px auto -60px;
  height: 120px;
  background: radial-gradient(circle at 30% 30%, rgba(255, 255, 255, 0.10), transparent 60%);
  transform: rotate(-8deg);
  pointer-events: none;
}

.card:hover {
  transform: translateY(-6px) rotate(-0.5deg);
  border-color: rgba(0, 255, 156, 0.60);
  box-shadow: 0 28px 64px rgba(0, 0, 0, 0.58), 0 0 28px rgba(0, 255, 156, 0.16), 0 0 64px rgba(0, 200, 255, 0.10);
  filter: saturate(1.10) contrast(1.06);
}

.card.selected {
  /* Selected card stays visible even if focus is elsewhere. */
  border-color: rgba(0, 255, 156, 0.78);
  box-shadow:
    0 26px 66px rgba(0, 0, 0, 0.60),
    0 0 0 2px rgba(0, 255, 156, 0.10) inset,
    0 0 28px rgba(0, 255, 156, 0.18);
  z-index: 220;
}

.card.selected.focused {
  /* Make focus unmistakable (keyboard + mouse): BIG glow + lift */
  border-color: rgba(0, 255, 156, 0.98);
  transform: translateY(-10px) rotate(-0.6deg) scale(1.045);
  box-shadow:
    0 34px 90px rgba(0, 0, 0, 0.68),
    0 0 0 2px rgba(0, 255, 156, 0.16) inset,
    0 0 0 2px rgba(0, 255, 156, 0.55),
    0 0 42px rgba(0, 255, 156, 0.34),
    0 0 120px rgba(0, 200, 255, 0.18);
  z-index: 800;
}

.card.selected.focused::after {
  content: "";
  position: absolute;
  inset: -4px;
  border-radius: calc(var(--card-r) + 6px);
  border: 3px solid rgba(0, 255, 156, 0.78);
  box-shadow:
    0 0 28px rgba(0, 255, 156, 0.32),
    0 0 82px rgba(0, 200, 255, 0.18);
  pointer-events: none;
  animation: selectedPulse 820ms ease-in-out infinite;
}

@keyframes selectedPulse {
  0%, 100% { opacity: 0.60; filter: saturate(1.05) brightness(1.02); }
  50% { opacity: 1.00; filter: saturate(1.25) brightness(1.06); }
}

.card.drop-in {
  animation: dropIn 220ms cubic-bezier(0.16, 1, 0.3, 1);
}

@keyframes dropIn {
  0% { transform: translateY(-6px) scale(0.98); }
  100% { transform: translateY(0px) scale(1.0); }
}

.card.dragging {
  /* Style only; positioning is handled by inline `left/top` while unparented. */
  transform: rotate(-1.2deg) scale(1.02);
  z-index: 20001;
  pointer-events: none;
  transition: none;
  filter: saturate(1.15) contrast(1.08);
  box-shadow: 0 32px 86px rgba(0, 0, 0, 0.66), 0 0 60px rgba(0, 255, 156, 0.18), 0 0 92px rgba(0, 200, 255, 0.10);
}

.app.is-dragging .card:hover {
  transform: none;
}

.app.is-dragging .card.selected {
  /* During drag we disable hover transforms; keep selection readable but stable. */
  transform: none;
}

.card.pop-in {
  animation: popIn 520ms cubic-bezier(0.16, 1, 0.3, 1);
}

@keyframes popIn {
  0% { transform: translateY(14px) scale(0.82); opacity: 0.0; filter: saturate(1.4) contrast(1.1); }
  50% { transform: translateY(-4px) scale(1.03); opacity: 1.0; }
  100% { transform: translateY(0px) scale(1.0); opacity: 1.0; }
}

/* Kind-specific color accents */
.card.kind-score { border-color: rgba(255, 158, 0, 0.40); box-shadow: 0 22px 54px rgba(0,0,0,0.50), 0 0 0 1px rgba(255,158,0,0.05) inset; }
.card.kind-economy { border-color: rgba(0, 200, 255, 0.40); box-shadow: 0 22px 54px rgba(0,0,0,0.50), 0 0 0 1px rgba(0,200,255,0.06) inset; }
.card.kind-control { border-color: rgba(0, 255, 156, 0.40); box-shadow: 0 22px 54px rgba(0,0,0,0.50), 0 0 0 1px rgba(0,255,156,0.06) inset; }
.card.kind-meta { border-color: rgba(255, 77, 255, 0.40); box-shadow: 0 22px 54px rgba(0,0,0,0.50), 0 0 0 1px rgba(255,77,255,0.06) inset; }

/* Theme overrides: punch up terminal + magic vibes */
.app.theme-terminal .brand-title {
  text-shadow: 0 0 20px rgba(124, 255, 0, 0.26);
}

.app.theme-terminal .card {
  border-color: rgba(124, 255, 0, 0.24);
  box-shadow: 0 22px 54px rgba(0, 0, 0, 0.50), 0 0 0 1px rgba(124, 255, 0, 0.06) inset;
}

.app.theme-terminal .card:hover {
  border-color: rgba(124, 255, 0, 0.78);
  box-shadow: 0 28px 64px rgba(0, 0, 0, 0.58), 0 0 30px rgba(124, 255, 0, 0.20), 0 0 64px rgba(0, 255, 156, 0.10);
}

.app.theme-magic .brand-title {
  text-shadow: 0 0 18px rgba(255, 77, 255, 0.24), 0 0 44px rgba(122, 92, 255, 0.14);
}

.app.theme-magic .card {
  border-color: rgba(255, 77, 255, 0.24);
  background:
    linear-gradient(135deg, rgba(255, 77, 255, 0.12), rgba(122, 92, 255, 0.10) 38%, rgba(0, 0, 0, 0.42));
  box-shadow: 0 22px 54px rgba(0, 0, 0, 0.52), 0 0 0 1px rgba(255, 77, 255, 0.06) inset;
}

.app.theme-magic .card:hover {
  border-color: rgba(255, 77, 255, 0.80);
  box-shadow: 0 28px 64px rgba(0, 0, 0, 0.58), 0 0 34px rgba(255, 77, 255, 0.20), 0 0 70px rgba(122, 92, 255, 0.14);
}

.card-top {
  position: absolute;
  left: 10px;
  right: 10px;
  top: 10px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  pointer-events: none;
  z-index: 5;
}

.card-index {
  pointer-events: none;
  font-size: 11px;
  letter-spacing: 0.4px;
  color: rgba(238, 255, 248, 0.75);
  border-radius: 999px;
  padding: 5px 9px;
  border: 1px solid rgba(238, 255, 248, 0.12);
  background: rgba(0, 0, 0, 0.30);
  box-shadow: 0 0 18px rgba(0, 255, 156, 0.08);
}

.card-docs {
  pointer-events: auto;
  border-radius: 999px;
  border: 1px solid rgba(238, 255, 248, 0.14);
  background: rgba(0, 0, 0, 0.32);
  color: rgba(238, 255, 248, 0.90);
  width: 32px;
  height: 32px;
  cursor: pointer;
  transition: transform 160ms cubic-bezier(0.16, 1, 0.3, 1), border-color 160ms ease, box-shadow 160ms ease, filter 160ms ease;
}
.card-docs:hover { transform: translateY(-1px) scale(1.03); border-color: rgba(255, 77, 255, 0.55); box-shadow: 0 0 18px rgba(255, 77, 255, 0.12); filter: saturate(1.15); }

.ghost-docs {
  opacity: 0;
  pointer-events: none;
}

.card-actions {
  position: absolute;
  left: 10px;
  right: 10px;
  bottom: 10px;
  display: grid;
  grid-template-columns: 1fr 1fr 1fr;
  gap: 8px;
}

.card-btn {
  border-radius: 12px;
  border: 1px solid rgba(0, 255, 156, 0.18);
  background: rgba(0, 0, 0, 0.30);
  color: rgba(238, 255, 248, 0.86);
  padding: 8px 8px;
  cursor: pointer;
  transition: transform 120ms ease, border-color 120ms ease, box-shadow 120ms ease;
}

.card-btn:hover {
  transform: translateY(-1px);
  border-color: rgba(0, 255, 156, 0.55);
  box-shadow: 0 0 16px rgba(0, 255, 156, 0.10);
}

.card-art {
  margin: 10px 10px 0;
  height: calc(var(--card-h) * 0.36);
  border-radius: 14px;
  border: 1px solid rgba(0, 255, 156, 0.18);
  background:
    radial-gradient(90px 60px at 20% 35%, rgba(0, 255, 156, 0.22), transparent 60%),
    radial-gradient(100px 70px at 70% 30%, rgba(0, 200, 255, 0.18), transparent 62%),
    linear-gradient(135deg, rgba(0, 0, 0, 0.15), rgba(0, 0, 0, 0.42));
  box-shadow: 0 0 0 1px rgba(0, 0, 0, 0.30) inset;
  position: relative;
  overflow: hidden;
}

.card-art::after {
  content: "";
  position: absolute;
  inset: 0;
  opacity: 0.22;
  background: repeating-linear-gradient(
    to bottom,
    rgba(255, 255, 255, 0.05),
    rgba(255, 255, 255, 0.05) 1px,
    rgba(0, 0, 0, 0.00) 2px,
    rgba(0, 0, 0, 0.00) 4px
  );
}

.card-body {
  padding: 10px 12px 12px;
}

.card-title {
  margin: 0;
  font-size: 13px;
  letter-spacing: 0.45px;
  text-transform: uppercase;
}

.card-sub {
  margin-top: 6px;
  font-size: 12px;
  color: var(--muted);
}

.card-script {
  margin-top: 10px;
  border-radius: 12px;
  border: 1px solid rgba(0, 255, 156, 0.14);
  background: rgba(0, 0, 0, 0.28);
  padding: 8px 9px;
  font-size: 12px;
  color: rgba(0, 255, 156, 0.86);
  text-shadow: 0 0 14px rgba(0, 255, 156, 0.14);
  overflow: hidden;
  white-space: nowrap;
  text-overflow: ellipsis;
}

.tok {
  white-space: pre;
}

.tok-ident {
  color: rgba(0, 200, 255, 0.92);
  text-shadow: 0 0 14px rgba(0, 200, 255, 0.14);
}

.tok-fn {
  color: rgba(255, 158, 0, 0.92);
  text-shadow: 0 0 14px rgba(255, 158, 0, 0.12);
}

.tok-reg {
  color: rgba(0, 200, 255, 0.94);
  text-shadow: 0 0 14px rgba(0, 200, 255, 0.14);
}

.tok-acc {
  color: rgba(255, 77, 255, 0.92);
  text-shadow: 0 0 18px rgba(255, 77, 255, 0.14);
}

.tok-num {
  color: rgba(0, 255, 156, 0.92);
}

.tok-op {
  color: rgba(255, 158, 0, 0.90);
}

.tok-punct {
  color: rgba(238, 255, 248, 0.60);
}

.tok-raw {
  color: rgba(0, 255, 156, 0.86);
}

.card-badge {
  position: absolute;
  top: 10px;
  right: 10px;
  border-radius: 999px;
  border: 1px solid rgba(0, 255, 156, 0.18);
  background: rgba(0, 0, 0, 0.22);
  padding: 7px 9px;
  font-size: 11px;
  color: var(--muted);
}

.empty {
  padding: 12px;
  color: var(--muted);
  font-size: 12px;
}

/* Legacy empty dropzones were replaced by ghost-card placeholders to avoid layout jumps. */

.trace-list {
  max-height: 56vh;
  overflow: auto;
  padding-right: 4px;
  font-size: 11px;
  color: var(--muted);
  border-radius: 12px;
}

.trace-item {
  padding: 8px 8px;
  border-bottom: 1px solid rgba(0, 255, 156, 0.10);
}

.trace-item:last-child {
  border-bottom: none;
}

.trace-item.error {
  color: rgba(255, 77, 109, 0.92);
}

.trace-item.call {
  color: rgba(0, 200, 255, 0.90);
}

.trace-item.effect {
  color: rgba(0, 255, 156, 0.92);
}

.modal-overlay {
  position: fixed;
  inset: 0;
  z-index: 50;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 18px;
  background: rgba(0, 0, 0, 0.62);
  backdrop-filter: blur(10px);
}

.modal {
  width: min(980px, 96vw);
  max-height: 92vh;
  overflow: hidden;
  border-color: rgba(0, 255, 156, 0.28);
}

.modal-header {
  padding: 14px 14px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 12px;
  border-bottom: 1px solid rgba(0, 255, 156, 0.12);
  background: rgba(0, 0, 0, 0.18);
}

.modal-title {
  display: flex;
  align-items: center;
  gap: 10px;
  font-size: 14px;
  text-transform: uppercase;
  letter-spacing: 0.55px;
}

.modal-glyph {
  color: rgba(0, 255, 156, 0.90);
  text-shadow: 0 0 18px rgba(0, 255, 156, 0.18);
}

.modal-body {
  padding: 14px;
  overflow: auto;
  max-height: calc(92vh - 62px);
}

.kdoc {
  color: var(--muted);
  font-size: 13px;
  line-height: 1.55;
  margin-bottom: 14px;
}

.kcard-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.kcard {
  border-radius: 14px;
  border: 1px solid rgba(0, 255, 156, 0.14);
  background: rgba(0, 0, 0, 0.18);
  overflow: hidden;
}

.kcard.highlight {
  border-color: rgba(0, 255, 156, 0.62);
  box-shadow: 0 0 0 1px rgba(0, 255, 156, 0.10) inset, 0 0 24px rgba(0, 255, 156, 0.16);
}

.kcard-head {
  display: grid;
  grid-template-columns: 44px 1fr 220px;
  gap: 12px;
  align-items: center;
  padding: 12px;
  border-bottom: 1px solid rgba(0, 255, 156, 0.10);
  background: rgba(0, 0, 0, 0.22);
}

.kcard-icon {
  display: grid;
  place-items: center;
  width: 44px;
  height: 44px;
  border-radius: 12px;
  border: 1px solid rgba(0, 255, 156, 0.18);
  background: rgba(0, 0, 0, 0.20);
  color: rgba(0, 255, 156, 0.86);
}

.kcard-title {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.kcard-name {
  font-size: 13px;
  letter-spacing: 0.45px;
  text-transform: uppercase;
}

.kcard-id {
  font-size: 11px;
  color: var(--muted);
}

.kcard-script {
  justify-self: end;
  font-size: 12px;
  color: rgba(0, 255, 156, 0.86);
  border-radius: 12px;
  border: 1px solid rgba(0, 255, 156, 0.14);
  background: rgba(0, 0, 0, 0.24);
  padding: 8px 10px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.kcard-doc {
  margin: 0;
  padding: 12px;
  white-space: pre-wrap;
  color: var(--muted);
  font-size: 12px;
  line-height: 1.55;
}
"#;
