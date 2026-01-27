# Health Check Module

This module provides health and readiness probes for the Renaissance API, essential for production deployments and monitoring.

## Endpoint

- **URL**: `/api/v1/health`
- **Method**: `GET`
- **Authentication**: Not required (public endpoint)
- **Rate Limiting**: Subject to global throttler settings

## Response Format

### Healthy Response (HTTP 200)

```json
{
  "status": "healthy",
  "timestamp": "2024-01-26T12:00:00.000Z",
  "checks": {
    "database": {
      "status": "up",
      "message": "Database connection successful",
      "responseTime": 5
    },
    "cache": {
      "status": "up",
      "message": "Cache connection successful",
      "responseTime": 2
    },
    "blockchain": {
      "status": "up",
      "message": "Blockchain service available",
      "responseTime": 150
    }
  }
}
```

### Unhealthy Response (HTTP 503)

```json
{
  "status": "unhealthy",
  "timestamp": "2024-01-26T12:00:00.000Z",
  "checks": {
    "database": {
      "status": "down",
      "message": "Database connection failed",
      "responseTime": 5000
    },
    "cache": {
      "status": "up",
      "message": "Cache connection successful",
      "responseTime": 2
    },
    "blockchain": {
      "status": "down",
      "message": "Blockchain RPC endpoint unreachable",
      "responseTime": 3000
    }
  }
}
```

## Health Checks

### 1. Database Connectivity
- Performs a simple `SELECT 1` query to verify PostgreSQL connection
- Measures response time
- Returns `down` if connection fails or times out

### 2. Cache Connectivity
- Checks if cache is enabled (if disabled, returns `up` with message)
- Performs test set/get/delete operations
- Supports both Redis and in-memory cache
- Returns `down` if cache operations fail

### 3. Blockchain Service Availability
- Verifies blockchain configuration exists
- Tests RPC endpoint connectivity by fetching latest ledger
- Returns `down` if configuration is missing or RPC is unreachable

## Status Codes

- **200 OK**: All services are healthy
- **503 Service Unavailable**: One or more services are down

## Security

- **No sensitive information exposed**: The endpoint only returns status information, no credentials, connection strings, or internal details
- **Public access**: Intentionally public for monitoring tools and load balancers
- **Rate limited**: Subject to global throttler configuration

## Usage Examples

### Kubernetes Liveness Probe

```yaml
livenessProbe:
  httpGet:
    path: /api/v1/health
    port: 3000
  initialDelaySeconds: 30
  periodSeconds: 10
  timeoutSeconds: 5
  failureThreshold: 3
```

### Kubernetes Readiness Probe

```yaml
readinessProbe:
  httpGet:
    path: /api/v1/health
    port: 3000
  initialDelaySeconds: 10
  periodSeconds: 5
  timeoutSeconds: 3
  failureThreshold: 2
```

### Docker Healthcheck

```dockerfile
HEALTHCHECK --interval=30s --timeout=3s --start-period=40s --retries=3 \
  CMD curl -f http://localhost:3000/api/v1/health || exit 1
```

### Monitoring Tools

```bash
# Check health
curl http://localhost:3000/api/v1/health

# Check health with jq for formatted output
curl -s http://localhost:3000/api/v1/health | jq .
```

## Implementation Details

- **Module**: `HealthModule`
- **Service**: `HealthService` - Performs health checks
- **Controller**: `HealthController` - Exposes `/health` endpoint
- **DTO**: `HealthResponseDto` - Response structure

## Notes

- Response times are measured in milliseconds
- Cache check is skipped if cache is disabled (returns `up`)
- Blockchain check verifies configuration and RPC connectivity
- All checks are performed asynchronously and in parallel where possible
- Errors are logged but not exposed in the response (only status and message)
