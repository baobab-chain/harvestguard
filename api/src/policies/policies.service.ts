import { Injectable } from '@nestjs/common';
import { nativeToScVal } from '@stellar/stellar-sdk';
import { SorobanClientService } from '../common/soroban-client.service';

@Injectable()
export class PoliciesService {
  constructor(private readonly soroban: SorobanClientService) {}

  async depositLiquidity(insurerAddress: string, tokenContractId: string, amount: number) {
    return this.soroban.invoke(this.soroban.harvestguardContractId, 'deposit_liquidity', [
      nativeToScVal(insurerAddress, { type: 'address' }),
      nativeToScVal(tokenContractId, { type: 'address' }),
      nativeToScVal(amount, { type: 'i128' }),
    ]);
  }

  async createPolicy(
    farmerAddress: string,
    tokenContractId: string,
    premium: number,
    payout: number,
    location: string,
    thresholdMm: number,
    comparison: 'Below' | 'Above',
    expiryLedger: number,
  ) {
    // NOTE: the contract's `Comparison` type is a fieldless Rust enum
    // (Below | Above). Encoding it as a {tag, values} instance is the
    // standard convention for Soroban contracttype enums in
    // @stellar/stellar-sdk, but double-check this against whatever SDK
    // version ends up pinned — enum encoding conventions have shifted
    // across major versions before. See ISSUES.md.
    const comparisonScVal = nativeToScVal({ tag: comparison, values: undefined }, { type: 'instance' });

    return this.soroban.invoke(this.soroban.harvestguardContractId, 'create_policy', [
      nativeToScVal(farmerAddress, { type: 'address' }),
      nativeToScVal(tokenContractId, { type: 'address' }),
      nativeToScVal(premium, { type: 'i128' }),
      nativeToScVal(payout, { type: 'i128' }),
      nativeToScVal(location, { type: 'symbol' }),
      nativeToScVal(thresholdMm, { type: 'i128' }),
      comparisonScVal,
      nativeToScVal(expiryLedger, { type: 'u32' }),
    ]);
  }

  async checkAndTriggerPayout(policyId: number) {
    return this.soroban.invoke(this.soroban.harvestguardContractId, 'check_and_trigger_payout', [
      nativeToScVal(policyId, { type: 'u32' }),
    ]);
  }

  async getPolicy(policyId: number) {
    return this.soroban.invoke(this.soroban.harvestguardContractId, 'get_policy', [
      nativeToScVal(policyId, { type: 'u32' }),
    ]);
  }
}
