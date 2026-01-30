import { Module } from '@nestjs/common';
import { SpinGameService } from './spin-game.service';
import { SpinGameController } from './spin-game.controller';

@Module({
  controllers: [SpinGameController],
  providers: [SpinGameService],
})
export class SpinGameModule {}
