#!/usr/bin/env bash
# Quick smoke test — verifies the server starts and the health endpoint responds.
set -euo pipefail

BASE_URL="${BASE_URL:-http://localhost:8080}"

echo "==> Smoke test: checking health endpoint at $BASE_URL"

HTTP_CODE=$(curl -s -o /dev/null -w "%{http_code}" --max-time 5 "$BASE_URL/api/v1/health" 2>/dev/null || echo "000")

if [ "$HTTP_CODE" -eq 200 ]; then
    echo "  OK — server is healthy (HTTP 200)"
    exit 0
else
    echo "  FAIL — expected HTTP 200, got $HTTP_CODE"
    exit 1
fi
