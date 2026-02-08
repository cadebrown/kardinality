# Kardinality

Kardinality is a **single‑player, browser‑deployed** card game where your “hand” is an ordered stack of executable code‑cards. The vibe is roguelike deck‑building (Balatro‑adjacent), but the mechanics are intentionally **programming‑shaped**: cards compose, control flow exists, and you win by hitting ever‑higher score targets.

## Tech stack

* **Rust**: all game logic (rules, state, interpreter)
* **WASM (web)**: cross‑platform browser deploy
* **Dioxus**: UI rendered via **HTML DOM + CSS**, no traditional rendering engine
* **Browser APIs**: accessed from Rust (`web_sys` / `wasm_bindgen`) with minimal/no hand‑written JS

## Quick start (dev)

Recommended workflow:

* Install Rust (stable) and add the WASM target:
  * `rustup target add wasm32-unknown-unknown`
* Install Dioxus CLI:
  * `cargo install dioxus-cli`
  * (optional, faster installs) `cargo install cargo-binstall && cargo binstall dioxus-cli`
* Run the dev server:
  * `dx serve`
* Build for release:
  * `dx build --release`
* Run core logic tests:
  * `cargo test`

## Automated GUI smoke tests (E2E)

This repo includes a headless browser harness (Playwright) that can boot the WASM app and drive the UI.
It’s intended for regression checks and future automated deployment confidence.

Prereqs:

* Node.js 18+
* `dioxus-cli` installed (`dx serve` must work)

Install deps:

* `npm install`

Run E2E tests (headless):

* `npm run e2e`

Run E2E tests with the Playwright UI:

* `npm run e2e:ui`

## Tutorial video generation

The project can auto-generate a cinematic tutorial/playthrough MP4 from deterministic Playwright scenes.

Prereqs:

* `ffmpeg` (required)
* Voiceover engine (optional but recommended):
  * premium neural + timing alignment: ElevenLabs API key (`ELEVENLABS_API_KEY`)
  * neural: `edge-tts` (`pip install edge-tts`)
  * macOS: built-in `say`
  * Linux: `espeak` / `espeak-ng`

Generate the video locally:

* `npm run tutorial:video` (explicitly requests `--voice-provider elevenlabs`, with fallback)
* `npm run tutorial:video:strict` (fails hard if ElevenLabs does not produce voice output)

Local defaults are loaded from `.env` automatically (if present).

Outputs:

* `artifacts/tutorial-video/tutorial.mp4`
* `artifacts/tutorial-video/captions.srt`
* `artifacts/tutorial-video/caption-cues.json`
* `artifacts/tutorial-video/scene-manifest.json`
* `artifacts/tutorial-video/tutorial-metadata.json`

Voice tuning knobs:

* `TUTORIAL_VOICE_PROVIDER=auto|elevenlabs|edge|say|espeak-ng|espeak|none`
* `TUTORIAL_VOICE_NAME=...` (for example `en-US-JennyNeural` with `edge-tts`)
* `TUTORIAL_VOICE_RATE=+2%` and `TUTORIAL_VOICE_PITCH=-2Hz` (edge)
* `TUTORIAL_VOICE_RATE_WPM=168` (say/espeak)
* `TUTORIAL_ELEVENLABS_VOICE_ID=...` and `TUTORIAL_ELEVENLABS_MODEL_ID=...`
* `TUTORIAL_ELEVENLABS_STABILITY=0.45`, `TUTORIAL_ELEVENLABS_SIMILARITY=0.78`, `TUTORIAL_ELEVENLABS_STYLE=0.30`
* `TUTORIAL_ELEVENLABS_SPEED=0.92` (slower/clearer narration cadence)
* `TUTORIAL_CAPTION_MAX_WORDS=8`, `TUTORIAL_CAPTION_MAX_SECONDS=2.6`, `TUTORIAL_CAPTION_MIN_SECONDS=0.6`
* `TUTORIAL_SCENE_FADE=0.42`, `TUTORIAL_SCENE_TRANSITIONS=fade,smoothleft,fadeblack,...`
* `TUTORIAL_SCENE_TRIM_START=0.18`, `TUTORIAL_SCENE_SETTLE_PAD=0.06` (hide first-frame resize glitches)
* `TUTORIAL_SCENE_FIRST_TRIM_START=0.95` (extra trim only for scene 1 to remove startup/loading flashes)
* `TUTORIAL_SCENE_AUDIO_GAP=0.12` (target pause at scene boundaries for a steadier cadence)
* `TUTORIAL_SCENE_AUDIO_CLIP_FADE=0.09` (short fade-out when narration must hard-trim)
* CLI flags: `--voice-provider elevenlabs|edge|say|espeak-ng|espeak|auto|none` and `--strict-voice`

When ElevenLabs is enabled, the composer uses API timing alignment for tighter subtitle sync. Scene voice clips are cached by provider + text + settings in `artifacts/tutorial-video/cache/voice`, so repeated compose runs reuse audio instead of re-calling the API.

## Docs

* `AGENTS.md`: guidance for future AI agents (commands, constraints, conventions)
* `KARDLANG.md`: language philosophy + cost model + registers + function reference
