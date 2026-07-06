import { Module } from '@nestjs/common';
import { OracleController } from './oracle.controller';
import { OracleService } from './oracle.service';
import { SorobanClientService } from '../common/soroban-client.service';

@Module({
  controllers: [OracleController],
  providers: [OracleService, SorobanClientService],
})
export class OracleModule {}
