import { MikroOrmModule } from "@mikro-orm/nestjs";
import { Module } from "@nestjs/common";

import { BudgetController } from "./budget.controller";
import { BudgetService } from "./budget.service";
import { Budget } from "./entities/budget.entity";

@Module({
  imports: [MikroOrmModule.forFeature([Budget])],
  controllers: [BudgetController],
  providers: [BudgetService],
})
export class BudgetModule {}
