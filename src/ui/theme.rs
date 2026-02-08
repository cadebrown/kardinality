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
  /* Allow FLIP animations to travel between Hand/Deck without being clipped. */
  overflow-y: visible;
  overflow-x: hidden;
}

/* Floating drag layer: dragged cards are truly unparented + fixed-positioned here. */
.drag-layer {
  position: fixed;
  inset: 0;
  pointer-events: none;
  z-index: 20000;
}

/* Focus halo layer: renders "outside" scroll clipping */
.focus-layer {
  position: fixed;
  inset: 0;
  pointer-events: none;
  z-index: 15000;
}

.focus-halo {
  position: fixed;
  border-radius: calc(var(--card-r) + 8px);
  border: 2px solid rgba(0, 255, 156, 0.38);
  box-shadow:
    0 0 0 2px rgba(0, 255, 156, 0.12) inset,
    0 0 34px rgba(0, 255, 156, 0.22),
    0 0 110px rgba(0, 200, 255, 0.14);
  transform: scale(1.06);
  transform-origin: 50% 50%;
  opacity: 0.95;
}

.focus-halo.deck { border-color: rgba(0, 200, 255, 0.38); box-shadow: 0 0 34px rgba(0, 200, 255, 0.20), 0 0 110px rgba(0, 255, 156, 0.12); }
.focus-halo.hand { border-color: rgba(0, 255, 156, 0.40); }

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
  /* Space so card glows don't get hard-clipped at edges */
  padding: 14px 18px;
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
  /* Space so card glows / selected-scale have breathing room */
  padding: 14px 18px;
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

/* === Radical 2026 redesign layer === */
:root {
  --bg0: #090b12;
  --bg1: #0f1422;
  --panel: rgba(15, 20, 32, 0.86);
  --panel2: rgba(12, 16, 28, 0.90);
  --border: rgba(112, 184, 255, 0.22);
  --border-strong: rgba(64, 233, 255, 0.62);
  --accent: #40e9ff;
  --accent2: #ff8f2d;
  --danger: #ff5f7f;
  --text: rgba(245, 250, 255, 0.95);
  --muted: rgba(208, 224, 240, 0.68);
  --shadow: rgba(0, 0, 0, 0.52);
  --radius: 22px;
  --radius-sm: 14px;
  --card-r: 20px;
  --card-w: clamp(158px, 16.4vw, 202px);
  --card-h: clamp(236px, 30vh, 336px);
}

