# LibreLink API Interactions & Protocol Reference

## Overview

This library is a Rust client for the **LibreLink Up (LLU)** API — the backend that powers Abbott's LibreLink Up mobile app for remote glucose monitoring. The API is a standard REST/JSON service hosted at `api.libreview.io` with regional variants.

## Base URLs

| Scope | URL |
|-------|-----|
| Global (default) | `https://api.libreview.io` |
| Regional | `https://api-{region}.libreview.io` (e.g. `api-eu.libreview.io`) |

The API uses a **redirect mechanism** to route clients to the correct regional endpoint (see [Regional Redirect](#regional-redirect) below).

---

## Authentication

### Login

**`POST /llu/auth/login`**

Authenticates with email/password credentials. Always hits the global endpoint first.

**Request body:**
```json
{
  "email": "user@example.com",
  "password": "secret"
}
```

**Required headers:**
```
version: 4.16.0
product: llu.android
User-Agent: Apidog/1.0.0 (https://apidog.com)
```

**Success response (status 0):**
```json
{
  "status": 0,
  "data": {
    "authTicket": {
      "token": "<jwt>",
      "expires": 1234567890,
      "duration": 86400
    },
    "user": {
      "id": "<uuid>"
    }
  }
}
```

**Error responses:**
- **Status 2:** Bad credentials (message in `error.message`)
- **Status 4:** Privacy policy acceptance required — user must first log in via the official LibreLink Up app to accept updated terms

### Token Usage

All subsequent requests use the token as a Bearer token:
```
Authorization: Bearer <token>
```

### Account ID Hashing

The `account-id` header is the **SHA-256 hex digest** of the user's UUID:
```
account-id: sha256(user.id)
```

This is sent on all authenticated requests to connections/graph endpoints.

---

## Regional Redirect

When calling endpoints on the global API, the server may return a redirect response instead of data:

```json
{
  "status": 0,
  "data": {
    "redirect": true,
    "region": "eu"
  }
}
```

**Client behavior:** When a redirect is received, the client must:
1. Update the base URL to `https://api-{region}.libreview.io`
2. Re-authenticate against the new regional endpoint
3. Retry the original request

This library handles this automatically in `get_connections()` when credentials are available. If using `from_token()` without credentials, a redirect will produce an error instructing the caller to set the region explicitly.

---

## Endpoints

### 1. Get Connections

**`GET /llu/connections`**

Returns the list of patients linked to this LibreLink Up account. Each connection includes the **latest glucose measurement** inline.

**Headers:**
```
User-Agent: Mozilla/5.0 (iPhone; CPU OS 17_4.1 like Mac OS X) ...
version: 4.16.0
product: llu.ios
Content-Type: application/json;charset=UTF-8
account-id: <sha256-hex-of-user-id>
Authorization: Bearer <token>
```

**Response:**
```json
{
  "status": 0,
  "data": [
    {
      "id": "<connection-id>",
      "patientId": "<patient-id>",
      "country": "DE",
      "status": 2,
      "firstName": "John",
      "lastName": "Doe",
      "targetLow": 70,
      "targetHigh": 180,
      "glucoseMeasurement": {
        "FactoryTimestamp": "3/5/2026 2:15:00 PM",
        "Timestamp": "3/5/2026 3:15:00 PM",
        "type": 1,
        "ValueInMgPerDl": 112,
        "MeasurementColor": 1,
        "GlucoseUnits": 0,
        "Value": 6.2,
        "isHigh": false,
        "isLow": false
      }
    }
  ]
}
```

### 2. Get Connection Graph

**`GET /llu/connections/{patientId}/graph`**

Returns recent glucose graph data (typically last 12-24 hours of readings) for a specific patient.

**Headers:** Same as Get Connections.

**Response:**
```json
{
  "status": 0,
  "data": {
    "connection": { /* same Connection object */ },
    "activeSensors": [
      {
        "sensor": {
          "deviceId": "...",
          "sn": "MH00ABCDEF",
          "a": 1709654400,
          "w": 20160,
          "pt": 4,
          "s": true,
          "lj": false
        },
        "device": {
          "did": "...",
          "dtid": 40068,
          "v": "3.10.0",
          "ll": 70,
          "hl": 240,
          "fixedLowAlarmValues": { "mgdl": 70, "mmoll": 3.9 },
          "alarms": true,
          "fixedLowThreshold": 70
        }
      }
    ],
    "graphData": [
      {
        "FactoryTimestamp": "3/5/2026 1:00:00 PM",
        "Timestamp": "3/5/2026 2:00:00 PM",
        "type": 0,
        "ValueInMgPerDl": 105,
        "MeasurementColor": 1,
        "GlucoseUnits": 0,
        "Value": 5.8,
        "isHigh": false,
        "isLow": false
      }
    ]
  },
  "ticket": {
    "token": "<refreshed-token>",
    "expires": 1234567890,
    "duration": 86400
  }
}
```

**Note:** The response includes a `ticket` with a refreshed auth token that can be used for subsequent requests.

### 3. Get Logbook

**`GET /llu/connections/{connectionId}/logbook`**

Returns the logbook entries for a connection — a historical list of glucose readings (typically more extensive than graph data).

**Headers:**
```
version: 4.16.0
product: llu.android
User-Agent: Apidog/1.0.0 (https://apidog.com)
Authorization: Bearer <token>
```

**Response:**
```json
{
  "status": 0,
  "data": [
    {
      "FactoryTimestamp": "...",
      "Timestamp": "...",
      "type": 0,
      "ValueInMgPerDl": 98,
      "MeasurementColor": 1,
      "GlucoseUnits": 0,
      "Value": 5.4,
      "isHigh": false,
      "isLow": false
    }
  ],
  "ticket": { "token": "...", "expires": 0, "duration": 0 }
}
```

### 4. Get Glucose History

**`GET /glucoseHistory?numPeriods={n}&period={p}`**

Returns aggregated glucose history with statistical blocks (percentiles). This endpoint provides long-term trend data rather than individual readings.

**Headers:** Same as Get Connections (iOS user-agent, `account-id` header).

**Query parameters:**
- `numPeriods` — Number of time periods to retrieve
- `period` — Period type/duration identifier

**Response:**
```json
{
  "status": 0,
  "data": {
    "lastUpload": 1709654400,
    "lastUploadCGM": 1709654400,
    "lastUploadPro": 0,
    "reminderSent": 0,
    "devices": [],
    "periods": [
      {
        "dateStart": 1707062400,
        "dateEnd": 1709654400,
        "noData": false,
        "dataType": "cgm",
        "avgGlucose": 115,
        "serialNumber": "MH00ABCDEF",
        "deviceId": "...",
        "deviceType": 40068,
        "hypoEvents": 2,
        "avgTestsPerDay": 96,
        "daysOfData": 14,
        "data": {
          "maxGlucoseRange": 250,
          "minGlucoseRange": 40,
          "maxGlucoseValue": 220,
          "blocks": [
            [
              {
                "time": 0,
                "percentile5": 72.0,
                "percentile25": 88.0,
                "percentile50": 105.0,
                "percentile75": 128.0,
                "percentile95": 175.0
              }
            ]
          ]
        }
      }
    ]
  },
  "ticket": { "token": "...", "expires": 0, "duration": 0 }
}
```

---

## Request Headers Summary

The client uses two distinct header profiles depending on the endpoint:

### iOS Profile (connections, graph, glucose history)
```
User-Agent: Mozilla/5.0 (iPhone; CPU OS 17_4.1 like Mac OS X) AppleWebKit/536.26 (KHTML, like Gecko) Version/17.4.1 Mobile/10A5355d Safari/8536.25
version: 4.16.0
product: llu.ios
Content-Type: application/json;charset=UTF-8
account-id: <sha256(user_id)>
Authorization: Bearer <token>
```

### Android Profile (login, logbook)
```
User-Agent: Apidog/1.0.0 (https://apidog.com)
version: 4.16.0
product: llu.android
Authorization: Bearer <token>
```

**Note:** The inconsistency between iOS and Android profiles is present in the original library. Both work, but standardizing on one profile would be cleaner.

---

## Glucose Data Format

All glucose readings share the `GraphData` structure:

| Field | Type | Description |
|-------|------|-------------|
| `FactoryTimestamp` | String | Sensor-local timestamp (factory time zone) |
| `Timestamp` | String | User-local timestamp |
| `type` | int | Reading type (0 = automatic/CGM, 1 = manual/scan) |
| `ValueInMgPerDl` | int | Glucose value in mg/dL |
| `Value` | float | Glucose value in mmol/L |
| `MeasurementColor` | int | Color code (1 = green/normal, 2 = yellow, 3 = orange, 4 = red) |
| `GlucoseUnits` | int | Unit preference (0 = mg/dL, 1 = mmol/L) |
| `isHigh` | bool | Above target range |
| `isLow` | bool | Below target range |

---

## Authentication Flow Diagram

```
┌─────────┐     POST /llu/auth/login      ┌──────────────────┐
│  Client  │ ────────────────────────────▶ │ api.libreview.io │
│          │ ◀──────────────────────────── │    (global)      │
│          │   { token, user.id }          └──────────────────┘
│          │
│          │     GET /llu/connections
│          │ ────────────────────────────▶  (global or regional)
│          │ ◀────────────────────────────
│          │   { redirect: true, region: "eu" }  ← possible redirect
│          │
│          │  (if redirected)
│          │     POST /llu/auth/login
│          │ ────────────────────────────▶ ┌────────────────────────┐
│          │ ◀──────────────────────────── │ api-eu.libreview.io    │
│          │   { new token }               │    (regional)          │
│          │                               └────────────────────────┘
│          │     GET /llu/connections
│          │ ────────────────────────────▶  (regional)
│          │ ◀────────────────────────────
│          │   { data: [...connections] }
└─────────┘
```

---

## Security Notes

- Tokens have an `expires` timestamp and `duration` (typically 86400s / 24h)
- The `ticket` field in graph/logbook/history responses provides a refreshed token
- Account IDs are SHA-256 hashed before being sent as headers (privacy measure)
- The API requires a valid `version` and `product` header — requests without these are rejected
- Privacy policy status 4 errors require interactive acceptance via the official app
