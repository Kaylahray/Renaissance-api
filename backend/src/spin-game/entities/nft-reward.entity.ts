import { 
  Entity, 
  PrimaryGeneratedColumn, 
  Column, 
  CreateDateColumn,
  ManyToOne,
  JoinColumn,
  Index,
  Unique
} from 'typeorm';
import { User } from '../../users/entities/user.entity';

export enum NFTTier {
  COMMON = 'COMMON',
  RARE = 'RARE',
  EPIC = 'EPIC',
  LEGENDARY = 'LEGENDARY',
}

@Entity('nft_rewards')
@Unique(['nftContractAddress', 'nftId'])
export class NFTReward {
  @PrimaryGeneratedColumn('uuid')
  id: string;

  @Column({ name: 'user_id', type: 'varchar', length: 56 })
  userId: string;

  @ManyToOne(() => User)
  @JoinColumn({ name: 'user_id' })
  user: User;

  @Column({ name: 'nft_contract_address', type: 'varchar', length: 56 })
  nftContractAddress: string;

  @Column({ name: 'nft_id', type: 'varchar' })
  nftId: string;

  @Column({ name: 'metadata_uri', type: 'varchar', nullable: true })
  metadataUri: string | null;

  @Column({ 
    type: 'enum', 
    enum: NFTTier,
    default: NFTTier.COMMON
  })
  tier: NFTTier;

  @Column({ name: 'is_minted', type: 'boolean', default: false })
  isMinted: boolean;

  @Column({ name: 'mint_transaction_hash', type: 'varchar', nullable: true })
  mintTransactionHash: string | null;

  @Column({ name: 'spin_game_id', type: 'varchar', nullable: true })
  spinGameId: string | null;

  @Column({ name: 'claimed_at', type: 'timestamp', nullable: true })
  claimedAt: Date | null;

  @CreateDateColumn({ name: 'created_at' })
  createdAt: Date;

  @Column({ name: 'is_withdrawable', type: 'boolean', default: false })
  isWithdrawable: boolean;

  @Index()
  @Column({ name: 'rarity_score', type: 'int', nullable: true })
  rarityScore: number | null;
}