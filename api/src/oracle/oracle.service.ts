import { Injectable } from '@nestjs/common';
import { nativeToScVal } from '@stellar/stellar-sdk';
import { SorobanClientService } from '../common/soroban-client.service';

@Injectable()
export class OracleService {
  constructor(private readonly soroban: SorobanClientService) {}

  async registerProvider(adminAddress: string, providerAddress: string) {
    return this.soroban.invoke(this.soroban.oracleContractId, 'register_provider', [
      nativeToScVal(adminAddress, { type: 'address' }),
      nativeToScVal(providerAddress, { type: 'address' }),
    ]);
  }

  async getRainfall(location: string) {
    return this.soroban.invoke(this.soroban.oracleContractId, 'get_rainfall', [
      nativeToScVal(location, { type: 'symbol' }),
    ]);
  }
}
