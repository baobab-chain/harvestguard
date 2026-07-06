import { IsString } from 'class-validator';

export class RegisterProviderDto {
  @IsString()
  adminAddress: string;

  @IsString()
  providerAddress: string;
}
