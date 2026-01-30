import { Test, TestingModule } from '@nestjs/testing';
import { SpinGameService } from './spin-game.service';

describe('SpinGameService', () => {
  let service: SpinGameService;

  beforeEach(async () => {
    const module: TestingModule = await Test.createTestingModule({
      providers: [SpinGameService],
    }).compile();

    service = module.get<SpinGameService>(SpinGameService);
  });

  it('should be defined', () => {
    expect(service).toBeDefined();
  });
});
