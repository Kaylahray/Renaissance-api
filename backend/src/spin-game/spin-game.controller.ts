import { Module } from '@nestjs/common';
import { TypeOrmModule } from '@nestjs/typeorm';
import { ConfigModule } from '@nestjs/config';
import { JwtModule } from '@nestjs/jwt';
import { SpinGameController } from './controllers/spin-game.controller';
import { SpinGameService } from './services/spin-game.service';
import { SpinGameRepository } from './repositories/spin-game.repository';
import { 
  SpinGame, 
  UserSpinStats, 
  FreeBetReward, 
  NFTReward 
} from './entities';

@Module({
  imports: [
    TypeOrmModule.forFeature([
      SpinGame,
      UserSpinStats,
      FreeBetReward,
      NFTReward,
    ]),
    ConfigModule,
    JwtModule,
  ],
  controllers: [SpinGameController],
  providers: [SpinGameService, SpinGameRepository],
  exports: [SpinGameService],
})
export class SpinGameModule {}