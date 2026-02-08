#!/usr/bin/env bash
set -euo pipefail

PORT="${E2E_PORT:-8080}"
ADDR="${E2E_ADDR:-127.0.0.1}"
BASE_URL="${E2E_BASE_URL:-http://${ADDR}:${PORT}}"
WORKERS="${E2E_WORKERS:-2}"
DX_LOG="${DX_LOG:-/tmp/kardinality-dx-e2e-${PORT}-$$.log}"

UI="${1:-}"

cleanup() {
  if [[ -n "${DX_PID:-}" ]]; then
    kill "${DX_PID}" >/dev/null 2>&1 || true
  fi
}
trap cleanup EXIT

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

echo "Running Playwright tests..."
export E2E_BASE_URL="${BASE_URL}"

if [[ "${UI}" == "--ui" ]]; then
  npx --yes playwright test --ui
else
  npx --yes playwright test --workers "${WORKERS}"
fi
