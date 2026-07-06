import { Body, Controller, Get, Param, ParseIntPipe, Post } from '@nestjs/common';
import { PoliciesService } from './policies.service';
import { CreatePolicyDto, DepositLiquidityDto } from './dto/policies.dto';

@Controller('policies')
export class PoliciesController {
  constructor(private readonly policiesService: PoliciesService) {}

  @Post('liquidity')
  depositLiquidity(@Body() dto: DepositLiquidityDto) {
    return this.policiesService.depositLiquidity(dto.insurerAddress, dto.tokenContractId, dto.amount);
  }

  @Post()
  create(@Body() dto: CreatePolicyDto) {
    return this.policiesService.createPolicy(
      dto.farmerAddress,
      dto.tokenContractId,
      dto.premium,
      dto.payout,
      dto.location,
      dto.thresholdMm,
      dto.comparison,
      dto.expiryLedger,
    );
  }

  @Get(':id')
  findOne(@Param('id', ParseIntPipe) id: number) {
    return this.policiesService.getPolicy(id);
  }

  @Post(':id/check-and-trigger')
  checkAndTrigger(@Param('id', ParseIntPipe) id: number) {
    return this.policiesService.checkAndTriggerPayout(id);
  }
}
