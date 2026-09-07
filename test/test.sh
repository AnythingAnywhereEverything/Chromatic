#!/bin/bash

set -e
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "=== HTTP TEST ==="

k6 run \
    -e BASE_URL=http://localhost \
    -e TOKEN="$TOKEN" \
    "$SCRIPT_DIR/test/https/homepage-http.js"

echo ""
echo "=== BROWSER TEST ==="

K6_BROWSER_ARGS='enable-gpu,gpu-rasterization,enable-zero-copy,ignore-gpu-blocklist' \
    k6 run \
    -e BASE_URL=http://localhost \
    -e TOKEN="$TOKEN" \
    "$SCRIPT_DIR/test/browser/homepage-browser.js"

echo ""
echo "=== ALL TESTS COMPLETE ==="