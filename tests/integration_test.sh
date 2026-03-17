#!/usr/bin/env bash
# Integration tests for the loka-zk-middleware API.
# Requires the server to be running on localhost:8080 (or set BASE_URL).
set -euo pipefail

BASE_URL="${BASE_URL:-http://localhost:8080}"
PASS=0
FAIL=0

check() {
    local name="$1"
    local expected_code="$2"
    local actual_code="$3"

    if [ "$actual_code" -eq "$expected_code" ]; then
        echo "  PASS: $name (HTTP $actual_code)"
        PASS=$((PASS + 1))
    else
        echo "  FAIL: $name — expected $expected_code, got $actual_code"
        FAIL=$((FAIL + 1))
    fi
}

echo "==> Running integration tests against $BASE_URL"

# 1. Health check
echo ""
echo "--- Health Check ---"
HTTP_CODE=$(curl -s -o /dev/null -w "%{http_code}" "$BASE_URL/api/v1/health")
check "GET /api/v1/health" 200 "$HTTP_CODE"

# 2. Square proof generation
echo ""
echo "--- Square Proof ---"
RESPONSE=$(curl -s -w "\n%{http_code}" -X POST "$BASE_URL/api/v1/prove/square" \
    -H "Content-Type: application/json" \
    -d '{"secret": 7}')
HTTP_CODE=$(echo "$RESPONSE" | tail -1)
BODY=$(echo "$RESPONSE" | sed '$d')
check "POST /api/v1/prove/square" 200 "$HTTP_CODE"

# Extract fields for verification
PROOF=$(echo "$BODY" | sed -n 's/.*"proof":"\([^"]*\)".*/\1/p')
VK=$(echo "$BODY" | sed -n 's/.*"verification_key":"\([^"]*\)".*/\1/p')
PUBLIC_INPUTS=$(echo "$BODY" | sed -n 's/.*"public_inputs":\[\([^]]*\)\].*/\1/p')

# 3. Verify the proof
echo ""
echo "--- Verify Proof ---"
HTTP_CODE=$(curl -s -o /dev/null -w "%{http_code}" -X POST "$BASE_URL/api/v1/verify" \
    -H "Content-Type: application/json" \
    -d "{\"proof\":\"$PROOF\",\"verification_key\":\"$VK\",\"public_inputs\":[$PUBLIC_INPUTS]}")
check "POST /api/v1/verify" 200 "$HTTP_CODE"

# 4. Sum proof generation
echo ""
echo "--- Sum Proof ---"
HTTP_CODE=$(curl -s -o /dev/null -w "%{http_code}" -X POST "$BASE_URL/api/v1/prove/sum" \
    -H "Content-Type: application/json" \
    -d '{"a": 10, "b": 20}')
check "POST /api/v1/prove/sum" 200 "$HTTP_CODE"

# 5. Invalid input
echo ""
echo "--- Invalid Input ---"
HTTP_CODE=$(curl -s -o /dev/null -w "%{http_code}" -X POST "$BASE_URL/api/v1/prove/square" \
    -H "Content-Type: application/json" \
    -d '{}')
check "POST /api/v1/prove/square with empty body returns 4xx" 400 "$HTTP_CODE"

# Summary
echo ""
echo "================================"
echo "Results: $PASS passed, $FAIL failed"
echo "================================"

[ "$FAIL" -eq 0 ] || exit 1
