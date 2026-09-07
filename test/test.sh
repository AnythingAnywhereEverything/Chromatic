#!/bin/bash

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

mkdir -p "$SCRIPT_DIR/results/http"
mkdir -p "$SCRIPT_DIR/results/browser"

echo "=== HTTP TEST ==="

K6_WEB_DASHBOARD_EXPORT="$SCRIPT_DIR/results/http/homepage-results.html" \
    k6 run \
    -e BASE_URL=http://localhost \
    -e TOKEN="$TOKEN" \
    "$SCRIPT_DIR/https/homepage-http.js"

echo "Opening HTTP results..."
xdg-open "$SCRIPT_DIR/results/http/homepage-results.html"

echo ""

echo "=== BROWSER TEST ==="

K6_WEB_DASHBOARD_EXPORT="$SCRIPT_DIR/results/browser/homepage-results.html" \
K6_BROWSER_ARGS='enable-gpu,gpu-rasterization,enable-zero-copy,ignore-gpu-blocklist' \
    k6 run \
    -e BASE_URL=http://localhost \
    -e TOKEN="$TOKEN" \
    "$SCRIPT_DIR/browser/homepage-browser.js"

echo "Opening browser results..."
xdg-open "$SCRIPT_DIR/results/browser/homepage-results.html"

echo ""

echo "=== ALL TESTS COMPLETE ==="