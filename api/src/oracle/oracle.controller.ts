import { Body, Controller, Get, Param, Post } from '@nestjs/common';
import { OracleService } from './oracle.service';
import { RegisterProviderDto } from './dto/oracle.dto';

@Controller('oracle')
export class OracleController {
  constructor(private readonly oracleService: OracleService) {}

  @Post('providers')
  registerProvider(@Body() dto: RegisterProviderDto) {
    return this.oracleService.registerProvider(dto.adminAddress, dto.providerAddress);
  }

  @Get(':location')
  getRainfall(@Param('location') location: string) {
    return this.oracleService.getRainfall(location);
  }
}
