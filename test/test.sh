#!/bin/bash

set -e

echo "=== HTTP TEST ==="

k6 run \
    -e BASE_URL=http://localhost \
    -e TOKEN="$TOKEN" \
    test/https/homepage-http.js

echo ""
echo "=== BROWSER TEST ==="

K6_BROWSER_ARGS='enable-gpu,gpu-rasterization,enable-zero-copy,ignore-gpu-blocklist' \
    k6 run \
    -e BASE_URL=http://localhost \
    -e TOKEN="$TOKEN" \
    test/browser/homepage-browser.js

echo ""
echo "=== ALL TESTS COMPLETE ==="