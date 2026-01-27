import {
  Controller,
  Get,
  HttpCode,
  HttpStatus,
  HttpException,
} from '@nestjs/common';
import { HealthService } from './health.service';
import { HealthResponseDto } from './dto/health-response.dto';

@Controller('health')
export class HealthController {
  constructor(private readonly healthService: HealthService) {}

  /**
   * Health check endpoint
   * Returns the health status of the application and its dependencies
   *
   * @returns {HealthResponseDto} Health status with individual service checks
   * @status 200 - Application is healthy
   * @status 503 - Application is unhealthy (one or more services are down)
   */
  @Get()
  async checkHealth(): Promise<HealthResponseDto> {
    const health = await this.healthService.checkHealth();

    // Return 503 if unhealthy, 200 if healthy
    if (health.status === 'unhealthy') {
      throw new HttpException(health, HttpStatus.SERVICE_UNAVAILABLE);
    }

    return health;
  }
}
