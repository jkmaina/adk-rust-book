#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

GOOGLE_PACKAGES=(
  "chapter3-quickstart"
  "chapter5-multi-turn"
  "chapter12-thinking-gemini"
  "chapter13-quickstart-validation"
  "chapter15-telemetry-demo"
  "chapter16-guardrails-advanced"
  "chapter16-auth-identity"
  "chapter16-auth-rbac"
)

OPENAI_PACKAGES=(
  "chapter12-realtime-audio"
)

run_group() {
  local group_name="$1"
  shift
  local packages=("$@")

  if [[ ${#packages[@]} -eq 0 ]]; then
    echo "No packages configured for ${group_name} smoke validation."
    return
  fi

  echo "Running ${group_name} smoke group..."
  for package in "${packages[@]}"; do
    echo "  -> ${package}"
    cargo +stable run -p "${package}"
  done
}

if [[ "${BOOK_RUN_LIVE_SMOKE:-0}" != "1" ]]; then
  echo "Skipping live smoke tests. Set BOOK_RUN_LIVE_SMOKE=1 to enable them."
  exit 0
fi

if [[ -n "${GOOGLE_API_KEY:-}" ]]; then
  run_group "Google-backed" "${GOOGLE_PACKAGES[@]}"
else
  echo "Skipping Google-backed smoke group: GOOGLE_API_KEY is not set."
fi

if [[ -n "${OPENAI_API_KEY:-}" ]]; then
  run_group "OpenAI-backed" "${OPENAI_PACKAGES[@]}"
else
  echo "Skipping OpenAI-backed smoke group: OPENAI_API_KEY is not set."
fi

echo "Skipping A2A smoke automation: it requires a coordinated server/client run."
echo "Manual path:"
echo "  BOOK_RUN_LIVE_SMOKE=1 BOOK_RUN_A2A_SERVER=1 GOOGLE_API_KEY=... cargo +stable run -p chapter14-a2a-server"
echo "  BOOK_RUN_LIVE_SMOKE=1 A2A_BASE_URL=http://localhost:8090 cargo +stable run -p chapter14-a2a-client"

echo "Live smoke tests completed."
