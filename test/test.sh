#!/bin/bash

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "=== HTTP TEST ==="
K6_WEB_DASHBOARD_OPEN=true \
K6_WEB_DASHBOARD=true \
    k6 run \
    -e BASE_URL=http://localhost \
    -e TOKEN="$TOKEN" \
    "$SCRIPT_DIR/https/homepage-http.js"

echo ""

echo "=== BROWSER TEST ==="

K6_WEB_DASHBOARD_OPEN=true \
K6_WEB_DASHBOARD=true \
K6_BROWSER_ARGS='enable-gpu,gpu-rasterization,enable-zero-copy,ignore-gpu-blocklist' \
    k6 run \
    -e BASE_URL=http://localhost \
    -e TOKEN="$TOKEN" \
    "$SCRIPT_DIR/browser/homepage-browser.js"

echo ""

echo "=== ALL TESTS COMPLETE ==="