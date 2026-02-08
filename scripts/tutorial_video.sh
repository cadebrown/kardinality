#!/usr/bin/env bash
set -euo pipefail

# Load local defaults and secrets (if present).
if [[ -f ".env" ]]; then
  set -a
  # shellcheck disable=SC1091
  source ".env"
  set +a
fi

PORT="${E2E_PORT:-8080}"
ADDR="${E2E_ADDR:-127.0.0.1}"
BASE_URL="${E2E_BASE_URL:-http://${ADDR}:${PORT}}"
OUT_DIR="${TUTORIAL_VIDEO_OUT_DIR:-artifacts/tutorial-video}"
DX_LOG="${DX_LOG:-/tmp/kardinality-dx-tutorial-${PORT}-$$.log}"
VOICE_PROVIDER="${TUTORIAL_VOICE_PROVIDER:-elevenlabs}"
STRICT_VOICE="${TUTORIAL_VOICE_STRICT:-0}"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --out-dir)
      OUT_DIR="${2:-}"
      shift 2
      ;;
    --voice-provider)
      VOICE_PROVIDER="${2:-}"
      shift 2
      ;;
    --strict-voice)
      STRICT_VOICE="1"
      shift
      ;;
    --no-strict-voice)
      STRICT_VOICE="0"
      shift
      ;;
    *)
      echo "Unknown option: $1"
      exit 2
      ;;
  esac
done

cleanup() {
  if [[ -n "${DX_PID:-}" ]]; then
    kill "${DX_PID}" >/dev/null 2>&1 || true
  fi
}
trap cleanup EXIT

mkdir -p "${OUT_DIR}"

echo "Starting dx serve at ${BASE_URL} ..."
dx serve --addr "${ADDR}" --port "${PORT}" --open false --interactive false --watch false >"${DX_LOG}" 2>&1 &
DX_PID="$!"

echo "Waiting for server..."
for _ in $(seq 1 600); do
  if ! kill -0 "${DX_PID}" >/dev/null 2>&1; then
    echo "dx serve exited before becoming ready."
    tail -n 80 "${DX_LOG}" || true
    exit 1
  fi
  if curl -fsS "${BASE_URL}" >/dev/null 2>&1; then
    break
  fi
  sleep 0.1
done

if ! curl -fsS "${BASE_URL}" >/dev/null 2>&1; then
  echo "Server did not become ready at ${BASE_URL}"
  tail -n 80 "${DX_LOG}" || true
  exit 1
fi

echo "Capturing tutorial scenes with Playwright..."
export E2E_BASE_URL="${BASE_URL}"
export TUTORIAL_VIDEO=1
export TUTORIAL_VIDEO_OUT_DIR="${OUT_DIR}"
npx --yes playwright test e2e/tutorial-video.spec.ts --workers 1

echo "Composing tutorial video..."
if [[ "${STRICT_VOICE}" == "1" ]]; then
  node ./scripts/compose_tutorial_video.mjs --out-dir "${OUT_DIR}" --voice-provider "${VOICE_PROVIDER}" --strict-voice
else
  node ./scripts/compose_tutorial_video.mjs --out-dir "${OUT_DIR}" --voice-provider "${VOICE_PROVIDER}"
fi

echo "Done: ${OUT_DIR}/tutorial.mp4"