body {
  font-family: "Avenir Next", "Futura", "Trebuchet MS", "Segoe UI", sans-serif;
  background:
    radial-gradient(1300px 900px at 10% -5%, rgba(64, 233, 255, 0.16), transparent 60%),
    radial-gradient(1100px 760px at 88% 8%, rgba(255, 143, 45, 0.12), transparent 62%),
    radial-gradient(900px 620px at 50% 120%, rgba(255, 95, 127, 0.10), transparent 64%),
    linear-gradient(165deg, #090b12 0%, #0c1220 38%, #14182a 100%);
}

.app {
  grid-template-columns: 324px 1fr;
}

.app::before {
  opacity: 0.08;
  background:
    linear-gradient(120deg, rgba(255, 255, 255, 0.05), transparent 35%),
    repeating-linear-gradient(
      to bottom,
      rgba(255, 255, 255, 0.025),
      rgba(255, 255, 255, 0.025) 1px,
      rgba(0, 0, 0, 0.0) 2px,
      rgba(0, 0, 0, 0.0) 5px
    );
}

.app::after {
  opacity: 0.74;
  background:
    radial-gradient(1200px 900px at 50% 50%, rgba(64, 233, 255, 0.04), transparent 60%),
    linear-gradient(120deg, rgba(255, 143, 45, 0.05), transparent 24%);
}

.sidebar {
  padding: 18px 14px;
  border-right: 1px solid rgba(92, 166, 255, 0.26);
  background:
    radial-gradient(360px 280px at 20% 2%, rgba(64, 233, 255, 0.14), transparent 62%),
    linear-gradient(180deg, rgba(12, 16, 28, 0.96), rgba(9, 12, 22, 0.92));
  box-shadow: 24px 0 60px rgba(0, 0, 0, 0.22);
}

.brand-title {
  font-size: 22px;
  letter-spacing: 1px;
  font-weight: 900;
  text-transform: uppercase;
  text-shadow: 0 0 22px rgba(64, 233, 255, 0.26);
}

.brand-subtitle {
  font-size: 11px;
  color: rgba(208, 224, 240, 0.70);
  letter-spacing: 0.28px;
}

.panel {
  border-radius: var(--radius);
  border: 1px solid rgba(120, 180, 255, 0.20);
  background:
    linear-gradient(140deg, rgba(255, 255, 255, 0.04), transparent 45%),
    var(--panel);
  box-shadow:
    0 18px 44px rgba(0, 0, 0, 0.40),
    0 0 0 1px rgba(64, 233, 255, 0.06) inset;
}

.tabs {
  gap: 8px;
}

.tab {
  border-color: rgba(120, 180, 255, 0.28);
  background: linear-gradient(180deg, rgba(255, 255, 255, 0.05), rgba(0, 0, 0, 0.22));
  color: rgba(208, 224, 240, 0.78);
  font-weight: 700;
  letter-spacing: 0.24px;
}

.tab.active {
  border-color: rgba(64, 233, 255, 0.66);
  background: linear-gradient(180deg, rgba(64, 233, 255, 0.20), rgba(0, 0, 0, 0.20));
  color: rgba(245, 250, 255, 0.96);
  box-shadow: 0 0 24px rgba(64, 233, 255, 0.18);
}

.btn {
  border-color: rgba(120, 180, 255, 0.34);
  background: linear-gradient(180deg, rgba(64, 233, 255, 0.14), rgba(0, 0, 0, 0.24));
  font-weight: 700;
  letter-spacing: 0.2px;
}

.btn.secondary {
  border-color: rgba(255, 143, 45, 0.38);
  background: linear-gradient(180deg, rgba(255, 143, 45, 0.14), rgba(0, 0, 0, 0.24));
}

.main {
  padding: 18px;
  gap: 12px;
}

.topbar {
  grid-template-columns: minmax(360px, 430px) 1fr 252px;
  gap: 12px;
}

.run-pane {
  border: 1px solid rgba(120, 180, 255, 0.24);
  background:
    radial-gradient(320px 220px at 14% 12%, rgba(64, 233, 255, 0.16), transparent 60%),
    linear-gradient(180deg, rgba(255, 255, 255, 0.04), rgba(0, 0, 0, 0.22));
}

.play-btn {
  border-color: rgba(64, 233, 255, 0.50);
  border-radius: 16px;
  background:
    radial-gradient(220px 120px at 16% 10%, rgba(64, 233, 255, 0.20), transparent 60%),
    linear-gradient(180deg, rgba(255, 255, 255, 0.06), rgba(0, 0, 0, 0.26));
}

.play-btn:hover {
  transform: translateY(-2px);
  border-color: rgba(64, 233, 255, 0.72);
}

.shop-btn {
  border-color: rgba(255, 143, 45, 0.46);
  background:
    radial-gradient(220px 120px at 18% 12%, rgba(255, 143, 45, 0.19), transparent 60%),
    linear-gradient(180deg, rgba(255, 255, 255, 0.06), rgba(0, 0, 0, 0.26));
}

.shop-btn:hover {
  transform: translateY(-2px);
  border-color: rgba(255, 143, 45, 0.72);
}

.play-text {
  letter-spacing: 0.8px;
  font-weight: 800;
}

.run-strip {
  border-color: rgba(120, 180, 255, 0.22);
  background: rgba(7, 10, 18, 0.52);
}

.strip-item + .strip-item {
  border-left: 1px solid rgba(120, 180, 255, 0.16);
}

.strip-k {
  color: rgba(208, 224, 240, 0.64);
}

.strip-v {
  font-size: 14px;
  font-weight: 900;
}

.run-progress {
  border-color: rgba(120, 180, 255, 0.24);
}

.run-progress-fill {
  background: linear-gradient(90deg, rgba(64, 233, 255, 0.62), rgba(255, 143, 45, 0.56), rgba(255, 95, 127, 0.60));
  box-shadow: 0 0 20px rgba(64, 233, 255, 0.20);
}

.content {
  gap: 12px;
}

.handbar,
.deckbar {
  border-radius: 20px;
  border: 1px solid rgba(120, 180, 255, 0.20);
  background:
    radial-gradient(280px 160px at 12% 0%, rgba(64, 233, 255, 0.12), transparent 62%),
    linear-gradient(170deg, rgba(255, 255, 255, 0.04), rgba(0, 0, 0, 0.24));
}

.hand-title {
  font-size: 13px;
  letter-spacing: 0.7px;
  font-weight: 900;
}

.pile-widget {
  border-radius: 20px;
  border-color: rgba(255, 143, 45, 0.26);
  background:
    radial-gradient(260px 180px at 70% 8%, rgba(255, 143, 45, 0.14), transparent 62%),
    linear-gradient(180deg, rgba(255, 255, 255, 0.04), rgba(0, 0, 0, 0.24));
}

.deck-widget {
  border-radius: 20px;
  border-color: rgba(64, 233, 255, 0.30);
}

.card {
  border-radius: var(--card-r);
  border: 1px solid rgba(130, 190, 255, 0.26);
  background:
    radial-gradient(220px 140px at 18% 14%, rgba(64, 233, 255, 0.16), transparent 60%),
    radial-gradient(220px 140px at 84% 18%, rgba(255, 143, 45, 0.15), transparent 62%),
    linear-gradient(160deg, rgba(255, 255, 255, 0.06), rgba(8, 12, 22, 0.88));
  box-shadow:
    0 24px 54px rgba(0, 0, 0, 0.52),
    0 0 0 1px rgba(120, 180, 255, 0.05) inset;
}

.card::before {
  opacity: 0.62;
  background:
    radial-gradient(circle at 22% 30%, rgba(255, 255, 255, 0.18), transparent 62%),
    radial-gradient(circle at 78% 10%, rgba(64, 233, 255, 0.15), transparent 55%);
}

.card:hover {
  transform: translateY(-8px) rotate(-0.55deg);
  border-color: rgba(64, 233, 255, 0.72);
  box-shadow:
    0 28px 64px rgba(0, 0, 0, 0.58),
    0 0 36px rgba(64, 233, 255, 0.16),
    0 0 70px rgba(255, 143, 45, 0.10);
}

.card-top {
  top: 9px;
  left: 9px;
  right: 9px;
}

.card-top-left {
  display: flex;
  align-items: center;
  gap: 6px;
  min-width: 0;
}

.card-index {
  padding: 4px 8px;
  border-color: rgba(130, 190, 255, 0.28);
  background: rgba(7, 11, 20, 0.54);
  font-weight: 700;
}

.card-kind-tag {
  display: inline-flex;
  align-items: center;
  gap: 5px;
  padding: 4px 8px;
  border-radius: 999px;
  border: 1px solid rgba(130, 190, 255, 0.28);
  background: rgba(7, 11, 20, 0.56);
  color: rgba(232, 244, 255, 0.90);
  min-width: 0;
}

.kind-glyph {
  font-size: 11px;
}

.kind-name {
  font-size: 10px;
  letter-spacing: 0.5px;
  text-transform: uppercase;
  white-space: nowrap;
}

.card-docs {
  border-color: rgba(130, 190, 255, 0.28);
  background: rgba(7, 11, 20, 0.56);
}

.card-art {
  margin: 48px 10px 0;
  height: calc(var(--card-h) * 0.29);
  border-radius: 16px;
  border: 1px solid rgba(130, 190, 255, 0.26);
  background:
    radial-gradient(120px 80px at 22% 34%, rgba(64, 233, 255, 0.24), transparent 62%),
    radial-gradient(130px 90px at 80% 20%, rgba(255, 143, 45, 0.20), transparent 60%),
    linear-gradient(145deg, rgba(255, 255, 255, 0.06), rgba(8, 10, 18, 0.72));
  display: grid;
  grid-template-rows: 1fr auto;
  align-items: center;
  justify-items: center;
  padding: 8px;
  gap: 8px;
}

.card-main-icon {
  font-size: 34px;
  line-height: 1;
  letter-spacing: 1px;
  color: rgba(245, 250, 255, 0.94);
  text-shadow: 0 0 28px rgba(64, 233, 255, 0.34), 0 0 42px rgba(255, 143, 45, 0.18);
}

.card-fx-ribbon {
  width: 100%;
  display: flex;
  gap: 5px;
  align-items: center;
  justify-content: center;
  flex-wrap: wrap;
}

.fx-dot {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  border-radius: 999px;
  border: 1px solid rgba(130, 190, 255, 0.30);
  background: rgba(8, 12, 22, 0.68);
  padding: 3px 6px;
  font-size: 10px;
  color: rgba(238, 247, 255, 0.92);
}

.fx-glyph {
  line-height: 1;
}

.fx-short {
  letter-spacing: 0.45px;
  text-transform: uppercase;
  font-weight: 800;
}

.fx-score {
  border-color: rgba(255, 193, 77, 0.58);
}

.fx-economy {
  border-color: rgba(97, 231, 201, 0.56);
}

.fx-control {
  border-color: rgba(64, 233, 255, 0.58);
}

.fx-meta {
  border-color: rgba(255, 122, 193, 0.58);
}

.fx-more,
.fx-unknown {
  border-color: rgba(208, 224, 240, 0.32);
}

.card-body {
  padding: 8px 11px 12px;
}

.card-title {
  font-size: 13px;
  letter-spacing: 0.7px;
  font-weight: 900;
  text-transform: uppercase;
}

.card-sub {
  margin-top: 4px;
  font-size: 11px;
  letter-spacing: 0.22px;
}

.card-script {
  margin-top: 8px;
  border-radius: 12px;
  border-color: rgba(130, 190, 255, 0.20);
  background: rgba(7, 11, 20, 0.58);
  font-family: "IBM Plex Mono", "SFMono-Regular", Menlo, Monaco, Consolas, monospace;
}

.card-fx-row {
  margin-top: 8px;
  display: flex;
  gap: 6px;
  flex-wrap: wrap;
}

.card-fx-chip {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  border-radius: 999px;
  border: 1px solid rgba(130, 190, 255, 0.30);
  background: rgba(8, 12, 22, 0.62);
  padding: 4px 7px;
  font-size: 10px;
  color: rgba(236, 246, 255, 0.92);
}

.chip-icon {
  line-height: 1;
}

.chip-label {
  text-transform: uppercase;
  letter-spacing: 0.42px;
  font-weight: 800;
}

.card-actions {
  bottom: 8px;
  left: 8px;
  right: 8px;
  gap: 7px;
}

.card-btn {
  border-color: rgba(130, 190, 255, 0.28);
  background: rgba(8, 12, 22, 0.56);
  font-weight: 800;
}

.card-btn:hover {
  transform: translateY(-1px);
  border-color: rgba(64, 233, 255, 0.66);
}

.card.kind-score {
  border-color: rgba(255, 193, 77, 0.54);
}

.card.kind-economy {
  border-color: rgba(97, 231, 201, 0.52);
}

.card.kind-control {
  border-color: rgba(64, 233, 255, 0.52);
}

.card.kind-meta {
  border-color: rgba(255, 122, 193, 0.52);
}

.modal {
  border-color: rgba(120, 180, 255, 0.30);
  background:
    radial-gradient(400px 220px at 20% 0%, rgba(64, 233, 255, 0.12), transparent 62%),
    rgba(10, 14, 24, 0.96);
}

.kcard {
  border-color: rgba(130, 190, 255, 0.22);
}

.kcard-head {
  grid-template-columns: 44px 1fr 220px;
}

.kcard-signals {
  padding: 10px 12px 0;
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
}

.kcard-kind,
.kcard-fx {
  display: inline-flex;
  align-items: center;
  gap: 5px;
  border-radius: 999px;
  border: 1px solid rgba(130, 190, 255, 0.28);
  background: rgba(8, 12, 22, 0.54);
  padding: 4px 8px;
  font-size: 10px;
  color: rgba(232, 244, 255, 0.90);
  text-transform: uppercase;
  letter-spacing: 0.4px;
  font-weight: 800;
}

.kcard-fx-icon,
.kcard-kind-icon {
  line-height: 1;
}

.puzzle-hint-hero {
  margin-top: 10px;
  min-height: clamp(180px, 31vh, 340px);
  border-radius: 22px;
  border: 2px solid rgba(64, 233, 255, 0.58);
  background:
    radial-gradient(280px 170px at 16% 10%, rgba(64, 233, 255, 0.24), transparent 64%),
    radial-gradient(240px 160px at 86% 16%, rgba(255, 143, 45, 0.22), transparent 62%),
    linear-gradient(155deg, rgba(255, 255, 255, 0.08), rgba(7, 12, 24, 0.84));
  clip-path: polygon(0 6%, 8% 0, 92% 0, 100% 7%, 100% 93%, 92% 100%, 8% 100%, 0 94%);
  box-shadow:
    0 24px 56px rgba(0, 0, 0, 0.46),
    0 0 34px rgba(64, 233, 255, 0.20),
    inset 0 0 0 1px rgba(255, 255, 255, 0.08);
  padding: 14px 16px 18px;
  display: flex;
  flex-direction: column;
  justify-content: center;
  gap: 10px;
  position: relative;
  overflow: hidden;
}

.puzzle-hint-hero::before {
  content: "";
  position: absolute;
  inset: 0;
  background:
    linear-gradient(90deg, rgba(255, 255, 255, 0.00), rgba(255, 255, 255, 0.12), rgba(255, 255, 255, 0.00));
  transform: translateX(-120%);
  animation: puzzleHintSweep 4.2s ease-in-out infinite;
  pointer-events: none;
}

.puzzle-hint-kicker {
  font-size: 14px;
  letter-spacing: 1.2px;
  text-transform: uppercase;
  font-weight: 900;
  color: rgba(255, 143, 45, 0.96);
  text-shadow: 0 0 16px rgba(255, 143, 45, 0.32);
}

.puzzle-hint-text {
  margin: 0;
  font-size: clamp(18px, 1.72vw, 24px);
  line-height: 1.34;
  letter-spacing: 0.24px;
  font-weight: 750;
  color: rgba(247, 251, 255, 0.98);
  text-wrap: pretty;
  text-shadow: 0 0 24px rgba(64, 233, 255, 0.18);
}

.puzzle-message-banner {
  margin-top: 10px;
  border-radius: 16px;
  border: 1px solid rgba(120, 180, 255, 0.34);
  background: rgba(8, 13, 23, 0.72);
  color: rgba(235, 246, 255, 0.92);
  padding: 10px 12px;
  font-size: 13px;
  line-height: 1.45;
}

@keyframes puzzleHintSweep {
  0% { transform: translateX(-120%); opacity: 0; }
  16% { opacity: 0.24; }
  54% { opacity: 0.20; }
  100% { transform: translateX(120%); opacity: 0; }
}

@media (max-width: 1100px) {
  .app {
    grid-template-columns: 1fr;
    grid-template-rows: auto 1fr;
  }

  .sidebar {
    max-height: 42vh;
    overflow: auto;
    border-right: none;
    border-bottom: 1px solid rgba(120, 180, 255, 0.26);
  }

  .topbar {
    grid-template-columns: 1fr;
  }

  .deck-widget {
    width: 100%;
  }

  .handrow {
    flex-direction: column;
  }

  .pile-widget {
    width: 100%;
    flex: 0 0 auto;
  }

  .puzzle-hint-hero {
    min-height: clamp(132px, 23vh, 240px);
  }

  .puzzle-hint-text {
    font-size: clamp(16px, 4.4vw, 21px);
  }
}

/* =======================================================================
   GRID REBOOT V2
   Intent: compact command-center UI with bold signal colors and clearer flow
   ======================================================================= */

:root {
  --card-w: clamp(132px, 14.2vw, 172px);
  --card-h: clamp(166px, 23.5vh, 246px);
  --layout-gap: 12px;
  --panel-radius: 16px;
  --grid-border: rgba(140, 182, 255, 0.26);
  --panel-bg: linear-gradient(165deg, rgba(8, 13, 24, 0.92), rgba(6, 10, 19, 0.9));
  --text: rgba(236, 244, 255, 0.95);
  --muted: rgba(190, 208, 232, 0.78);
  --accent: #4dd6ff;
  --accent2: #7dff74;
  --accent3: #ffcc6e;
}

body {
  font-family: "Space Grotesk", "Sora", "IBM Plex Sans", "Avenir Next", "Segoe UI", sans-serif;
  background:
    radial-gradient(1200px 740px at -10% -20%, rgba(95, 210, 255, 0.16), transparent 60%),
    radial-gradient(980px 680px at 106% 6%, rgba(125, 255, 116, 0.14), transparent 62%),
    radial-gradient(760px 520px at 50% 120%, rgba(255, 204, 110, 0.10), transparent 70%),
    linear-gradient(180deg, #060b18, #050912 62%, #04070f);
}

.app {
  height: 100vh;
  padding: var(--layout-gap);
  gap: var(--layout-gap);
  display: grid;
  grid-template-columns: minmax(276px, 320px) minmax(0, 1fr);
  grid-template-rows: minmax(0, 1fr);
  color: var(--text);
}

.app::before {
  opacity: 0.12;
  background:
    linear-gradient(rgba(255, 255, 255, 0.02) 1px, transparent 1px),
    linear-gradient(90deg, rgba(255, 255, 255, 0.02) 1px, transparent 1px);
  background-size: 34px 34px;
  animation: none;
}

.app::after {
  opacity: 0.55;
  background:
    radial-gradient(ellipse at top, rgba(97, 227, 255, 0.07), transparent 60%),
    radial-gradient(ellipse at bottom, rgba(146, 255, 98, 0.05), transparent 64%);
  animation: none;
}

.app.theme-terminal {
  --accent: #7dff74;
  --accent2: #6be6ff;
  --accent3: #ffe46e;
  --grid-border: rgba(125, 255, 116, 0.26);
}

.app.theme-magic {
  --accent: #ff89d1;
  --accent2: #8d8bff;
  --accent3: #ffb76b;
  --grid-border: rgba(255, 137, 209, 0.26);
}

.panel {
  border: 1px solid var(--grid-border);
  border-radius: var(--panel-radius);
  background: var(--panel-bg);
  box-shadow:
    0 10px 26px rgba(0, 0, 0, 0.35),
    inset 0 1px 0 rgba(255, 255, 255, 0.04);
}

.sidebar {
  min-height: 0;
  border: 1px solid var(--grid-border);
  border-radius: var(--panel-radius);
  background:
    radial-gradient(500px 260px at -16% -20%, rgba(122, 232, 255, 0.16), transparent 64%),
    linear-gradient(180deg, rgba(9, 14, 26, 0.94), rgba(7, 11, 20, 0.92));
  padding: 14px;
  display: grid;
  grid-template-rows: auto auto minmax(0, 1fr);
  gap: 10px;
  overflow: hidden;
}

.brand {
  padding: 8px 10px;
  border-radius: 12px;
  border: 1px solid rgba(130, 175, 255, 0.2);
  background: rgba(8, 12, 22, 0.6);
}

.brand-title {
  margin: 0;
  font-size: 17px;
  line-height: 1.15;
  letter-spacing: 0.25px;
  text-transform: uppercase;
}

.brand-subtitle {
  margin-top: 5px;
  font-size: 11px;
  line-height: 1.35;
  color: var(--muted);
}

.tabs {
  display: grid;
  grid-template-columns: repeat(3, minmax(0, 1fr));
  gap: 8px;
}

.tab {
  padding: 8px 8px;
  border-radius: 10px;
  border: 1px solid rgba(132, 172, 248, 0.24);
  background: rgba(7, 11, 20, 0.72);
  color: rgba(198, 218, 244, 0.86);
  font-size: 11px;
  font-weight: 700;
  letter-spacing: 0.3px;
  text-transform: uppercase;
}

.tab.active {
  color: rgba(247, 252, 255, 0.98);
  border-color: rgba(93, 213, 255, 0.62);
  background:
    linear-gradient(165deg, rgba(92, 190, 255, 0.24), rgba(62, 140, 255, 0.14)),
    rgba(8, 12, 22, 0.84);
  box-shadow: 0 0 0 1px rgba(126, 218, 255, 0.34) inset, 0 8px 20px rgba(48, 120, 255, 0.22);
}

.sidebar-panel {
  padding: 10px;
  display: grid;
  grid-auto-rows: min-content;
  gap: 8px;
}

.sidebar-panel h3 {
  margin: 0;
  font-size: 12px;
  text-transform: uppercase;
  letter-spacing: 0.45px;
  color: rgba(230, 241, 255, 0.95);
}

.btn {
  border-radius: 10px;
  border: 1px solid rgba(132, 172, 248, 0.28);
  background: rgba(11, 18, 32, 0.72);
  color: rgba(233, 244, 255, 0.96);
  padding: 8px 10px;
  font-size: 12px;
  font-weight: 700;
  cursor: pointer;
  transition: transform 120ms ease, border-color 120ms ease, background 120ms ease, box-shadow 120ms ease;
}

.btn:hover {
  transform: translateY(-1px);
  border-color: rgba(90, 213, 255, 0.65);
}

.btn.secondary {
  background: rgba(8, 13, 23, 0.58);
  color: rgba(197, 222, 248, 0.92);
}

.btn.danger {
  border-color: rgba(255, 119, 149, 0.42);
  background: rgba(44, 10, 20, 0.58);
}

.btn.focused {
  box-shadow: 0 0 0 2px rgba(89, 216, 255, 0.52);
}

.main.command-shell {
  min-width: 0;
  min-height: 0;
  display: grid;
  grid-template-rows: auto minmax(0, 1fr);
  gap: var(--layout-gap);
  /* Keep viewport framing stable during interaction tests; inner rows own scrolling. */
  overflow: hidden;
}

.topbar.status-grid {
  display: grid;
  grid-template-columns: minmax(0, 1.45fr) minmax(0, 1fr) minmax(0, 1fr);
  gap: var(--layout-gap);
  align-items: stretch;
}

.run-pane {
  padding: 10px;
  display: grid;
  grid-template-rows: auto auto;
  gap: 10px;
}

.run-buttons {
  display: grid;
  grid-template-columns: minmax(0, 1fr) minmax(0, 1fr);
  gap: 8px;
}

.play-btn {
  min-height: 42px;
  border-radius: 12px;
  border: 1px solid rgba(130, 172, 248, 0.3);
  background: rgba(10, 16, 28, 0.78);
  color: rgba(242, 249, 255, 0.98);
  transition: transform 120ms ease, border-color 120ms ease, box-shadow 120ms ease, filter 160ms ease;
}

.play-btn:hover {
  transform: translateY(-1px);
  filter: brightness(1.08);
}

.play-btn.focused {
  box-shadow: 0 0 0 2px rgba(95, 214, 255, 0.52), 0 0 28px rgba(90, 214, 255, 0.24);
}

.shop-btn {
  border-color: rgba(170, 151, 255, 0.35);
}

.play-head {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  font-weight: 800;
  letter-spacing: 0.35px;
  text-transform: uppercase;
}

.play-icon {
  font-size: 13px;
  color: var(--accent3);
}

.play-text {
  font-size: 12px;
}

.run-bottom {
  display: grid;
  gap: 8px;
}

.run-strip {
  display: grid;
  grid-template-columns: repeat(3, minmax(0, 1fr));
  gap: 8px;
}

.strip-item {
  border-radius: 11px;
  border: 1px solid rgba(130, 172, 248, 0.26);
  background: rgba(8, 12, 22, 0.66);
  padding: 7px 8px;
  display: grid;
  gap: 2px;
}

.strip-k {
  font-size: 10px;
  text-transform: uppercase;
  letter-spacing: 0.45px;
  color: rgba(186, 208, 235, 0.8);
}

.strip-v {
  font-size: 14px;
  font-weight: 800;
  letter-spacing: 0.2px;
  color: rgba(240, 249, 255, 0.98);
}

.run-progress {
  height: 10px;
  border-radius: 999px;
  border: 1px solid rgba(130, 172, 248, 0.24);
  background: rgba(7, 11, 20, 0.66);
  overflow: hidden;
}

.run-progress-fill {
  height: 100%;
  border-radius: inherit;
  background: linear-gradient(90deg, #57d8ff, #88ff8a 65%, #ffd773);
  box-shadow: 0 0 14px rgba(92, 214, 255, 0.44);
  transition: width 220ms ease;
}

.deck-widget,
.hud-panel,
.pile-widget,
.handbar,
.deckbar {
  padding: 10px;
}

.deck-title,
.hud-title {
  margin: 0;
  font-size: 12px;
  text-transform: uppercase;
  letter-spacing: 0.45px;
  color: rgba(234, 244, 255, 0.94);
}

.hint {
  font-size: 11px;
  line-height: 1.4;
  color: var(--muted);
}

.register-grid {
  margin-top: 6px;
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 6px;
}

.kv {
  display: grid;
  grid-template-columns: minmax(0, 1fr) auto;
  align-items: center;
  gap: 6px;
  border-radius: 10px;
  border: 1px solid rgba(130, 172, 248, 0.2);
  background: rgba(8, 12, 22, 0.58);
  padding: 6px 8px;
}

.kv span {
  font-size: 10px;
  text-transform: uppercase;
  letter-spacing: 0.42px;
  color: rgba(188, 210, 236, 0.82);
}

.kv code {
  font-family: "JetBrains Mono", "IBM Plex Mono", Menlo, Consolas, monospace;
  font-size: 11px;
  font-weight: 700;
  color: rgba(243, 250, 255, 0.98);
}

.deck-stack {
  margin-top: 8px;
  display: grid;
  grid-auto-flow: column;
  justify-content: start;
  gap: 6px;
}

.deck-card {
  width: 52px;
  height: 74px;
  border-radius: 10px;
  border: 1px solid rgba(126, 173, 255, 0.34);
  background:
    linear-gradient(165deg, rgba(97, 206, 255, 0.24), rgba(91, 255, 134, 0.12)),
    rgba(8, 12, 22, 0.78);
}

.deck-meta,
.pile-meta {
  margin-top: 8px;
  display: grid;
  gap: 6px;
}

.pill {
  border-radius: 10px;
  border: 1px solid rgba(130, 172, 248, 0.22);
  background: rgba(8, 12, 22, 0.56);
  padding: 6px 8px;
  display: grid;
  grid-template-columns: minmax(0, 1fr) auto;
  align-items: center;
  gap: 8px;
}

.pill span {
  font-size: 10px;
  text-transform: uppercase;
  letter-spacing: 0.42px;
  color: rgba(190, 211, 236, 0.82);
}

.pill strong {
  font-size: 12px;
  color: rgba(240, 249, 255, 0.97);
}

.playfield-grid {
  min-height: 0;
  display: grid;
  grid-template-rows: minmax(0, 1fr) auto;
  gap: var(--layout-gap);
}

.queue-grid {
  min-height: 0;
  display: grid;
  grid-template-columns: minmax(0, 1fr) 260px;
  gap: var(--layout-gap);
}

.handbar,
.deckbar {
  min-height: 0;
  display: grid;
  grid-template-rows: auto minmax(0, 1fr);
  gap: 8px;
  position: relative;
}

.handbar {
  z-index: 6;
}

.deckbar {
  z-index: 2;
}

.hand-title {
  display: flex;
  align-items: baseline;
  justify-content: space-between;
  gap: 8px;
}

.hand-title > span:first-child {
  font-size: 12px;
  text-transform: uppercase;
  letter-spacing: 0.42px;
  font-weight: 800;
}

.row-scroll {
  min-height: 0;
  overflow-x: auto;
  overflow-y: hidden;
  padding-bottom: 4px;
}

.row {
  min-height: calc(var(--card-h) + 8px);
  display: grid;
  grid-auto-flow: column;
  grid-auto-columns: var(--card-w);
  align-items: start;
  gap: 10px;
}

.card-wrap {
  position: relative;
  width: var(--card-w);
  min-height: var(--card-h);
}

.drop-slit {
  position: absolute;
  top: 8px;
  bottom: 8px;
  width: 8px;
  border-radius: 999px;
  background: rgba(96, 213, 255, 0.12);
  border: 1px dashed rgba(96, 213, 255, 0.24);
  opacity: 0;
  pointer-events: none;
  transition: opacity 120ms ease, background 140ms ease, border-color 140ms ease;
}

.drop-slit.left {
  left: -6px;
}

.drop-slit.right {
  right: -6px;
}

.drop-slit.active {
  opacity: 1;
  background: rgba(102, 218, 255, 0.26);
  border-color: rgba(102, 218, 255, 0.7);
}

.app.is-dragging .drop-slit {
  opacity: 0.28;
  pointer-events: auto;
}

.app.is-dragging .drop-slit.active {
  opacity: 1;
}

.card {
  position: relative;
  width: var(--card-w);
  height: var(--card-h);
  border-radius: 14px;
  border: 1px solid rgba(133, 174, 249, 0.33);
  background:
    linear-gradient(165deg, rgba(14, 22, 40, 0.92), rgba(10, 17, 31, 0.9)),
    rgba(8, 13, 24, 0.92);
  box-shadow:
    0 12px 28px rgba(0, 0, 0, 0.32),
    inset 0 1px 0 rgba(255, 255, 255, 0.05);
  display: grid;
  grid-template-rows: auto auto 1fr auto;
  overflow: hidden;
  transform-origin: center;
  transition: transform 120ms ease, box-shadow 120ms ease, border-color 120ms ease;
}

.card:hover {
  transform: translateY(-2px);
  border-color: rgba(95, 214, 255, 0.62);
  box-shadow: 0 16px 30px rgba(0, 0, 0, 0.34), 0 0 0 1px rgba(95, 214, 255, 0.3);
}

.card.selected,
.card.focused {
  border-color: rgba(123, 232, 255, 0.84);
  box-shadow: 0 0 0 2px rgba(97, 215, 255, 0.42), 0 16px 34px rgba(0, 0, 0, 0.36);
}

/* Neutralize older high-specificity focus transforms from earlier theme layers. */
.card.selected.focused {
  transform: none;
}

.card.selected.focused::after {
  content: none;
  animation: none;
}

.card.dragging {
  pointer-events: none;
  z-index: 2400;
  transform: rotate(1deg) scale(1.03);
}

.card-top {
  padding: 8px 8px 0;
  display: flex;
  justify-content: space-between;
  align-items: start;
}

.card-index {
  border-radius: 999px;
  border: 1px solid rgba(130, 172, 248, 0.35);
  background: rgba(8, 12, 22, 0.7);
  padding: 2px 7px;
  font-size: 10px;
  font-weight: 700;
  color: rgba(205, 224, 246, 0.94);
}

.card-kind-tag {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  margin-top: 5px;
  padding: 2px 7px;
  border-radius: 999px;
  border: 1px solid rgba(130, 172, 248, 0.3);
  font-size: 9px;
  text-transform: uppercase;
  letter-spacing: 0.4px;
  color: rgba(206, 225, 247, 0.9);
}

.card-docs {
  border-radius: 8px;
  border: 1px solid rgba(130, 172, 248, 0.3);
  background: rgba(8, 12, 22, 0.62);
  color: rgba(228, 241, 255, 0.95);
  padding: 3px 7px;
  font-size: 12px;
}

.card-art {
  min-height: 56px;
  margin: 6px 8px 0;
  border-radius: 10px;
  border: 1px solid rgba(130, 172, 248, 0.2);
  background:
    radial-gradient(120px 62px at 20% 22%, rgba(92, 210, 255, 0.2), transparent 75%),
    radial-gradient(130px 70px at 80% 78%, rgba(129, 255, 109, 0.16), transparent 78%),
    rgba(8, 12, 22, 0.62);
  display: grid;
  grid-template-rows: 1fr auto;
  align-items: center;
}

.card-main-icon {
  text-align: center;
  font-size: 24px;
  filter: drop-shadow(0 0 6px rgba(130, 220, 255, 0.45));
}

.card-fx-ribbon {
  display: flex;
  gap: 4px;
  padding: 4px;
  overflow: hidden;
}

.fx-dot {
  min-width: 0;
  border-radius: 999px;
  border: 1px solid rgba(130, 172, 248, 0.24);
  background: rgba(8, 12, 22, 0.66);
  padding: 2px 4px;
  font-size: 8px;
  display: inline-flex;
  align-items: center;
  gap: 3px;
}

.fx-short {
  text-transform: uppercase;
  letter-spacing: 0.3px;
  font-weight: 700;
}

.card-body {
  padding: 8px 8px 6px;
  display: grid;
  grid-template-rows: auto auto minmax(0, 1fr) auto;
  gap: 5px;
}

.card-title {
  margin: 0;
  font-size: 12px;
  line-height: 1.2;
  letter-spacing: 0.18px;
}

.card-sub {
  font-size: 10px;
  text-transform: uppercase;
  letter-spacing: 0.36px;
  color: rgba(186, 208, 236, 0.8);
}

.card-script {
  min-height: 38px;
  max-height: 52px;
  overflow: hidden;
  border-radius: 8px;
  border: 1px solid rgba(130, 172, 248, 0.2);
  background: rgba(7, 11, 20, 0.7);
  padding: 6px;
  line-height: 1.3;
  display: block;
  font-family: "JetBrains Mono", "IBM Plex Mono", Menlo, Consolas, monospace;
}

.tok {
  font-size: 10px;
}

.tok-fn { color: #97ecff; font-weight: 700; }
.tok-reg { color: #f6d887; }
.tok-acc { color: #a5ff9f; font-weight: 700; }
.tok-op { color: #ffb4c7; }
.tok-num { color: #ffd67e; }
.tok-punct { color: rgba(198, 216, 241, 0.7); }

.card-fx-row {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 4px;
}

.card-fx-chip {
  border-radius: 8px;
  border: 1px solid rgba(130, 172, 248, 0.24);
  background: rgba(8, 12, 22, 0.62);
  padding: 3px 5px;
  display: inline-flex;
  align-items: center;
  gap: 4px;
  font-size: 9px;
}

.chip-label {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  text-transform: uppercase;
  letter-spacing: 0.32px;
}

.card-actions {
  position: absolute;
  left: 8px;
  right: 8px;
  bottom: 8px;
  display: grid;
  grid-template-columns: repeat(3, minmax(0, 1fr));
  gap: 6px;
}

.card-btn {
  border-radius: 8px;
  border: 1px solid rgba(130, 172, 248, 0.28);
  background: rgba(8, 12, 22, 0.72);
  color: rgba(235, 246, 255, 0.96);
  padding: 4px 0;
  font-size: 11px;
}

.ghost-card {
  height: var(--card-h);
  border-radius: 14px;
  border: 1px dashed rgba(130, 172, 248, 0.4);
  background: rgba(8, 12, 22, 0.5);
  display: grid;
  place-items: center;
  gap: 4px;
}

.ghost-plus {
  font-size: 24px;
  color: rgba(126, 214, 255, 0.85);
}

.ghost-hint {
  font-size: 10px;
  color: rgba(191, 211, 236, 0.82);
  text-transform: uppercase;
}

.pile-widget {
  min-height: 0;
  display: grid;
  grid-template-rows: auto auto auto minmax(0, 1fr);
  gap: 8px;
}

.pile-face {
  border-radius: 12px;
  border: 1px solid rgba(130, 172, 248, 0.24);
  background: rgba(8, 12, 22, 0.62);
  padding: 8px;
}

.pile-face-title {
  font-size: 12px;
  font-weight: 700;
}

.pile-face-sub {
  margin-top: 2px;
  font-size: 10px;
  color: var(--muted);
}

.pile-menu {
  min-height: 0;
  border-radius: 12px;
  border: 1px solid rgba(130, 172, 248, 0.2);
  background: rgba(8, 12, 22, 0.5);
  padding: 6px;
  display: grid;
  grid-auto-rows: min-content;
  gap: 4px;
  overflow: auto;
}

.pile-menu-title {
  font-size: 10px;
  text-transform: uppercase;
  letter-spacing: 0.35px;
  color: rgba(190, 211, 236, 0.82);
}

.pile-item {
  font-size: 11px;
  border-radius: 8px;
  border: 1px solid rgba(130, 172, 248, 0.18);
  background: rgba(8, 12, 22, 0.44);
  padding: 4px 6px;
  color: rgba(230, 241, 255, 0.92);
}

.trace-list {
  display: grid;
  gap: 6px;
  overflow: auto;
  min-height: 0;
}

.trace-item {
  border-radius: 8px;
  border: 1px solid rgba(130, 172, 248, 0.2);
  background: rgba(8, 12, 22, 0.56);
  padding: 6px 7px;
  font-size: 11px;
}

.trace-item.call { border-color: rgba(95, 214, 255, 0.34); }
.trace-item.effect { border-color: rgba(125, 255, 116, 0.34); }
.trace-item.error { border-color: rgba(255, 121, 150, 0.42); color: rgba(255, 215, 226, 0.96); }

.modal-overlay {
  backdrop-filter: blur(5px);
}

.modal {
  width: min(1200px, calc(100vw - 48px));
  max-height: calc(100vh - 44px);
  border-radius: 16px;
}

.modal-body {
  display: grid;
  grid-template-rows: auto minmax(0, 1fr);
  gap: 10px;
  min-height: 0;
}

.kcard-list {
  display: grid;
  gap: 8px;
  max-height: min(65vh, 720px);
  overflow: auto;
}

.kcard {
  border-radius: 12px;
  border: 1px solid rgba(130, 172, 248, 0.22);
  background: rgba(8, 12, 22, 0.58);
}

.kcard.highlight {
  border-color: rgba(95, 214, 255, 0.65);
  box-shadow: 0 0 0 1px rgba(95, 214, 255, 0.28);
}

.fx-layer {
  position: fixed;
  inset: 0;
  pointer-events: none;
  z-index: 1400;
}

.fx-step,
.fx-bump,
.fx-proj {
  position: fixed;
  transform: translate(-50%, -50%);
  border-radius: 10px;
  border: 1px solid rgba(133, 174, 249, 0.34);
  background: rgba(9, 14, 26, 0.86);
  padding: 6px 9px;
  font-size: 11px;
  font-weight: 700;
  letter-spacing: 0.2px;
  color: rgba(242, 249, 255, 0.96);
}

.fx-step.info,
.fx-bump.info,
.fx-proj.info {
  border-color: rgba(94, 213, 255, 0.5);
}

.fx-step.score,
.fx-bump.score,
.fx-proj.score {
  border-color: rgba(255, 203, 105, 0.58);
}

.fx-step.money,
.fx-bump.money,
.fx-proj.money {
  border-color: rgba(128, 255, 119, 0.56);
}

.fx-step.error,
.fx-bump.error,
.fx-proj.error {
  border-color: rgba(255, 121, 150, 0.62);
}

.fx-burst {
  position: fixed;
  left: var(--x);
  top: var(--y);
  width: 0;
  height: 0;
  pointer-events: none;
}

.fx-burst span {
  position: absolute;
  width: 2px;
  height: 12px;
  background: linear-gradient(180deg, rgba(255, 243, 204, 1), rgba(255, 169, 83, 0.08));
  transform-origin: 50% 100%;
  transform: rotate(var(--a)) translateY(calc(-1 * var(--d)));
  animation: burstRay 420ms ease-out forwards;
}

@keyframes burstRay {
  0% { opacity: 1; transform: rotate(var(--a)) translateY(0); }
  100% { opacity: 0; transform: rotate(var(--a)) translateY(calc(-1 * var(--d))); }
}

.focus-layer {
  pointer-events: none;
  z-index: 1300;
}

.focus-halo {
  position: fixed;
  border-radius: 12px;
  border: 2px solid rgba(95, 214, 255, 0.84);
  box-shadow: 0 0 0 2px rgba(95, 214, 255, 0.22), 0 0 24px rgba(95, 214, 255, 0.3);
  transition: all 110ms ease;
}

.deck-shake {
  animation: deckJolt 320ms ease;
}

@keyframes deckJolt {
  0%, 100% { transform: translateX(0); }
  20% { transform: translateX(-2px); }
  40% { transform: translateX(2px); }
  60% { transform: translateX(-1px); }
  80% { transform: translateX(1px); }
}

@media (max-width: 1180px) {
  .topbar.status-grid {
    grid-template-columns: 1fr;
  }

  .queue-grid {
    grid-template-columns: 1fr;
  }

  .pile-widget {
    min-height: 220px;
  }
}

@media (max-width: 980px) {
  .app {
    grid-template-columns: 1fr;
    grid-template-rows: auto minmax(0, 1fr);
    padding: 8px;
    gap: 8px;
  }

  .sidebar {
    max-height: 43vh;
  }

  .playfield-grid {
    grid-template-rows: minmax(0, 1fr) minmax(0, 1fr);
  }

  .row {
    grid-auto-columns: clamp(126px, 44vw, 164px);
  }
}

@media (max-width: 760px) {
  .tabs {
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }

  .register-grid {
    grid-template-columns: 1fr;
  }

  .run-strip {
    grid-template-columns: 1fr;
  }
}
"#;
