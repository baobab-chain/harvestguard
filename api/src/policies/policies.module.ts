import { Module } from '@nestjs/common';
import { PoliciesController } from './policies.controller';
import { PoliciesService } from './policies.service';
import { SorobanClientService } from '../common/soroban-client.service';

@Module({
  controllers: [PoliciesController],
  providers: [PoliciesService, SorobanClientService],
})
export class PoliciesModule {}
