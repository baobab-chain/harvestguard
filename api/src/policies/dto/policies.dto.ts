import { IsIn, IsInt, IsPositive, IsString } from 'class-validator';

export class CreatePolicyDto {
  @IsString()
  farmerAddress: string;

  @IsString()
  tokenContractId: string;

  @IsInt()
  @IsPositive()
  premium: number;

  @IsInt()
  @IsPositive()
  payout: number;

  @IsString()
  location: string;

  @IsInt()
  thresholdMm: number;

  @IsIn(['Below', 'Above'])
  comparison: 'Below' | 'Above';

  @IsInt()
  @IsPositive()
  expiryLedger: number;
}

export class DepositLiquidityDto {
  @IsString()
  insurerAddress: string;

  @IsString()
  tokenContractId: string;

  @IsInt()
  @IsPositive()
  amount: number;
}
