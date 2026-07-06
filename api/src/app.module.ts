import { Module } from '@nestjs/common';
import { ConfigModule } from '@nestjs/config';
import { PoliciesModule } from './policies/policies.module';
import { OracleModule } from './oracle/oracle.module';

@Module({
  imports: [
    ConfigModule.forRoot({ isGlobal: true }),
    PoliciesModule,
    OracleModule,
  ],
})
export class AppModule {}
