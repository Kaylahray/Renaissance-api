import { Module } from '@nestjs/common';
import { TypeOrmModule } from '@nestjs/typeorm';
import { LeaderboardStats } from './entities/leaderboard-stats.entity';
import { LeaderboardService } from './leaderboard.service';

@Module({
    imports: [TypeOrmModule.forFeature([LeaderboardStats])],
    providers: [LeaderboardService],
    exports: [LeaderboardService],
})
export class LeaderboardModule { }
