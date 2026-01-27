/**
 * Health check response DTO
 * Contains health status information without exposing sensitive data
 */
export class HealthResponseDto {
  status: 'healthy' | 'unhealthy';
  timestamp: string;
  checks: {
    database: HealthCheckResult;
    cache: HealthCheckResult;
    blockchain: HealthCheckResult;
  };
}

export class HealthCheckResult {
  status: 'up' | 'down';
  message?: string;
  responseTime?: number; // in milliseconds
}
