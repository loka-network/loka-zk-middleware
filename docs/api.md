# API Reference

## Base URL

```
http://<host>:<port>/api/v1
```

## Endpoints

### Health Check

```
GET /api/v1/health
```

Returns service status and supported capabilities.

**Response** `200 OK`

```json
{
  "status": "healthy",
  "service": "loka-zk-middleware",
  "version": "0.1.0",
  "supported_schemes": ["groth16"],
  "supported_curves": ["bn254"]
}
```

---

### Generate Square Proof

```
POST /api/v1/prove/square
Content-Type: application/json
```

Proves knowledge of a secret `x` such that `x² = y` without revealing `x`.

**Request Body**

| Field    | Type   | Description          |
|----------|--------|----------------------|
| `secret` | `u64`  | The secret witness   |

**Response** `200 OK`

```json
{
  "proof_id": "uuid-v4",
  "proof": "hex-encoded-proof",
  "verification_key": "hex-encoded-vk",
  "public_inputs": ["hex-encoded-y"],
  "scheme": "groth16",
  "curve": "bn254"
}
```

---

### Generate Sum Proof

```
POST /api/v1/prove/sum
Content-Type: application/json
```

Proves knowledge of secret values `a` and `b` such that `a + b = sum` without revealing `a` or `b`.

**Request Body**

| Field | Type   | Description          |
|-------|--------|----------------------|
| `a`   | `u64`  | First secret addend  |
| `b`   | `u64`  | Second secret addend |

**Response** `200 OK`

Same schema as the square proof response.

---

### Verify Proof

```
POST /api/v1/verify
Content-Type: application/json
```

Verifies a Groth16 proof.

**Request Body**

| Field              | Type       | Description                  |
|--------------------|-----------|-------------------------------|
| `proof`            | `string`  | Hex-encoded proof data        |
| `verification_key` | `string`  | Hex-encoded verification key  |
| `public_inputs`    | `string[]`| Hex-encoded public inputs     |

**Response** `200 OK`

```json
{
  "valid": true,
  "scheme": "groth16"
}
```

---

## Error Responses

All error responses follow a common format:

```json
{
  "error": "Human-readable error message",
  "code": "ERROR_CODE"
}
```

| Code                       | HTTP Status | Description                  |
|----------------------------|-------------|------------------------------|
| `INVALID_INPUT`            | 400         | Malformed or invalid request |
| `VERIFICATION_FAILED`      | 400         | Proof verification error     |
| `PROOF_GENERATION_FAILED`  | 500         | Proof generation error       |
| `INTERNAL_ERROR`           | 500         | Unexpected server error      |
