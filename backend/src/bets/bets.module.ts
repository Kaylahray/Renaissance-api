import { Module } from '@nestjs/common';
import { TypeOrmModule } from '@nestjs/typeorm';
import { BetsController } from './bets.controller';
import { BetsService } from './bets.service';
import { Bet } from './entities/bet.entity';
import { Match } from '../matches/entities/match.entity';
import { FreeBetVouchersModule } from '../free-bet-vouchers/free-bet-vouchers.module';
import { SpinModule } from 'src/spin/spin.module';

@Module({
  imports: [
    TypeOrmModule.forFeature([Bet, Match]),
    FreeBetVouchersModule,
    SpinModule,
  ],
  controllers: [BetsController],
  providers: [BetsService],
  exports: [BetsService],
})
export class BetsModule {}
