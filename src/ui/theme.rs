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
  display: grid;
  grid-template-rows: auto auto 1fr;
  gap: 14px;
  position: relative;
  z-index: 1;
  min-height: 0;
}

.topbar {
  display: flex;
  gap: 14px;
  align-items: stretch;
}

.right-rail {
  width: 238px;
  display: grid;
  grid-template-rows: auto 1fr;
  gap: 12px;
}

.play-btn {
  width: 238px;
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
  transform: translateY(-1px) scale(1.01);
  border-color: rgba(0, 255, 156, 0.62);
  box-shadow: 0 22px 60px rgba(0, 0, 0, 0.55), 0 0 64px rgba(0, 255, 156, 0.28);
  filter: saturate(1.12) contrast(1.08);
}
.play-btn:active {
  transform: translateY(0px) scale(0.99);
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
  padding: 8px 10px 8px;
}

.hud-title {
  margin: 0 0 10px 0;
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
  margin-bottom: 10px;
}

.scorebig {
  font-size: 26px;
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
  height: 12px;
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
  margin-top: 12px;
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
  padding: 10px;
  position: relative;
  overflow: hidden;
}

.deck-title {
  margin: 0 0 8px 0;
  font-size: 12px;
  text-transform: uppercase;
  letter-spacing: 0.45px;
  color: var(--muted);
}

.deck-stack {
  position: relative;
  height: 76px;
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

.deck-card:nth-child(2) { transform: translate(8px, 7px); opacity: 0.70; }
.deck-card:nth-child(3) { transform: translate(16px, 14px); opacity: 0.45; }

.deck-meta {
  position: relative;
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 8px 12px;
  margin-top: 8px;
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
  overflow-x: scroll;
  overflow-y: visible;
  padding-top: 10px;
  padding-bottom: 10px;
  scrollbar-gutter: stable both-edges;
  scrollbar-color: rgba(0, 255, 156, 0.35) rgba(0, 0, 0, 0.20);
}

.row {
  display: flex;
  gap: 12px;
  align-items: flex-end;
}

.row.center {
  justify-content: center;
  min-width: 100%;
}

.drop-slot {
  width: 18px;
  height: 242px;
  border-radius: 10px;
  border: 1px dashed rgba(0, 255, 156, 0.00);
  background: rgba(0, 0, 0, 0.00);
  transition: border-color 120ms ease, box-shadow 120ms ease, background 120ms ease;
}
.drop-slot:hover {
  border-color: rgba(0, 255, 156, 0.38);
  background: rgba(0, 255, 156, 0.04);
  box-shadow: 0 0 22px rgba(0, 255, 156, 0.10);
}

.ghost-card {
  width: 176px;
  min-width: 176px;
  height: 242px;
  border-radius: 18px;
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
  width: 176px;
  min-width: 176px;
  height: 242px;
  border-radius: 18px;
  border: 1px solid rgba(0, 255, 156, 0.22);
  background:
    linear-gradient(135deg, rgba(0, 255, 156, 0.10), rgba(0, 200, 255, 0.06) 35%, rgba(0, 0, 0, 0.38));
  box-shadow: 0 22px 54px rgba(0, 0, 0, 0.50), 0 0 0 1px rgba(0, 255, 156, 0.05) inset;
  overflow: hidden;
  position: relative;
  z-index: 1;
  transition: transform 140ms ease, box-shadow 140ms ease, border-color 140ms ease;
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
}

.card.selected {
  border-color: rgba(0, 255, 156, 0.92);
  box-shadow: 0 28px 64px rgba(0, 0, 0, 0.58), 0 0 34px rgba(0, 255, 156, 0.26), 0 0 84px rgba(0, 200, 255, 0.14);
  z-index: 80;
}

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

.card-actions {
  position: absolute;
  left: 10px;
  right: 10px;
  bottom: 10px;
  display: grid;
  grid-template-columns: 1fr 1fr 1fr 1fr;
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
  height: 88px;
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


